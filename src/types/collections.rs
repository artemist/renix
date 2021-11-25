use super::{LazyObject, ObjectKind};
use std::collections::HashMap;

pub struct NixList {
    value: Vec<LazyObject>,
}
impl ObjectKind for NixList {
    fn type_name(&self) -> &'static str {
        return "list";
    }
}

pub struct NixAttrSet {
    value: HashMap<String, LazyObject>,
}
impl ObjectKind for NixAttrSet {
    fn type_name(&self) -> &'static str {
        return "set";
    }
}
