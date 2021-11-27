use super::{LazyObject, ObjectKind};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct NixList {
    value: Vec<LazyObject>,
}
impl ObjectKind for NixList {
    fn type_name(&self) -> &'static str {
        return "list";
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct NixAttrSet {
    value: HashMap<String, LazyObject>,
}
impl ObjectKind for NixAttrSet {
    fn type_name(&self) -> &'static str {
        return "set";
    }
}
