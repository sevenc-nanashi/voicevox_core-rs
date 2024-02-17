use crate::{i32_to_result, Result, SpeakerMeta};
use std::{ffi::CString, mem::MaybeUninit, path::Path};
use voicevox_core_sys as sys;

/// 音声モデル。VVMファイルと対応する。
pub struct VoiceModel {
    pub(crate) inner: *mut sys::VoicevoxVoiceModel,
}

impl VoiceModel {
    /// 音声モデルを読み込む。
    pub fn from_path<S: AsRef<Path>>(model_path: S) -> Result<Self> {
        let model_path = model_path.as_ref();
        let model_path = CString::new(model_path.to_str().unwrap()).unwrap();

        let inner = unsafe {
            let mut ptr = MaybeUninit::uninit();
            i32_to_result(sys::voicevox_voice_model_new_from_path(
                model_path.as_ptr(),
                ptr.as_mut_ptr(),
            ))?;
            ptr.assume_init()
        };

        Ok(Self { inner })
    }

    /// メタ情報を取得する。
    pub fn metas(&self) -> Vec<SpeakerMeta> {
        let metas = unsafe { sys::voicevox_voice_model_get_metas_json(self.inner) };
        let metas_json = unsafe { std::ffi::CStr::from_ptr(metas) }
            .to_str()
            .unwrap()
            .to_string();
        serde_json::from_str(&metas_json).unwrap()
    }

    /// IDを取得する。
    pub fn id(&self) -> String {
        let id = unsafe { sys::voicevox_voice_model_id(self.inner) };
        unsafe { std::ffi::CStr::from_ptr(id) }
            .to_str()
            .unwrap()
            .to_string()
    }
}

impl Drop for VoiceModel {
    fn drop(&mut self) {
        unsafe {
            sys::voicevox_voice_model_delete(self.inner);
        }
    }
}
