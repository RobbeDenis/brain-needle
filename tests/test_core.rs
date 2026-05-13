
mod common;
use common::*;

use ansi_term::*;
use std::io::Write;


#[test]
fn current_dev_test()
{
    let ctx = TargetContext {
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Custom
    };
    
    let captured_output = Rc::new(RefCell::new(String::new()));
    {
        let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
            was_dest_created: false, 
            output: Rc::clone(&captured_output)
        };
        
        test_generate_output(hello_world_instr_tree!(), ctx, &mut factory);
    }
    let captured_output = captured_output.take();
    
    let s_test = Color::Cyan.bold().underline();
    let s_header = Color::White.bold();
    let s_data = Color::White.on(Color::Black).fg(Color::White);

    println!("\n{}", s_test.paint("         CURRENT DEV         "));
    println!("{}", s_header.paint("[String]"));
    println!("{}", s_data.paint(&captured_output));
    println!("{}", s_header.paint("[Bytes]"));
    println!("{}", s_data.paint(format!("{:?}", captured_output.as_bytes())));
    print!("");
    println!("");
    std::io::stdout().flush().unwrap();
}

#[test]
fn custom_emit_factory_creation()
{
    let ctx = TargetContext {
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Custom
    };
    
    let captured_output = Rc::new(RefCell::new(String::new()));
    let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
        was_dest_created: false, 
        output: Rc::clone(&captured_output)
    };

    test_generate_output(hello_world_instr_tree!(), ctx, &mut factory);
    assert!(factory.was_dest_created, "create_dest was never called");
}