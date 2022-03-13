use rstgen::{Custom, Formatter, swift, Tokens};
use rstgen::swift::Swift;
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;

#[derive(Clone)]
pub(crate) struct SwiftType {
    pub ast_type: AstType,
}

impl SwiftType {
    pub fn new(ast_type: AstType) -> SwiftType {
        SwiftType { ast_type }
    }

    pub fn to_array(&self) -> Swift<'static> {
        let base_name = Swift::from(self.clone());
        self.to_swift_array(base_name)
    }

    pub fn to_str(&self) -> String {
        return match self.ast_type.clone() {
            AstType::Void => "Void".to_string(),
            AstType::Byte(_) => "Int8".to_string(),
            AstType::Int(_) => "Int32".to_string(),
            AstType::Long(_) => "Int64".to_string(),
            AstType::Float(_) => "Float".to_string(),
            AstType::Double(_) => "Double".to_string(),
            AstType::Boolean => "Bool".to_string(),
            AstType::String => "String".to_string(),
            AstType::Vec(ref base) => {
                let base_ty = SwiftType {
                    ast_type: AstType::from(base.clone()),
                };
                format!("[{}]", base_ty.to_str())
            }
            AstType::Callback(origin) => origin,
            AstType::Struct(origin) => origin,
        };
    }

    fn to_swift_array(&self, swift: Swift<'static>) -> Swift<'static> {
        Swift::Array {
            inner: Box::new(swift),
        }
    }
}

impl From<SwiftType> for Swift<'static> {
    fn from(item: SwiftType) -> Self {
        match item.ast_type {
            AstType::Void => swift::VOID,
            AstType::Byte(_) => swift::BYTE,
            AstType::Int(_) => swift::INTEGER,
            AstType::Long(_) => swift::LONG,
            AstType::Float(_) => swift::FLOAT,
            AstType::Double(_) => swift::DOUBLE,
            AstType::Boolean => swift::BOOLEAN,
            AstType::String => swift::local("String"),
            AstType::Vec(base) => match base {
                AstBaseType::Struct(_) => SwiftType::new(AstType::from(base)).to_array(),
                AstBaseType::Byte(_) => SwiftType::new(AstType::from(base)).to_array(),
                _ => SwiftType::new(AstType::from(base)).to_array(),
            },
            AstType::Callback(origin) | AstType::Struct(origin) => swift::local(origin),
        }
    }
}

pub(crate) fn to_swift_file(tokens: Tokens<Swift>) -> Result<String> {
    let mut buf = String::new();
    {
        let mut formatter = Formatter::new(&mut buf);
        let mut extra = ();
        swift::Swift::write_file(tokens, &mut formatter, &mut extra, 0)?;
    }
    Ok(buf)
}
