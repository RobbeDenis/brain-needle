
mod common;

use common::*;

const CTX: TargetContext = TargetContext {
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Custom
};

#[test]
fn counter()
{
    let output = Rc::new(RefCell::new(String::new()));
    {
        let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
            was_dest_created: false, 
            output: Rc::clone(&output)
        };
        
        use BNNode::*;
        let tree: Vec<BNNode> = vec![
            Add(72), Loop(6), Right(1), Add(1), Left(1), Sub(1), EndLoop(1), Right(1), Out(1)
        ];

        test_generate_output(tree, CTX, &mut factory);
    }
    let output = output.take();
    
    let expected = vec![72];
    let expected = String::from_utf8(expected).unwrap(); 
    
    print_captured_output!(output, "SIMPLE LOOP");
    assert!(output == expected, "Expected value: {}\nActual value: {}", expected, output);
}