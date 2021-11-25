use super::ObjectKind;

pub struct NixInteger {
    value: i64,
}

impl ObjectKind for NixInteger {
    fn type_name(&self) -> &'static str {
        return "int";
    }
}

pub struct NixFloat {
    value: f64,
}
impl ObjectKind for NixFloat {
    fn type_name(&self) -> &'static str {
        return "float";
    }
}

pub struct NixString {
    value: Box<str>,
}
impl ObjectKind for NixString {
    fn type_name(&self) -> &'static str {
        return "string";
    }
}
