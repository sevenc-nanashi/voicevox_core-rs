# voicevox_core.rs / Voicevox Core C API の Rust バインディング

voicevox_core.rs は、[Voicevox Core: C API](https://voicevox.github.io/voicevox_core/apis/c_api/voicevox__core_8h.html) の Rust バインディングです。

このリポジトリは以下の 2 つのパッケージから構成されています：

- `voicevox_core-sys`：Voicevox Core C API の最小限のラッパー。[bindgen](https://github.com/rust-lang/rust-bindgen) を使用して自動生成されたものです。
- `voicevox_core-rs`：`voicevox_core-sys` を使用して、より Rust らしいインターフェースを提供するライブラリです。

## インストール

```toml
[dependencies]
voicevox_core = { git = "https://github.com/sevenc-nanashi/voicevox_core-rs" }
```

## ライセンス

このリポジトリは、MIT ライセンスのもとで公開されています。詳細は[LICENSE](LICENSE)を参照してください。
また、このリポジトリは Voicevox Core のコードを含んでいます。

```
-- voicevox/voicevox_core --

Copyright (c) 2021 Hiroshiba Kazuyuki
Released under the MIT license
https://github.com/VOICEVOX/voicevox_core/blob/main/LICENSE
```
