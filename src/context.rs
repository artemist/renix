use crate::types::LazyObject;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Context {
    parent: Option<Rc<Context>>,
    objects: HashMap<String, LazyObject>,
}

impl Context {
    pub fn empty() -> Self {
        Self {
            parent: None,
            objects: HashMap::new(),
        }
    }

    pub fn with_builtins(builtins: LazyObject) -> Self {
        let mut map = HashMap::new();
        map.insert("builtins".to_string(), builtins);
        Self {
            parent: None,
            objects: map,
        }
    }
}
