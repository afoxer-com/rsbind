use super::super::types::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct ArgDesc {
    pub name: String,
    pub ty: AstType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct MethodDesc {
    pub name: String,
    pub return_type: AstType,
    pub args: Vec<ArgDesc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct TraitDesc {
    pub name: String,
    pub ty: String,
    pub mod_name: String,
    pub mod_path: String,
    pub crate_name: String,
    pub is_callback: bool,
    pub methods: Vec<MethodDesc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct StructDesc {
    pub name: String,
    pub ty: String,
    pub mod_name: String,
    pub mod_path: String,
    pub crate_name: String,
    pub fields: Vec<ArgDesc>,
}
