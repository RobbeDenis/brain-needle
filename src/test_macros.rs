
////////////////////////////
////    assert macros   ////
////////////////////////////

#[macro_export]
macro_rules! bn_fnident {
    () => {
        {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            name.strip_suffix("::f").unwrap()
        }
    };
}

#[macro_export]
macro_rules! bn_assert_eq {
    ($expected:expr, $found:expr, $index:literal) => {{
        bn_assert_eq!(crate::bn_fnident!(), $expected, $found, $index);
    }};
    ($expected:expr, $found:expr) => {{
        bn_assert_eq!(crate::bn_fnident!(), $expected, $found);
    }};
    ($func:expr, $expected:expr, $found:expr) => {{
        let exp = &$expected;
        let fnd = &$found;
        if exp != fnd {
            use ansi_term::Color::{Green, Red, Yellow};
            panic!(
                "\nTest failed at {}\n{}\n{}: {:?}\n{}: {:?}\n",
                format!("{}", format!("{}:{}:{}:", file!(), line!(), column!())),
                Yellow.bold().paint(format!("[{}]", $func)),
                Green.paint("Expected"), exp,
                Red.paint("   Found"), fnd
            );
        }
    }};
    ($func:expr, $expected:expr, $found:expr, $index:literal) => {{
        let exp = &$expected;
        let fnd = &$found;
        if exp != fnd {
            use ansi_term::Color::{Green, Red, Yellow};
            panic!(
                "\nTest failed at {}\n{}\n{}: {:?} at index {}\n{}: {:?}\n",
                format!("{}", format!("{}:{}:{}:", file!(), line!(), column!())),
                Yellow.bold().paint(format!("[{}]", $func)),
                Green.paint("Expected"), exp, $index,
                Red.paint("   Found"), fnd
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
                    "\nTest failed at {}\n{}\n{}\n",
                    format!("{}", format!("{}:{}:{}:", file!(), line!(), column!())),
                    Yellow.bold().paint(format!("[{}]", crate::bn_fnident!())),
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
        match $result {
            Ok(_) => {
                use ansi_term::Color::{Green, Red, Yellow};
                panic!(
                    "\nTest failed at {}\n{}\n{}:\n{}\n{}\n",
                    format!("{}", format!("{}:{}:{}:", file!(), line!(), column!())),
                    Yellow.bold().paint(format!("[{}]", crate::bn_fnident!())),
                    Green.paint("Expected error"), $error,
                    Red.paint("None was thrown")
                );
            }
            Err(err) => {
                if err != $error {
                    use ansi_term::Color::{Green, Red, Yellow};
                    panic!(
                        "\nTest failed at {}\n{}\n{}:\n{}\n{}:\n{}\n",
                        format!("{}", format!("{}:{}:{}:", file!(), line!(), column!())),
                        Yellow.bold().paint(format!("[{}]", crate::bn_fnident!())),
                        Green.paint("Expected error"), $error,
                        Red.paint("Found"), err
                    );
                }
    }}}};
}

#[macro_export]
macro_rules! bn_some {
    ($value:expr) => {{
        match $value {
            None => {
                use ansi_term::Color::Yellow;
                panic!(
                    "\nTest failed at {}\n{}\n",
                    format!("{}", format!("{}:{}:{}:", file!(), line!(), column!())),
                    Yellow.bold().paint(format!("[{}]", crate::bn_fnident!()))
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
        vec![                                                           // index
            Add(10),    Loop,       Right(1),   Add(1),     Right(1),   // 4
            Add(3),     Right(1),   Add(7),     Right(1),   Add(10),    // 9
            Left(4),    Sub(1),     Back,       Right(3),   Add(2),     // 14
            Out,        Right(1),   Add(1),     Out,        Add(7),     // 19
            Out,        Out,        Add(3),     Out,        Left(2),    // 24
            Add(2),     Out,        Right(1),   Add(15),    Out,        // 29
            Right(1),   Out,        Add(3),     Out,        Sub(6),     // 34
            Out,        Sub(8),     Out                                 // 37
        ]
    }};
}

#[macro_export]
macro_rules! hello_world_instr_tree {
    () => {{
        use $crate::bncore::Node::*;
        vec![                                                                       // index
            Add(10),    JumpIfZero(12), Right(1),           Add(1),     Right(1),   // 4
            Add(3),     Right(1),       Add(7),             Right(1),   Add(10),    // 9
            Left(4),    Sub(1),         JumpIfNotZero(1),   Right(3),   Add(2),     // 14
            Out,        Right(1),       Add(1),             Out,        Add(7),     // 19
            Out,        Out,            Add(3),             Out,        Left(2),    // 24
            Add(2),     Out,            Right(1),           Add(15),    Out,        // 29
            Right(1),   Out,            Add(3),             Out,        Sub(6),     // 34
            Out,        Sub(8),         Out                                         // 37
        ]
    }};
}