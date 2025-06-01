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
