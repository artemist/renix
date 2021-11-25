use super::{NixAttrSet, NixList};
use super::{NixFloat, NixInteger, NixString};
use crate::context::Context;
use enum_dispatch::enum_dispatch;
use rnix::SyntaxNode;
use std::{cell::RefCell, rc::Rc};

use super::NixLambda;

/// Functions that all Nix Objects must implement
#[enum_dispatch]
pub trait ObjectKind {
    fn type_name(&self) -> &'static str;
}

/// A Nix Object
#[enum_dispatch(ObjectKind)]
pub enum Object {
    Integer(NixInteger),
    Float(NixFloat),
    Str(NixString),
    List(NixList),
    AttrSet(NixAttrSet),
    Lambda(NixLambda),
}

pub enum UnwrappedLazyObject {
    Evaluated(Object),
    Deferred { context: Context, node: SyntaxNode },
}

pub struct LazyObject {
    value: Rc<RefCell<UnwrappedLazyObject>>,
}
