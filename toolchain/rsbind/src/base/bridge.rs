use crate::ast::contract::desc::TraitDesc;
use crate::ast::imp::desc::ImpDesc;
use crate::ast::types::AstType;
use crate::base::lang::{
    ArgumentContext, BridgeContext, CallbackContext, Direction, LangImp, MethodContext, ModContext,
    ServiceContext, StructContext,
};
use crate::errors::*;
use crate::ErrorKind::GenerateError;
use crate::{ident, AstResult};
use proc_macro2::{Ident, TokenStream};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;

type TokenResult = Result<TokenStream>;

///
/// Used for generating bridge modules.
/// Bridge module is a rust module used for bridging rust code and native code.
///
/// We assume that bridge module have several files:
/// 1. lib.rs
/// 2. common.rs
/// 4. bridge files for each mod.
///
/// Bridge files have several parts:
/// 1. use package part.
/// 2. common part.
/// 3. bridge code part.
///
/// Bridge code part have several parts:
/// 1. Bridge function signature.
/// 2. Arguments converting code block.
/// 3. Real code invoking code block.
/// 4. Return value converting code block.
///
pub(crate) struct BaseBridgeGen<'a, Lang, Extra> {
    pub lang_name: String,
    pub ast: &'a AstResult,
    pub bridge_dir: &'a Path,
    pub crate_name: String,
    pub lang_imp: Box<dyn LangImp<Lang, Extra>>,
    pub extra: Extra,
    pub generator: FilesGenerator<Lang, Extra>,
}

impl<'a, Lang, Extra> BaseBridgeGen<'a, Lang, Extra> {
    pub(crate) fn gen(&self) -> Result<()> {
        let ctx = BridgeContext {
            ast: self.ast,
            crate_name: self.crate_name.to_owned(),
            extra: &self.extra,
            lang_imp: &self.lang_imp,
            lang_name: self.lang_name.to_owned(),
        };
        let file_vec = self.generator.gen_files(&ctx)?;
        for file_token in file_vec.into_iter() {
            self.write(&file_token.0, &file_token.1)?;
        }

        Ok(())
    }

    fn write(&self, file: &str, tokens: &TokenStream) -> Result<()> {
        let file_path = self.bridge_dir.join(file);
        let mut file = File::create(&file_path)?;
        file.write_all(&tokens.to_string().into_bytes())?;
        Ok(())
    }
}

///
/// Used for generating all files.
///
pub(crate) struct FilesGenerator<Lang, Extra> {
    pub(crate) quote_lib_file: Box<dyn Fn(&BridgeContext<Lang, Extra>) -> TokenResult>,
    pub(crate) quote_common_file: Box<dyn Fn(&BridgeContext<Lang, Extra>) -> TokenResult>,
    pub(crate) bridge_file_generator: BridgeFileGenerator<Lang, Extra>,
}

impl<Lang, Extra> FilesGenerator<Lang, Extra> {
    fn gen_files(&self, ctx: &BridgeContext<Lang, Extra>) -> Result<Vec<(String, TokenStream)>> {
        let mut file_vec = vec![];
        let empty_vec = vec![];
        let mut bridges: Vec<String> = vec![];

        for (mod_name, trait_vec) in &ctx.ast.traits {
            let struct_vec = ctx.ast.structs.get(mod_name).unwrap_or(&empty_vec);

            // generate bridge files.
            let out_mod_name = format!("{}_{}", &ctx.lang_name, mod_name);
            let out_file_name = format!("{}.rs", &out_mod_name);

            let ctx = ModContext {
                traits: trait_vec,
                structs: struct_vec,
                imps: &ctx.ast.imps,
                mod_name: mod_name.clone(),
                bridge_ctx: ctx,
            };

            let tokens = self.bridge_file_generator.quote_one_bridge_file(&ctx)?;
            file_vec.push((out_file_name, tokens));
            bridges.push(out_mod_name)
        }

        // generate lib.rs
        bridges.push("common".to_owned());
        let bridge_ident = bridges
            .iter()
            .map(|bridge| ident!(bridge))
            .collect::<Vec<Ident>>();

        let bridge_mod_tokens = quote! {
            #(pub mod #bridge_ident;)*
        };

        let sdk_tokens = (*self.quote_lib_file)(ctx)?;
        let lib_tokens = quote! {
            #sdk_tokens
            #bridge_mod_tokens
        };
        file_vec.push(("lib.rs".to_owned(), lib_tokens));

        // generate common.rs
        let tokens = (*self.quote_common_file)(ctx)?;
        file_vec.push(("common.rs".to_owned(), tokens));

        Ok(file_vec)
    }
}

impl<Lang, Extra> Default for FilesGenerator<Lang, Extra> {
    fn default() -> Self {
        FilesGenerator {
            quote_lib_file: Box::new(|ctx| ctx.lang_imp.quote_lib_file(ctx)),
            quote_common_file: Box::new(|ctx| ctx.lang_imp.quote_common_file(ctx)),
            bridge_file_generator: BridgeFileGenerator {
                bridge_code_generator: BridgeCodeGenerator {
                    trait_generator: TraitCodeGenerator {
                        trait_method_generator: TraitMethodGenerator {
                            quote_method_sig: Box::new(|ctx| {
                                ctx.service_ctx
                                    .mod_ctx
                                    .bridge_ctx
                                    .lang_imp
                                    .quote_method_sig(ctx)
                            }),
                            quote_arg_convert: Box::new(|ctx| {
                                println!(
                                    "[bridge]  🔆  begin quote bridge method argument convert => {}:{}",
                                    &ctx.arg.name,
                                    &ctx.arg.ty.origin()
                                );
                                let rust_arg_str = format!("r_{}", &ctx.arg.name);
                                let rust_arg_name = ident!(&rust_arg_str);
                                let arg_name_ident = ident!(&ctx.arg.name);
                                let convert = ctx
                                    .method_ctx
                                    .service_ctx
                                    .mod_ctx
                                    .bridge_ctx
                                    .lang_imp
                                    .provide_converter(&ctx.arg.ty)
                                    .transferable_to_rust(
                                        quote! {#arg_name_ident},
                                        Direction::Down,
                                    );
                                let result = quote! {
                                    let #rust_arg_name = #convert;
                                };

                                println!(
                                    "[bridge] ✅ end quote bridge method argument convert => {}:{}",
                                    &ctx.arg.name,
                                    &ctx.arg.ty.origin()
                                );
                                Ok(result)
                            }),
                            quote_method_imp_call: Box::new(|ctx| {
                                println!(
                                    "[bridge][{}.{}]  🔆 ️begin quote imp call.",
                                    &ctx.service_ctx.imp.name, &ctx.method.name
                                );

                                let ret_name_str = "result";
                                let imp_fun_name = ident!(&ctx.method.name);
                                let ret_name_ident = ident!(ret_name_str);

                                let tmp_arg_names = ctx
                                    .method
                                    .args
                                    .iter()
                                    .map(|e| &e.name)
                                    .map(|arg_name| ident!(&format!("r_{}", arg_name)))
                                    .collect::<Vec<Ident>>();

                                let rust_args_repeat = quote! {
                                    #(#tmp_arg_names),*
                                };

                                let imp_ident = ident!(&ctx.service_ctx.imp.name);
                                let imp_call = quote! {
                                    let #ret_name_ident = #imp_ident::#imp_fun_name(#rust_args_repeat);
                                };

                                println!(
                                    "[bridge][{}.{}]  ✅ end quote imp call.",
                                    ctx.service_ctx.imp.name, &ctx.method.name
                                );

                                Ok(imp_call)
                            }),
                            quote_method_return_convert: Box::new(|ctx| {
                                println!(
                                    "[bridge]  🔆  begin quote jni bridge method return convert => {}",
                                    ctx.method.return_type.origin()
                                );

                                let result = ctx
                                    .service_ctx
                                    .mod_ctx
                                    .bridge_ctx
                                    .lang_imp
                                    .provide_converter(&ctx.method.return_type)
                                    .rust_to_transferable(quote! {result}, Direction::Down);

                                println!(
                                    "[bridge]  ✅  end quote jni bridge method return convert => {}",
                                    ctx.method.return_type.origin()
                                );

                                Ok(result)
                            }),
                        },
                    },
                    callback_generator: CallbackCodeGenerator {
                        phantom_lang: Default::default(),
                        phantom_extra: Default::default(),
                    },
                    struct_generator: StructCodeGenerator {
                        phantom_lang: Default::default(),
                        phantom_extra: Default::default(),
                    },
                },
                quote_use_part: Box::new(|ctx| {
                    println!("[bridge]  🔆  begin quote use part.");
                    let mut merge = ctx.bridge_ctx.lang_imp.quote_use_part(ctx).unwrap();

                    let origin_crate = ctx.bridge_ctx.crate_name.to_owned();
                    let crate_ident = ident!(&origin_crate.replace("-", "_"));
                    for trait_desc in ctx.traits.iter() {
                        if trait_desc.is_callback {
                            println!("Skip callback trait {}", &trait_desc.name);
                            continue;
                        }

                        if let Ok(imp) = find_imp(trait_desc, ctx.imps) {
                            let trait_mod_splits: Vec<Ident> = trait_desc
                                .mod_path
                                .split("::")
                                .collect::<Vec<&str>>()
                                .iter()
                                .map(|str| ident!(str))
                                .collect();
                            let imp_mod_splits: Vec<Ident> = imp
                                .mod_path
                                .split("::")
                                .collect::<Vec<&str>>()
                                .iter()
                                .map(|str| ident!(str))
                                .collect();

                            let use_part = quote! {
                                use #crate_ident::#(#trait_mod_splits::)**;
                                use #crate_ident::#(#imp_mod_splits::)**;
                            };

                            merge = quote! {
                                #use_part
                                #merge
                            };
                        }
                    }
                    println!("[bridge]  ✅  end quote use part.");
                    Ok(merge)
                }),
                quote_common_part: Box::new(|ctx| ctx.bridge_ctx.lang_imp.quote_common_part(ctx)),
            },
        }
    }
}

///
/// Used for quote one bridge file.
///
pub(crate) struct BridgeFileGenerator<Lang, Extra> {
    pub(crate) bridge_code_generator: BridgeCodeGenerator<Lang, Extra>,
    pub(crate) quote_use_part: Box<dyn Fn(&ModContext<Lang, Extra>) -> TokenResult>,
    pub(crate) quote_common_part: Box<dyn Fn(&ModContext<Lang, Extra>) -> TokenResult>,
}

impl<Lang, Extra> BridgeFileGenerator<Lang, Extra> {
    fn quote_one_bridge_file(&self, ctx: &ModContext<Lang, Extra>) -> TokenResult {
        println!("[bridge] 🔆  begin generate bridge file.");
        let use_part = (*self.quote_use_part)(ctx)?;
        let common_part = (*self.quote_common_part)(ctx)?;
        let bridge_codes = self.bridge_code_generator.gen_bridge_code(ctx);

        let mut merge_tokens = quote! {
            #use_part
            #common_part
        };

        for bridge_code in bridge_codes.into_iter().flatten() {
            merge_tokens = quote! {
                #merge_tokens
                #bridge_code
            };
        }

        println!("[bridge] ✅  end generate bridge file.");
        Ok(merge_tokens)
    }
}

///
/// Used for generating bridge code.
///
pub(crate) struct BridgeCodeGenerator<Lang, Extra> {
    pub(crate) trait_generator: TraitCodeGenerator<Lang, Extra>,
    pub(crate) callback_generator: CallbackCodeGenerator<Lang, Extra>,
    pub(crate) struct_generator: StructCodeGenerator<Lang, Extra>,
}

impl<Lang, Extra> BridgeCodeGenerator<Lang, Extra> {
    fn gen_bridge_code(&self, ctx: &ModContext<Lang, Extra>) -> Vec<TokenResult> {
        let mut results: Vec<TokenResult> = vec![];

        for trait_ in ctx.traits.iter() {
            if trait_.is_callback {
                continue;
            }

            if let Ok(imp) = find_imp(trait_, ctx.imps) {
                let ctx = ServiceContext {
                    trait_,
                    imp,
                    mod_ctx: ctx,
                };
                results.push(self.trait_generator.quote_for_one_trait(&ctx));
            }
        }

        let callbacks = ctx
            .traits
            .iter()
            .filter(|desc| desc.is_callback)
            .collect::<Vec<&TraitDesc>>();

        // Generate callback enums, like below:
        // enum CallbackEnum {
        //     DemoCallback(Box<dyn DemoCallback>),
        //     DemoCallback2(Box<dyn DemoCallback2>),
        // }
        let enum_items = callbacks
            .iter()
            .map(|item| ident!(&item.name))
            .collect::<Vec<Ident>>();

        let enum_tokens = quote! {
            enum CallbackEnum {
                #(#enum_items(Box<dyn #enum_items>)),*
            }
        };

        results.push(Ok(enum_tokens));

        for callback in callbacks.iter() {
            let ctx = CallbackContext {
                callback,
                mod_ctx: ctx,
            };
            let tokens = self.callback_generator.quote_for_one_callback(&ctx);
            results.push(tokens);
        }

        for struct_ in ctx.structs.iter() {
            let ctx = StructContext {
                struct_,
                mod_ctx: ctx,
            };
            let tokens = self.struct_generator.quote_for_one_struct(&ctx);
            results.push(tokens);
        }

        results
    }
}

///
/// Used for generating for one trait.
///
pub(crate) struct TraitCodeGenerator<Lang, Extra> {
    pub(crate) trait_method_generator: TraitMethodGenerator<Lang, Extra>,
}

impl<Lang, Extra> TraitCodeGenerator<Lang, Extra> {
    pub(crate) fn quote_for_one_trait(&self, ctx: &ServiceContext<Lang, Extra>) -> TokenResult {
        println!(
            "[bridge][{}]  🔆  begin generate bridge on trait.",
            &ctx.trait_.name
        );
        let mut merge: TokenStream = TokenStream::new();

        for method in ctx.trait_.methods.iter() {
            println!(
                "[bridge][{}.{}]  🔆  begin generate bridge method.",
                &ctx.trait_.name, &method.name
            );
            let ctx = MethodContext {
                method,
                service_ctx: ctx,
            };
            let one_method = self
                .trait_method_generator
                .quote_for_one_trait_method(&ctx)?;

            println!(
                "[bridge][{}.{}]  ✅  end generate bridge method.",
                &ctx.service_ctx.trait_.name, &method.name
            );

            merge = quote! {
                #merge
                #one_method
            };
        }
        println!(
            "[bridge][{}]  ✅  end generate bridge on trait.",
            &ctx.trait_.name
        );
        Ok(merge)
    }
}

///
/// Used for generate code for one trait method.
///
pub(crate) struct TraitMethodGenerator<Lang, Extra> {
    pub(crate) quote_method_sig: Box<dyn Fn(&MethodContext<Lang, Extra>) -> TokenResult>,
    pub(crate) quote_arg_convert: Box<dyn Fn(&ArgumentContext<Lang, Extra>) -> TokenResult>,
    pub(crate) quote_method_imp_call: Box<dyn Fn(&MethodContext<Lang, Extra>) -> TokenResult>,
    pub(crate) quote_method_return_convert: Box<dyn Fn(&MethodContext<Lang, Extra>) -> TokenResult>,
}

impl<Lang, Extra> TraitMethodGenerator<Lang, Extra> {
    fn quote_for_one_trait_method(&self, ctx: &MethodContext<Lang, Extra>) -> TokenResult {
        println!(
            "[bridge][{}.{}]  🔆 ️begin quote method.",
            &ctx.service_ctx.trait_.name, &ctx.method.name
        );
        let sig_define = (*self.quote_method_sig)(ctx).unwrap();

        let mut arg_convert = TokenStream::new();
        for arg in ctx.method.args.iter() {
            let ctx = ArgumentContext {
                arg,
                method_ctx: ctx,
            };
            let arg_tokens = (*self.quote_arg_convert)(&ctx).unwrap();
            arg_convert = quote! {
                #arg_convert
                #arg_tokens
            }
        }

        let call_imp = (*self.quote_method_imp_call)(ctx)?;

        let return_handle = (*self.quote_method_return_convert)(ctx)?;

        // combine all the parts
        let result = quote! {
            #sig_define {
                #arg_convert
                #call_imp
                #return_handle
            }
        };

        println!(
            "[bridge][{}.{}] ✅ end quote method.",
            &ctx.service_ctx.trait_.name, &ctx.method.name
        );
        Ok(result)
    }
}

///
/// Used for quote for one callback
///
pub(crate) struct CallbackCodeGenerator<Lang, Extra> {
    phantom_lang: PhantomData<Lang>,
    phantom_extra: PhantomData<Extra>,
}

impl<Lang, Extra> CallbackCodeGenerator<Lang, Extra> {
    fn quote_for_one_callback(&self, context: &CallbackContext<Lang, Extra>) -> TokenResult {
        context
            .mod_ctx
            .bridge_ctx
            .lang_imp
            .quote_for_one_callback(context)
    }
}

pub(crate) struct StructCodeGenerator<Lang, Extra> {
    phantom_lang: PhantomData<Lang>,
    phantom_extra: PhantomData<Extra>,
}

impl<Lang, Extra> StructCodeGenerator<Lang, Extra> {
    fn quote_for_one_struct(&self, context: &StructContext<Lang, Extra>) -> TokenResult {
        context
            .mod_ctx
            .bridge_ctx
            .lang_imp
            .quote_for_one_struct(context)
    }
}

fn find_imp<'a>(trait_: &'a TraitDesc, imps: &'a [ImpDesc]) -> Result<&'a ImpDesc> {
    let imps = imps
        .iter()
        .filter(|info| info.contract == trait_.name)
        .collect::<Vec<&ImpDesc>>();

    return match imps.len().cmp(&1) {
        Ordering::Less => {
            println!("No impl found for trait {}", trait_.name);
            Err(GenerateError(format!("No impl found for trait {}", trait_.name)).into())
        }
        Ordering::Equal => Ok(imps[0]),
        Ordering::Greater => {
            panic!("You have more than one impl for trait {}", trait_.name);
        }
    };
}
