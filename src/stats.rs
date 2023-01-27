pub mod stats {
    use std::{io, io::BufRead, io::BufReader};

    #[inline]
    pub fn count<F: io::Read>(buf: BufReader<F>) -> Result<u32, io::Error> {
        Ok(buf.lines().into_iter().count() as u32)
    }

    #[inline]
    pub fn count_alt<F: io::Read>(buf: BufReader<F>) -> Result<u32, io::Error> {
        Ok(buf
            .lines()
            .into_iter()
            .enumerate()
            .fold(0, |sum, _| sum + 1))
    }

    #[inline]
    pub fn count_eclark<F: io::Read>(mut reader: BufReader<F>) -> Result<u32, io::Error> {
        // inspired by eclarke: https://github.com/eclarke/linecount/blob/master/src/lib.rs
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
        Ok(count as u32)
    }

    fn get_buf<F: io::Read>(file: F) -> BufReader<F> {
        BufReader::with_capacity(1024 * 32, file)
    }

    #[cfg(test)]
    mod tests {
        use crate::stats::stats;

        const TEXT1: &[u8] = b"text\nwith\nfour\nlines\n";
        const TEXT2: &[u8] = b"text\nwith\nfour\nlines";

        #[test]
        fn count() {
            assert_eq!(stats::count(stats::get_buf(TEXT1)).unwrap(), 4);
        }
        #[test]
        fn count_no_end_newline() {
            assert_eq!(stats::count(stats::get_buf(TEXT2)).unwrap(), 4);
        }
        #[test]
        fn count_alt() {
            assert_eq!(stats::count_alt(stats::get_buf(TEXT1)).unwrap(), 4);
        }
        #[test]
        fn count_alt_no_end_newline() {
            assert_eq!(stats::count_alt(stats::get_buf(TEXT2)).unwrap(), 4);
        }
        #[test]
        fn count_eclark() {
            assert_eq!(stats::count_eclark(stats::get_buf(TEXT1)).unwrap(), 4);
        }
        #[test]
        #[should_panic] // known issue where it can't handle the last \n missing
        fn count_eclark_no_end_newline() {
            assert_eq!(stats::count_eclark(stats::get_buf(TEXT2)).unwrap(), 4);
        }
    }
}
