use xxhash_rust::xxh3::xxh3_64;
use std::cmp;

const BITS: usize = 10;                                         // レジスタのビット数
const REGISTERS: usize = 1 << BITS;                             // レジスタの数（2^BITS）
const ALPHA_M: f64 = 0.7213 / (1.0 + 1.079 / REGISTERS as f64); // バイアス補正


struct HyperLogLog {
    registers: Vec<u8>,
}

impl HyperLogLog {
    /// HLLのインスタンスの初期化
    fn new() -> Self {
        HyperLogLog {
            registers: vec![0; REGISTERS],
        }
    }

    /// リーディングゼロの数を数える
    fn count_leading_zeros(hash: u64, bits: usize) -> u8 {
        (hash << bits).leading_zeros() as u8 + 1
    }

    /// ハッシュ値を計算し、レジスタを更新する
    fn add(&mut self, item: &str) {
        let hash = xxh3_64(item.as_bytes());
        let index = (hash >> (64 - BITS)) as usize;  // 上位ビットでインデックスを決定
        let rank = Self::count_leading_zeros(hash, BITS);
        self.registers[index] = cmp::max(self.registers[index], rank);
    }

    /// カーディナリーを推定
    fn estimate(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for &register in &self.registers {
            sum += 2.0_f64.powi(-(register as i32));
        }
        let estimate = ALPHA_M * (REGISTERS as f64).powi(2) / sum;

        // バイアス補正
        if estimate <= (5.0 / 2.0) * (REGISTERS as f64) {
            let zeros = self.registers.iter().filter(|&&x| x==0).count() as f64;
            if zeros > 0.0 {
                (REGISTERS as f64) * (REGISTERS as f64 / zeros).ln()
            } else {
                estimate
            }
        } else {
            estimate
        }
    }
}
fn main() {
    let mut hll = HyperLogLog::new();

    // テストデータ
    let data = vec!["apple", "banana", "cherry", "date", "apple", "banana"];
    for item in data {
        hll.add(item);
    }

    // 推定結果を出力
    let estimate_count = hll.estimate();
    println!("推定されたユニークな要素数: {}", estimate_count);
}
