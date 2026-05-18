

mod case_macros;

pub(crate) use brain_needle::bnctx::*;
pub(crate) use brain_needle::bnemit::BNEmitterFactory;
pub(crate) use brain_needle::bndest::BNDest;
pub(crate) use brain_needle::bndest::BNDestFactory;
pub(crate) use brain_needle::bncore::Node;
pub(crate) use std::rc::Rc;
pub(crate) use std::cell::RefCell;

pub fn test_generate_output<TFactory: BNEmitterFactory>(instr_tree: Vec<Node>, ctx: TargetContext, factory: &mut TFactory)
{
    let mut codegen = factory.create(&ctx);
    
    codegen.emit_setup();
    
    let mut i: usize = 0;
    while i < instr_tree.len() {
    let node = &instr_tree[i];
        match  *node {
            Node::Add(__) | 
            Node::Sub(__) => codegen.emit_arithmetic(node),
            Node::Right(__) | 
            Node::Left(__) => codegen.emit_shift(node),
            Node::JumpIfZero(__) | 
            Node::JumpIfNotZero(__) => {
                if let Some(target) = codegen.emit_jump(node) {
                    i = target;
                }
            },
            Node::Out => codegen.emit_out(),
            Node::In => codegen.emit_in()
        }
        i += 1;
    }

    codegen.emit_exit();
    codegen.finalize();
}

pub struct TestBNEmitterFactory
{
    pub was_dest_created: bool,
    pub output: Rc<RefCell<String>>
}

impl BNEmitterFactory for TestBNEmitterFactory
{
    fn create_dest(& mut self, target_ctx: &TargetContext) -> Box<dyn BNDest>
    {
        self.was_dest_created = true;

        if target_ctx.dest == OutputDest::Custom {
            return Box::new(TestPubDest::new(Rc::clone(&self.output)));
        } else {
            return BNDestFactory::create(&target_ctx);
        }
    }
}

pub struct TestPubDest
{
    output: Rc<RefCell<String>>
}

impl TestPubDest
{
    pub const fn new(out: Rc<RefCell<String>>) -> TestPubDest
    {
        return TestPubDest{ output: out };
    }
}

impl BNDest for TestPubDest
{
    fn push(&mut self, data: &[u8])
    {
        if let Ok(s) = std::str::from_utf8(data) {
            self.output.borrow_mut().push_str(s);
        }
    }

    fn finalize(&self)
    {
        
    }
}