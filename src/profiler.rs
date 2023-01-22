use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Datatype {
    Integer,
    String,
}

#[derive(Debug)]
struct Column<'a> {
    name: &'a str,
    datatype: Datatype,
}

// fn read_file(filename: &str) -> io::Result<File> {
//     let f = File::open(filename)?;
//     Ok(f)
// }
fn read_file(filename: &str) -> File {
    match File::open(filename) {
        Err(why) => panic!("Can't open file: {} ({})", filename, why),
        Ok(file) => return file,
    };
}

fn get_buf(file: File) -> BufReader<File> {
    BufReader::new(file)
}

pub fn profile(filename: &str) {
    // let a = Column {
    //     name: "col",
    //     datatype: Datatype::Integer,
    // };
    // println!("{:?}", a);

    // add a file
    let f = read_file(filename);
    // let buf = BufReader::new(&f);
    // for line in buf.lines() {
    //     println!("{}", line.unwrap());
    // }

    println!("{:?}", get_headers(get_buf(read_file(filename))));
    for i in get_headers(get_buf(read_file(filename))).iter() {
        println!("{}", i);
    }

    // println!("{}", count(get_buf(read_file(filename))));
    // println!("{}", count1(get_buf(read_file(filename))));
    // println!("{:?}", first_col(get_buf(read_file(filename))));
    // println!("{:?}", all_col(get_buf(read_file(filename))));
}

// https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust

fn count(buf: BufReader<File>) -> i32 {
    buf.lines().into_iter().count() as i32
}
fn count1(buf: BufReader<File>) -> i32 {
    buf.lines()
        .into_iter()
        .enumerate()
        .fold(0, |sum, _| sum + 1)
}
fn first_col(buf: BufReader<File>) -> Vec<String> {
    let mut first_word: Vec<String> = Vec::new();
    for line in buf.lines() {
        first_word.push(line.unwrap().split(",").next().unwrap_or("").to_string());
    }
    first_word
}
fn all_col(buf: BufReader<File>) {
    for line in buf.lines() {
        for (i, col) in line.unwrap().split(",").enumerate() {
            println!("{}: {}", i, col);
        }
    }
}
fn get_headers(buf: BufReader<File>) -> Vec<String> {
    buf.lines()
        .next()
        .unwrap()
        .expect("couldn't get first line")
        .split(",")
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}
