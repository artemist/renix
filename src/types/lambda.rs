use super::ObjectKind;
use crate::context::Context;
use rnix::SyntaxNode;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub struct NixLambda {
    value: SyntaxNode,
    context: Rc<Context>,
}

impl ObjectKind for NixLambda {
    fn type_name(&self) -> &'static str {
        return "lambda";
    }
}
