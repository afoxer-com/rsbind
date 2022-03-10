use std::fs::File;
use std::io::Write;
use std::path::Path;

use proc_macro2::{Ident, Span, TokenStream};

use crate::ast::contract::desc::*;
use crate::ast::imp::desc::*;
use crate::ast::types::*;
use crate::errors::ErrorKind::*;
use crate::errors::*;

pub(crate) const TMP_ARG_PREFIX: &str = "r";

struct GenResult {
    pub result: Result<TokenStream>,
}

pub(crate) enum TypeDirection {
    Argument,
    Return,
}
///
/// Executor for generationg core files of bridge mod.
///
pub(crate) struct BridgeFileGen<'a, T: FileGenStrategy> {
    pub out_dir: &'a Path,
    pub trait_descs: &'a [TraitDesc],
    pub struct_descs: &'a [StructDesc],
    pub imp_desc: &'a [ImpDesc],
    pub strategy: T,
}

///
/// Strategy for generating core files in bridge mod.
///
pub(crate) trait FileGenStrategy {
    fn gen_sdk_file(&self, mod_names: &[String]) -> Result<TokenStream>;
    fn quote_common_use_part(&self) -> Result<TokenStream>;
    fn quote_common_part(&self, trait_desc: &[TraitDesc]) -> Result<TokenStream>;
    fn quote_callback_structures(&self, callback: &TraitDesc) -> Result<TokenStream>;
    fn quote_for_structures(&self, struct_desc: &StructDesc) -> Result<TokenStream>;
    fn quote_method_sig(
        &self,
        trait_desc: &TraitDesc,
        impl_desc: &ImpDesc,
        method: &MethodDesc,
        callbacks: &[&TraitDesc],
        structs: &[StructDesc],
    ) -> Result<TokenStream>;
    fn quote_arg_convert(
        &self,
        trait_desc: &TraitDesc,
        args: &ArgDesc,
        callbacks: &[&TraitDesc],
    ) -> Result<TokenStream>;
    fn quote_return_convert(&self, return_ty: &AstType, ret_name: &str) -> Result<TokenStream>;
    fn ty_to_tokens(&self, ast_type: &AstType, direction: TypeDirection) -> Result<TokenStream>;
}

pub(crate) trait CallbackGenStrategy {
    fn arg_convert(
        &self,
        arg: &ArgDesc,
        trait_desc: &TraitDesc,
        callbacks: &[&TraitDesc],
    ) -> TokenStream;
}

impl<'a, T: FileGenStrategy + 'a> BridgeFileGen<'a, T> {
    ///
    /// generate sdk.rs files
    ///
    pub(crate) fn gen_sdk_file(&self, file_name: &str, mod_names: &[String]) -> Result<()> {
        let result = self.strategy.gen_sdk_file(mod_names).unwrap();

        let out_file_path = self.out_dir.join(file_name);
        let mut f = File::create(&out_file_path).unwrap();
        f.write_all(&result.to_string().into_bytes()).unwrap();

        Ok(())
    }

    ///
    /// generate one bridge file for one contract mod.
    ///
    pub(crate) fn gen_one_bridge_file(&self, file_name: &str) -> Result<()> {
        println!("[bridge][{}]  🔆  begin generate bridge file.", file_name);
        let use_part = self.quote_use_part().unwrap();
        let common_part = self.strategy.quote_common_part(self.trait_descs).unwrap();
        let bridge_codes = self.gen_for_one_mod().unwrap();

        let mut merge_tokens = quote! {
            #use_part
            #common_part
        };

        for bridge_code in bridge_codes {
            if let Ok(code) = bridge_code.result {
                merge_tokens = quote! {
                    #merge_tokens
                    #code
                };
            }
        }

        let out_file_path = self.out_dir.join(file_name);
        let mut f = File::create(&out_file_path).unwrap();
        f.write_all(&merge_tokens.to_string().into_bytes()).unwrap();

        println!("[bridge][{}]  ✅  end generate bridge file.", file_name);
        Ok(())
    }

    ///
    /// generate bridge file from a file of trait.
    ///
    fn gen_for_one_mod(&self) -> Result<Vec<GenResult>> {
        let mut results: Vec<GenResult> = vec![];

        let callbacks = self
            .trait_descs
            .iter()
            .filter(|desc| {
                for imp in self.imp_desc.iter() {
                    if imp.contract == desc.name {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<&TraitDesc>>();

        println!("callbacks is {:?}", &callbacks);

        for struct_desc in self.struct_descs.iter() {
            let tokens = self.strategy.quote_for_structures(struct_desc);
            results.push(GenResult { result: tokens });
        }

        for desc in self.trait_descs.iter() {
            let imps = self
                .imp_desc
                .iter()
                .filter(|info| info.contract == desc.name)
                .collect::<Vec<&ImpDesc>>();

            println!("desc => {:?}", desc);
            println!("imps => {:?}", imps);
            println!("all imps => {:?}", &self.imp_desc);

            if imps.len() > 1 {
                println!("You have more than one impl for trait {}", desc.name);
                return Err(GenerateError(format!(
                    "You have more than one impl for trait {}",
                    desc.name
                ))
                .into());
            } else if imps.is_empty() {
                println!(
                    "You haven't impl the trait {}, so I guess it is a callback",
                    desc.name
                );
                results.push(GenResult {
                    result: self.strategy.quote_callback_structures(desc),
                });
            } else {
                results.push(GenResult {
                    result: self.generate_for_one_trait(
                        desc,
                        imps[0],
                        &callbacks,
                        self.struct_descs,
                    ),
                });
            }
        }

        Ok(results)
    }

    fn generate_for_one_trait(
        &self,
        trait_desc: &TraitDesc,
        imp: &ImpDesc,
        callbacks: &[&TraitDesc],
        structs: &[StructDesc],
    ) -> Result<TokenStream> {
        println!(
            "[bridge][{}]  🔆  begin generate bridge on trait.",
            &trait_desc.name
        );
        let mut merge: TokenStream = TokenStream::new();

        for method in trait_desc.methods.iter() {
            println!(
                "[bridge][{}.{}]  🔆  begin generate bridge method.",
                &trait_desc.name, &method.name
            );
            let one_method = self
                .quote_one_method(trait_desc, imp, method, callbacks, structs)
                .unwrap();

            println!(
                "[bridge][{}.{}]  ✅  end generate bridge method.",
                &trait_desc.name, &method.name
            );

            merge = quote! {
                #merge
                #one_method
            };
        }
        println!(
            "[bridge][{}]  ✅  end generate bridge on trait.",
            &trait_desc.name
        );
        Ok(merge)
    }

    ///
    /// quote use part
    ///
    fn quote_use_part(&self) -> Result<TokenStream> {
        println!("[bridge]  🔆  begin quote use part.");
        let mut merge = self.strategy.quote_common_use_part().unwrap();

        for trait_desc in self.trait_descs.iter() {
            let imps = self
                .imp_desc
                .iter()
                .filter(|info| info.contract == trait_desc.name)
                .collect::<Vec<&ImpDesc>>();

            if imps.len() > 1 {
                println!("You have more than one impl for trait {}", trait_desc.name);
                return Err(GenerateError(format!(
                    "You have more than one impl for trait {}",
                    trait_desc.name
                ))
                .into());
            } else if imps.is_empty() {
                println!(
                    "You haven't impl the trait {}, I guess it is a callback",
                    trait_desc.name
                );
            } else {
                let use_part = self
                    .quote_one_use_part(&trait_desc.mod_name, &imps[0].mod_name)
                    .unwrap();
                merge = quote! {
                   #use_part
                   #merge
                };
            }
        }
        println!("[bridge]  ✅  end quote use part.");
        Ok(merge)
    }

    fn quote_one_use_part(&self, trait_mod_name: &str, imp_mod_name: &str) -> Result<TokenStream> {
        let trait_ident = Ident::new(trait_mod_name, Span::call_site());
        let mod_ident = Ident::new(imp_mod_name, Span::call_site());
        let use_part = quote! {
            use ::imp::#mod_ident::*;
            use ::contract::#trait_ident::*;
        };

        Ok(use_part)
    }

    ///
    /// quote one method
    ///
    fn quote_one_method(
        &self,
        trait_desc: &TraitDesc,
        imp: &ImpDesc,
        method: &MethodDesc,
        callbacks: &[&TraitDesc],
        structs: &[StructDesc],
    ) -> Result<TokenStream> {
        println!(
            "[bridge][{}.{}]  🔆 ️begin quote method.",
            &trait_desc.name, &method.name
        );
        let sig_define = self
            .strategy
            .quote_method_sig(trait_desc, imp, method, callbacks, structs)
            .unwrap();

        let mut arg_convert = TokenStream::new();
        for arg in method.args.iter() {
            let arg_tokens = self
                .strategy
                .quote_arg_convert(trait_desc, arg, callbacks)
                .unwrap();
            arg_convert = quote! {
                #arg_convert
                #arg_tokens
            }
        }

        let call_imp = self.quote_imp_call(&imp.name, method)?;

        let return_handle = self
            .strategy
            .quote_return_convert(&method.return_type, "ret_value")?;

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
            &trait_desc.name, &method.name
        );
        Ok(result)
    }

    fn quote_imp_call(&self, impl_name: &str, method: &MethodDesc) -> Result<TokenStream> {
        println!(
            "[bridge][{}.{}]  🔆 ️begin quote imp call.",
            impl_name, &method.name
        );

        let ret_name_str = "ret_value";
        let imp_fun_name = Ident::new(&method.name, Span::call_site());
        let ret_name_ident = Ident::new(ret_name_str, Span::call_site());

        let tmp_arg_names = method
            .args
            .iter()
            .map(|e| &e.name)
            .map(|arg_name| {
                Ident::new(
                    &format!("{}_{}", TMP_ARG_PREFIX, arg_name),
                    Span::call_site(),
                )
            })
            .collect::<Vec<Ident>>();

        let rust_args_repeat = quote! {
            #(#tmp_arg_names),*
        };

        let imp_ident = Ident::new(impl_name, Span::call_site());
        let imp_call = match method.return_type.clone() {
            AstType::Void => quote! {
                #imp_ident::#imp_fun_name(#rust_args_repeat);
            },
            AstType::Vec(AstBaseType::Byte(origin)) => {
                if origin.contains("i8") {
                    quote! {
                        let mut #ret_name_ident = #imp_ident::#imp_fun_name(#rust_args_repeat);
                    }
                } else {
                    quote! {
                        let #ret_name_ident = #imp_ident::#imp_fun_name(#rust_args_repeat);
                    }
                }
            }
            _ => quote! {
                let #ret_name_ident = #imp_ident::#imp_fun_name(#rust_args_repeat);
            },
        };

        println!(
            "[bridge][{}.{}]  ✅ end quote imp call.",
            impl_name, &method.name
        );

        Ok(imp_call)
    }
}
