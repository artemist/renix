use super::{NixAttrSet, NixList};
use super::{NixFloat, NixInteger, NixString};
use crate::context::Context;
use enum_dispatch::enum_dispatch;
use rnix::SyntaxNode;
use std::{cell::RefCell, rc::Rc};

use super::NixLambda;
use crate::eval::eval_node;

use std::ops::Deref;

/// Functions that all Nix Objects must implement
#[enum_dispatch]
pub trait ObjectKind {
    fn type_name(&self) -> &'static str;
}

/// A Nix Object
#[enum_dispatch(ObjectKind)]
#[derive(PartialEq, Clone, Debug)]
pub enum Object {
    Integer(NixInteger),
    Float(NixFloat),
    Str(NixString),
    List(NixList),
    AttrSet(NixAttrSet),
    Lambda(NixLambda),
}

#[derive(Clone, Debug)]
enum UnwrappedLazyObject {
    Evaluated(Object),
    Error(Rc<anyhow::Error>),
    Deferred { context: Context, node: SyntaxNode },
}

impl UnwrappedLazyObject {
    fn new_deferred(context: Context, node: SyntaxNode) -> Self {
        Self::Deferred { context, node }
    }
}

impl From<anyhow::Result<Object>> for UnwrappedLazyObject {
    fn from(orig: anyhow::Result<Object>) -> Self {
        match orig {
            Ok(obj) => Self::Evaluated(obj),
            Err(err) => Self::Error(Rc::new(err)),
        }
    }
}

impl PartialEq for UnwrappedLazyObject {
    fn eq(&self, rhs: &Self) -> bool {
        match &self {
            Self::Evaluated(obj) => {
                if let &Self::Evaluated(ref robj) = rhs {
                    obj == robj
                } else {
                    false
                }
            }
            Self::Deferred { context, node } => {
                if let &Self::Deferred {
                    context: ref rcontext,
                    node: ref rnode,
                } = rhs
                {
                    context == rcontext && node == rnode
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct LazyObject {
    value: Rc<RefCell<UnwrappedLazyObject>>,
}

impl LazyObject {
    pub fn new_deferred(context: Context, node: SyntaxNode) -> Self {
        Self {
            value: Rc::new(RefCell::new(UnwrappedLazyObject::new_deferred(
                context, node,
            ))),
        }
    }

    pub fn eval(&self) {
        let mut new_object = Option::None;
        if let &UnwrappedLazyObject::Deferred {
            ref context,
            ref node,
        } = self.value.borrow().deref()
        {
            new_object = Some(eval_node(&node, &context).into());
        }
        if let Some(obj) = new_object {
            self.value.replace(obj);
        }
    }
}
