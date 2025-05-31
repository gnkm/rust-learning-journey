use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // コマンドライン引数をベクタに取得
    let args: Vec<String> = env::args().collect();

    // 第1引数があればそれをファイル名に、なければ "output.txt" をデフォルトに
    let filename: String = if args.len() > 1 {
        args[1].clone()
    } else {
        "output.txt".to_string()
    };

    // ファイルを作成して書き込み
    let mut file: File = File::create(&filename).expect("ファイル作成に失敗しました");
    writeln!(file, "Hello, Rust!").expect("書き込みに失敗しました");

    // 何度でも filename を使い回せる
    println!("ファイル '{}' に書き込みました", filename);
}
