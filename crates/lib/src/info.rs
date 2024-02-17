use crate::{i32_to_result, Result};
use std::{ffi::CStr, mem::MaybeUninit};
use voicevox_core_sys as sys;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportedFeatures {
    cpu: bool,
    cuda: bool,
    dml: bool,
}

impl SupportedFeatures {
    /// CPUが利用可能。
    ///
    /// 常に`true`。
    /// DirectMLが利用可能。
    ///
    pub fn cpu(&self) -> bool {
        self.cpu
    }

    /// CUDAが利用可能。
    ///
    /// ONNX Runtimeの[CUDA Execution Provider] (`CUDAExecutionProvider`)に対応する。必要な環境につ
    /// いてはそちらを参照。
    ///
    /// [CUDA Execution Provider]: https://onnxruntime.ai/docs/execution-providers/CUDA-ExecutionProvider.html
    pub fn cuda(&self) -> bool {
        self.cuda
    }

    /// ONNX Runtimeの[DirectML Execution Provider] (`DmlExecutionProvider`)に対応する。必要な環境に
    /// ついてはそちらを参照。
    ///
    /// [DirectML Execution Provider]: https://onnxruntime.ai/docs/execution-providers/DirectML-ExecutionProvider.html
    pub fn dml(&self) -> bool {
        self.dml
    }

    /// サポートしてる機能の一覧を取得する。
    pub fn get() -> Result<SupportedFeatures> {
        let json_ptr = unsafe {
            let mut ptr = MaybeUninit::uninit();
            i32_to_result(sys::voicevox_create_supported_devices_json(
                ptr.as_mut_ptr(),
            ))?;

            ptr.assume_init()
        };
        let json = unsafe { CStr::from_ptr(json_ptr) }
            .to_str()
            .unwrap()
            .to_string();

        Ok(serde_json::from_str(&json).unwrap())
    }
}

pub fn version() -> String {
    let version = unsafe { sys::voicevox_get_version() };
    unsafe { CStr::from_ptr(version) }
        .to_str()
        .unwrap()
        .to_string()
}
