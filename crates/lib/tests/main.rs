use voicevox_core_rs as vv;

fn create_synthesizer() -> (vv::OpenJtalkRc, vv::Synthesizer, u32) {
    let open_jtalk = vv::OpenJtalkRc::new(test_resources::get_dict_path()).unwrap();
    let synthesizer = vv::Synthesizer::new(&open_jtalk, Default::default()).unwrap();

    let voice_model = vv::VoiceModel::from_path(test_resources::get_vvm_path()).unwrap();

    synthesizer.load_voice_model(&voice_model).unwrap();

    let style_id = synthesizer.get_metas().unwrap()[0].styles()[0].id();

    (open_jtalk, synthesizer, style_id)
}

#[test]
fn test_synthesis() {
    let (_, synthesizer, style_id) = create_synthesizer();

    let text = "ハローワールド";

    let audio_query = synthesizer.create_audio_query(text, style_id).unwrap();
    let audio = synthesizer
        .synthesis(&audio_query, style_id, Default::default())
        .unwrap();

    let _ = audio;
}

#[test]
fn test_tts() {
    let (_, synthesizer, style_id) = create_synthesizer();

    let text = "ハローワールド";

    let audio = synthesizer.tts(text, style_id, Default::default()).unwrap();

    let _ = audio;
}

#[test]
fn test_dict() {
    let (open_jtalk, synthesizer, style_id) = create_synthesizer();
    let dict = vv::UserDict::new().unwrap();

    let dummy_word = "this_is_a_very_long_phrase_that_hopefully_is_not_in_any_dictionary";
    dict.add_word(vv::UserDictWord::new(dummy_word, "アイウエオ"))
        .unwrap();

    let before_kana = synthesizer
        .create_audio_query(dummy_word, style_id)
        .unwrap()
        .kana;
    open_jtalk.use_user_dict(&dict).unwrap();
    let after_kana = synthesizer
        .create_audio_query(dummy_word, style_id)
        .unwrap()
        .kana;

    assert_ne!(before_kana, after_kana);
}
