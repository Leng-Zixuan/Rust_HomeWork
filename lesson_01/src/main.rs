mod mod_01 {

    pub mod mod_02 {
        pub fn print_02_a_z() {
            for i in b'A'..=b'z' {
                println!("mod_02: {} ", i as char);
            }
        }
    }

    pub fn print_01_a_z() {
        // 此题隐藏的坑（ascii 小a是 97  大z 是90）所以终端无结果
        for i in b'a'..=b'Z' {
            println!("mod_01: {} ", i as char);
        }
    }
}

use mod_01::mod_02;

fn main() {
    // 循环打印a~Z之间的所有字符
    mod_01::print_01_a_z();

    // 循环打印A~z之间的所有字符 此题隐藏的坑（ascii 小a是 97  大z 是90）
    mod_02::print_02_a_z();
}
