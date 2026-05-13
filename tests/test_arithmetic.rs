
mod common;
use common::*;

const CTX: TargetContext = TargetContext {
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Custom
    };

fn expected_char(num: u8) -> String
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
        
        use Node::*;
        let tree: Vec<Node> = vec![Sub(1), Out];

        test_generate_output(tree, CTX, &mut factory);
    }
    let output = output.take();
    assert!(output == expected_char(255), "Expected value: {}\nActual value: {}", expected_char(255), output);
    print_captured_output!(output, "WRAP NEG");
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
        
        use Node::*;
        let tree: Vec<Node> = vec![Add(256), Out];

        test_generate_output(tree, CTX, &mut factory);
    }
    let output = output.take();
    assert!(output == expected_char(0), "Expected value: {}\nActual value: {}", expected_char(0), output);
    print_captured_output!(output, "WRAP POS");
}