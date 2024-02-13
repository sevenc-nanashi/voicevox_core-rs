pub(crate) use semver::Version;
use serde::Deserialize;

fn get_dest_path() -> std::path::PathBuf {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target = std::env::var("TARGET").unwrap();
    let target = VvHost::from_triplet(&target).unwrap();
    std::path::Path::new(&out_dir).join(format!(
        "voicevox_core-{}-{}",
        target.to_string(),
        target.device()
    ))
}

fn get_target_path() -> std::path::PathBuf {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("../../../");
    dest_path.canonicalize().unwrap()
}

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug)]
struct VvHost<'a> {
    host: &'a str,
    arch: &'a str,
}

impl VvHost<'_> {
    fn from_triplet(triplet: &str) -> Option<Self> {
        let triplet: Vec<&str> = triplet.split('-').collect();
        match (triplet[2], triplet[0]) {
            ("windows", "x86_64") => Some(Self {
                host: "windows",
                arch: "x64",
            }),
            ("windows", "i686") => Some(Self {
                host: "windows",
                arch: "x86",
            }),
            ("linux", "x86_64") => Some(Self {
                host: "linux",
                arch: "x64",
            }),
            ("linux", "aarch64") => Some(Self {
                host: "linux",
                arch: "arm64",
            }),
            ("apple", "x86_64") => Some(Self {
                host: "osx",
                arch: "x64",
            }),
            ("apple", "aarch64") => Some(Self {
                host: "osx",
                arch: "arm64",
            }),
            _ => None,
        }
    }

    fn device(&self) -> &'static str {
        match (
            cfg!(feature = "cuda"),
            cfg!(feature = "directml"),
            cfg!(feature = "gpu"),
        ) {
            (false, false, false) => "cpu",
            (true, _, _) if self.host == "windows" && self.arch == "x64" => "cuda",
            (_, true, _) if self.host == "windows" && self.arch == "x64" => "directml",
            (_, _, true) if self.host == "windows" && self.arch == "x64" => "gpu",
            (true, _, _) if self.host == "linux" && self.arch == "x64" => "cuda",
            (_, _, true) if self.host == "linux" && self.arch == "x64" => "gpu",
            _ => panic!(
                "unsupported device: {:?}, cuda={}, directml={}, gpu={}",
                self,
                cfg!(feature = "cuda"),
                cfg!(feature = "directml"),
                cfg!(feature = "gpu")
            ),
        }
    }
}

impl ToString for VvHost<'_> {
    fn to_string(&self) -> String {
        format!("{}-{}", self.host, self.arch)
    }
}

fn download() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target = VvHost::from_triplet(&std::env::var("TARGET").unwrap()).unwrap();

    let dest_path = get_dest_path();
    if dest_path.exists() {
        eprintln!("voicevox_core already exists at {:?}", dest_path);
        return;
    }

    let mut releases = ureq::get("https://api.github.com/repos/VOICEVOX/voicevox_core/releases");

    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        releases = releases.set("Authorization", &format!("token {}", token));
    }

    let releases = releases.call().unwrap().into_string().unwrap();
    let releases: Vec<Release> = serde_json::from_str(&releases).unwrap();
    // // 0.16.0以降、または0.15.0のプレリリースのみを対象にする
    // let tag_names: Vec<Version> = releases
    //     .iter()
    //     .map(|r| Version::parse(&r.tag_name).unwrap())
    //     .filter(|v| {
    //         v > &Version::parse("0.15.0").unwrap()
    //             || (v.major == 0 && v.minor == 15 && v.patch == 0 && v.pre.starts_with("preview"))
    //     })
    //     .collect();
    // let latest = tag_names.iter().max().unwrap();

    // 0.15.0-preview.16はdownloaderがないので0.15.0-preview.15を対象にする、そのうち最新のリリースを取得する
    let latest = Version::parse("0.15.0-preview.15").unwrap();
    eprintln!("Downloading voicevox_core {}", latest);
    let downloader_path = std::path::Path::new(&out_dir).join("voicevox_core_downloader");
    if !downloader_path.exists() {
        eprintln!(
            "Downloading voicevox_core downloader to {:?}",
            downloader_path
        );
        let release = releases
            .iter()
            .find(|r| Version::parse(&r.tag_name).unwrap() == latest)
            .unwrap();

        let downloader = release
            .assets
            .iter()
            .find(|a| {
                a.name
                    .starts_with(&format!("download-{}", target.to_string()))
            })
            .expect("downloader not found");

        let mut downloader_exe = ureq::get(&downloader.browser_download_url)
            .call()
            .ok()
            .unwrap()
            .into_reader();

        {
            let mut downloader_file = std::fs::File::create(&downloader_path).unwrap();
            std::io::copy(&mut downloader_exe, &mut downloader_file).unwrap();
        }
        if cfg!(unix) {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = std::fs::metadata(&downloader_path).unwrap().permissions();
            permissions.set_mode(0o755);
            std::fs::set_permissions(&downloader_path, permissions).unwrap();
        }
    }

    eprintln!("Downloading voicevox_core to {:?}", dest_path);
    duct::cmd!(
        &downloader_path,
        "--version",
        latest.to_string(),
        "--device",
        target.device(),
        "--os",
        target.host,
        "--cpu-arch",
        target.arch,
        "--output",
        &dest_path,
        "--only",
        "core",
        "--only",
        "additional-libraries"
    )
    .run()
    .unwrap();

    eprintln!("Downloaded voicevox_core to {:?}", dest_path);
}

fn copy_dll() {
    let vv_path = get_dest_path();
    if !vv_path.exists() {
        panic!("voicevox_core not found at {:?}", vv_path);
    }
    let dlls: Vec<std::path::PathBuf> = vv_path
        .read_dir()
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| {
                    s.ends_with(".dll")
                        || s.ends_with(".dylib")
                        || s.contains(".so.")
                        || s.ends_with(".so")
                })
                .unwrap_or(false)
        })
        .map(|e| e.path())
        .collect();
    let target_path = get_target_path();
    for dll in dlls {
        let dest = target_path.join(dll.file_name().unwrap());
        eprintln!("Copying {:?} to {:?}", dll, dest);
        std::fs::copy(&dll, &dest).unwrap();
    }
}

fn generate_bindings() {
    let dest_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("generated.rs");
    eprintln!("Generating bindings to {:?}", dest_path);
    let vv_path = get_dest_path();
    if !vv_path.exists() {
        panic!("voicevox_core not found at {:?}", vv_path);
    }
    let bindings = bindgen::Builder::default()
        .header(vv_path.join("voicevox_core.h").to_str().unwrap())
        .clang_arg(format!("-I{}", vv_path.to_str().unwrap()))
        .generate()
        .expect("Unable to generate bindings");
    let bindings_rs = bindings.to_string();

    std::fs::write(
        &dest_path,
        format!("#![allow(warnings, unused)]\n{}", bindings_rs),
    )
    .unwrap();
}

fn main() {
    if cfg!(feature = "download") {
        download();
    }
    if cfg!(feature = "copy-dll") {
        copy_dll();
    }
    println!(
        "cargo:rustc-link-search={}",
        get_dest_path().to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=voicevox_core");
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(feature = "generate-bindings") {
        generate_bindings();
    }
}
