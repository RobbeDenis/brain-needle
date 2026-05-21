
////////////////////////////
////    assert macros   ////
////////////////////////////

#[macro_export]
macro_rules! bn_assert_eq {
    ($expected:expr, $found:expr) => {{
        let ident = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            name.strip_suffix("::f").unwrap()
        };
        bn_assert_eq!(ident, $expected, $found);
    }};
    ($ident:expr, $expected:expr, $found:expr) => {{
        let exp = $expected;
        let fnd = $found;
        if exp != fnd {
            use ansi_term::Color::{Green, Red, Yellow};
            
            panic!(
                "\nTest failed at {}\n{}\n{}: {:?}\n{}: {:?}\n",
                format!("{}", format!("{}:{}:{}:", file!(), line!(), column!())),
                Yellow.bold().paint(format!("[{}]", $ident)),
                Green.paint("Expected"), exp,
                Red.paint("   Found"), fnd
            );
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