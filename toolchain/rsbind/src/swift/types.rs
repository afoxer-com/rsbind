use rstgen::swift::Swift;
use rstgen::{swift, Custom, Formatter, Tokens};

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
            AstType::Short(_) => swift::SHORT,
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
            AstType::Callback(origin) | AstType::Struct(origin) => swift::local(origin.origin),
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
