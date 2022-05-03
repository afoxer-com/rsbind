//!
//! Parse files that standing for contract of ffi.
//!
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use syn::{TypeParamBound, TypePath};

use crate::errors::ErrorKind::*;
use crate::errors::*;

use super::super::types::*;
use super::desc::*;

pub(crate) struct ContractResult {
    pub(crate) traits: Vec<TraitDesc>,
    pub(crate) structs: Vec<StructDesc>,
}

///
/// parse a syn file to TraitDesc which depicting the structure of the trait.
///
pub(crate) fn parse(
    crate_name: String,
    file_path: &Path,
    mod_path: &str,
) -> Result<ContractResult> {
    let mut file = File::open(file_path).map_err(|e| ParseError(e.to_string()))?;

    let mut src = String::new();
    file.read_to_string(&mut src)
        .map_err(|e| ParseError(e.to_string()))?;

    let mod_name = PathBuf::from(file_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let parse_ctx = ParseContext {
        crate_name,
        mod_name,
        mod_path: mod_path.to_string(),
    };

    parse_from_str(&parse_ctx, &src)
}

pub(crate) struct ParseContext {
    pub(crate) crate_name: String,
    pub(crate) mod_name: String,
    pub(crate) mod_path: String,
}

pub(crate) fn parse_from_str(ctx: &ParseContext, src: &str) -> Result<ContractResult> {
    let syn_file = syn::parse_file(src).map_err(|e| ParseError(e.to_string()))?;

    let mut trait_descs = vec![];
    let mut struct_descs = vec![];

    // loop all the trait
    for item in syn_file.items.iter() {
        match *item {
            syn::Item::Trait(ref trait_inner) => {
                let trait_name = trait_inner.ident.to_string();
                println!("found trait => {}", trait_inner.ident);

                let mut send_derived = false;
                let mut sync_derived = false;
                for supertraits in trait_inner.supertraits.iter() {
                    if let TypeParamBound::Trait(trait_bound) = supertraits {
                        let segments = &(trait_bound.path.segments);
                        let ident = (&segments[segments.len() - 1].ident).to_string();
                        if ident == "Send" {
                            send_derived = true;
                        }
                        if ident == "Sync" {
                            sync_derived = true;
                        }
                    }
                }

                if !send_derived || !sync_derived {
                    panic!("Please derive 'Sync' and 'Send' for your trait '{}', Like: trait {} : Send + Sync {{ .. }}", &trait_name, &trait_name)
                }

                let methods = parse_methods(ctx, &trait_inner.items)?;
                let trait_desc = TraitDesc {
                    name: trait_name,
                    ty: "trait".to_string(),
                    mod_name: ctx.mod_name.clone(),
                    mod_path: ctx.mod_path.clone(),
                    crate_name: ctx.crate_name.clone(),
                    is_callback: methods.1,
                    methods: methods.0,
                };

                trait_descs.push(trait_desc);
            }
            syn::Item::Struct(ref struct_inner) => {
                println!("found struct => {}", &struct_inner.ident);
                let stuct_name = struct_inner.ident.to_string();

                let mut field_descs = vec![];
                let fields = &struct_inner.fields;
                for field in fields.iter() {
                    let field_name = match field.ident {
                        Some(ref value) => value.to_owned().to_string(),
                        _ => "".to_owned(),
                    };

                    let field_ty;
                    match field.ty {
                        syn::Type::Path(ref type_path) => {
                            let segments = &(type_path.path.segments);
                            let ident = (&segments[segments.len() - 1].ident).to_string();
                            if ident == "Box" {
                                println!("found Box argument.");
                                field_ty = Some(parse_boxed_ast(ctx, type_path));
                            } else if ident == "Vec" {
                                println!("found Vec argument.");
                                field_ty = Some(parse_vec_ast(ctx, type_path));
                            } else {
                                // normal arguments
                                field_ty = Some(AstType::new(&ident, &ident, ctx));
                                println!("found args type => {:?}", ident);
                            }
                        }
                        _ => {
                            field_ty = Some(AstType::Void);
                        }
                    };
                    let field_desc = ArgDesc {
                        name: field_name,
                        ty: field_ty.unwrap(),
                    };
                    field_descs.push(field_desc);
                }

                let struct_desc = StructDesc {
                    name: stuct_name,
                    ty: "struct".to_string(),
                    mod_name: ctx.mod_name.clone(),
                    mod_path: ctx.mod_path.clone(),
                    crate_name: ctx.crate_name.clone(),
                    fields: field_descs,
                };
                struct_descs.push(struct_desc);
            }
            _ => (),
        }
    }

    if !trait_descs.is_empty() || !struct_descs.is_empty() {
        println!("final trait desc => {:#?}", trait_descs);
        Ok(ContractResult {
            traits: trait_descs,
            structs: struct_descs,
        })
    } else {
        Err(ParseError("Can't find invalid trait and struct.".to_string()).into())
    }
}

///
/// Loop all the methods
///
fn parse_methods(ctx: &ParseContext, items: &[syn::TraitItem]) -> Result<(Vec<MethodDesc>, bool)> {
    let mut method_descs: Vec<MethodDesc> = vec![];
    let mut is_callback = false;
    for method in items.iter() {
        if let syn::TraitItem::Method(ref method_inner) = method {
            let method_name: String = method_inner.sig.ident.to_string();
            let mut args: Vec<ArgDesc> = vec![];

            println!("found method => {}", method_inner.sig.ident);

            let return_type = parse_return_type(ctx, &method_inner.sig.output)?;

            // arguments
            let mut swallow_self = false;
            for input in method_inner.sig.inputs.iter() {
                match input {
                    syn::FnArg::Receiver(ref arg) => {
                        swallow_self = arg.reference.is_none();
                        is_callback = true;
                        continue;
                    }
                    _ => {
                        let arg = parse_one_arg(ctx, input)?;
                        args.push(arg);
                    }
                }
            }

            let method_desc = MethodDesc {
                name: method_name,
                return_type,
                args,
                swallow_self,
            };
            method_descs.push(method_desc);
        }
    }

    if !method_descs.is_empty() {
        Ok((method_descs, is_callback))
    } else {
        Err(ParseError("Can't parse methods.".to_string()).into())
    }
}

///
/// parse return type
///
fn parse_return_type(ctx: &ParseContext, output: &syn::ReturnType) -> Result<AstType> {
    // return type
    match output {
        syn::ReturnType::Type(_, ref boxed) => {
            let path = &**boxed;
            if let syn::Type::Path(ref type_path) = path {
                let segments = &(type_path.path.segments);
                let ident = &(segments[segments.len() - 1].ident);

                // Generic parsing
                println!("found return type => {:?}", ident);
                return if *ident == "Vec" {
                    println!("found Vec return type.");
                    Ok(parse_vec_ast(ctx, type_path))
                } else if *ident == "Box" {
                    println!("found Box return type.");
                    Ok(parse_boxed_ast(ctx, type_path))
                } else {
                    let origin = ident.to_string();
                    Ok(AstType::new(&origin, &origin, ctx))
                };
            }
        }
        syn::ReturnType::Default => return Ok(AstType::Void),
    }

    Err(ParseError("can't parse return type".to_string()).into())
}

///
/// parse one argument
///
fn parse_one_arg(ctx: &ParseContext, input: &syn::FnArg) -> Result<ArgDesc> {
    let mut arg_name: Option<String> = Some("".to_owned());
    let mut arg_type: Option<AstType> = Some(AstType::Void);
    if let syn::FnArg::Typed(ref arg) = input {
        if let syn::Pat::Ident(ref pat_ident) = *(arg.pat) {
            arg_name = Some(pat_ident.ident.to_string());
            println!("found arg pat = {:?}", pat_ident.ident.to_string());
        }

        if let syn::Type::Path(ref type_path) = *(arg.ty) {
            let segments = &(type_path.path.segments);
            let ident = (&segments[segments.len() - 1].ident).to_string();
            if ident == "Box" {
                println!("found Box argument.");
                arg_type = Some(parse_boxed_ast(ctx, type_path));
            } else if ident == "Vec" {
                println!("found Vec argument.");
                arg_type = Some(parse_vec_ast(ctx, type_path));
            } else {
                // normal arguments
                arg_type = Some(AstType::new(&ident, &ident, ctx));
                println!("found args type => {:?}", ident);
            }
        }
    }

    match (arg_name, arg_type) {
        (Some(arg_name), Some(arg_type)) => Ok(ArgDesc {
            name: arg_name,
            ty: arg_type,
        }),
        _ => Err(ParseError("parse argments error!".to_string()).into()),
    }
}

fn parse_vec_ast(ctx: &ParseContext, type_path: &TypePath) -> AstType {
    let mut arg_type: AstType = AstType::Void;
    let segments = &(type_path.path.segments);
    let angle_bracketed = &segments[segments.len() - 1].arguments;
    if let syn::PathArguments::AngleBracketed(t) = angle_bracketed {
        let arg = &t.args[0];
        if let syn::GenericArgument::Type(syn::Type::Path(ref type_path)) = arg {
            println!("found vec types = {:?})", type_path);
            let ident = parse_ident_in_path(ctx, type_path);
            arg_type = AstType::Vec(AstBaseType::new(&ident, &ident.to_string(), ctx));
        }
    }

    arg_type
}

fn parse_boxed_ast(ctx: &ParseContext, type_path: &TypePath) -> AstType {
    let segments = &(type_path.path.segments);
    let mut ty: AstType = AstType::Void;
    let angle_bracketed = &segments[segments.len() - 1].arguments;
    if let syn::PathArguments::AngleBracketed(t) = angle_bracketed {
        println!("parsing Boxed inner.");
        match &t.args[0] {
            syn::GenericArgument::Type(syn::Type::Path(ref type_path)) => {
                println!("found boxed types = {:?})", type_path);
                let ident = parse_ident_in_path(ctx, type_path);
                ty = AstType::new("Box", &ident, ctx);
            }
            syn::GenericArgument::Type(syn::Type::TraitObject(ref trait_obj)) => {
                if trait_obj.dyn_token.is_some() {
                    let bounds = &trait_obj.bounds;
                    for bound in bounds.iter() {
                        if let TypeParamBound::Trait(trait_bound) = bound.clone() {
                            let segments = trait_bound.path.segments;
                            let ident = (&segments[segments.len() - 1].ident).to_string();
                            ty = AstType::new("Box", &ident, ctx);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    ty
}

fn parse_ident_in_path(ctx: &ParseContext, type_path: &TypePath) -> String {
    let segments = &(type_path.path.segments);
    (&segments[segments.len() - 1].ident).to_string()
}
