use crate::{
    datatypes::datatypes::{self, IdentifyType},
    stats::stats::count,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
pub struct Column {
    name: String,
    datatype: Option<datatypes::RustDatatype>,
}

enum InputType {
    File(&'static str),
    String,
}

pub struct Profile {
    inputtype: InputType,
    columns: Option<Vec<Column>>,
    buf: Option<BufReader<Box<dyn io::Read>>>,
}

impl Default for Profile {
    fn default() -> Profile {
        Profile {
            inputtype: InputType::String,
            columns: None,
            buf: None,
        }
    }
}
// impl io::Read for Profile {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
//         Ok(buf.len() as usize)
//     }
// }
// impl BufRead for Profile {
//     fn fill_buf(&mut self) -> io::Result<&[u8]> {
//         Ok(&[1 as u8])
//     }
//     fn consume(&mut self, amt: usize) {}
// }
impl datatypes::IdentifyType for Column {
    fn get_datatype(&self) -> &Option<datatypes::RustDatatype> {
        &self.datatype
    }
    fn set_datatype(&mut self, a: &Option<datatypes::RustDatatype>) {
        self.datatype = a.clone();
    }
}
const BUF_CAPACITY: usize = 1024 * 32;
impl Profile {
    pub fn new(s: &'static str) -> Self {
        Profile {
            inputtype: InputType::String,
            buf: Some(BufReader::with_capacity(
                BUF_CAPACITY,
                Box::new(s.as_bytes()),
            )),
            ..Default::default()
        }
    }

    // https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
    // fn get_buf_<R: io::Read>(mut self) {
    //     let f: Box<dyn io::Read> = match self.inputtype {
    //         InputType::String => Box::new(s.as_bytes()),
    //         InputType::File(filename) => Box::new(match File::open(filename) {
    //             Err(why) => panic!("Can't open file: {} ({})", filename, why),
    //             Ok(file) => file,
    //         }),
    //     };
    //     self.buf = Some(BufReader::with_capacity(BUF_CAPACITY, f));
    // }

    fn remake(mut self) {
        self.buf = match self.inputtype {
            InputType::String => None,
            InputType::File(s) => Profile::from(s).buf,
        };
    }

    pub fn count_only(self) -> Result<u32, io::Error> {
        // count
        let count_ = count(self.buf.unwrap());
        println!("{:?}", count_);

        // TODO: make the remake function work with the borrow checker
        // call remake (remakes buffer for files)
        // self.remake();

        // return count
        count_
    }

    pub fn profile(mut self) {
        let mut lines = self.buf.unwrap().lines();

        // get headers and assign to columns
        self.columns = Some(
            lines
                .next()
                .unwrap()
                .expect("couldn't get first line")
                .split(",")
                .into_iter()
                .map(|x| Column {
                    name: x.to_string(),
                    datatype: None,
                })
                .collect::<Vec<Column>>(),
        );

        // profile data
        for line in lines {
            for (x, y) in line
                .unwrap()
                .split(",")
                .into_iter()
                .zip(self.columns.as_mut().unwrap().into_iter())
            {
                y.identify_type(x);
                // println!("{} : {:?}", x, y.datatype);
            }
        }

        println!("{:?}", &self.columns);
    }
}

impl From<&'static str> for Profile {
    fn from(filename: &'static str) -> Self {
        // read file from filename
        let file = match File::open(filename) {
            Err(why) => panic!("Can't open file: {} ({})", filename, why),
            Ok(file) => Box::new(file) as Box<dyn io::Read>,
        };
        // make Profile struct with inputtype and buf updated correctly
        Profile {
            inputtype: InputType::File(filename),
            buf: Some(BufReader::with_capacity(BUF_CAPACITY, file)),
            ..Default::default()
        }
    }
}

// fn first_col<F: io::Read>(buf: BufReader<F>) -> Vec<String> {
//     let mut first_word: Vec<String> = Vec::new();
//     for line in buf.lines() {
//         first_word.push(line.unwrap().split(",").next().unwrap_or("").to_string());
//     }
//     first_word
// }
// fn all_col<F: io::Read>(buf: BufReader<F>) {
//     for line in buf.lines() {
//         for (i, col) in line.unwrap().split(",").enumerate() {
//             println!("{}: {}", i, col);
//         }
//     }
// }
// fn get_headers<F: io::Read>(buf: BufReader<F>) -> Vec<String> {
//     buf.lines()
//         .next()
//         .unwrap()
//         .expect("couldn't get first line")
//         .split(",")
//         .into_iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>()
// }
