use std::{fs::File, io::BufReader};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv_profiler;
// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }
// fn fibonacci2(n: u64) -> u64 {
//     let mut a = 0;
//     let mut b = 1;
//
//     match n {
//         0 => b,
//         _ => {
//             for _ in 0..n {
//                 let c = a + b;
//                 a = b;
//                 b = c;
//             }
//             b
//         }
//     }
// }
fn get_buf(filename: &str) -> BufReader<File> {
    let f = match File::open(filename) {
        Err(why) => panic!("Can't open file: {} ({})", filename, why),
        Ok(file) => file,
    };
    // BufReader::new(f);
    BufReader::with_capacity(1024 * 32, f)
}
const TEST_CSV_1: &str = "./test_csvs/test_1.csv";
const TEST_CSV_2: &str = "./test_csvs/test_2.csv";
fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 1", |b| b.iter(|| fibonacci(black_box(20))));
    // c.bench_function("fib 2", |b| b.iter(|| fibonacci2(black_box(20))));
    for (i, x) in vec![TEST_CSV_1, TEST_CSV_2].iter().enumerate() {
        c.bench_function(format!("count - test_csv{}", i + 1).as_str(), |b| {
            b.iter(|| csv_profiler::profiler::count(black_box(get_buf(x))));
        });
        c.bench_function(format!("count1 - test_csv{}", i + 1).as_str(), |b| {
            b.iter(|| csv_profiler::profiler::count_alt(black_box(get_buf(x))));
        });
        c.bench_function(format!("count_eclark - test_csv{}", i + 1).as_str(), |b| {
            b.iter(|| csv_profiler::profiler::count_eclark(black_box(get_buf(x))));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
