// generator.rs
use rand::Rng; // 取り込んだrandモジュールを取り込む

pub fn gen_ran() -> u8 {
    let mut rang = rand::thread_rng();
    let n: u8 = rang.gen(); // u8を指定しているので8bit(0~255)の間で乱数を発生させる。
    n
}
