
////////////////////////////
////    helper macros   ////
////////////////////////////

#[macro_export]
macro_rules! bn_fnident {
    () => {{
        #[inline]
        fn f() {}
        #[inline]
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }};
}

////////////////////////////
////    assert macros   ////
////////////////////////////

#[macro_export]
macro_rules! bn_assert_eq {
    ($expected:expr, $found:expr, $(header)? $(h)?: $header:expr) => {{
        bn_assert_eq!($expected, $found, None, $header);
    }};
    ($expected:expr, $found:expr, $ctx_hint:expr, $(header)? $(h)?: $header:expr) => {{
        bn_assert_eq!($expected, $found, Some($ctx_hint), $header);
    }};
    ($expected:expr, $found:expr, $ctx_hint:expr) => {{
        bn_assert_eq!($expected, $found, Some($ctx_hint), $crate::bn_fnident!());
    }};
    ($expected:expr, $found:expr) => {{
        bn_assert_eq!($expected, $found, None, $crate::bn_fnident!());
    }};
    ($expected:expr, $found:expr, $ctx_hint:expr, $header:expr) => {{
        if &$expected != &$found {
            use ansi_term::Color::{Green, Red, Yellow};
            let hint: Option<&str> = $ctx_hint;
            let hint = match hint {
                Some(v) => format!("{} ", v),
                None => String::new(),
            };
            panic!(
                "\nAssert equal failed at {}:{}:\n{}\n{}{}: {:?}\n{}{}: {:?}\n",
                file!(), line!(),
                Yellow.bold().paint(format!("[{}]", $header)),
                Green.paint("Expected "), hint, &$expected,
                Red.paint("   Found "), hint, &$found
            );
        }
    }};
}

#[macro_export]
macro_rules! bn_unwrap {
    ($result:expr) => {{
        match $result {
            Err(err) => {
                use ansi_term::Color::Yellow;
                panic!(
                    "\nUnwrap panicked at {}:{}:\n{}\n{}\n",
                    file!(), line!(),
                    Yellow.bold().paint(format!("[{}]", $crate::bn_fnident!())),
                    err
                );
            }
            Ok(value) => value
        }
    }};
}

#[macro_export]
macro_rules! bn_expect_error {
    ($result:expr, $error:expr) => {{
        use ansi_term::Color::{Green, Red, Yellow};
        match $result {
            Ok(_) => {
                panic!(
                    "\nExpected error at {}:{}:\n{}\n{}:\n{}\n{}\n",
                    file!(), line!(),
                    Yellow.bold().paint(format!("[{}]", $crate::bn_fnident!())),
                    Green.paint("Expected error"), $error,
                    Red.paint("None was thrown")
                );
            }
            Err(err) => { 
                if err != $error {
                panic!(
                    "\nExpected other error at {}:{}:\n{}\n{}:\n{}\n{}:\n{}\n",
                    file!(), line!(),
                    Yellow.bold().paint(format!("[{}]", $crate::bn_fnident!())),
                    Green.paint("Expected error"), $error,
                    Red.paint("Found"), err
                );
            }}
        }
    }};
}

#[macro_export]
macro_rules! bn_some {
    ($value:expr) => {{
        match $value {
            None => {
                use ansi_term::Color::Yellow;
                panic!(
                    "\nExpected Some at {}:{}:\n{}\n",
                    file!(), line!(),
                    Yellow.bold().paint(format!("[{}]", $crate::bn_fnident!()))
                );
            }
            Some(v) => v
        }
    }};
}

////////////////////////////
////  test case macros  ////
////////////////////////////

#[macro_export]
macro_rules! hello_world_tokens {
    () => {{
        use $crate::bncore::Token::*;
        vec!{                                                           // index
            Add(10),    Loop,       Right(1),   Add(1),     Right(1),   // 4
            Add(3),     Right(1),   Add(7),     Right(1),   Add(10),    // 9
            Left(4),    Sub(1),     Back,       Right(3),   Add(2),     // 14
            Out,        Right(1),   Add(1),     Out,        Add(7),     // 19
            Out,        Out,        Add(3),     Out,        Left(2),    // 24
            Add(2),     Out,        Right(1),   Add(15),    Out,        // 29
            Right(1),   Out,        Add(3),     Out,        Sub(6),     // 34
            Out,        Sub(8),     Out                                 // 37
        }
    }};
}

#[macro_export]
macro_rules! hello_world_instr_tree {
    () => {{
        use $crate::bncore::Node::*;
        vec!{                                                                       // index
            Add(10),    JumpIfZero(12), Right(1),           Add(1),     Right(1),   // 4
            Add(3),     Right(1),       Add(7),             Right(1),   Add(10),    // 9
            Left(4),    Sub(1),         JumpIfNotZero(1),   Right(3),   Add(2),     // 14
            Out,        Right(1),       Add(1),             Out,        Add(7),     // 19
            Out,        Out,            Add(3),             Out,        Left(2),    // 24
            Add(2),     Out,            Right(1),           Add(15),    Out,        // 29
            Right(1),   Out,            Add(3),             Out,        Sub(6),     // 34
            Out,        Sub(8),         Out                                         // 37
        }
    }};
}