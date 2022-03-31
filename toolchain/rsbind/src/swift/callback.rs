use heck::ToLowerCamelCase;
use rstgen::swift::{Argument, Class, Method, Modifier, Protocol, Swift};
use rstgen::{swift, IntoTokens, Tokens};

use crate::ast::contract::desc::{ArgDesc, MethodDesc, TraitDesc};
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::swift::mapping::SwiftMapping;
use crate::swift::types::{to_swift_file, SwiftType};

pub(crate) struct CallbackGen<'a> {
    pub desc: &'a TraitDesc,
}

impl<'a> CallbackGen<'a> {
    pub fn gen(&self) -> Result<String> {
        let mut protocol = Protocol::new(self.desc.name.clone());
        protocol.modifiers = vec![Modifier::Public];

        for method in self.desc.methods.iter() {
            let mut m = Method::new(method.name.to_lower_camel_case());
            m.modifiers = vec![];
            m.returns = Some(Swift::from(SwiftType::new(method.return_type.clone())));
            for arg in method.args.iter() {
                let arg_ty = Swift::from(SwiftType::new(arg.ty.clone()));
                let argument = swift::Argument::new(arg_ty, arg.name.as_ref());
                m.arguments.push(argument)
            }
            protocol.methods.push(m);
        }

        to_swift_file(protocol.into_tokens())
    }
}

pub(crate) struct InternalCallbackGen<'a> {
    pub desc: &'a TraitDesc,
    pub callbacks: &'a [&'a TraitDesc],
}

impl<'a> InternalCallbackGen<'a> {
    pub fn gen(&self) -> Result<String> {
        let mut tokens = Tokens::new();

        let global_vars = toks!(
            "private  var globalIndex : Int64 = 0\n",
            "private  var globalCallbacks : [Int64: Any] = [Int64: Any]()\n"
        );
        tokens.push(global_vars);

        let class_name = format!("Internal{}", self.desc.name.clone());
        let mut class = Class::new(class_name);
        class.modifiers = vec![Modifier::Internal];

        self.fill_callback_to_model(&mut class)?;
        self.fill_model_to_callback(&mut class)?;

        tokens.push(class.into_tokens());

        to_swift_file(tokens)
    }

    fn fill_model_to_callback(&self, outer_cls: &mut Class<'a>) -> Result<()> {
        let mut m = Method::new("modelToCallback");
        m.modifiers = vec![Modifier::Internal, Modifier::Static];
        let model_name = format!("{}_{}_Model", &self.desc.mod_name, &self.desc.name);
        m.arguments
            .push(Argument::new(swift::local(model_name), "model"));
        m.returns(swift::local(self.desc.name.clone()));

        let mut body = Tokens::new();

        let class_name = format!("{}Wrapper", self.desc.name);
        let mut class = swift::Class::new(class_name.clone());
        class.modifiers = vec![];
        class.implements.push(swift::local(self.desc.name.clone()));

        push!(class.body, "deinit {");
        nested!(class.body, "self.model.free_callback(self.model.index)");
        class.body.push("}");

        let callback_model_str = format!("{}_{}_Model", &self.desc.mod_name, &self.desc.name);
        let mut proxy_field = swift::Field::new(swift::local(callback_model_str.clone()), "model");
        proxy_field.modifiers = vec![swift::Modifier::Private];
        class.fields.push(proxy_field);

        let mut constructor = swift::Constructor::new();
        let constructor_arg = swift::Argument::new(swift::local(callback_model_str), "model");
        constructor.arguments.push(constructor_arg);
        push!(constructor.body, "self.model = model");
        class.constructors.push(constructor);

        for method in self.desc.methods.iter() {
            let mut cls_method = swift::Method::new(method.name.to_lower_camel_case());
            for arg in method.args.iter() {
                let cls_method_arg = swift::Argument::new(
                    SwiftMapping::map_swift_sig_type(&arg.ty),
                    arg.name.clone(),
                );
                cls_method.arguments.push(cls_method_arg);
            }
            cls_method.returns(SwiftMapping::map_swift_sig_type(&method.return_type));

            let mut method_body = Tokens::new();

            let mut byte_count = 0;
            for arg in method.args.iter() {
                match arg.ty.clone() {
                    AstType::Vec(AstBaseType::Byte(_))
                    | AstType::Vec(AstBaseType::Short(_))
                    | AstType::Vec(AstBaseType::Int(_))
                    | AstType::Vec(AstBaseType::Long(_)) => {
                        byte_count += 1;
                        push!(
                            method_body,
                            arg.name.clone(),
                            ".withUnsafeBufferPointer { ",
                            arg.name.clone(),
                            "_buffer in"
                        );
                    }
                    _ => {}
                }
            }

            // argument convert
            for arg in method.args.iter() {
                crate::swift::artifact_s2r::fill_arg_convert(
                    &mut method_body,
                    arg,
                    self.callbacks,
                )?;
            }

            // call native method
            self.fill_call_native_method(&mut method_body, method)?;

            // return convert
            crate::swift::artifact_s2r::fill_return_type_convert(
                &mut method_body,
                &method.return_type,
                &self.desc.crate_name,
                self.callbacks,
            )?;

            for _i in 0..byte_count {
                method_body.push("}");
            }

            cls_method.body = method_body;
            class.methods.push(cls_method);
        }
        body.push(class.into_tokens());
        push!(body, "return ", class_name, "(model: model)");
        m.body.push(body);
        outer_cls.methods.push(m);

        Ok(())
    }

    fn fill_call_native_method(
        &self,
        method_body: &mut Tokens<Swift>,
        method: &MethodDesc,
    ) -> Result<()> {
        let method_name = method.name.clone();
        match method.return_type.clone() {
            AstType::Void => {
                push!(method_body, "self.model.", method_name, "(");
            }
            _ => {
                println!("quote method call for {}", method_name);
                push!(method_body, "let result = self.model.", method_name, "(");
            }
        }

        method_body.append(toks!("self.model.index"));
        if !method.args.is_empty() {
            method_body.append(toks!(","));
        }
        for (index, item) in method.args.clone().into_iter().enumerate() {
            let converted = format!("s_{}", &item.name);
            if index == method.args.len() - 1 {
                method_body.append(toks!(converted));
            } else {
                method_body.append(toks!(converted, ", "));
            }
        }

        method_body.append(")");
        Ok(())
    }

    fn fill_callback_to_model(&self, class: &mut Class<'a>) -> Result<()> {
        let mut m = Method::new("callbackToModel");
        m.modifiers = vec![Modifier::Internal, Modifier::Static];
        let model_name = format!("{}_{}_Model", &self.desc.mod_name, &self.desc.name);
        m.returns(swift::local(model_name));
        m.arguments.push(Argument::new(
            swift::local(self.desc.name.clone()),
            "callback",
        ));
        let mut method_body: Tokens<Swift> = toks!();

        // Store the callback to global callback map.
        self.fill_callback_index(&mut method_body)?;

        let cb = self.desc;

        let mut cb_args_model = "".to_string();
        for cb_method in cb.methods.iter() {
            self.fill_cb_closure_method_sig(cb_method, self.callbacks, &mut method_body)?;

            nested!(
                method_body,
                "let origin_callback = globalCallbacks[index] as! ",
                cb.name.clone()
            );

            for cb_arg in cb_method.args.iter() {
                self.fill_cb_closure_arg_convert(cb_arg, self.callbacks, &mut method_body)?;
            }

            self.fill_cb_closure_call(cb_method, &mut method_body)?;

            self.fill_cb_closure_return_convert(cb_method, self.callbacks, &mut method_body)?;

            push!(method_body, "}");

            cb_args_model = format!(
                "{}{}:arg_{},",
                cb_args_model, &cb_method.name, &cb_method.name
            );
        }
        self.fill_cb_closure_free_fn(&mut method_body)?;
        self.fill_cb_closure_free_ptr_fn(&mut method_body)?;

        method_body.push(toks!(format!(
            "return {}_{}_Model({}free_callback: arg_callback_free, free_ptr: arg_ptr_free, index: callback_index)\n",
            &self.desc.mod_name,
            &cb.name,
            cb_args_model,
        )));

        m.body.push(method_body);
        class.methods.push(m);
        Ok(())
    }

    fn fill_callback_index(&self, method_body: &mut Tokens<Swift>) -> Result<()> {
        push!(method_body, "let callback_index = globalIndex + 1");
        push!(method_body, "globalIndex = callback_index");
        push!(method_body, "globalCallbacks[callback_index] = callback",);

        Ok(())
    }

    fn fill_cb_closure_method_sig(
        &self,
        cb_method: &MethodDesc,
        callbacks: &[&TraitDesc],
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        let mut arg_params = "(index".to_owned();
        let mut args_str = "(Int64".to_owned();
        for cb_arg in cb_method.args.iter() {
            let cb_arg_ty = SwiftMapping::map_transfer_type(&cb_arg.ty, callbacks);
            arg_params = format!("{}, {}", &arg_params, &cb_arg.name);
            args_str = format!("{}, {}", &args_str, &cb_arg_ty);
        }
        arg_params = format!("{})", &arg_params);
        args_str = format!("{})", &args_str);

        let cb_return_ty = SwiftMapping::map_transfer_type(&cb_method.return_type, callbacks);
        let closure = format!("{} -> {}", &args_str, &cb_return_ty);
        arg_params = format!("{} -> {}", &arg_params, &cb_return_ty);

        push!(
            method_body,
            "let ",
            format!("arg_{}", &cb_method.name),
            ": @convention(c) ",
            closure,
            " = {"
        );
        nested!(method_body, arg_params, " in\n");
        Ok(())
    }

    fn fill_cb_closure_arg_convert<'b>(
        &self,
        cb_arg: &'a ArgDesc,
        callbacks: &'a [&'a TraitDesc],
        method_body: &'b mut Tokens<'a, Swift<'a>>,
    ) -> Result<()> {
        crate::swift::artifact_r2s::fill_arg_convert(cb_arg, callbacks, method_body)
    }

    fn fill_cb_closure_call(
        &self,
        cb_method: &MethodDesc,
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        let mut cb_method_call = "(".to_string();
        for (index, cb_arg) in cb_method.args.iter().enumerate() {
            cb_method_call = format!("{}{}: c_{}", &cb_method_call, &cb_arg.name, &cb_arg.name);
            if index != cb_method.args.len() - 1 {
                cb_method_call = format!("{}, ", &cb_method_call);
            }
        }

        cb_method_call = format!("{})", &cb_method_call);

        match cb_method.return_type.clone() {
            AstType::Void => {
                nested!(
                    method_body,
                    "origin_callback.",
                    cb_method.name.to_lower_camel_case(),
                    cb_method_call
                );
            }
            _ => {
                nested!(
                    method_body,
                    "let result = origin_callback.",
                    cb_method.name.to_lower_camel_case(),
                    cb_method_call
                );
            }
        }

        Ok(())
    }

    fn fill_cb_closure_return_convert(
        &self,
        cb_method: &MethodDesc,
        callbacks: &[&TraitDesc],
        method_body: &mut Tokens<Swift>,
    ) -> Result<()> {
        crate::swift::artifact_r2s::fill_return_convert(cb_method, callbacks, method_body)
    }

    fn fill_cb_closure_free_fn(&self, method_body: &mut Tokens<Swift>) -> Result<()> {
        push!(
            method_body,
            "let arg_callback_free : @convention(c)(Int64) -> () = {"
        );
        nested!(method_body, "(index) in");
        nested!(method_body, "globalCallbacks.removeValue(forKey: index)");
        push!(method_body, "}");
        Ok(())
    }

    fn fill_cb_closure_free_ptr_fn(&self, method_body: &mut Tokens<Swift>) -> Result<()> {
        push!(
            method_body,
            "let arg_ptr_free : @convention(c) (UnsafeMutablePointer<Int8>?, Int32) -> () = {"
        );
        nested!(method_body, "(ptr, count) in");
        nested!(method_body, "ptr?.deinitialize(count: Int(count))");
        nested!(method_body, "ptr?.deallocate()");
        push!(method_body, "}");
        Ok(())
    }
}
