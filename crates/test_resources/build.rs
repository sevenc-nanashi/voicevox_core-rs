fn get_dest_path(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("downloaded")
        .join(name)
}

fn download_dict() {
    let dest_path = get_dest_path("dict");
    if dest_path.exists() {
        return;
    }

    let tar_gz = ureq::get("https://jaist.dl.sourceforge.net/project/open-jtalk/Dictionary/open_jtalk_dic-1.11/open_jtalk_dic_utf_8-1.11.tar.gz")
    .call()
    .unwrap()
    .into_reader();

    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    std::fs::create_dir(&dest_path).unwrap();
    archive.unpack(&dest_path).unwrap();
}

fn download_vvm() {
    let dest_path = get_dest_path("0.vvm");
    if dest_path.exists() {
        return;
    }

    let mut vvm =
        ureq::get("https://github.com/VOICEVOX/voicevox_fat_resource/raw/main/core/model/0.vvm")
            .call()
            .unwrap()
            .into_reader();

    let mut vvm_file = std::fs::File::create(&dest_path).unwrap();
    std::io::copy(&mut vvm, &mut vvm_file).unwrap();
}

fn main() {
    println!("rerun-if-changed=build.rs");
    download_dict();
    download_vvm();
}
