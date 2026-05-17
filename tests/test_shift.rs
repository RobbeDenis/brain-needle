
mod common;

use common::*;
use brain_needle::bncore::MAX_PROGRAM_BYTES;

const CTX: TargetContext = TargetContext {
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Custom
    };

#[test]
fn wrap_right()
{
    let output = Rc::new(RefCell::new(String::new()));
    {
        let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
            was_dest_created: false, 
            output: Rc::clone(&output)
        };
        
        use Node::*;
        let max = MAX_PROGRAM_BYTES as u16;
        let tree: Vec<Node> = vec![
            Add(64), Right(max), Out,
            Add(20), Right(max / 2), Add(80), Out,
            Right((max / 2) + 1), Sub(20), Out,
            Left(1), Out
        ];

        test_generate_output(tree, CTX, &mut factory);
    }
    let output = output.take();
    
    let expected = vec![64, 80, 108, 84];
    let expected = String::from_utf8(expected).unwrap(); 
    
    print_captured_output!(output, "WRAP RIGHT");
    assert!(output == expected, "Expected value: {}\nActual value: {}", expected, output);
}

#[test]
fn wrap_left()
{
    let output = Rc::new(RefCell::new(String::new()));
    {
        let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
            was_dest_created: false, 
            output: Rc::clone(&output)
        };
        
        use Node::*;
        let max = MAX_PROGRAM_BYTES as u16;
        let tree: Vec<Node> = vec![
            Add(64), Left(max), Out,
            Add(20), Left(max / 2), Add(80), Out,
            Left((max / 2) + 1), Sub(20), Out,
            Right(1), Out
        ];

        test_generate_output(tree, CTX, &mut factory);
    }
    let output = output.take();

    let expected = vec![64, 80, 108, 84];
    let expected = String::from_utf8(expected).unwrap();

    assert!(output == expected, "Expected value: {}\nActual value: {}", expected, output);
    print_captured_output!(output, "WRAP LEFT");
}