use std::convert::From;

///
/// Ast types are bridges between rust origin types and C/Swift/Java types.
///
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub(crate) enum AstBaseType {
    Void,
    Byte,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    String,
    Callback,
    Struct,
}

impl<'a> From<&'a str> for AstBaseType {
    fn from(ident: &'a str) -> Self {
        match ident {
            "u8" | "i8" => AstBaseType::Byte,
            "u16" | "u32" | "i16" | "i32" | "isize" | "usize" => AstBaseType::Int,
            "f32" => AstBaseType::Float,
            "f64" => AstBaseType::Double,
            "u64" | "i64" => AstBaseType::Long,
            "str" | "String" => AstBaseType::String,
            "bool" => AstBaseType::Boolean,
            // Right now, all callbacks are wrapped with Box
            "Box" => AstBaseType::Callback,
            // If the ident can't recognized, we assume it is a struct,
            // but if we add enum support, it should be changed.
            _ => AstBaseType::Struct,
        }
    }
}

impl From<String> for AstBaseType {
    fn from(ident: String) -> Self {
        return AstBaseType::from(ident.as_ref());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub(crate) enum AstType {
    Void,
    Byte,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    String,
    Vec(AstBaseType),
    Callback,
    Struct,
}

///
/// used for converting rust types to ast supported type.
///
impl<'a> From<&'a str> for AstType {
    fn from(ident: &'a str) -> Self {
        match ident {
            "u8" | "i8" => AstType::Byte,
            "u16" | "u32" | "i16" | "i32" | "isize" | "usize" => AstType::Int,
            "f32" => AstType::Float,
            "f64" => AstType::Double,
            "u64" | "i64" => AstType::Long,
            "str" | "String" => AstType::String,
            "bool" => AstType::Boolean,
            // Right now, all callbacks are wrapped with Box
            "Box" => AstType::Callback,
            // If the ident can't recognized, we assume it is a struct,
            // but if we add enum support, it should be changed.
            _ => AstType::Struct,
        }
    }
}

impl From<AstBaseType> for AstType {
    fn from(base_ty: AstBaseType) -> Self {
        match base_ty {
            AstBaseType::Void => AstType::Void,
            AstBaseType::Byte => AstType::Byte,
            AstBaseType::Int => AstType::Int,
            AstBaseType::Long => AstType::Long,
            AstBaseType::Float => AstType::Float,
            AstBaseType::Double => AstType::Double,
            AstBaseType::Boolean => AstType::Boolean,
            AstBaseType::String => AstType::String,
            AstBaseType::Callback => AstType::Callback,
            AstBaseType::Struct => AstType::Struct,
        }
    }
}

impl AstType {
    pub(crate) fn to_java_sig(&self) -> String {
        match *self {
            AstType::Void => "V".to_owned(),
            AstType::Byte => "B".to_owned(),
            AstType::Int => "I".to_owned(),
            AstType::Long => "J".to_owned(),
            AstType::Float => "F".to_owned(),
            AstType::Double => "D".to_owned(),
            AstType::Boolean => "I".to_owned(),
            AstType::String => "Ljava/lang/String;".to_owned(),
            AstType::Callback => "Ljava/lang/String;".to_owned(),
            AstType::Struct => "Ljava/lang/String;".to_owned(),
            AstType::Vec(base) => match base {
                AstBaseType::Byte => "[B".to_owned(),
                _ => "Ljava/lang/String;".to_owned(),
            },
        }
    }
}

///
/// used for converting rust types to ast supported type.
///
impl From<String> for AstType {
    fn from(ident: String) -> Self {
        return AstType::from(ident.as_ref());
    }
}
