use super::ObjectKind;
use rnix::value::{remove_common_indent, unescape};
use std::str::FromStr;

pub struct NixInteger {
    value: i64,
}

impl ObjectKind for NixInteger {
    fn type_name(&self) -> &'static str {
        return "int";
    }
}

impl FromStr for NixInteger {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: i64::from_str(value)?,
        })
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
impl FromStr for NixFloat {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: f64::from_str(value)?,
        })
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

impl FromStr for NixString {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let unescaped = if value.starts_with('"') && value.ends_with('"') {
            unescape(
                value.strip_prefix('"').unwrap().strip_suffix('"').unwrap(),
                false,
            )
        } else if value.starts_with("''") && value.ends_with("''") {
            let indented = unescape(
                value
                    .strip_prefix("''")
                    .unwrap()
                    .strip_suffix("''")
                    .unwrap(),
                true,
            );
            remove_common_indent(&indented)
        } else {
            return Err(anyhow::anyhow!(
                "String does not start and end with valid quotation"
            ));
        };

        Ok(Self {
            value: unescaped.into_boxed_str(),
        })
    }
}
