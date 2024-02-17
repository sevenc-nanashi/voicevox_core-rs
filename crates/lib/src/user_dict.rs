use crate::{i32_to_result, Result};
use std::{
    collections::{BTreeMap, HashMap},
    ffi::{CStr, CString},
    mem::MaybeUninit,
};
use voicevox_core_sys as sys;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub use uuid::Uuid;

/// ユーザー辞書の単語の種類。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserDictWordType {
    /// 固有名詞。
    ProperNoun,
    /// 一般名詞。
    CommonNoun,
    /// 動詞。
    Verb,
    /// 形容詞。
    Adjective,
    /// 接尾辞。
    Suffix,
}

/// ユーザー辞書の単語。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDictWord {
    /// 表記。
    pub surface: String,
    /// 読み。
    pub pronunciation: String,
    /// アクセント型。
    pub accent_type: usize,
    /// 単語の種類。
    pub word_type: UserDictWordType,
    /// 優先度。
    pub priority: u32,
}

impl UserDictWord {
    pub fn new(surface: &str, pronunciation: &str) -> Self {
        let surface = CString::new(surface).unwrap();
        let pronunciation = CString::new(pronunciation).unwrap();
        let word =
            unsafe { sys::voicevox_user_dict_word_make(surface.as_ptr(), pronunciation.as_ptr()) };

        word.into()
    }
}

/// ユーザー辞書。
pub struct UserDict {
    pub(crate) inner: *mut sys::VoicevoxUserDict,
}

impl UserDict {
    /// ユーザー辞書を構築する。
    pub fn new() -> Result<Self> {
        let inner = unsafe { sys::voicevox_user_dict_new() };

        Ok(Self { inner })
    }

    /// 他のユーザー辞書を読み込む。
    pub fn import(&self, other: &UserDict) -> Result<()> {
        i32_to_result(unsafe { sys::voicevox_user_dict_import(self.inner, other.inner) })
    }

    /// ユーザー辞書をファイルに保存する。
    pub fn save<S: AsRef<std::path::Path>>(&self, path: S) -> Result<()> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        i32_to_result(unsafe { sys::voicevox_user_dict_save(self.inner, path.as_ptr()) })
    }

    /// ユーザー辞書に単語を追加する。
    pub fn add_word(&self, word: UserDictWord) -> Result<Uuid> {
        let mut word_uuid = [0u8; 16];
        i32_to_result(unsafe {
            sys::voicevox_user_dict_add_word(self.inner, &word.into(), &mut word_uuid)
        })?;

        Ok(uuid::Uuid::from_slice(&word_uuid).unwrap())
    }

    /// ユーザー辞書から単語を削除する。
    pub fn remove_word(&self, word_uuid: &Uuid) -> Result<()> {
        i32_to_result(unsafe {
            sys::voicevox_user_dict_remove_word(self.inner, word_uuid.as_bytes().as_ptr() as _)
        })
    }

    /// ユーザー辞書の単語を更新する。
    pub fn update_word(&self, word_uuid: Uuid, word: UserDictWord) -> Result<()> {
        i32_to_result(unsafe {
            sys::voicevox_user_dict_update_word(
                self.inner,
                word_uuid.as_bytes().as_ptr() as _,
                &word.into(),
            )
        })
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        let json_ptr = unsafe {
            let mut ptr = MaybeUninit::uninit();
            i32_to_result(sys::voicevox_user_dict_to_json(
                self.inner,
                ptr.as_mut_ptr(),
            ))?;

            ptr.assume_init()
        };
        let json = unsafe { CStr::from_ptr(json_ptr) }
            .to_str()
            .unwrap()
            .to_string();
        unsafe { sys::voicevox_json_free(json_ptr as _) };

        Ok(json)
    }
}

impl Serialize for UserDict {
    fn serialize<S: serde::ser::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        let json = self.to_json().unwrap();
        let map: HashMap<Uuid, UserDictWord> = serde_json::from_str(&json).unwrap();
        map.serialize(serializer)
    }
}

macro_rules! into_map {
    ($t:ident) => {
        impl From<UserDict> for $t<Uuid, UserDictWord> {
            fn from(user_dict: UserDict) -> Self {
                let json = user_dict.to_json().unwrap();
                serde_json::from_str(&json).unwrap()
            }
        }
    };
}
into_map!(HashMap);
into_map!(BTreeMap);
into_map!(IndexMap);

impl Drop for UserDict {
    fn drop(&mut self) {
        unsafe {
            sys::voicevox_user_dict_delete(self.inner);
        }
    }
}

impl From<sys::VoicevoxUserDictWord> for UserDictWord {
    fn from(word: sys::VoicevoxUserDictWord) -> Self {
        let surface = unsafe { CStr::from_ptr(word.surface) }
            .to_str()
            .unwrap()
            .to_string();
        let pronunciation = unsafe { CStr::from_ptr(word.pronunciation) }
            .to_str()
            .unwrap()
            .to_string();
        let word_type = word.word_type.into();
        let accent_type = word.accent_type;
        let priority = word.priority;

        Self {
            surface,
            pronunciation,
            accent_type,
            word_type,
            priority,
        }
    }
}

impl From<UserDictWord> for sys::VoicevoxUserDictWord {
    fn from(word: UserDictWord) -> Self {
        let surface = CString::new(word.surface).unwrap();
        let pronunciation = CString::new(word.pronunciation).unwrap();

        Self {
            surface: surface.into_raw(),
            pronunciation: pronunciation.into_raw(),
            accent_type: word.accent_type,
            word_type: word.word_type.into(),
            priority: word.priority,
        }
    }
}

impl From<sys::VoicevoxUserDictWordType> for UserDictWordType {
    fn from(word_type: sys::VoicevoxUserDictWordType) -> Self {
        match word_type {
            sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_PROPER_NOUN => {
                Self::ProperNoun
            }
            sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_COMMON_NOUN => {
                Self::CommonNoun
            }
            sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_VERB => Self::Verb,
            sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_ADJECTIVE => Self::Adjective,
            sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_SUFFIX => Self::Suffix,
            _ => unreachable!(),
        }
    }
}

impl From<UserDictWordType> for sys::VoicevoxUserDictWordType {
    fn from(word_type: UserDictWordType) -> Self {
        match word_type {
            UserDictWordType::ProperNoun => {
                sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_PROPER_NOUN
            }
            UserDictWordType::CommonNoun => {
                sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_COMMON_NOUN
            }
            UserDictWordType::Verb => {
                sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_VERB
            }
            UserDictWordType::Adjective => {
                sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_ADJECTIVE
            }
            UserDictWordType::Suffix => {
                sys::VoicevoxUserDictWordType_VOICEVOX_USER_DICT_WORD_TYPE_SUFFIX
            }
        }
    }
}
