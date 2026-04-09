use std::io;

use crate::task::FromString;

// 静的な名前空間として利用する構造体
pub struct InputUtils;

impl InputUtils {
    // ジェネリックな静的関数
    pub fn execute<T: FromString>(title: &str) -> T {
        let input_text = Self::input(title); // 内部で自分自身の関数を呼ぶ

        match T::new(&input_text) {
            Ok(object) => object,
            Err(e) => {
                eprintln!("入力エラー: {}", e);
                Self::execute::<T>(title) // 再帰呼び出し
            }
        }
    }

    // 静的関数（関連関数）
    fn input(string: &str) -> String {
        let mut input = String::new();
        println!("{}", string);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
}
