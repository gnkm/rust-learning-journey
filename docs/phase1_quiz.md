# Rust フェーズ1 確認テスト

## 問題1: 開発環境のセットアップ
以下の質問に日本語で回答してください。

1. Rustの開発環境をセットアップするために必要な手順を3つ挙げてください。
2. `rustup`とは何ですか？その主な役割を説明してください。
3. `cargo`とは何ですか？その主な機能を2つ挙げてください。

## 問題2: 基本的なデータ型
以下のコードを実行したときの出力を予想し、その理由を説明してください。

```rust
fn main() {
    let x: i32 = 42;
    let y: f64 = 3.14;
    let z: bool = true;
    let c: char = 'A';

    println!("x: {}, y: {}, z: {}, c: {}", x, y, z, c);
}
```

## 問題3: 変数束縛
以下のコードの実行結果を予想し、その理由を説明してください。

```rust
fn main() {
    let x = 5;
    x = 6;  // この行はコンパイルエラーになりますか？

    let mut y = 5;
    y = 6;  // この行はコンパイルエラーになりますか？
}
```

## 問題4: 関数の定義
以下の要件を満たす関数を実装してください：

1. 関数名: `calculate_area`
2. 引数: 長方形の幅（`width: f64`）と高さ（`height: f64`）
3. 戻り値: 長方形の面積（`f64`型）
4. 機能: 幅と高さから長方形の面積を計算する

## 問題5: 制御フロー
以下の要件を満たすプログラムを実装してください：

1. 1から10までの数字を順番に表示する
2. 3の倍数の場合は「Fizz」と表示する
3. 5の倍数の場合は「Buzz」と表示する
4. 3と5の両方の倍数の場合は「FizzBuzz」と表示する

# 回答

## 1.1: Rustの開発環境をセットアップするために必要な手順を3つ挙げてください。

手順 1: rustup のインストール
手順 2: コンポーネントのインストール

```
rustup component add rustfmt      # コードフォーマッター
rustup component add clippy       # リンター
rustup component add rust-analyzer # LSP（Cursor用）
rustup component add rust-src     # ソースコード（補完用）
```

手順 3: エディタに `rust-analyzer` 追加、設定ファイル作成

`.vscode/settings.json`

```jsonc
{
    // =========== エディタ全般 ===========
    // 保存時に自動でコードフォーマットを実行する
    "editor.formatOnSave": true,
    // ファイルを開いている間に自動でインデントやフォーマットを適用する
    "editor.formatOnPaste": true,
    "editor.formatOnType": true,
    // デフォルトのフォーマッターとして rust-analyzer の rustfmt を指定
    // Rust ファイル（.rs）を保存するときは rustfmt で整形する
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },
    // 保存時に不要な末尾スペースを削除する
    "files.trimTrailingWhitespace": true,
    // ファイル末尾に改行を強制する
    "files.insertFinalNewline": true,
    // =========== rust-analyzer 設定 ===========
    "rust-analyzer.cargo.features": "all",
    // =========== デバッグ設定 ===========
    // デバッグ時に標準出力をターミナルで表示する
    "lldb.launch.runInTerminal": true,
    // コード補完の候補表示を少しタイムアウトを延ばして安定させる
    "editor.quickSuggestionsDelay": 50,
    // =========== ファイルウォッチャー設定 ===========
    // Rust のビルド成果物やターゲットフォルダをウォッチャーから除外し、パフォーマンス向上
    "files.watcherExclude": {
        "**/target/**": true,
        "**/node_modules/**": true
    },
    // =========== 検索設定 ===========
    // 検索時に target フォルダをデフォルトで除外する
    "search.exclude": {
        "**/target/**": true
    },
    // =========== 拡張機能の推奨設定 ===========
    // 拡張機能「crates」（Rust のクレートバージョンを管理する）の有効化
    "crates.autoupdate": true,
    "crates.upgradeToPreReleases": false,
}
```

## 1.2: `rustup`とは何ですか？その主な役割を説明してください。

`rustup` とはRust のエコシステムを管理するツール。
コンポーネントの追加、削除をおこなう。

## 1.3: `cargo`とは何ですか？その主な機能を2つ挙げてください。

`cargo` とは、Rust のコードを実行するコマンド。
以下の機能を持っている。

- `cargo run`: Rust のコードを実行する
- `cargo build`: Rust のコードをビルドし、実行ファイルを作る

## 2: 基本的なデータ型

出力:

```
x: 42, y: 3.14, z: true, c: A
```

理由: x, y, z, c に変数の値が設定されていて、`println!()` で出力されるコードがあるから。

## 3: 変数束縛

```rust
fn main() {
    let x = 5;
    // 以下の行はコンパイルエラーになる
    // 理由: イミュータブルな変数に再代入しているため。
    x = 6;

    let mut y = 5;
    // 以下の行はコンパイルエラーにならない
    // 理由: ミュータブルな変数に再代入しているため。
    y = 6;
}
```

## 4: 関数の定義
以下の要件を満たす関数を実装する。

1. 関数名: `calculate_area`
2. 引数: 長方形の幅（`width: f64`）と高さ（`height: f64`）
3. 戻り値: 長方形の面積（`f64`型）
4. 機能: 幅と高さから長方形の面積を計算する

```rust
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
```

## 5: 制御フロー
以下の要件を満たすプログラムを実装する。

1. 1から10までの数字を順番に表示する
2. 3の倍数の場合は「Fizz」と表示する
3. 5の倍数の場合は「Buzz」と表示する
4. 3と5の両方の倍数の場合は「FizzBuzz」と表示する

```rust
fn main() {
    for i in 1..=10 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
```
