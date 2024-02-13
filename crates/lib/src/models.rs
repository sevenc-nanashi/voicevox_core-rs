use serde::{Deserialize, Serialize};

/// モーラ（子音＋母音）ごとの情報。
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoraModel {
    /// 文字。
    pub text: String,
    /// 子音の音素。
    pub consonant: Option<String>,
    /// 子音の音長。
    pub consonant_length: Option<f32>,
    /// 母音の音素。
    pub vowel: String,
    /// 母音の音長。
    pub vowel_length: f32,
    /// 音高。
    pub pitch: f32,
}

/// AccentPhrase (アクセント句ごとの情報)。
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccentPhrase {
    /// モーラの配列。
    pub moras: Vec<MoraModel>,
    /// アクセント箇所。
    pub accent: usize,
    /// 後ろに無音を付けるかどうか。
    pub pause_mora: Option<MoraModel>,
    /// 疑問系かどうか。
    #[serde(default)]
    pub is_interrogative: bool,
}

/// AudioQuery (音声合成用のクエリ)。
#[derive(Clone, Deserialize, Serialize)]
pub struct AudioQuery {
    /// アクセント句の配列。
    pub accent_phrases: Vec<AccentPhrase>,
    /// 全体の話速。
    pub speed_scale: f32,
    /// 全体の音高。
    pub pitch_scale: f32,
    /// 全体の抑揚。
    pub intonation_scale: f32,
    /// 全体の音量。
    pub volume_scale: f32,
    /// 音声の前の無音時間。
    pub pre_phoneme_length: f32,
    /// 音声の後の無音時間。
    pub post_phoneme_length: f32,
    /// 音声データの出力サンプリングレート。
    pub output_sampling_rate: u32,
    /// 音声データをステレオ出力するか否か。
    pub output_stereo: bool,
    /// AquesTalk風記法。
    ///
    /// [`Synthesizer::audio_query`]が返すもののみ`Some`となる。入力としてのAudioQueryでは無視され
    /// る。
    pub kana: Option<String>,
}

/// 話者のバージョン。
pub type StyleVersion = String;

/// 話者のメタ情報。
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpeakerMeta {
    /// 話者名。
    name: String,
    /// 話者に属するスタイル。
    styles: Vec<StyleMeta>,
    /// 話者のバージョン。
    version: StyleVersion,
    /// 話者のUUID。
    speaker_uuid: String,
}

impl SpeakerMeta {
    /// 話者名を取得する。
    pub fn name(&self) -> &str {
        &self.name
    }

    /// スタイルのメタ情報を取得する。
    pub fn styles(&self) -> &[StyleMeta] {
        &self.styles
    }

    /// 話者のバージョンを取得する。
    pub fn version(&self) -> &StyleVersion {
        &self.version
    }

    /// 話者のUUIDを取得する。
    pub fn speaker_uuid(&self) -> &str {
        &self.speaker_uuid
    }
}

/// スタイルのID。
pub type StyleId = u32;

/// スタイルのメタ情報。
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StyleMeta {
    /// スタイルID。
    id: StyleId,
    /// スタイル名。
    name: String,
}

impl StyleMeta {
    /// スタイルIDを取得する。
    pub fn id(&self) -> StyleId {
        self.id
    }

    /// スタイル名を取得する。
    pub fn name(&self) -> &str {
        &self.name
    }
}
