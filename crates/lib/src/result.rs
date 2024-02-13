use thiserror::Error;

pub type Result<T> = std::result::Result<T, VoicevoxError>;

pub(crate) fn i32_to_result(val: i32) -> Result<()> {
    if val == 0 {
        Ok(())
    } else {
        Err(val.into())
    }
}

/// Voicevoxのエラー。
#[derive(Error, Debug)]
pub enum VoicevoxError {
    /// open_jtalk辞書ファイルが読み込まれていない
    #[error("open_jtalk辞書ファイルが読み込まれていない")]
    NotLoadedOpenjtalkDict = 1,

    /// サポートされているデバイス情報取得に失敗した
    #[error("サポートされているデバイス情報取得に失敗した")]
    GetSupportedDevices = 3,

    /// GPUモードがサポートされていない
    #[error("GPUモードがサポートされていない")]
    GpuSupport = 4,

    /// スタイルIDに対するスタイルが見つからなかった
    #[error("スタイルIDに対するスタイルが見つからなかった")]
    StyleNotFound = 6,

    /// 音声モデルIDに対する音声モデルが見つからなかった
    #[error("音声モデルIDに対する音声モデルが見つからなかった")]
    ModelNotFound = 7,

    /// 推論に失敗した
    #[error("推論に失敗した")]
    Inference = 8,

    /// コンテキストラベル出力に失敗した
    #[error("コンテキストラベル出力に失敗した")]
    ExtractFullContextLabel = 11,

    /// AquesTalk風記法のテキストの解析に失敗した
    #[error("AquesTalk風記法のテキストの解析に失敗した")]
    ParseKana = 13,

    /// 無効なAudioQuery
    #[error("無効なAudioQuery")]
    InvalidAudioQuery = 14,

    /// 無効なAccentPhrase
    #[error("無効なAccentPhrase")]
    InvalidAccentPhrase = 15,

    /// ZIPファイルを開くことに失敗した
    #[error("ZIPファイルを開くことに失敗した")]
    OpenZipFile = 16,

    /// ZIP内のファイルが読めなかった
    #[error("ZIP内のファイルが読めなかった")]
    ReadZipEntry = 17,

    /// すでに読み込まれている音声モデルを読み込もうとした
    #[error("すでに読み込まれている音声モデルを読み込もうとした")]
    ModelAlreadyLoaded = 18,

    /// すでに読み込まれているスタイルを読み込もうとした
    #[error("すでに読み込まれているスタイルを読み込もうとした")]
    StyleAlreadyLoaded = 26,

    /// 無効なモデルデータ
    #[error("無効なモデルデータ")]
    InvalidModelData = 27,

    /// ユーザー辞書を読み込めなかった
    #[error("ユーザー辞書を読み込めなかった")]
    LoadUserDict = 20,

    /// ユーザー辞書を書き込めなかった
    #[error("ユーザー辞書を書き込めなかった")]
    SaveUserDict = 21,

    /// ユーザー辞書に単語が見つからなかった
    #[error("ユーザー辞書に単語が見つからなかった")]
    UserDictWordNotFound = 22,

    /// OpenJTalkのユーザー辞書の設定に失敗した
    #[error("OpenJTalkのユーザー辞書の設定に失敗した")]
    UseUserDict = 23,

    /// ユーザー辞書の単語のバリデーションに失敗した
    #[error("ユーザー辞書の単語のバリデーションに失敗した")]
    InvalidUserDictWord = 24,

    /// UUIDの変換に失敗した
    #[error("UUIDの変換に失敗した")]
    InvalidUuid = 25,
}

impl From<i32> for VoicevoxError {
    fn from(err: i32) -> Self {
        match err {
            1 => VoicevoxError::NotLoadedOpenjtalkDict,
            3 => VoicevoxError::GetSupportedDevices,
            4 => VoicevoxError::GpuSupport,
            6 => VoicevoxError::StyleNotFound,
            7 => VoicevoxError::ModelNotFound,
            8 => VoicevoxError::Inference,
            11 => VoicevoxError::ExtractFullContextLabel,
            13 => VoicevoxError::ParseKana,
            14 => VoicevoxError::InvalidAudioQuery,
            15 => VoicevoxError::InvalidAccentPhrase,
            16 => VoicevoxError::OpenZipFile,
            17 => VoicevoxError::ReadZipEntry,
            18 => VoicevoxError::ModelAlreadyLoaded,
            20 => VoicevoxError::LoadUserDict,
            21 => VoicevoxError::SaveUserDict,
            22 => VoicevoxError::UserDictWordNotFound,
            23 => VoicevoxError::UseUserDict,
            24 => VoicevoxError::InvalidUserDictWord,
            25 => VoicevoxError::InvalidUuid,
            26 => VoicevoxError::StyleAlreadyLoaded,
            27 => VoicevoxError::InvalidModelData,
            _ => panic!("Unknown error code: {}", err),
        }
    }
}
