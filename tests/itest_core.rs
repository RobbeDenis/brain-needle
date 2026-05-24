
mod common;
use common::*;

const CTX: TargetContext = TargetContext {
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        format: OutputFormat::Interpreted,
        dest:   OutputDest::Custom
    };

#[test]
fn custom_emit_factory_creation() {
    let output = Rc::new(RefCell::new(String::new()));
    let mut factory: TestBNEmitterFactory = TestBNEmitterFactory {
        was_dest_created: false, 
        output: Rc::clone(&output)
    };

    test_generate_output(hello_world_instr_tree!(), CTX, &mut factory);
    assert!(factory.was_dest_created, "create_dest was never called");
}