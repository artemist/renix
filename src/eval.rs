#![allow(unreachable_patterns)]

use crate::context::Context;
use crate::types::*;
use rnix::SyntaxKind::*;
use rnix::AST;
use rnix::{SyntaxNode, SyntaxToken};
use std::str::FromStr;

pub fn eval_ast(ast: AST, context: Context, node: Option<String>) -> anyhow::Result<Object> {
    let root = ast.node();
    let base_context = Context::empty();

    let lazy_root = LazyObject::new_deferred(base_context, root);

    todo!()
}

pub fn eval_node(node: &SyntaxNode, context: &Context) -> anyhow::Result<Object> {
    Ok(match &node.kind() {
        NODE_PAREN => eval_node(
            &node
                .first_child()
                .ok_or_else(|| anyhow::anyhow!("Empty Parenthesis"))?,
            context,
        )?,
        NODE_VALUE => parse_value_token(
            &node
                .first_token()
                .ok_or_else(|| anyhow::anyhow!("No value token in value node"))?,
        )?,
        _ => todo!(),
    })
}

fn parse_value_token(token: &SyntaxToken) -> anyhow::Result<Object> {
    Ok(match token.kind() {
        TOKEN_INTEGER => Object::Integer(NixInteger::from_str(token.text())?),
        TOKEN_FLOAT => Object::Float(NixFloat::from_str(token.text())?),
        TOKEN_STRING => Object::Str(NixString::from_str(token.text())?),
        _ => todo!(),
    })
}
