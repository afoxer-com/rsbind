use std::convert::From;

///
/// Ast types are bridges between rust origin types and C/Swift/Java types.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum AstBaseType {
    Void,
    Byte(String),
    Int(String),
    Long(String),
    Float(String),
    Double(String),
    Boolean,
    String,
    Callback(String),
    Struct(String),
}

impl<'a> AstBaseType {
    pub fn new(ident: &'a str, sub: &'a str) -> Self {
        let origin = ident.to_string();
        match ident {
            "u8" | "i8" => AstBaseType::Byte(origin),
            "u16" | "u32" | "i16" | "i32" | "isize" | "usize" => AstBaseType::Int(origin),
            "f32" => AstBaseType::Float(origin),
            "f64" => AstBaseType::Double(origin),
            "u64" | "i64" => AstBaseType::Long(origin),
            "str" | "String" => AstBaseType::String,
            "bool" => AstBaseType::Boolean,
            // Right now, all callbacks are wrapped with Box
            "Box" => AstBaseType::Callback(sub.to_string()),
            // If the ident can't recognized, we assume it is a struct,
            // but if we add enum support, it should be changed.
            _ => AstBaseType::Struct(sub.to_string()),
        }
    }

    pub fn origin(&self) -> String {
        match &self {
            AstBaseType::Void => "".to_owned(),
            AstBaseType::Byte(origin) => origin.clone(),
            AstBaseType::Int(origin) => origin.clone(),
            AstBaseType::Long(origin) => origin.clone(),
            AstBaseType::Float(origin) => origin.clone(),
            AstBaseType::Double(origin) => origin.clone(),
            AstBaseType::Boolean => "bool".to_owned(),
            AstBaseType::String => "String".to_owned(),
            AstBaseType::Callback(origin) => origin.clone(),
            AstBaseType::Struct(origin) => origin.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum AstType {
    Void,
    Byte(String),
    Int(String),
    Long(String),
    Float(String),
    Double(String),
    Boolean,
    String,
    Vec(AstBaseType),
    Callback(String),
    Struct(String),
}

///
/// used for converting rust types to ast supported type.
///
impl<'a> AstType {
    pub fn new(ident: &'a str, sub: &'a str) -> Self {
        let origin = ident.to_string();
        match ident {
            "u8" | "i8" => AstType::Byte(origin),
            "u16" | "u32" | "i16" | "i32" | "isize" | "usize" => AstType::Int(origin),
            "f32" => AstType::Float(origin),
            "f64" => AstType::Double(origin),
            "u64" | "i64" => AstType::Long(origin),
            "str" | "String" => AstType::String,
            "bool" => AstType::Boolean,
            // Right now, all callbacks are wrapped with Box
            "Box" => AstType::Callback(sub.to_string()),
            // If the ident can't recognized, we assume it is a struct,
            // but if we add enum support, it should be changed.
            _ => AstType::Struct(sub.to_string()),
        }
    }

    pub fn origin(&self) -> String {
        match &self {
            AstType::Void => "".to_owned(),
            AstType::Byte(origin) => origin.clone(),
            AstType::Int(origin) => origin.clone(),
            AstType::Long(origin) => origin.clone(),
            AstType::Float(origin) => origin.clone(),
            AstType::Double(origin) => origin.clone(),
            AstType::Boolean => "bool".to_owned(),
            AstType::String => "String".to_owned(),
            AstType::Vec(base) => format!("Vec<{}>", &base.origin()),
            AstType::Callback(origin) => origin.clone(),
            AstType::Struct(origin) => origin.clone(),
        }
    }
}

impl From<AstBaseType> for AstType {
    fn from(base_ty: AstBaseType) -> Self {
        match base_ty {
            AstBaseType::Void => AstType::Void,
            AstBaseType::Byte(origin) => AstType::Byte(origin),
            AstBaseType::Int(origin) => AstType::Int(origin),
            AstBaseType::Long(origin) => AstType::Long(origin),
            AstBaseType::Float(origin) => AstType::Float(origin),
            AstBaseType::Double(origin) => AstType::Double(origin),
            AstBaseType::Boolean => AstType::Boolean,
            AstBaseType::String => AstType::String,
            AstBaseType::Callback(origin) => AstType::Callback(origin),
            AstBaseType::Struct(origin) => AstType::Struct(origin),
        }
    }
}

impl AstType {
    pub(crate) fn to_java_sig(&self) -> String {
        match self {
            AstType::Void => "V".to_owned(),
            AstType::Byte(_) => "B".to_owned(),
            AstType::Int(_) => "I".to_owned(),
            AstType::Long(_) => "J".to_owned(),
            AstType::Float(_) => "F".to_owned(),
            AstType::Double(_) => "D".to_owned(),
            AstType::Boolean => "I".to_owned(),
            AstType::String => "Ljava/lang/String;".to_owned(),
            AstType::Callback(_) => "Ljava/lang/String;".to_owned(),
            AstType::Struct(_) => "Ljava/lang/String;".to_owned(),
            AstType::Vec(base) => match base {
                AstBaseType::Byte(_) => "[B".to_owned(),
                _ => "Ljava/lang/String;".to_owned(),
            },
        }
    }
}
