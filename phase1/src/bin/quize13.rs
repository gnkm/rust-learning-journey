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
