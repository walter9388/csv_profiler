use csv_profiler::profiler;

fn main() {
    let filename = "./test_csvs/test_1.csv";
    profiler::profile(filename);
    let filename = "./test_csvs/test_2.csv";
    profiler::profile(filename);
    let filename = "./test_csvs/test_3.csv";
    profiler::profile(filename);
}
