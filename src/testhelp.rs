
#[macro_export]
macro_rules! hello_world_tokens {
    () => {{
        use $crate::bnlex::Token::*;
        vec![
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
        use $crate::bnparse::InstrNode::*;
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