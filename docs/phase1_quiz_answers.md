# Rust フェーズ1 確認テスト - 模範解答

## 第1部：理解度確認問題

### 問題1：Rust開発環境について

1. **`rustup` とは何ですか？**
   - `rustup` は Rust のツールチェインマネージャーです。Rust コンパイラ（rustc）、標準ライブラリ、cargo などのツールのインストールと管理を行います。異なるバージョンの Rust を切り替えたり、ナイトリー版やベータ版を試したりすることができます。

2. **`cargo` の主な役割を3つ：**
   - プロジェクトのビルドシステム：コードのコンパイルと実行
   - パッケージマネージャー：外部クレート（ライブラリ）の依存関係管理
   - プロジェクト管理：新規プロジェクトの作成、テストの実行、ドキュメントの生成

3. **`cargo new` で作成されるディレクトリ構造：**
   ```
   プロジェクト名/
   ├── Cargo.toml    # プロジェクトの設定ファイル（名前、バージョン、依存関係など）
   ├── src/
   │   └── main.rs   # メインのソースファイル（バイナリクレートの場合）
   └── .gitignore    # Git用の無視ファイルリスト
   ```

### 問題2：基本的なデータ型

コンパイルエラーになる行：
- **2行目**: `let y: i32 = 5.0;` - 整数型に浮動小数点リテラルを代入しようとしているため
- **3行目**: `let z: u8 = -1;` - 符号なし整数型に負の値を代入しようとしているため
- **4行目**: `let w: char = "A";` - 文字型に文字列リテラルを代入しようとしているため（正しくは `'A'`）

### 問題3：可変性と不変性

**修正版：**
```rust
fn main() {
    let mut x = 5;  // mut キーワードを追加
    println!("xの値は: {}", x);
    x = 10;
    println!("xの新しい値は: {}", x);
}
```

**エラーの理由：**
Rust では変数はデフォルトで不変（immutable）です。一度値を束縛した変数に再代入するには、`mut` キーワードを使って可変（mutable）として宣言する必要があります。

### 問題4：関数について

1. **戻り値を返す方法：**
   - セミコロンなしの式：関数の最後の式の値が自動的に返される
   - `return` キーワード：明示的に値を返す（早期リターンにも使用）

2. **型注釈の必要性：**
   - 引数：必須。コンパイラが関数の使用方法を明確に理解するため
   - 戻り値：値を返す場合は必須。型推論が関数の境界を越えないため
   - 理由：Rust は強い型付け言語であり、関数のシグネチャは明確である必要がある

### 問題5：所有権の基礎

**エラーの理由：**
`String` 型は所有権を持つ型です。`let s2 = s;` で `s` の所有権が `s2` に移動（ムーブ）します。所有権が移動した後の `s` は無効になるため、その後の `println!` で使用しようとするとコンパイルエラーになります。これは Rust のメモリ安全性を保証する仕組みの一部です。

### 問題6：制御フロー

1. **`if` が式であることの意味：**
   `if` は値を返すことができ、その値を変数に代入したり、関数の戻り値として使用できます。文（statement）ではなく式（expression）として扱われます。

2. **例：**
   ```rust
   let number = 5;
   let message = if number > 0 {
       "正の数"
   } else if number < 0 {
       "負の数"
   } else {
       "ゼロ"
   };
   ```

### 問題7：ループ

- **`loop`**: 無限ループが必要な場合、明示的な `break` で終了する場合
- **`while`**: 条件が真の間繰り返す場合、繰り返し回数が事前に分からない場合
- **`for`**: コレクションの要素を反復処理する場合、決まった回数繰り返す場合（範囲を使用）

### 問題8：マクロ

**特徴的な記号：** `!`（エクスクラメーションマーク）

**マクロである理由（推測）：**
- 可変長引数を受け取れる（通常の関数では難しい）
- コンパイル時に展開され、型安全な出力フォーマットを保証できる
- フォーマット文字列の検証をコンパイル時に行える

---

## 第2部：コーディング問題

### 問題9：基本的な変数操作（10点）

```rust
fn main() {
    // 各種データ型の変数を宣言
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let character: char = 'R';
    let boolean: bool = true;

    // 簡単な計算や操作
    let doubled = integer * 2;
    let circle_area = float * float; // πr² の簡略版
    let is_rust = character == 'R';
    let result = if boolean { "真" } else { "偽" };

    // 結果を表示
    println!("整数 {} を2倍すると {}", integer, doubled);
    println!("半径 {} の円の面積（π=rとして）: {:.2}", float, circle_area);
    println!("文字 '{}' は 'R' ですか？ {}", character, is_rust);
    println!("ブーリアン値 {} は {}", boolean, result);
}
```

### 問題10：関数の実装（10点）

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

### 問題11：条件分岐の活用（10点）

```rust
fn main() {
    let ages = vec![5, 15, 30, 70, -5];
    for age in ages {
        println!("{}歳は{}です", age, age_category(age));
    }
}

fn age_category(age: i32) -> &'static str {
    if age < 0 {
        "無効な年齢"
    } else if age <= 12 {
        "子供"
    } else if age <= 19 {
        "ティーンエイジャー"
    } else if age <= 64 {
        "大人"
    } else {
        "シニア"
    }
}
```

### 問題12：ループの活用（10点）

```rust
fn main() {
    let n = 10; // 最初の10個のフィボナッチ数を表示
    println!("最初の{}個のフィボナッチ数:", n);

    let mut a = 0;
    let mut b = 1;

    for i in 0..n {
        if i == 0 {
            println!("{}: {}", i + 1, a);
        } else if i == 1 {
            println!("{}: {}", i + 1, b);
        } else {
            let next = a + b;
            println!("{}: {}", i + 1, next);
            a = b;
            b = next;
        }
    }
}
```

### 問題13：総合問題 - 簡単な計算機（10点）

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

fn calculate(a: f64, b: f64, op: char) -> String {
    match op {
        '+' => format!("{} + {} = {}", a, b, a + b),
        '-' => format!("{} - {} = {}", a, b, a - b),
        '*' => format!("{} * {} = {}", a, b, a * b),
        '/' => {
            if b == 0.0 {
                "エラー: 0での除算はできません".to_string()
            } else {
                format!("{} / {} = {}", a, b, a / b)
            }
        }
        _ => format!("エラー: '{}' は無効な演算子です", op),
    }
}
```

### 問題14：統合演習 - 成績評価システム（10点）

```rust
fn main() {
    let scores = vec![95, 82, 70, 58, 45, 100, -5, 105, 88, 76];

    // 各学生の成績を判定
    for (i, &score) in scores.iter().enumerate() {
        let (grade, pass_fail) = evaluate_score(score);
        println!("学生{}: {}点 - {} ({})", i + 1, score, grade, pass_fail);
    }

    // クラスの平均点を計算（無効な点数は除外）
    let valid_scores: Vec<i32> = scores.into_iter()
        .filter(|&s| s >= 0 && s <= 100)
        .collect();

    if !valid_scores.is_empty() {
        let sum: i32 = valid_scores.iter().sum();
        let average = sum as f64 / valid_scores.len() as f64;
        println!("\nクラス平均点: {:.1}点", average);
    }
}

fn evaluate_score(score: i32) -> (&'static str, &'static str) {
    if score < 0 || score > 100 {
        ("エラー", "無効な点数")
    } else if score >= 90 {
        ("A", "合格")
    } else if score >= 80 {
        ("B", "合格")
    } else if score >= 70 {
        ("C", "合格")
    } else if score >= 60 {
        ("D", "合格")
    } else {
        ("F", "不合格")
    }
}
```

---

## 学習のポイント

この確認テストを通じて、以下の点を再確認できたでしょうか：

1. **Rust の基本的な型システム**の理解
2. **所有権の概念**への初歩的な理解
3. **パターンマッチング**の便利さ
4. **エラー処理**の重要性
5. **イテレータ**を使った効率的なコード

フェーズ2では、これらの概念をさらに深く学んでいきます！
