use crate::ast::contract::desc::{ArgDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::base::lang::{Convertible, Direction};
use crate::errors::*;
use crate::ident;
use crate::java::bridge::TypeDirection;
use crate::java::converter::JavaConvert;
use proc_macro2::TokenStream;

pub(crate) fn quote_arg_convert(
    arg: &ArgDesc,
    _namespace: &str,
    _trait_desc: &TraitDesc,
) -> Result<TokenStream> {
    if let AstType::Void = arg.ty.clone() {
        return Ok(quote! {});
    }

    let rust_arg_str = format!("r_{}", &arg.name);
    let rust_arg_name = ident!(&rust_arg_str);
    let arg_name_ident = ident!(&arg.name);
    let convert = JavaConvert { ty: arg.ty.clone() }
        .transferable_to_rust(quote! {#arg_name_ident}, Direction::Down);
    let result = quote! {
        let #rust_arg_name = #convert;
    };
    Ok(result)
}

pub(crate) fn quote_return_convert(
    return_ty: &AstType,
    _trait_desc: &TraitDesc,
    _callbacks: &[&TraitDesc],
    ret_name: &str,
) -> Result<TokenStream> {
    if let AstType::Void = return_ty.clone() {
        return Ok(quote! {});
    }

    let ret_name_ident = ident!(ret_name);
    let result = JavaConvert {
        ty: return_ty.clone(),
    }
    .rust_to_transferable(quote! {#ret_name_ident}, Direction::Down);

    Ok(result)
}

pub(crate) fn basic_ty_to_tokens(ast_type: AstType) -> TokenStream {
    match ast_type {
        AstType::Byte(_) => quote!(i8),
        AstType::Short(_) => quote!(i16),
        AstType::Int(_) => quote!(i32),
        AstType::Long(_) => quote!(i64),
        AstType::Float(_) => quote!(f32),
        AstType::Double(_) => quote!(f64),
        AstType::Boolean => quote!(u8),
        _ => quote! {},
    }
}

pub(crate) fn ty_to_tokens(ast_type: &AstType, direction: TypeDirection) -> Result<TokenStream> {
    Ok(match ast_type.clone() {
        AstType::Byte(_)
        | AstType::Short(_)
        | AstType::Int(_)
        | AstType::Long(_)
        | AstType::Float(_)
        | AstType::Double(_)
        | AstType::Boolean => basic_ty_to_tokens(ast_type.clone()),
        AstType::String => match direction {
            TypeDirection::Argument => quote!(JString),
            TypeDirection::Return => quote!(jstring),
        },
        AstType::Vec(base) => match direction {
            TypeDirection::Argument => match base {
                AstBaseType::Byte(_) => {
                    quote!(jbyteArray)
                }
                _ => quote!(JString),
            },
            TypeDirection::Return => match base {
                AstBaseType::Byte(_) => {
                    quote!(jbyteArray)
                }
                _ => quote!(jstring),
            },
        },
        AstType::Struct(_) => match direction {
            TypeDirection::Argument => quote!(JString),
            TypeDirection::Return => quote!(jstring),
        },
        AstType::Callback(_) => quote!(i64),
        AstType::Void => quote!(()),
    })
}
