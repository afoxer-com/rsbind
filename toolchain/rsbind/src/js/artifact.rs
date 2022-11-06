use crate::ast::types::AstType;
use crate::base::artifact::{FileNameStyle, NativeCodeGen, NativeGenStrategy};
use crate::base::lang::{Convertible, Direction};
use crate::errors::*;
use crate::js::ty::converter::JsConverter;
use crate::{js, AstResult};
use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use rstgen::{Cons, Custom, Formatter, JavaScript, Tokens};
use std::path::PathBuf;

pub(crate) struct JSCodeGen<'a> {
    pub js_gen_dir: &'a PathBuf,
    pub ast: &'a AstResult,
}

impl<'a> JSCodeGen<'a> {
    pub fn gen_files(&self) -> Result<()> {
        let strategy = NativeGenStrategy {
            gen_bridge_callback: Box::new(|ctx, desc| Ok("".to_string())),
            gen_callback: Box::new(|ctx, desc| {
                let mut tokens = Tokens::new();
                push_f!(tokens, "export interface {} {{", &desc.name);
                nested!(tokens, |t| {
                    for method in desc.methods.iter() {
                        let mut method_tokens = Tokens::new();
                        method_tokens.append(format!("{}(", &method.name));
                        for (index, arg) in method.args.iter().enumerate() {
                            let ty = JsConverter {
                                ty: arg.ty.clone(),
                                ast: ctx.ast.clone(),
                            }
                            .native_type();
                            method_tokens.append(format!("{}: {}", &arg.name, ty));
                            if index != method.args.len() - 1 {
                                method_tokens.append(",");
                            }
                        }
                        let return_type = JsConverter {
                            ty: method.return_type.clone(),
                            ast: ctx.ast.clone(),
                        }
                        .native_type();
                        method_tokens.append(format!("): {};", return_type));
                        nested!(t, method_tokens);
                    }
                });
                push_f!(tokens, "}}");
                to_js_file(tokens)
            }),
            gen_bridge_trait: Box::new(|ctx, desc| {
                let mut tokens = Tokens::new();
                push!(tokens, "import { createRequire } from \"module\";");
                push!(tokens, "const require = createRequire(import.meta.url);");
                push!(tokens, "const rustLib = require(\"./rustlib.node\");");
                push!(tokens, "const callbacks = new Map<number, any>();");
                for (_mod_name, structs) in ctx.ast.structs.iter() {
                    for each in structs.iter() {
                        push_f!(tokens, "import {} from \'./{}.js\'", each.name, each.name);
                        push_f!(
                            tokens,
                            "import {{_{}}} from \'./{}.js\'",
                            each.name,
                            each.name
                        );
                    }
                }

                for method in desc.methods.iter() {
                    let method_name = format!(
                        "{}{}",
                        desc.name.to_lower_camel_case(),
                        method.name.to_upper_camel_case()
                    );
                    let mut sig_tokens = toks_f!("export function {}(", &method_name);
                    nested!(sig_tokens, |t| {
                        for (index, arg) in method.args.iter().enumerate() {
                            t.append(format!(
                                "{}: {}",
                                &arg.name,
                                JsConverter {
                                    ty: arg.ty.clone(),
                                    ast: ctx.ast.clone()
                                }
                                .native_type()
                            ));
                            if index != method.args.len() - 1 {
                                t.append(", ");
                            }
                        }
                        if AstType::Void == method.return_type {
                            t.append(") {");
                        } else {
                            t.append(format!(
                                "): {} {{",
                                JsConverter {
                                    ty: method.return_type.clone(),
                                    ast: ctx.ast.clone()
                                }
                                .native_type()
                            ));
                        }
                    });

                    push!(tokens, sig_tokens);

                    // Call functions.
                    let origin_method_name = format!(
                        "{}_{}",
                        desc.name.to_snake_case(),
                        method.name.to_snake_case()
                    );
                    for arg in method.args.iter() {
                        nested_f!(
                            tokens,
                            "const r_{} = {};",
                            arg.name,
                            JsConverter {
                                ty: arg.ty.clone(),
                                ast: ctx.ast.clone()
                            }
                            .native_to_transferable(arg.name.clone(), Direction::Down)
                        );
                    }
                    let mut call_tokens =
                        toks_f!("const result = rustLib.{}(", &origin_method_name);
                    for (index, arg) in method.args.iter().enumerate() {
                        call_tokens.append(format!("r_{}", &arg.name));
                        if index != method.args.len() - 1 {
                            call_tokens.append(", ");
                        }
                    }
                    call_tokens.append(");");
                    nested_f!(tokens, call_tokens);
                    nested_f!(
                        tokens,
                        "return {};",
                        JsConverter {
                            ty: method.return_type.clone(),
                            ast: ctx.ast.clone()
                        }
                        .transferable_to_native("result".to_string(), Direction::Down)
                    );

                    push!(tokens, "}");
                }

                to_js_file(tokens)
            }),
            gen_wrapper_trait: Box::new(|ctx, desc| {
                let mut tokens = Tokens::new();
                let internal_name = format!("Internal{}", &desc.name.to_upper_camel_case());
                push_f!(
                    tokens,
                    "import * as {} from \"./{}.js\"",
                    internal_name,
                    internal_name
                );
                for (_mod_name, structs) in ctx.ast.structs.iter() {
                    for each in structs.iter() {
                        push_f!(tokens, "import {} from \'./{}.js\'", each.name, each.name);
                        push_f!(
                            tokens,
                            "import {{_{}}} from \'./{}.js\'",
                            each.name,
                            each.name
                        );
                    }
                }
                push_f!(tokens, "export class {} {{", &desc.name);
                for method in desc.methods.iter() {
                    let mut method_sig_tokens =
                        toks_f!("{} = (", method.name.to_lower_camel_case());
                    for (index, arg) in method.args.iter().enumerate() {
                        method_sig_tokens.append(format!(
                            "{}: {}",
                            &arg.name,
                            JsConverter {
                                ty: arg.ty.clone(),
                                ast: ctx.ast.clone()
                            }
                            .native_type()
                        ));
                        if index != method.args.len() - 1 {
                            method_sig_tokens.append(", ");
                        }
                    }

                    if AstType::Void == method.return_type {
                        method_sig_tokens.append(") => {");
                    } else {
                        method_sig_tokens.append(format!(
                            "): {} => {{",
                            JsConverter {
                                ty: method.return_type.clone(),
                                ast: ctx.ast.clone()
                            }
                            .native_type()
                        ));
                    }
                    nested!(tokens, method_sig_tokens);

                    //Call methods.
                    let method_name = format!(
                        "{}{}",
                        desc.name.to_lower_camel_case(),
                        method.name.to_upper_camel_case()
                    );
                    let mut call_method_tokens =
                        toks_f!("return {}.{}(", internal_name, method_name);
                    for (index, arg) in method.args.iter().enumerate() {
                        call_method_tokens.append(arg.name.clone());
                        if index != method.args.len() - 1 {
                            call_method_tokens.append(",");
                        }
                    }
                    call_method_tokens.append(");");
                    nested!(tokens, |t| { nested!(t, call_method_tokens) });

                    nested!(tokens, "}")
                }
                push!(tokens, "}");
                to_js_file(tokens)
            }),
            gen_trait: Box::new(|ctx, desc| Ok("".to_string())),
            gen_struct: Box::new(|ctx, desc| {
                let mut tokens = Tokens::new();
                push_f!(tokens, "export default interface {} {{", desc.name);
                nested!(tokens, |t| {
                    for (index, field) in desc.fields.iter().enumerate() {
                        let native_type = JsConverter {
                            ty: field.ty.clone(),
                            ast: ctx.ast.clone(),
                        }
                        .native_type();
                        nested_f!(t, "{}: {};", field.name, native_type);
                    }
                });
                push!(tokens, "}");

                push_f!(tokens, "export interface _{} {{", desc.name);
                nested!(tokens, |t| {
                    for (index, field) in desc.fields.iter().enumerate() {
                        let native_type = JsConverter {
                            ty: field.ty.clone(),
                            ast: ctx.ast.clone(),
                        }
                        .native_transferable_type(Direction::Down);
                        nested_f!(t, "{}: {};", field.name, native_type);
                    }
                });
                push!(tokens, "}");
                to_js_file(tokens)
            }),
            gen_manager: Box::new(|ctx| {
                let mut tokens = Tokens::new();
                for (_, trait_list) in ctx.ast.traits.iter() {
                    for desc in trait_list.iter() {
                        let wrapper_name = format!("Rust{}", &desc.name.to_upper_camel_case());
                        push_f!(
                            tokens,
                            "import * as {} from \'./{}.js\'",
                            desc.name,
                            wrapper_name
                        );
                    }
                }
                for (_mod_name, structs) in ctx.ast.structs.iter() {
                    for each in structs.iter() {
                        push_f!(tokens, "export * from \'./{}.js\'", each.name);
                    }
                }
                push!(tokens, "export default class RustLib {");
                for (_, trait_list) in ctx.ast.traits.iter() {
                    for desc in trait_list.iter() {
                        let cls_name = format!("{}.{}", &desc.name, &desc.name);
                        let mut fn_tokens = toks_f!(
                            "new{} = (): {} => {{",
                            desc.name.to_upper_camel_case(),
                            cls_name
                        );
                        nested_f!(fn_tokens, "return new {}();", cls_name);
                        push!(fn_tokens, "}");
                        nested!(tokens, fn_tokens);
                    }
                }
                push!(tokens, "}");
                to_js_file(tokens)
            }),
        };

        let gen = NativeCodeGen {
            gen_dir: self.js_gen_dir,
            file_ext: "ts".to_string(),
            file_name_style: FileNameStyle::CamelCase,
            ast: self.ast,
            extra: (),
            strategy,
        };

        gen.gen_files()
    }
}

pub(crate) fn to_js_file(tokens: Tokens<JavaScript>) -> Result<String> {
    let mut buf = String::new();
    {
        let mut formatter = Formatter::new(&mut buf);
        let mut extra = ();
        js::JavaScript::write_file(tokens, &mut formatter, &mut extra, 0)?;
    }
    Ok(buf)
}
