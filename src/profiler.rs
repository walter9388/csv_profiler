use crate::datatypes::datatypes;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
extern crate bytecount;

// #[derive(Debug)]
// enum Datatype {
//     Integer,
//     String,
// }
struct Test {
    datatypes: Option<datatypes::Datatype>,
}
impl datatypes::IdentifyType for Test {
    fn get_datatype(&self) -> &Option<datatypes::Datatype> {
        &self.datatypes
    }
    fn set_datatype(&mut self, a: Option<datatypes::Datatype>) {
        self.datatypes = a;
    }
}

// #[derive(Debug)]
// struct Column<'a> {
//     name: &'a str,
//     datatype: datatypes::Datatype,
// }

// fn read_file(filename: &str) -> io::Result<File> {
//     let f = File::open(filename)?;
//     Ok(f)
// }
fn get_buf(filename: &str) -> BufReader<File> {
    let f = match File::open(filename) {
        Err(why) => panic!("Can't open file: {} ({})", filename, why),
        Ok(file) => file,
    };
    BufReader::with_capacity(1024 * 32, f)
}

// fn get_buf(file: File) -> BufReader<File> {
//     BufReader::with_capacity(1024 * 32, file)
// }

pub fn profile(filename: &str) {
    println!("{:?}", get_headers(get_buf(filename)));
    for i in get_headers(get_buf(filename)).iter() {
        println!("{}", i);
    }

    println!("{}", count(get_buf(filename)));
    println!("{}", count_alt(get_buf(filename)));
    println!("{:?}", first_col(get_buf(filename)));
    println!("{:?}", all_col(get_buf(filename)));
    println!("{}", count(get_buf(filename)));
    println!("{}", count_alt(get_buf(filename)));
    println!("{:?}", count_eclark(get_buf(filename)));
    first_col(get_buf(filename));
    all_col(get_buf(filename));

    let _a = Test { datatypes: None };
}

// https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust

#[inline]
pub fn count(buf: BufReader<File>) -> i32 {
    buf.lines().into_iter().count() as i32
}
#[inline]
pub fn count_alt(buf: BufReader<File>) -> i32 {
    buf.lines()
        .into_iter()
        .enumerate()
        .fold(0, |sum, _| sum + 1)
}
// pub fn count_eclark(buf: BufReader<File>) -> i32 {
//     // inspired by eclarke: https://github.com/eclarke/linecount/blob/master/src/lib.rs
//     buf.lines().into_iter().count() as i32
#[inline]
pub fn count_eclark(mut reader: BufReader<File>) -> Result<i32, io::Error> {
    let mut count = 0;
    loop {
        let len = {
            let buf = reader.fill_buf()?;
            if buf.is_empty() {
                break;
            }
            count += bytecount::count(&buf, b'\n');
            buf.len()
        };
        reader.consume(len);
    }
    Ok(count as i32)
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
