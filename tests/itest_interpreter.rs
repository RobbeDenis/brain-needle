
mod common;
use common::*;

const CTX: TargetContext = TargetContext {
    os:     TargetOS::Linux, 
    arch:   Architecture::X86_64, 
    format: OutputFormat::Interpreted,
    dest:   OutputDest::Custom
};

mod arithmetic
{
    use super::*;

    #[test]
    fn wrap_neg() {
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
        bn_assert_eq!(expected, output);
    }

    #[test]
    fn wrap_pos() {
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

        print_captured_output!(output, "WRAP POS");
        bn_assert_eq!(expected, output);
    }
}

mod shift
{
    use super::*;
    use bncore::MAX_PROGRAM_BYTES;

    #[test]
    fn wrap_right() {
        let output = Rc::new(RefCell::new(String::new()));
        {
            let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
                was_dest_created: false, 
                output: Rc::clone(&output)
            };
            
            use BNNode::*;
            let max = MAX_PROGRAM_BYTES as u16;
            let tree: Vec<BNNode> = vec![
                Add(64), Right(max), Out(1),
                Add(20), Right(max / 2), Add(80), Out(1),
                Right((max / 2) + 1), Sub(20), Out(1),
                Left(1), Out(1)
            ];

            test_generate_output(tree, CTX, &mut factory);
        }
        let output = output.take();
        
        let expected = vec![64, 80, 108, 84];
        let expected = String::from_utf8(expected).unwrap(); 
        
        print_captured_output!(output, "WRAP RIGHT");
        bn_assert_eq!(expected, output);
    }

    #[test]
    fn wrap_left() {
        let output = Rc::new(RefCell::new(String::new()));
        {
            let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
                was_dest_created: false, 
                output: Rc::clone(&output)
            };
            
            use BNNode::*;
            let max = MAX_PROGRAM_BYTES as u16;
            let tree: Vec<BNNode> = vec![
                Add(64), Left(max), Out(1),
                Add(20), Left(max / 2), Add(80), Out(1),
                Left((max / 2) + 1), Sub(20), Out(1),
                Right(1), Out(1)
            ];

            test_generate_output(tree, CTX, &mut factory);
        }
        let output = output.take();

        let expected = vec![64, 80, 108, 84];
        let expected = String::from_utf8(expected).unwrap();

        print_captured_output!(output, "WRAP LEFT");
        bn_assert_eq!(expected, output);
    }
}

mod looping
{
    use super::*;

    #[test]
    fn counter() {
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
        
        print_captured_output!(output, "COUNTER");
        bn_assert_eq!(expected, output);
    }
}