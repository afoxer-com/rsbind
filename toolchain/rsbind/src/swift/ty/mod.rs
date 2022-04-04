use crate::ast::types::AstType;
use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;

pub(crate) mod basic;
pub(crate) mod callback;
pub(crate) mod str;
pub(crate) mod struct_;
pub(crate) mod vec_base;
pub(crate) mod vec_default;
pub(crate) mod vec_struct;
