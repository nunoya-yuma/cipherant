# Cipherant - パーソナルリサーチエージェント

## プロジェクト概要
Rust製のLLMを活用したローカル処理優先の個人向け調査・情報整理アシスタント。複数の情報源からデータを収集・統合し、ユーザーの文脈に合わせた深い洞察を提供する。

**コンセプト**: Cipher（暗号/情報）+ Servant（使用人）= 情報を扱う従者

## 技術スタック

### コア機能
- **言語**: Rust (edition 2024)
- **非同期処理**: tokio
- **HTTP通信**: reqwest
- **並列処理**: rayon
- **シリアライゼーション**: serde, serde_json
- **CLI**: clap

### LLMインテグレーション（予定）
- llm-chain-rs
- candle
- OpenAI API / Gemini API

### 情報処理（予定）
- **PDF処理**: lopdf / pdf
- **HTMLスクレイピング**: scraper / select
- **全文検索**: tantivy
- **ベクトルDB**: qdrant-client

## プロジェクト構造

```
cipherant/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli/              # CLIインターフェース
│   ├── core/             # コアエンジン
│   ├── sources/          # 情報ソース（web, pdf, local_files）
│   ├── llm/              # LLMコネクタ
│   ├── storage/          # ストレージ（vector_db, metadata, history）
│   └── utils/            # ユーティリティ
└── docs/
    ├── design.md         # アーキテクチャ設計
    └── development.md    # 開発ガイド
```

## 開発コマンド

```bash
# ビルドと実行
cargo run

# テスト
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy

# リリースビルド
cargo build --release
```

## ドキュメント
- アーキテクチャ設計: [docs/design.md](docs/design.md)
- 開発ガイド: [docs/development.md](docs/development.md)

## 現在の開発フェーズ
フェーズ1: 基盤実装
- [x] プロジェクト初期化
- [ ] 基本的なCLI構造
- [ ] モジュール分割
- [ ] シンプルなウェブスクレイピング
- [ ] LLMインテグレーション（初期モデル）
