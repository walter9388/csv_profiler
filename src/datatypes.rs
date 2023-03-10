pub mod datatypes {
    use chrono::{NaiveDate, NaiveDateTime};
    use std::{collections::HashSet, str::FromStr};

    #[derive(PartialEq, Clone, Debug)]
    pub enum RustDatatype {
        BOOL,
        U8,
        U16,
        U32,
        U64,
        U128,
        I8,
        I16,
        I32,
        I64,
        I128,
        F32,
        F64,
        CHAR,
        // DATETYPE(Vec<&'static str>),
        DATETYPE(HashSet<&'static str>),
        STRING,
    }

    pub trait IdentifyType {
        fn get_datatype(&self) -> &Option<RustDatatype>;
        fn set_datatype(&mut self, datatype: &Option<RustDatatype>);

        fn is_type(
            &mut self,
            input: &str,
            func: fn(&str) -> bool,
            datatype: &RustDatatype,
        ) -> bool {
            if *self.get_datatype() != Some(datatype.clone()) {
                // println!("{:?} {:?}", datatype, func(input));
                if func(input) {
                    self.set_datatype(&Some(datatype.clone()));
                    return true;
                }
            }
            false
        }

        fn identify_type(&mut self, input: &str) {
            // check if empty
            if input.len() == 0 {
                return;
            };

            // check existing first (maintain if empty)
            match self.get_datatype() {
                Some(RustDatatype::STRING) => return,
                Some(RustDatatype::DATETYPE(s)) => {
                    // only do if s.len() > 1
                    if s.len() > 0 {
                        match is_datetime(input, Some(s.clone())) {
                            Some(_datetime) => {
                                // update datetime HashSet if it has been reduced
                                if s.len() != _datetime.len() {
                                    self.set_datatype(&Some(RustDatatype::DATETYPE(_datetime)));
                                }
                            }
                            None => self.set_datatype(&Some(RustDatatype::STRING)),
                        }
                        return;
                    }
                }
                Some(RustDatatype::CHAR) => {
                    if is_char(input) {
                        return;
                    }
                }
                Some(RustDatatype::F64) => {
                    if is_::<f64>(input) {
                        return;
                    }
                }
                Some(RustDatatype::F32) => {
                    if is_::<f32>(input) {
                        return;
                    }
                }
                Some(RustDatatype::I128) => {
                    if is_::<i128>(input) {
                        return;
                    }
                }
                Some(RustDatatype::I64) => {
                    if is_::<i64>(input) {
                        return;
                    }
                }
                Some(RustDatatype::I32) => {
                    if is_::<i32>(input) {
                        return;
                    }
                }
                Some(RustDatatype::I16) => {
                    if is_::<i16>(input) {
                        return;
                    }
                }
                Some(RustDatatype::I8) => {
                    if is_::<i8>(input) {
                        return;
                    }
                }
                Some(RustDatatype::U128) => {
                    if is_::<u128>(input) {
                        return;
                    }
                }
                Some(RustDatatype::U64) => {
                    if is_::<u64>(input) {
                        return;
                    }
                }
                Some(RustDatatype::U32) => {
                    if is_::<u32>(input) {
                        return;
                    }
                }
                Some(RustDatatype::U16) => {
                    if is_::<u16>(input) {
                        return;
                    }
                }
                Some(RustDatatype::U8) => {
                    if is_::<u8>(input) {
                        return;
                    }
                }
                Some(RustDatatype::BOOL) => {
                    if is_bool(input) {
                        return;
                    }
                }
                None => (),
            }

            // check to see if input &str can be parsed to any of the following
            if self.is_type(input, is_bool, &RustDatatype::BOOL) {
                return;
            }
            if self.is_type(input, is_::<u8>, &RustDatatype::U8) {
                return;
            }
            if self.is_type(input, is_::<u16>, &RustDatatype::U16) {
                return;
            }
            if self.is_type(input, is_::<u32>, &RustDatatype::U32) {
                return;
            }
            if self.is_type(input, is_::<u64>, &RustDatatype::U64) {
                return;
            }
            if self.is_type(input, is_::<u128>, &RustDatatype::U128) {
                return;
            }
            if self.is_type(input, is_::<i8>, &RustDatatype::I8) {
                return;
            }
            if self.is_type(input, is_::<i16>, &RustDatatype::I16) {
                return;
            }
            if self.is_type(input, is_::<i32>, &RustDatatype::I32) {
                return;
            }
            if self.is_type(input, is_::<i64>, &RustDatatype::I64) {
                return;
            }
            if self.is_type(input, is_::<i128>, &RustDatatype::I128) {
                return;
            }
            if self.is_type(input, is_::<f32>, &RustDatatype::F32) {
                return;
            }
            if self.is_type(input, is_::<f64>, &RustDatatype::F64) {
                return;
            }
            // only check for char if previously a type with a possibiliy of length=1 (or None)
            match self.get_datatype() {
                Some(RustDatatype::U8) | Some(RustDatatype::BOOL) | None => {
                    if self.is_type(input, is_char, &RustDatatype::CHAR) {
                        return;
                    }
                }
                _ => (),
            }
            if let Some(datetime) = is_datetime(input, None) {
                self.set_datatype(&Some(RustDatatype::DATETYPE(datetime)));
                return;
            }
            if input.len() == 0 {
                self.set_datatype(&None);
                return;
            }

            // if nothing else, leave as RustDatatype::String self.set_datatype(Some(RustDatatype::String));
            self.set_datatype(&Some(RustDatatype::STRING));
        }
    }

    fn is_char(_x: &str) -> bool {
        if _x.len() == 1 {
            return true;
        }
        false
    }

    const BOOL_POSSIBILITIES: [&'static str; 6] = ["0", "1", "f", "t", "true", "false"];
    fn is_bool(_x: &str) -> bool {
        BOOL_POSSIBILITIES.contains(&_x.to_lowercase().as_str())
    }

    fn is_<T>(x: &str) -> bool
    where
        T: FromStr,
    {
        x.parse::<T>().is_ok()
    }

    fn is_datetime(
        _x: &str,
        format: Option<HashSet<&'static str>>,
    ) -> Option<HashSet<&'static str>> {
        // let temp = NaiveDate::parse_from_str("2001-01-01", "%Y-%m-%d");
        // println!("{:?}", temp);

        let mut a = format.clone().unwrap_or(HashSet::new());

        // if format is not empty, just check those ones
        if a.len() > 0 {
            for i in a.clone() {
                match NaiveDate::parse_from_str(_x, i) {
                    Ok(_) => (),
                    Err(_) => {
                        a.remove(i);
                    }
                };
            }
        }
        // otherwise check everything (combinations found here:
        // https://docs.rs/chrono/0.4.23/chrono/format/strftime/index.html)
        // TODO: There is probs a better way to do this rather than list everything out...
        else {
            match _x.len() {
                10 => {
                    date_check(&mut a, _x, "%Y-%m-%d");
                    date_check(&mut a, _x, "%Y/%m/%d");
                    date_check(&mut a, _x, "%Y.%m.%d");
                    date_check(&mut a, _x, "%Y-%d-%m");
                    date_check(&mut a, _x, "%Y/%d/%m");
                    date_check(&mut a, _x, "%Y.%d.%m");
                }
                _ => (),
            }
        }

        if a.len() > 0 {
            return Some(a);
        }
        None
    }

    fn date_check(aa: &mut HashSet<&'static str>, x: &str, f: &'static str) {
        if NaiveDate::parse_from_str(x, f).is_ok() {
            aa.insert(f);
        }
    }

    fn datetime_check(aa: &mut HashSet<&'static str>, x: &str, f: &'static str) {
        if NaiveDateTime::parse_from_str(x, f).is_ok() {
            aa.insert(f);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        // // strum is used to iterate through enum for the tests https://stackoverflow.com/a/55056427
        // use strum::IntoEnumIterator;
        // use strum_macros::EnumIter;

        // test struct with impl IdentifyType
        struct Test {
            datatypes: Option<RustDatatype>,
        }
        impl IdentifyType for Test {
            fn get_datatype(&self) -> &Option<RustDatatype> {
                &self.datatypes
            }
            fn set_datatype(&mut self, a: &Option<RustDatatype>) {
                self.datatypes = a.clone();
            }
        }

        // macro for making lots of checks simple
        macro_rules! check {
            ($input:expr, $expected:expr) => {
                let mut a = Test { datatypes: None };
                // println!("before: {:?}", a.datatypes);
                a.identify_type($input);
                // println!("after: {:?}", a.datatypes);
                assert_eq!(a.datatypes, $expected);
            };
            ($input:expr, $expected:expr, $assumed:expr) => {
                let mut a = Test {
                    datatypes: $assumed,
                };
                // println!("before: {:?}", a.datatypes);
                a.identify_type($input);
                // println!("after: {:?}", a.datatypes);
                assert_eq!(a.datatypes, $expected);
            };
        }

        #[test]
        fn simple_checks_numbers() {
            // (test, expected_result)
            check!(&u8::max_value().to_string(), Some(RustDatatype::U8));
            check!(
                &((u8::max_value() as u128 + 1).to_string()),
                Some(RustDatatype::U16)
            );
            check!(
                &((u16::max_value() as u128 + 1).to_string()),
                Some(RustDatatype::U32)
            );
            check!(
                &((u32::max_value() as u128 + 1).to_string()),
                Some(RustDatatype::U64)
            );
            check!(
                &((u64::max_value() as u128 + 1).to_string()),
                Some(RustDatatype::U128)
            );
            check!(
                &((u128::max_value() as u128).to_string()),
                Some(RustDatatype::U128)
            );
            check!("123", Some(RustDatatype::U8));
            check!("123.0", Some(RustDatatype::F32));
            check!("1210", Some(RustDatatype::U16));
            check!("0", Some(RustDatatype::BOOL));
            check!("-123.0", Some(RustDatatype::F32));
            check!("-123", Some(RustDatatype::I8));
        }

        #[test]
        fn simple_checks_other() {
            // (test, expected_result)
            check!("", None);
            check!(
                "Mary had a little lamb!!?!@][1234x",
                Some(RustDatatype::STRING)
            );
            check!("0", Some(RustDatatype::BOOL));
            check!("1", Some(RustDatatype::BOOL));
            check!("false", Some(RustDatatype::BOOL));
            check!("TRUE", Some(RustDatatype::BOOL));
            check!("tRuE", Some(RustDatatype::BOOL));
            check!("T", Some(RustDatatype::BOOL));
            check!("@", Some(RustDatatype::CHAR));
            check!(" ", Some(RustDatatype::CHAR));
            check!("\t", Some(RustDatatype::CHAR));
        }

        #[test]
        fn with_assumptions() {
            // (test, expected_result, assumption)
            check!("", Some(RustDatatype::F32), Some(RustDatatype::F32));
            check!("aa", Some(RustDatatype::STRING), Some(RustDatatype::F32));
            check!("aa", Some(RustDatatype::STRING), Some(RustDatatype::F32));
            check!("a", Some(RustDatatype::CHAR), Some(RustDatatype::U8));
            check!("T", Some(RustDatatype::BOOL), None);
            check!("T", Some(RustDatatype::BOOL), Some(RustDatatype::BOOL));
            check!("T", Some(RustDatatype::CHAR), Some(RustDatatype::CHAR));
            check!("T", Some(RustDatatype::STRING), Some(RustDatatype::STRING));
            check!("0", Some(RustDatatype::BOOL), None);
            check!("0", Some(RustDatatype::BOOL), Some(RustDatatype::BOOL));
            check!("0", Some(RustDatatype::U8), Some(RustDatatype::U8));
            check!("0", Some(RustDatatype::CHAR), Some(RustDatatype::CHAR));
            check!("0", Some(RustDatatype::STRING), Some(RustDatatype::STRING));
        }

        #[test]
        fn datetime_checks() {
            // (test, expected_result)
            check!(
                "2001-01-01",
                Some(RustDatatype::DATETYPE(HashSet::from([
                    "%Y-%m-%d", "%Y-%d-%m"
                ])))
            );
            check!(
                "2001-13-01",
                Some(RustDatatype::DATETYPE(HashSet::from(["%Y-%d-%m"])))
            );
            check!(
                "2001-01-13",
                Some(RustDatatype::DATETYPE(HashSet::from(["%Y-%m-%d"])))
            );
            check!(
                "2001/01/13",
                Some(RustDatatype::DATETYPE(HashSet::from(["%Y/%m/%d"])))
            );
        }
        #[test]

        fn datetime_checks_with_assumption() {
            // (test, expected_result, assumption)
            check!(
                "2001-01-01",
                Some(RustDatatype::DATETYPE(HashSet::from(["%Y-%m-%d"]))),
                Some(RustDatatype::DATETYPE(HashSet::from(["%Y-%m-%d"])))
            );
            check!(
                "2001-01-21",
                Some(RustDatatype::DATETYPE(HashSet::from(["%Y-%m-%d"]))),
                Some(RustDatatype::DATETYPE(HashSet::from([
                    "%Y-%m-%d", "%Y-%d-%m"
                ])))
            );
            check!(
                "2001-13-01",
                Some(RustDatatype::STRING),
                Some(RustDatatype::DATETYPE(HashSet::from(["%Y-%m-%d"])))
            );
            check!(
                "2001-01-13",
                Some(RustDatatype::STRING),
                Some(RustDatatype::DATETYPE(HashSet::from(["%Y-%d-%m"])))
            );
        }
    }
}
