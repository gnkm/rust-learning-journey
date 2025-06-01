# フェーズ 1 確認テスト回答

## 問題1.1. `rustup` とは何ですか？その役割を簡潔に説明してください。

`rustup` とは Rust のツールを管理するためのツール。
例えば `rustup component add` でツールを追加することができる。

## 問題1.2. `cargo` の主な役割を3つ挙げてください。

1. Rust プログラムを実行する(`cargo run`)
2. Rust プログラムをビルドし、実行ファイルを作る(`cargo build`)
3. Rust プログラムの静的解析をおこなう(`cargo check`)

## 問題1.3. `cargo new` コマンドで作成されるプロジェクトの基本的なディレクトリ構造を説明してください。

以下のように、`Cargo.toml`, `Cargo.lock` ファイル、`src` ディレクトリ、`src` ディレクトリの下には `main.rs` が作成される。

```
project-name
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```

## 問題2：基本的なデータ型
以下のコードの中で、コンパイルエラーになる行は 2, 3, 4。
2 がエラーになる理由: `5.0` は int ではなく float だから。
3 がエラーになる理由: `-1` は unsigned ではないから。
4 がエラーになる理由: char では、シングルクオートを使う必要があるから。

```rust
fn main() {
    let x = 5;              // 1
    let y: i32 = 5.0;       // 2
    let z: u8 = -1;         // 3
    let w: char = "A";      // 4
    let v: bool = true;     // 5
    let u = 3.14;           // 6
}
```

## 問題3：可変性と不変性
元のコードがエラーになる理由: イミュータブルな変数に再代入するため。

```rust
fn main() {
    let mut x = 5;
    println!("xの値は: {}", x);
    x = 10;
    println!("xの新しい値は: {}", x);
}
```

## 問題4：関数について

## 問題4.1. Rustの関数で戻り値を返す方法を2つ挙げてください。

(1)以下のように式を記述する。

```rust
fn something_function() {
    // 何か処理
    x
}
```

(2)以下のように `return` 文を使う。

```rust
fn something_function() {
    // 何か処理
    return x;
}
```

## 問題4.2. 関数の引数と戻り値には型注釈が必要ですか？その理由も説明してください。

不要。型推論が働くため。

## 問題5：所有権の基礎
以下のコードがコンパイルエラーになる理由: 実質的に `s` が使われているだけで、`s2` は不要だから。

```rust
fn main() {
    let s = String::from("hello");
    let s2 = s;
    println!("{}", s);
}
```

## 問題6：制御フロー

### 問題6.1. Rustの `if` が「式」であることの意味は何ですか？

`if` の結果を変数に格納できる。

### 問題6.2. `if` 式を使って変数に値を代入する例を示してください。

```rust
fn main() {
    let x = if () {
        // 何か処理
    } else {
        // 何か処理
    }
}
```

## 問題7：ループ
使い分けは以下のとおり。
- `loop`: 無限ループを使う場合
- `while`: 条件に応じてループを終了する場合
- `for`: vector, hash など、要素の数が決まっている場合

## 問題8：マクロ
マクロ呼び出しの特徴的な記号は `!`。
`println` が関数ではなくマクロとして実装されている理由: 返り値がないから。

## 問題9：基本的な変数操作
以下の要件を満たすプログラムを実装する。

1. 整数型、浮動小数点型、文字型、ブーリアン型の変数をそれぞれ1つずつ宣言する
2. それらの値を使って簡単な計算や操作を行う
3. 結果を見やすく表示する

```rust
fn main() {
    let x: i32 = 42;
    let y: f64 = 3.14;
    let z: bool = true;
    let c: String = String::from("ABC");

    let x_plus_1 = x + 1;

    println!(
        "x: {}, y: {}, z: {}, c: {}, x_plus_1: {}",
        x, y, z, c, x_plus_1
    );
}
```

## 問題10：関数の実装
摂氏を華氏に変換する関数 `celsius_to_fahrenheit` を実装する。
- 引数：摂氏温度（f64型）
- 戻り値：華氏温度（f64型）
- 変換式：華氏 = 摂氏 × 9/5 + 32

```rust
fn main() {
    let celsius = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C は {}°F です", celsius, fahrenheit);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
```

## 問題11：条件分岐の活用
年齢を受け取って、以下の分類を行う関数 `age_category` を実装する。
- 0-12歳: "子供"
- 13-19歳: "ティーンエイジャー"
- 20-64歳: "大人"
- 65歳以上: "シニア"
- 負の値: "無効な年齢"

```rust
fn main() {
    let ages = vec![5, 15, 30, 70, -5];
    for age in ages {
        println!("{}歳は{}です", age, age_category(age));
    }
}

// 年齢を受け取って、以下の分類を行う関数 `age_category` を実装する。
// - 0-12歳: "子供"
// - 13-19歳: "ティーンエイジャー"
// - 20-64歳: "大人"
// - 65歳以上: "シニア"
// - 負の値: "無効な年齢"
fn age_category(age: i32) -> String {
    if age < 0 {
        "無効な年齢".to_string()
    } else if age <= 12 {
        "子供".to_string()
    } else if age <= 19 {
        "ティーンエイジャー".to_string()
    } else if age <= 64 {
        "大人".to_string()
    } else {
        "シニア".to_string()
    }
}
```

### 問題12：ループの活用
1から指定された数までのフィボナッチ数列を計算して表示するプログラムを作成してください。
- `for` ループまたは `while` ループを使用すること
- 最初の2つの数は0と1とする

```rust
fn main() {
    let n = 10;
    println!("最初の{}個のフィボナッチ数:", n);

    let mut a = 0;
    let b = 1;
    for _i in 0..n {
        println!("{}", a);
        let _next = a + b;
        a = b;
    }
}
```

### 問題13：総合問題 - 簡単な計算機（10点）
以下の要件を満たす簡単な計算機プログラムを作成する。

1. 2つの数値と演算子（+, -, *, /）を受け取る関数 `calculate` を実装
2. 0での除算の場合は適切なメッセージを返す
3. 不正な演算子の場合もエラーメッセージを返す

```rust
fn main() {
    // テストケース
    println!("{}", calculate(10.0, 5.0, '+'));
    println!("{}", calculate(10.0, 5.0, '-'));
    println!("{}", calculate(10.0, 5.0, '*'));
    println!("{}", calculate(10.0, 5.0, '/'));
    println!("{}", calculate(10.0, 0.0, '/'));
    println!("{}", calculate(10.0, 5.0, '%'));
}

// ここに関数を実装してください
fn calculate(a: f64, b: f64, op: char) -> String {
    match op {
        '+' => format!("{}", a + b),
        '-' => format!("{}", a - b),
        '*' => format!("{}", a * b),
        '/' => {
            if b == 0.0 {
                "エラー: 0で割ることはできません".to_string()
            } else {
                format!("{}", a / b)
            }
        }
        '%' => {
            if b == 0.0 {
                "エラー: 0で割ることはできません".to_string()
            } else {
                format!("{}", a % b)
            }
        }
        _ => todo!(),
    }
}
```

### 問題14：統合演習 - 成績評価システム（10点）
学生の点数（0-100）を受け取って、成績（A, B, C, D, F）と合否を判定するプログラムを作成する。

要件：
- 90点以上: A（合格）
- 80-89点: B（合格）
- 70-79点: C（合格）
- 60-69点: D（合格）
- 60点未満: F（不合格）
- 0未満または100を超える場合: エラーメッセージ
- さらに、複数の学生の点数を処理し、クラスの平均点も計算する。

```rust
//! 学生の点数（0-100）を受け取って、成績（A, B, C, D, F）と合否を判定するプログラム。
//! 要件：
//! - 90点以上: A（合格）
//! - 80-89点: B（合格）
//! - 70-79点: C（合格）
//! - 60-69点: D（合格）
//! - 60点未満: F（不合格）
//! - 0未満または100を超える場合: エラーメッセージ
//! - さらに、複数の学生の点数を処理し、クラスの平均点も計算する。

fn main() {
    let scores = vec![95, 82, 70, 58, 45, 100, -5, 105, 88, 76];

    // 各学生の成績を判定
    for (i, &score) in scores.iter().enumerate() {
        println!("{}: {}", i, grade(score));
    }

    // クラスの平均点を計算（無効な点数は除外）
    let valid_scores = scores
        .iter()
        .filter(|&&score| score >= 0 && score <= 100)
        .collect::<Vec<_>>();
    let average = valid_scores.iter().copied().sum::<i32>() as f64 / valid_scores.len() as f64;
    println!("クラスの平均点: {}", average);
}

fn grade(score: i32) -> String {
    if score < 0 || score > 100 {
        "エラー: 0未満または100を超える点数です".to_string()
    } else if score >= 90 {
        "A".to_string()
    } else if score >= 80 {
        "B".to_string()
    } else if score >= 70 {
        "C".to_string()
    } else if score >= 60 {
        "D".to_string()
    } else {
        "F".to_string()
    }
}
```
