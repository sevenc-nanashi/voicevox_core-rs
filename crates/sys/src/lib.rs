mod generated;
pub use generated::*;

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;

    #[test]
    fn test() {
        unsafe { CStr::from_ptr(voicevox_get_version()).to_str().unwrap() };
    }
}
