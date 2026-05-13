
#[macro_export]
macro_rules! print_captured_output {
    ($out:expr,$name:expr) => {
        use std::io::Write;
        use ansi_term::*;
        let s_test_name = Color::Cyan.bold().underline();
        let s_header = Color::White.bold();
        let s_data = Color::White.on(Color::Black).fg(Color::White);

        std::io::stdout().flush().unwrap();
        println!("\n{}", s_test_name.paint(format!("         {}         ", $name)));
        println!("{}", s_header.paint("[String]"));
        println!("{}", s_data.paint(&$out));
        println!("{}", s_header.paint("[Bytes]"));
        println!("{}", s_data.paint(format!("{:?}", $out.as_bytes())));
        println!("");
    };
}

#[macro_export]
macro_rules! hello_world_instr_tree {
    () => {{
        use brain_needle::bncore::Node::*;
        vec![
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
