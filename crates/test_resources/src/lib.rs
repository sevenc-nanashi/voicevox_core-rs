pub fn get_dict_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("downloaded")
        .join("dict")
        .join("open_jtalk_dic_utf_8-1.11")
}

pub fn get_vvm_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("downloaded")
        .join("0.vvm")
}
