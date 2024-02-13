use std::path::PathBuf;

use voicevox_core_rs as vv;
use clap::{Parser, ValueEnum};

/// モード。
#[derive(Debug, Clone, Default, ValueEnum, PartialEq)]
enum Mode {
    /// 自動選択。
    #[default]
    Auto,
    /// CPU。
    Cpu,
    /// GPU。
    Gpu,
}

/// voicevox_core-rsのサンプル。
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// モード。
    #[clap(short, long, default_value = "auto")]
    mode: Mode,

    /// vvmファイルへのパス。
    #[clap(short, long)]
    vvm: PathBuf,

    /// Open JTalkの辞書ディレクトリへのパス。
    #[clap(short, long)]
    dict: PathBuf,

    /// 読み上げさせたい文章。
    #[clap(short, long, default_value = "この音声は、ボイスボックスを使用して、出力されています。")]
    text: String,

    /// 出力wavファイルへのパス。
    #[clap(short, long, default_value = "./output.wav")]
    out: PathBuf,

    /// 話者ID。
    #[clap(short, long, default_value = "0")]
    speaker_id: u32,
}

fn main() {
    let args = Args::parse();

    println!("Open Jtalkを初期化中...");
    let open_jtalk = vv::OpenJtalkRc::new(&args.dict).unwrap();

    println!("Synthesizerを初期化中...");
    let synthesizer = vv::Synthesizer::new(&open_jtalk, Default::default()).unwrap();

    println!("VVMを読み込み中...");
    let voice_model = vv::VoiceModel::from_path(&args.vvm).unwrap();
    synthesizer.load_voice_model(&voice_model).unwrap();

    println!("音声を合成中...");
    let wav = synthesizer
        .tts(&args.text, args.speaker_id, Default::default())
        .unwrap();

    println!("wavファイルを書き込み中...");
    std::fs::write(&args.out, wav).unwrap();

    println!("書き込み完了：{}", args.out.display());
}
