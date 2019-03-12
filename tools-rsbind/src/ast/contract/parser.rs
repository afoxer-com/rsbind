//!
//! Parse files that standing for contract of ffi.
//!
use super::super::types::*;
use super::desc::*;
use syn;

use ast::imp::desc::ImpDesc;
use errors::ErrorKind::*;
use errors::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

///
/// parse a syn file to TraitDesc which depicting the structure of the trait.
///
pub(crate) fn parse(
    crate_name: String,
    file_path: &PathBuf,
) -> Result<(Vec<TraitDesc>, Vec<StructDesc>)> {
    let mut file = File::open(file_path).map_err(|e| ParseError(e.to_string()))?;

    let mut src = String::new();
    file.read_to_string(&mut src)
        .map_err(|e| ParseError(e.to_string()))?;

    let syn_file = syn::parse_file(&src).map_err(|e| ParseError(e.to_string()))?;

    let mut trait_descs = vec![];
    let mut struct_descs = vec![];

    // loop all the trait
    for item in syn_file.items.iter() {
        let mod_name = PathBuf::from(file_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        match *item {
            syn::Item::Trait(ref trait_inner) => {
                let trait_name = trait_inner.ident.to_string();
                println!("found trait => {}", trait_inner.ident);

                let methods = parse_methods(&trait_inner.items)?;

                let trait_desc = TraitDesc {
                    name: trait_name,
                    ty: "trait".to_string(),
                    mod_name,
                    crate_name: crate_name.clone(),
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

                    let (field_ty, field_origin_ty) = match field.ty {
                        syn::Type::Path(ref type_path) => {
                            let segments = &(type_path.path.segments);
                            let ident = &(segments[segments.len() - 1].ident);
                            let origin = ident.to_string();
                            (AstType::from(ident.to_string()), origin)
                        }
                        _ => (AstType::Void, "".to_owned()),
                    };
                    let field_desc = ArgDesc {
                        name: field_name,
                        ty: field_ty,
                        origin_ty: field_origin_ty,
                    };
                    field_descs.push(field_desc);
                }

                let struct_desc = StructDesc {
                    name: stuct_name,
                    ty: "struct".to_string(),
                    mod_name,
                    crate_name: crate_name.clone(),
                    fields: field_descs,
                };
                struct_descs.push(struct_desc);
            }
            _ => (),
        }
    }

    if trait_descs.len() > 0 || struct_descs.len() > 0 {
        println!("final trait desc => {:#?}", trait_descs);
        Ok((trait_descs, struct_descs))
    } else {
        Err(ParseError("Can't find invalid trait and struct.".to_string()).into())
    }
}

///
/// Loop all the methods
///
fn parse_methods(items: &Vec<syn::TraitItem>) -> Result<(Vec<MethodDesc>, bool)> {
    let mut method_descs: Vec<MethodDesc> = vec![];
    let mut is_callback = false;
    for method in items.iter() {
        match method {
            syn::TraitItem::Method(ref method_inner) => {
                let method_name: String = method_inner.sig.ident.to_string();
                let mut args: Vec<ArgDesc> = vec![];

                println!("found method => {}", method_inner.sig.ident);

                let (return_type, origin_return_ty) =
                    parse_return_type(&method_inner.sig.decl.output)?;

                // arguments
                for input in method_inner.sig.decl.inputs.iter() {
                    match input {
                        syn::FnArg::SelfRef(ref arg) => {
                            is_callback = true;
                            continue;
                        }
                        _ => {
                            let arg = parse_one_arg(input)?;
                            args.push(arg);
                        }
                    }
                }

                let method_desc = MethodDesc {
                    name: method_name,
                    return_type,
                    origin_return_ty,
                    args,
                };
                method_descs.push(method_desc);
            }

            _ => (),
        }
    }

    if method_descs.len() > 0 {
        Ok((method_descs, is_callback))
    } else {
        Err(ParseError("Can't parse methods.".to_string()).into())
    }
}

///
/// parse return type
///
fn parse_return_type(output: &syn::ReturnType) -> Result<(AstType, String)> {
    // return type
    match output {
        syn::ReturnType::Type(_, ref boxed) => {
            let path = &**boxed;
            match path {
                syn::Type::Path(ref type_path) => {
                    let segments = &(type_path.path.segments);
                    let ident = &(segments[segments.len() - 1].ident);

                    // Generic parsing
                    let mut generic_ident = None;
                    let argument = &(segments[segments.len() - 1].arguments);
                    match argument {
                        syn::PathArguments::None => (),
                        syn::PathArguments::AngleBracketed(t) => {
                            match t.args[t.args.len() - 1] {
                                syn::GenericArgument::Type(ref gen_ty_path) => match gen_ty_path {
                                    syn::Type::Path(arg_ty_path) => {
                                        let generic_segments = &(arg_ty_path.path.segments);
                                        generic_ident = Some(
                                            &(generic_segments[generic_segments.len() - 1].ident),
                                        );
                                    }
                                    _ => (),
                                },
                                _ => (),
                            }
                            println!("angle bracketed = {:?}", t)
                        }
                        _ => (),
                    }

                    println!("found return type => {:?}", ident);
                    return if ident.to_owned().to_string() == "Vec" {
                        match generic_ident {
                            Some(generic_ident) => {
                                let ast = AstType::Vec(AstBaseType::from(
                                    generic_ident.to_owned().to_string(),
                                ));
                                Ok((
                                    ast,
                                    format!("Vec<{}>", generic_ident.to_owned().to_string()),
                                ))
                            }
                            None => {
                                let origin = ident.to_string();
                                Ok((AstType::from(ident.to_string()), origin))
                            }
                        }
                    } else if ident.to_owned().to_owned() == "Box" {
                        let origin = generic_ident.unwrap().to_owned().to_string();
                        Ok((AstType::from("Box".to_owned()), origin.to_owned()))
                    } else {
                        let origin = ident.to_string();
                        Ok((AstType::from(ident.to_string()), origin))
                    };
                }

                _ => (),
            }
        }
        syn::ReturnType::Default => return Ok((AstType::Void, "".to_owned())),
    }

    Err(ParseError("can't parse return type".to_string()).into())
}

///
/// parse one argument
///
fn parse_one_arg(input: &syn::FnArg) -> Result<ArgDesc> {
    let mut arg_name: Option<String> = Some("".to_owned());
    let mut arg_type: Option<AstType> = Some(AstType::Void);
    let mut origin_arg_ty: Option<String> = Some("".to_owned());
    match input {
        syn::FnArg::Captured(ref arg) => {
            match arg.pat {
                syn::Pat::Ident(ref pat_ident) => {
                    arg_name = Some(pat_ident.ident.to_string());
                    println!("found arg pat = {:?}", pat_ident.ident.to_string());
                }
                _ => (),
            }

            match arg.ty {
                syn::Type::Path(ref type_path) => {
                    let segments = &(type_path.path.segments);
                    let ident = (&segments[segments.len() - 1].ident).to_string();
                    if ident.clone() == "Box" {
                        println!("found Box argument.");
                        let angle_bracketed = &segments[segments.len() - 1].arguments;
                        match angle_bracketed {
                            syn::PathArguments::AngleBracketed(t) => {
                                let arg = &t.args[0];
                                match arg {
                                    syn::GenericArgument::Type(ty) => match ty {
                                        syn::Type::Path(ref type_path) => {
                                            println!("found boxed types = {:?})", type_path);
                                            let segments = &(type_path.path.segments);
                                            let ident =
                                                (&segments[segments.len() - 1].ident).to_string();
                                            arg_type = Some(AstType::from("Box".clone()));
                                            origin_arg_ty = Some(ident.clone());
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    } else if ident.clone() == "Vec" {
                        println!("found Vec argument.");
                        let angle_bracketed = &segments[segments.len() - 1].arguments;
                        match angle_bracketed {
                            syn::PathArguments::AngleBracketed(t) => {
                                let arg = &t.args[0];
                                match arg {
                                    syn::GenericArgument::Type(ty) => match ty {
                                        syn::Type::Path(ref type_path) => {
                                            println!("found vec types = {:?})", type_path);
                                            let segments = &(type_path.path.segments);
                                            let ident =
                                                (&segments[segments.len() - 1].ident).to_string();
                                            arg_type = Some(AstType::Vec(AstBaseType::from(
                                                ident.clone(),
                                            )));
                                            origin_arg_ty = Some(format!("Vec<{}>", ident.clone()));
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    } else {
                        // normal arguments
                        arg_type = Some(AstType::from(ident.clone()));
                        origin_arg_ty = Some(ident.clone());
                        println!("found args type => {:?}", ident);
                    }
                }

                _ => (),
            }
        }
        _ => (),
    }

    match (arg_name, arg_type, origin_arg_ty) {
        (Some(arg_name), Some(arg_type), Some(origin_ty)) => Ok(ArgDesc {
            name: arg_name,
            ty: arg_type,
            origin_ty,
        }),
        _ => Err(ParseError("parse argments error!".to_string()).into()),
    }
}
