pub mod datatypes {
    use std::str::FromStr;

    #[derive(PartialEq, Clone, Copy, Debug)]
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
        DATETYPE(&'static str),
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
            if *self.get_datatype() != Some(*datatype) {
                println!("{:?} {:?}", datatype, func(input));
                if func(input) {
                    self.set_datatype(&Some(*datatype));
                    return true;
                }
            }
            false
        }

        fn identify_type_(&mut self, input: &str) {
            // check if empty
            if input.len() == 0 {
                return;
            };

            // check existing first (maintain if empty)
            match self.get_datatype() {
                Some(RustDatatype::STRING) => return,
                Some(RustDatatype::DATETYPE(s)) => {
                    if let Some(_datetime) = is_datetime(input, Some(s)) {
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
            if self.is_type(input, is_char, &RustDatatype::CHAR) {
                return;
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
        false
    }

    fn is_bool(_x: &str) -> bool {
        false
    }

    fn is_<T>(x: &str) -> bool
    where
        T: FromStr,
    {
        x.parse::<T>().is_ok()
    }

    fn is_datetime(_x: &str, _format: Option<&str>) -> Option<&'static str> {
        if _x == "hello" {
            return Some("datetime string");
        }
        None
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
                self.datatypes = *a;
            }
        }

        // macro for making lots of checks simple
        macro_rules! check {
            ($input:expr, $expected:expr) => {
                let mut a = Test { datatypes: None };
                // println!("before: {:?}", a.datatypes);
                a.identify_type_($input);
                // println!("after: {:?}", a.datatypes);
                assert_eq!(a.datatypes, $expected);
            };
            ($input:expr, $expected:expr, $assumed:expr) => {
                let mut a = Test {
                    datatypes: $assumed,
                };
                // println!("before: {:?}", a.datatypes);
                a.identify_type_($input);
                // println!("after: {:?}", a.datatypes);
                assert_eq!(a.datatypes, $expected);
            };
        }
        // // // OLD // // //
        // macro_rules! check {
        //     ($input:expr, $expected:expr) => {
        //         assert_eq!(identify_type($input, None), $expected);
        //         // for i in RustDatatype::iter() {
        //         //     assert_eq!(identify_type($input, Some(i)), $expected);
        //         // }
        //     };
        //     ($input:expr, $expected:expr, $assumed:expr) => {
        //         assert_eq!(identify_type($input, $assumed), $expected);
        //     };
        // }

        #[test]
        fn simple_checks() {
            check!("", None);
            check!(
                "Mary had a little lamb!!?!@][1234x",
                Some(RustDatatype::STRING)
            );
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
            check!("0", Some(RustDatatype::U8));
            check!("-123.0", Some(RustDatatype::F32));
            check!("-123", Some(RustDatatype::I8));
        }

        #[test]
        fn with_assumptions() {
            check!("", Some(RustDatatype::F32), Some(RustDatatype::F32));
            check!("aa", Some(RustDatatype::STRING), Some(RustDatatype::F32));
        }
    }
}

// // // // OLD // // //
// // pub fn identify_type(input: &str, current_assumption: Option<Datatype>) -> Option<Datatype> {
// //     // check if empty
// //     if is_empty(input) {
// //         return current_assumption;
// //     };
// //
// //     // check current_assumption first (maintain if empty)
// //     match current_assumption {
// //         Some(Datatype::String) => return current_assumption,
// //         Some(Datatype::Float) => {
// //             if is_::<f32>(input) {
// //                 return current_assumption;
// //             }
// //         }
// //         Some(Datatype::Integer) => {
// //             if is_::<i32>(input) {
// //                 return current_assumption;
// //             }
// //         }
// //         Some(Datatype::Datetime) => {
// //             if is_datetime(input) {
// //                 return current_assumption;
// //             }
// //         }
// //         None => (),
// //     }
// //
// //     // check to see if Datatype::Integer
// //     if is_type(input, is_::<i32>, Datatype::Integer, &current_assumption) {
// //         return Some(Datatype::Integer);
// //     }
// //     // check to see if Datatype::Float
// //     if is_type(input, is_::<f32>, Datatype::Float, &current_assumption) {
// //         return Some(Datatype::Float);
// //     }
// //     // check to see if Datatype::Datetime
// //     if is_type(input, is_datetime, Datatype::Datetime, &current_assumption) {
// //         return Some(Datatype::Datetime);
// //     }
// //     // check to see if Datatype::Empty
// //     if input.len() == 0 {
// //         return None;
// //     };
// //     // if nothing else, leave as Datatype::String
// //     Some(Datatype::String)
// // }
