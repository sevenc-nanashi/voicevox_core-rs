use crate::{i32_to_result, Result};
use std::{ffi::CString, mem::MaybeUninit, path::Path};
use voicevox_core_sys as sys;

/// テキスト解析器としてのOpen JTalk。
pub struct OpenJtalkRc {
    pub(crate) inner: *mut sys::OpenJtalkRc,
}

impl OpenJtalkRc {
    pub fn new<S: AsRef<Path>>(dict_dir: S) -> Result<Self> {
        let dict_dir = dict_dir.as_ref();
        let dict_dir = CString::new(dict_dir.to_str().unwrap()).unwrap();

        let inner = unsafe {
            let mut ptr = MaybeUninit::uninit();
            i32_to_result(sys::voicevox_open_jtalk_rc_new(
                dict_dir.as_ptr(),
                ptr.as_mut_ptr(),
            ))?;
            ptr.assume_init()
        };

        Ok(Self { inner })
    }

    pub fn use_user_dict(&self, user_dict: &crate::UserDict) -> Result<()> {
        i32_to_result(unsafe {
            sys::voicevox_open_jtalk_rc_use_user_dict(self.inner, user_dict.inner)
        })
    }
}

impl Drop for OpenJtalkRc {
    fn drop(&mut self) {
        unsafe {
            sys::voicevox_open_jtalk_rc_delete(self.inner);
        }
    }
}
