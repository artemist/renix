use crate::types::Object;
use rnix::SyntaxNode;
use std::rc::Rc;

pub enum Value {
    Evaluated(Object),
    Deferred { context: Context, expr: SyntaxNode },
}

pub struct Context {
    parent: Rc<Context>,
}
