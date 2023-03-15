use std::io;

fn main() {
    let mut input = String::new();  // 標準入力を格納する文字列変数を作成する
    io::stdin().read_line(&mut input).expect("Failed to read line");  // 標準入力から文字列を読み込む

    println!("You typed: {}", input);  // 読み込んだ文字列を表示する
}
