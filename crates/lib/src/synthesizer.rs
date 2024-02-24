use crate::{i32_to_result, AccentPhrase, AudioQuery, Result, SpeakerMeta, StyleId, VoiceModel};
use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
};
use voicevox_core_sys as sys;

/// 音声シンセサイザ。
pub struct Synthesizer {
    pub(crate) inner: *mut sys::VoicevoxSynthesizer,
}

/// ハードウェアアクセラレーションモード。
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccelerationMode {
    /// 実行環境に合った適切なハードウェアアクセラレーションモードを選択する。
    #[default]
    Auto,
    /// ハードウェアアクセラレーションモードを"CPU"に設定する。
    Cpu,
    /// ハードウェアアクセラレーションモードを"GPU"に設定する。
    Gpu,
}

/// [`Synthesizer::new`]のオプション。
#[derive(Debug, Clone, Copy)]
pub struct InitializeOptions {
    pub acceleration_mode: AccelerationMode,
    pub cpu_num_threads: u16,
}

impl Default for InitializeOptions {
    fn default() -> Self {
        unsafe { sys::voicevox_make_default_initialize_options() }.into()
    }
}

/// [`Synthesizer::synthesis`]のオプション。
#[derive(Debug, Clone, Copy)]
pub struct SynthesisOptions {
    pub enable_interrogative_upspeak: bool,
}

impl Default for SynthesisOptions {
    fn default() -> Self {
        unsafe { sys::voicevox_make_default_synthesis_options() }.into()
    }
}

/// [`Synthesizer::tts`]のオプション。
#[derive(Debug, Clone, Copy)]
pub struct TtsOptions {
    pub enable_interrogative_upspeak: bool,
}

impl Default for TtsOptions {
    fn default() -> Self {
        unsafe { sys::voicevox_make_default_tts_options() }.into()
    }
}

macro_rules! call_json {
    ($inner:expr, $text:expr, $style_id:expr, $func:ident) => {{
        let return_ptr = unsafe {
            let mut ptr = MaybeUninit::uninit();
            let text = CString::new($text).unwrap();
            i32_to_result(sys::$func(
                $inner,
                text.as_ptr(),
                $style_id,
                ptr.as_mut_ptr(),
            ))?;
            ptr.assume_init()
        };
        let return_str = unsafe { CStr::from_ptr(return_ptr).to_str().unwrap().to_string() };

        unsafe {
            sys::voicevox_json_free(return_ptr);
        };

        return_str
    }};
}

impl Synthesizer {
    /// 新しい音声シンセサイザを作成する。
    pub fn new(open_jtalk: &crate::OpenJtalkRc, options: InitializeOptions) -> Result<Self> {
        let inner = unsafe {
            let mut ptr = MaybeUninit::uninit();
            i32_to_result(sys::voicevox_synthesizer_new(
                open_jtalk.inner,
                options.into(),
                ptr.as_mut_ptr(),
            ))?;
            ptr.assume_init()
        };

        Ok(Self { inner })
    }

    /// GPU モードかどうかを返す。
    pub fn is_gpu_mode(&self) -> bool {
        unsafe { sys::voicevox_synthesizer_is_gpu_mode(self.inner) }
    }

    /// 音声モデルを読み込む。
    pub fn load_voice_model(&self, voice_model: &VoiceModel) -> Result<()> {
        i32_to_result(unsafe {
            sys::voicevox_synthesizer_load_voice_model(self.inner, voice_model.inner)
        })
    }

    /// 音声モデルが読み込まれているかを返す。
    pub fn is_loaded_voice_model(&self, voice_model: &VoiceModel) -> bool {
        let voice_model_id = voice_model.id();
        let voice_model_id = CString::new(voice_model_id).unwrap();
        unsafe {
            sys::voicevox_synthesizer_is_loaded_voice_model(self.inner, voice_model_id.as_ptr())
        }
    }

    /// 音声モデルの読み込みを解除する。
    pub fn unload_voice_model(&self, voice_model: &VoiceModel) -> Result<()> {
        let voice_model_id = voice_model.id();
        let voice_model_id = CString::new(voice_model_id).unwrap();
        i32_to_result(unsafe {
            sys::voicevox_synthesizer_unload_voice_model(self.inner, voice_model_id.as_ptr())
        })
    }

    /// 今読み込んでいる音声モデルのメタ情報を取得する。
    pub fn get_metas(&self) -> Result<Vec<SpeakerMeta>> {
        let return_ptr = unsafe { sys::voicevox_synthesizer_create_metas_json(self.inner) };
        let return_str = unsafe { CStr::from_ptr(return_ptr).to_str().unwrap().to_string() };

        unsafe {
            sys::voicevox_json_free(return_ptr);
        };

        Ok(serde_json::from_str(&return_str).unwrap())
    }

    /// 日本語テキストからAudioQueryを生成する。
    ///
    /// # Arguments
    ///
    /// * `text` - 音声合成するテキスト。
    /// * `style_id` - 音声のスタイルID。
    ///
    pub fn create_audio_query(&self, text: &str, style_id: StyleId) -> Result<AudioQuery> {
        let audio_query = call_json!(
            self.inner,
            text,
            style_id,
            voicevox_synthesizer_create_audio_query
        );

        Ok(serde_json::from_str(&audio_query).unwrap())
    }

    /// AquesTalk風記法からAudioQueryを生成する。
    ///
    /// # Arguments
    /// * `kana` - AquesTalk風記法のカナ。
    /// * `style_id` - 音声のスタイルID。
    pub fn create_audio_query_from_kana(
        &self,
        kana: &str,
        style_id: StyleId,
    ) -> Result<AudioQuery> {
        let audio_query = call_json!(
            self.inner,
            kana,
            style_id,
            voicevox_synthesizer_create_audio_query_from_kana
        );

        Ok(serde_json::from_str(&audio_query).unwrap())
    }

    /// 日本語テキストからAccentPhraseの配列を生成する。
    ///
    /// # Arguments
    /// * `text` - 日本語テキスト。
    /// * `style_id` - 音声のスタイルID。
    pub fn create_accent_phrases(
        &self,
        text: &str,
        style_id: StyleId,
    ) -> Result<Vec<AccentPhrase>> {
        let accent_phrases = call_json!(
            self.inner,
            text,
            style_id,
            voicevox_synthesizer_create_accent_phrases
        );

        Ok(serde_json::from_str(&accent_phrases).unwrap())
    }

    /// AquesTalk風記法からAccentPhraseの配列を生成する。
    ///
    /// # Arguments
    /// * `kana` - AquesTalk風記法のカナ。
    /// * `style_id` - 音声のスタイルID。
    pub fn create_accent_phrases_from_kana(
        &self,
        kana: &str,
        style_id: StyleId,
    ) -> Result<Vec<AccentPhrase>> {
        let accent_phrases = call_json!(
            self.inner,
            kana,
            style_id,
            voicevox_synthesizer_create_accent_phrases_from_kana
        );

        Ok(serde_json::from_str(&accent_phrases).unwrap())
    }
    /// AccentPhraseの配列の音高・音素長を、特定の声で生成しなおす。
    pub fn replace_mora_data(
        &self,
        accent_phrases: &[AccentPhrase],
        style_id: StyleId,
    ) -> Result<Vec<AccentPhrase>> {
        let accent_phrases = serde_json::to_string(&accent_phrases).unwrap();
        let accent_phrases = call_json!(
            self.inner,
            accent_phrases,
            style_id,
            voicevox_synthesizer_replace_mora_data
        );

        Ok(serde_json::from_str(&accent_phrases).unwrap())
    }

    /// AccentPhraseの配列の音高を、特定の声で生成しなおす。
    pub fn replace_mora_pitch(
        &self,
        accent_phrases: &[AccentPhrase],
        style_id: StyleId,
    ) -> Result<Vec<AccentPhrase>> {
        let accent_phrases = serde_json::to_string(&accent_phrases).unwrap();
        let accent_phrases = call_json!(
            self.inner,
            accent_phrases,
            style_id,
            voicevox_synthesizer_replace_mora_pitch
        );

        Ok(serde_json::from_str(&accent_phrases).unwrap())
    }

    /// AccentPhraseの配列の音素長を、特定の声で生成しなおす。
    pub fn replace_phoneme_length(
        &self,
        accent_phrases: &[AccentPhrase],
        style_id: StyleId,
    ) -> Result<Vec<AccentPhrase>> {
        let accent_phrases = serde_json::to_string(&accent_phrases).unwrap();
        let accent_phrases = call_json!(
            self.inner,
            accent_phrases,
            style_id,
            voicevox_synthesizer_replace_phoneme_length
        );

        Ok(serde_json::from_str(&accent_phrases).unwrap())
    }

    /// AudioQueryから音声を合成する。
    ///
    /// # Arguments
    /// * `audio_query` - 音声合成するためのクエリ。
    /// * `style_id` - 音声のスタイルID。
    /// * `options` - 音声合成のオプション。
    ///
    /// # Returns
    ///
    /// WAV形式の音声データ。
    pub fn synthesis(
        &self,
        audio_query: &AudioQuery,
        style_id: StyleId,
        options: SynthesisOptions,
    ) -> Result<Vec<u8>> {
        let audio_query = serde_json::to_string(audio_query).unwrap();
        let audio_query = CString::new(audio_query).unwrap();
        let (wav, len) = unsafe {
            let mut wav_ptr = MaybeUninit::uninit();
            let mut len_ptr = MaybeUninit::uninit();
            i32_to_result(sys::voicevox_synthesizer_synthesis(
                self.inner,
                audio_query.as_ptr(),
                style_id,
                options.into(),
                len_ptr.as_mut_ptr(),
                wav_ptr.as_mut_ptr(),
            ))?;
            (wav_ptr.assume_init(), len_ptr.assume_init())
        };

        let result = unsafe { std::slice::from_raw_parts(wav, len).to_vec() };

        unsafe {
            sys::voicevox_wav_free(wav);
        }

        Ok(result)
    }

    /// 日本語テキストから音声を合成する。
    ///
    /// # Arguments
    ///
    /// * `text` - 音声合成するテキスト。
    /// * `style_id` - 音声のスタイルID。
    /// * `options` - 音声合成のオプション。
    ///
    /// # Returns
    ///
    /// WAV形式の音声データ。
    pub fn tts(&self, text: &str, style_id: StyleId, options: TtsOptions) -> Result<Vec<u8>> {
        let text = CString::new(text).unwrap();
        let (wav, len) = unsafe {
            let mut wav_ptr = MaybeUninit::uninit();
            let mut len_ptr = MaybeUninit::uninit();
            i32_to_result(sys::voicevox_synthesizer_tts(
                self.inner,
                text.as_ptr(),
                style_id,
                options.into(),
                len_ptr.as_mut_ptr(),
                wav_ptr.as_mut_ptr(),
            ))?;
            (wav_ptr.assume_init(), len_ptr.assume_init())
        };

        let result = unsafe { std::slice::from_raw_parts(wav, len).to_vec() };

        unsafe {
            sys::voicevox_wav_free(wav);
        }

        Ok(result)
    }

    /// AquesTalk風記法のカナから音声を合成する。
    ///
    /// # Arguments
    /// * `kana` - AquesTalk風記法のカナ。
    /// * `style_id` - 音声のスタイルID。
    /// * `options` - 音声合成のオプション。
    ///
    /// # Returns
    /// WAV形式の音声データ。
    pub fn tts_from_kana(
        &self,
        kana: &str,
        style_id: StyleId,
        options: TtsOptions,
    ) -> Result<Vec<u8>> {
        let kana = CString::new(kana).unwrap();
        let (wav, len) = unsafe {
            let mut wav_ptr = MaybeUninit::uninit();
            let mut len_ptr = MaybeUninit::uninit();
            i32_to_result(sys::voicevox_synthesizer_tts_from_kana(
                self.inner,
                kana.as_ptr(),
                style_id,
                options.into(),
                len_ptr.as_mut_ptr(),
                wav_ptr.as_mut_ptr(),
            ))?;
            (wav_ptr.assume_init(), len_ptr.assume_init())
        };

        let result = unsafe { std::slice::from_raw_parts(wav, len).to_vec() };

        unsafe {
            sys::voicevox_wav_free(wav);
        }

        Ok(result)
    }
}

impl From<AccelerationMode> for sys::VoicevoxAccelerationMode {
    fn from(mode: AccelerationMode) -> sys::VoicevoxAccelerationMode {
        match mode {
            AccelerationMode::Auto => sys::VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_AUTO,
            AccelerationMode::Cpu => sys::VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_CPU,
            AccelerationMode::Gpu => sys::VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_GPU,
        }
    }
}

impl From<sys::VoicevoxAccelerationMode> for AccelerationMode {
    fn from(mode: sys::VoicevoxAccelerationMode) -> AccelerationMode {
        match mode {
            sys::VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_AUTO => AccelerationMode::Auto,
            sys::VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_CPU => AccelerationMode::Cpu,
            sys::VoicevoxAccelerationMode_VOICEVOX_ACCELERATION_MODE_GPU => AccelerationMode::Gpu,
            _ => panic!("Invalid acceleration mode"),
        }
    }
}

impl From<InitializeOptions> for sys::VoicevoxInitializeOptions {
    fn from(options: InitializeOptions) -> sys::VoicevoxInitializeOptions {
        sys::VoicevoxInitializeOptions {
            acceleration_mode: i32::from(options.acceleration_mode),
            cpu_num_threads: options.cpu_num_threads,
        }
    }
}

impl From<sys::VoicevoxInitializeOptions> for InitializeOptions {
    fn from(options: sys::VoicevoxInitializeOptions) -> InitializeOptions {
        InitializeOptions {
            acceleration_mode: AccelerationMode::from(options.acceleration_mode),
            cpu_num_threads: options.cpu_num_threads,
        }
    }
}

impl From<TtsOptions> for sys::VoicevoxTtsOptions {
    fn from(options: TtsOptions) -> sys::VoicevoxTtsOptions {
        sys::VoicevoxTtsOptions {
            enable_interrogative_upspeak: options.enable_interrogative_upspeak,
        }
    }
}

impl From<sys::VoicevoxTtsOptions> for TtsOptions {
    fn from(options: sys::VoicevoxTtsOptions) -> TtsOptions {
        TtsOptions {
            enable_interrogative_upspeak: options.enable_interrogative_upspeak,
        }
    }
}

impl From<SynthesisOptions> for sys::VoicevoxSynthesisOptions {
    fn from(options: SynthesisOptions) -> sys::VoicevoxSynthesisOptions {
        sys::VoicevoxSynthesisOptions {
            enable_interrogative_upspeak: options.enable_interrogative_upspeak,
        }
    }
}

impl From<sys::VoicevoxSynthesisOptions> for SynthesisOptions {
    fn from(options: sys::VoicevoxSynthesisOptions) -> SynthesisOptions {
        SynthesisOptions {
            enable_interrogative_upspeak: options.enable_interrogative_upspeak,
        }
    }
}
