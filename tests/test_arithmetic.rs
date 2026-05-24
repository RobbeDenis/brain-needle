
mod common;

use common::*;

const CTX: TargetContext = TargetContext {
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Custom
    };

#[allow(unused)]
fn expected_char_as_string(num: u8) -> String
{
    return (num as char).to_string();
}

#[test]
fn wrap_neg()
{
    let output = Rc::new(RefCell::new(String::new()));
    {
        let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
            was_dest_created: false, 
            output: Rc::clone(&output)
        };
        
        use BNNode::*;
        let tree: Vec<BNNode> = vec![
            Sub(1), Out(1), Right(1), Sub(24708), Out(1), 
            Right(1), Sub(40), Sub(40), Out(1),
            Right(1), Sub(489), Sub(123), Sub(100), Out(1)
        ];

        test_generate_output(tree, CTX, &mut factory);
    }
    let output = output.take();
    
    let expected = vec![127, 124, 48, 56];
    let expected = String::from_utf8(expected).unwrap();   
    
    print_captured_output!(output, "WRAP NEG");
    assert!(output == expected, "Expected value: {}\nActual value: {}", expected, output);
}

#[test]
fn wrap_pos()
{
    let output = Rc::new(RefCell::new(String::new()));
    {
        let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
            was_dest_created: false, 
            output: Rc::clone(&output)
        };
        
        use BNNode::*;
        let tree: Vec<BNNode> = vec![
            Add(256), Out(1), Right(1), Add(8541), Out(1),
            Right(1), Add(145), Add(14854), Add(50), Out(1),
            Right(1), Add(588), Out(1)
        ];

        test_generate_output(tree, CTX, &mut factory);
    }
    let output = output.take();

    let expected = vec![0, 93, 73, 76];
    let expected = String::from_utf8(expected).unwrap();  

    assert!(output == expected, "Expected value: {}\nActual value: {}", expected, output);
    print_captured_output!(output, "WRAP POS");
}