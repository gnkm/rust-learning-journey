# rust-learning-journey

Rustプログラミング言語を体系的に学習するためのプロジェクトです。

## 🎯 プロジェクトの目的

このプロジェクトは、Rustを基礎から学び、実践的なスキルを身につけることを目的としています。各フェーズごとに明確な目標を設定し、段階的にレベルアップしていきます。

## 📚 学習フェーズ

### フェーズ1: Rustの第一歩と環境構築（完了）✅

基本的な文法と開発環境の構築を学びました。

**学習内容：**
- Rust開発環境のセットアップ（rustup, cargo）
- 基本的なデータ型（整数、浮動小数点、ブーリアン、文字）
- 変数束縛と可変性（let, let mut）
- 関数の定義と呼び出し
- 制御フロー（if/else）
- ループ（loop, while, for）

**確認テスト：**
- 📝 [フェーズ1確認テスト](docs/phase1_quiz.md)
- 📖 [模範解答](docs/phase1_quiz_answers.md)
- 💡 [ヒント集](docs/phase1_quiz_hints.md)

### フェーズ2: 所有権・データ構造・エラー処理の基礎（進行中）🚀

Rustの中核概念である所有権システムと、実用的なデータ構造を学習中です。

### フェーズ3: ライフタイム・ジェネリクス・トレイトによる抽象化（予定）

### フェーズ4: クレート活用とプロジェクト構成（予定）

### フェーズ5: 非同期プログラミングと高度なトピック入門（予定）

## 📂 プロジェクト構成

```
rust-learning-journey/
├── README.md              # このファイル
├── memory-bank/          # 学習計画と進捗管理
│   ├── curriculum.md     # 詳細なカリキュラム
│   └── tasks.md         # タスクリストと進捗状況
├── docs/                # ドキュメント類
│   ├── phase1_quiz.md   # フェーズ1確認テスト
│   ├── phase1_quiz_answers.md  # 模範解答
│   └── phase1_quiz_hints.md    # ヒント集
└── examples/            # 各フェーズの練習コード（予定）
```

## 🚀 始め方

1. Rust開発環境をセットアップ
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. リポジトリをクローン
   ```bash
   git clone https://github.com/gnkm/rust-learning-journey.git
   cd rust-learning-journey
   ```

3. 学習計画を確認
   - [カリキュラム](memory-bank/curriculum.md)を読む
   - [タスクリスト](memory-bank/tasks.md)で進捗を管理

## 📝 学習の進め方

1. 各フェーズのタスクを順番に進める
2. 実際にコードを書いて動作を確認する
3. フェーズ終了時に確認テストで理解度をチェック
4. 分からないことは公式ドキュメントやヒント集を参照

## 🔗 参考資料

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - 公式ドキュメント
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 実例で学ぶRust
- [rust-analyzer](https://rust-analyzer.github.io/) - VSCode/Cursor用の拡張機能

## 📈 進捗状況

- [x] フェーズ1: Rustの第一歩と環境構築
- [ ] フェーズ2: 所有権・データ構造・エラー処理の基礎
- [ ] フェーズ3: ライフタイム・ジェネリクス・トレイトによる抽象化
- [ ] フェーズ4: クレート活用とプロジェクト構成
- [ ] フェーズ5: 非同期プログラミングと高度なトピック入門

---

**Happy Rust Learning! 🦀**
