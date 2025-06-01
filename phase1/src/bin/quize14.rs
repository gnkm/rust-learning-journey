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
