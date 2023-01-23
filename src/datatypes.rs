pub mod datatypes {
    use std::str::FromStr;

    #[derive(PartialEq, Debug)]
    // #[cfg_attr(test, derive(strum_macros::EnumIter))]
    pub enum Datatype {
        String,
        Float,
        Integer,
        Datetime,
    }

    pub trait IdentifyType {
        fn get_datatype(&self) -> &Option<Datatype>;
        fn set_datatype(&mut self, datatype: Option<Datatype>);

        fn identify_type_(&mut self, input: &str) {
            // check if empty
            if is_empty(input) {
                return;
            };

            // check existing first (maintain if empty)
            match self.get_datatype() {
                Some(Datatype::String) => return,
                Some(Datatype::Float) => {
                    if is_::<f32>(input) {
                        return;
                    }
                }
                Some(Datatype::Integer) => {
                    if is_::<i32>(input) {
                        return;
                    }
                }
                Some(Datatype::Datetime) => {
                    if is_datetime(input) {
                        return;
                    }
                }
                None => (),
            }

            // check to see if Datatype::Integer
            if is_type(input, is_::<i32>, Datatype::Integer, &self.get_datatype()) {
                self.set_datatype(Some(Datatype::Integer));
                return;
            }
            // check to see if Datatype::Float
            if is_type(input, is_::<f32>, Datatype::Float, &self.get_datatype()) {
                self.set_datatype(Some(Datatype::Float));
                return;
            }
            // check to see if Datatype::Datetime
            if is_type(input, is_datetime, Datatype::Datetime, &self.get_datatype()) {
                self.set_datatype(Some(Datatype::Datetime));
                return;
            }
            // check to see if Datatype::Empty
            if input.len() == 0 {
                self.set_datatype(None);
                return;
            };
            // if nothing else, leave as Datatype::String
            self.set_datatype(Some(Datatype::String));
        }
    }

    fn is_type(
        input: &str,
        func: fn(&str) -> bool,
        datatype: Datatype,
        assumption: &Option<Datatype>,
    ) -> bool {
        if *assumption != Some(datatype) {
            println!("funcuffasdf {:?}", func(input));
            return func(input);
        }
        false
    }

    fn is_empty(x: &str) -> bool {
        x.len() == 0
    }

    fn is_<T>(x: &str) -> bool
    where
        T: FromStr,
    {
        x.parse::<T>().is_ok()
    }

    fn is_datetime(_x: &str) -> bool {
        false
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        // // strum is used to iterate through enum for the tests https://stackoverflow.com/a/55056427
        // use strum::IntoEnumIterator;
        // use strum_macros::EnumIter;

        // test struct with impl IdentifyType
        struct Test {
            datatypes: Option<Datatype>,
        }
        impl IdentifyType for Test {
            fn get_datatype(&self) -> &Option<Datatype> {
                &self.datatypes
            }
            fn set_datatype(&mut self, a: Option<Datatype>) {
                self.datatypes = a;
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
        //         // for i in Datatype::iter() {
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
            check!("Mary had a little lamb!!?!@][1234x", Some(Datatype::String));
            check!("123", Some(Datatype::Integer));
            check!("123.0", Some(Datatype::Float));
            check!("0", Some(Datatype::Integer));
            check!("-123.0", Some(Datatype::Float));
            check!("-123", Some(Datatype::Integer));
        }

        #[test]
        fn with_assumptions() {
            check!("", Some(Datatype::Float), Some(Datatype::Float));
        }

        #[test]
        fn float() {
            check!("123.0", Some(Datatype::Float));
        }
    }
}

// // // OLD // // //
// pub fn identify_type(input: &str, current_assumption: Option<Datatype>) -> Option<Datatype> {
//     // check if empty
//     if is_empty(input) {
//         return current_assumption;
//     };
//
//     // check current_assumption first (maintain if empty)
//     match current_assumption {
//         Some(Datatype::String) => return current_assumption,
//         Some(Datatype::Float) => {
//             if is_::<f32>(input) {
//                 return current_assumption;
//             }
//         }
//         Some(Datatype::Integer) => {
//             if is_::<i32>(input) {
//                 return current_assumption;
//             }
//         }
//         Some(Datatype::Datetime) => {
//             if is_datetime(input) {
//                 return current_assumption;
//             }
//         }
//         None => (),
//     }
//
//     // check to see if Datatype::Integer
//     if is_type(input, is_::<i32>, Datatype::Integer, &current_assumption) {
//         return Some(Datatype::Integer);
//     }
//     // check to see if Datatype::Float
//     if is_type(input, is_::<f32>, Datatype::Float, &current_assumption) {
//         return Some(Datatype::Float);
//     }
//     // check to see if Datatype::Datetime
//     if is_type(input, is_datetime, Datatype::Datetime, &current_assumption) {
//         return Some(Datatype::Datetime);
//     }
//     // check to see if Datatype::Empty
//     if input.len() == 0 {
//         return None;
//     };
//     // if nothing else, leave as Datatype::String
//     Some(Datatype::String)
// }
