
---

# 🦀 Rust 講座：実践で学ぶシステム開発入門

本リポジトリは、Rust を用いたシステム開発のスキルを段階的に習得するための教材です。CLI ツールから非同期 API、データベース、エラーハンドリングまで、モダンな Rust 開発のエッセンスを体験できます。

---

## 📁 ディレクトリ構成

```
.
├── section1/   # 環境構築と基本操作
│   ├── section1-1/
│   ├── section1-2/
│   ├── section1-3/
│   └── exercises/
├── section2/   # 実用的な機能とライブラリ
│   ├── section2-1/
│   ├── section2-2/
│   ├── section2-3/
│   └── exercises/
├── section3/   # 実践的なテクニック
│   ├── section3-1/
│   ├── section3-2/
│   ├── section3-3/
│   └── exercises/
├── section4/   # 発展的なトピック
│   ├── section4-1/
│   ├── section4-2/
│   ├── section4-3/
│   └── exercises/
└── README.md  
```

---

## 🚀 実行方法

この教材に含まれる各パート・演習は、**プロジェクトルートから相対パスで実行**する構成になっています。  
Cargo ワークスペースを活用しているため、以下のようにルートから `cargo run` / `test` が可能です。

```bash
# 例：section1 の part1 を実行
cargo run --package section1-part1

# 例：section3 の exercises をテスト実行
cargo test --package section3-exercises
```

> 📌 全てのプロジェクトは `Cargo.toml` でワークスペースとして統合されています。

---

## 🎓 この講座で学べること

| セクション | 内容 |
|------------|------|
| Section 1  | 環境構築と基本操作 |
| Section 2  | 実用的な機能とライブラリ |
| Section 3  | 実践的なテクニック |
| Section 4  |   発展的なトピック |

---

## 🏆 修了証 & バッジ制度

全てのセクションを修了すると、以下のような称号が授与されます：

- 🟢 `CLI NOVICE`
- 🟠 `DATA DRIVER`
- 🟣 `ASYNC NINJA`
- 🟡 `TYPE TAMER`
- 🔴 `ERROR MASTER`
- 🏅 **RUST ARCHITECT**（全章クリア）

---

## 📮 コントリビューション歓迎

誤字・改善提案・PR も大歓迎です。学びを共有して、みんなで Rustacean を目指しましょう 🦀

---

## 🧪 Rust バージョン

本教材は以下の Rust バージョンで動作確認済みです：

```
rustc 1.76.0 (or later)
cargo 1.76.0
```

---

## License

MIT

