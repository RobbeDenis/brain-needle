
mod common;

use brain_needle::bnctx::*;
use brain_needle::bnemit::BNEmitterFactory;

use std::path::PathBuf;

#[test]
fn custom_emit_factory_creation()
{
    let ctx = TargetContext {
        os:     TargetOS::Linux, 
        arch:   Architecture::X86_64, 
        format: OutputFormat::Interpreted,
        dest:   OutputDest::File(Some(PathBuf::from("output\\test\\dump.txt")))
    };
    
    let mut factory = common::TBNEmitterFactory{was_create_dest_called: false};
    let _emitter = factory.create(&ctx);
    
    assert!(factory.was_create_dest_called, "create_dest was never called");
}