use rstgen::swift::{local, Argument, Constructor, Field, Method, Modifier, Swift};
use rstgen::{swift, IntoTokens};
use syn::__private::str;

use crate::ast::contract::desc::StructDesc;
use crate::ast::types::{AstBaseType, AstType};
use crate::errors::*;
use crate::swift::mapping::SwiftMapping;
use crate::swift::types::{to_swift_file, SwiftType};

pub(crate) struct StructGen<'a> {
    pub desc: &'a StructDesc,
}

impl<'a> StructGen<'a> {
    pub fn gen(&self) -> Result<String> {
        let mut struct_ = swift::Struct::new(self.desc.name.clone());
        struct_.modifiers.push(Modifier::Public);
        struct_.implements.push(local("Codable"));

        let mut constructor1 = Constructor::new();
        constructor1.modifiers = vec![Modifier::Public];
        for arg in self.desc.fields.iter() {
            let field_ty = SwiftType::new(arg.ty.clone());
            let swift_ty = Swift::from(field_ty);
            let mut swift_field = Field::new(swift_ty.clone(), arg.name.clone());
            swift_field.modifiers = vec![Modifier::Public];
            struct_.fields.push(swift_field);

            constructor1
                .arguments
                .push(Argument::new(swift_ty.clone(), arg.name.clone()));
            push!(
                constructor1.body,
                "self.",
                arg.name.clone(),
                " = ",
                arg.name.clone()
            );
        }
        struct_.constructors.push(constructor1);
        struct_.constructors.push(self.create_proxy_constructor());
        struct_.methods.push(self.create_into_proxy_fn());

        to_swift_file(struct_.into_tokens())
    }

    fn create_into_proxy_fn(&self) -> Method {
        let mut method = swift::Method::new("intoProxy");
        method.modifiers = vec![Modifier::Internal];
        method.returns(swift::local(format!("Proxy{}", &self.desc.name)));
        push!(method.body, "return Proxy", self.desc.name.clone(), " (");
        for (index, field) in self.desc.fields.iter().enumerate() {
            match field.ty.clone() {
                AstType::Void => {
                    nested!(
                        method.body,
                        field.name.clone(),
                        " : self.",
                        field.name.clone(),
                    )
                }
                AstType::Byte(_)
                | AstType::Int(_)
                | AstType::Short(_)
                | AstType::Long(_)
                | AstType::Float(_)
                | AstType::Double(_) => {
                    let ty = SwiftMapping::map_base_transfer_type(&field.ty);
                    nested!(
                        method.body,
                        field.name.clone(),
                        " : ",
                        ty,
                        "(self.",
                        field.name.clone(),
                        ")"
                    )
                }
                AstType::Boolean => {
                    nested!(
                        method.body,
                        field.name.clone(),
                        " : ",
                        "self.",
                        field.name.clone(),
                        " ? 1 : 0"
                    )
                }
                AstType::String => {
                    nested!(
                        method.body,
                        field.name.clone(),
                        " : ",
                        "self.",
                        field.name.clone(),
                        ".withCString{ $0 }"
                    )
                }
                AstType::Vec(AstBaseType::Byte(_))
                | AstType::Vec(AstBaseType::Short(_))
                | AstType::Vec(AstBaseType::Int(_))
                | AstType::Vec(AstBaseType::Long(_)) => {
                    let transfer_ty = SwiftMapping::map_base_transfer_type(&field.ty);
                    let base_ty = match field.ty.clone() {
                        AstType::Vec(base) => {
                            SwiftMapping::map_base_transfer_type(&AstType::from(base))
                        }
                        _ => "".to_string(),
                    };
                    nested!(method.body, field.name.clone(), ": {");
                    nested!(method.body, |t| {
                        nested!(
                            t,
                            "let tmp_ptr = UnsafeMutablePointer<",
                            base_ty,
                            ">.allocate(capacity: self.",
                            field.name.clone(),
                            ".count)"
                        );
                        nested!(
                            t,
                            "tmp_ptr.initialize(from: self.",
                            field.name.clone(),
                            ", count: self.",
                            field.name.clone(),
                            ".count)"
                        );
                        nested!(
                            t,
                            "return ",
                            transfer_ty,
                            "(ptr: tmp_ptr, len: Int32(self.",
                            field.name.clone(),
                            ".count))"
                        );
                    });
                    push!(method.body, "}()")
                }
                AstType::Vec(_) => {
                    nested!(method.body, field.name.clone(), ": {");
                    nested!(method.body, |t| {
                        nested!(t, "return autoreleasepool { () -> UnsafePointer<Int8>? in");
                        nested!(t, |tt| {
                            nested!(tt, "let encoder = JSONEncoder()");
                            nested!(
                                tt,
                                "let data_result = try! encoder.encode(self.",
                                field.name.clone(),
                                ")"
                            );
                            nested!(
                                tt,
                                "let str_result = String(data: data_result, encoding: .utf8)"
                            );
                            nested!(tt, "return str_result?.withCString{$0}");
                        });
                        nested!(t, "}");
                    });
                    nested!(method.body, "}()");
                }
                AstType::Callback(_) => {}
                AstType::Struct(_) => {}
            }
            if index != self.desc.fields.len() - 1 {
                method.body.append(",")
            }
        }
        push!(method.body, ")");

        method
    }

    fn create_proxy_constructor(&self) -> Constructor {
        let mut constructor2 = Constructor::new();
        constructor2.modifiers = vec![Modifier::Internal];
        constructor2.arguments.push(Argument::new(
            swift::local(format!("Proxy{}", &self.desc.name)),
            "proxy",
        ));

        for field in self.desc.fields.iter() {
            match field.ty.clone() {
                AstType::Void => {
                    push!(
                        constructor2.body,
                        "self.",
                        field.name.clone(),
                        " = proxy.",
                        field.name.clone()
                    );
                }
                AstType::Byte(_)
                | AstType::Int(_)
                | AstType::Short(_)
                | AstType::Long(_)
                | AstType::Float(_)
                | AstType::Double(_) => {
                    let ty = SwiftMapping::map_swift_sig_type(&field.ty);
                    push!(
                        constructor2.body,
                        "self.",
                        field.name.clone(),
                        " = ",
                        ty,
                        "(proxy.",
                        field.name.clone(),
                        ")"
                    )
                }
                AstType::Boolean => {
                    push!(
                        constructor2.body,
                        "self.",
                        field.name.clone(),
                        " = proxy.",
                        field.name.clone(),
                        " > 0 ? true : false"
                    )
                }
                AstType::String => {
                    push!(
                        constructor2.body,
                        "self.",
                        field.name.clone(),
                        " = String(cString: proxy.",
                        field.name.clone(),
                        "!)"
                    )
                }
                AstType::Vec(AstBaseType::Byte(_))
                | AstType::Vec(AstBaseType::Short(_))
                | AstType::Vec(AstBaseType::Int(_))
                | AstType::Vec(AstBaseType::Long(_)) => {
                    let ty = SwiftMapping::map_swift_sig_type(&field.ty);
                    push!(
                        constructor2.body,
                        "self.",
                        field.name.clone(),
                        " = ",
                        ty,
                        "(UnsafeBufferPointer(start: proxy.",
                        field.name.clone(),
                        ".ptr, count: Int(proxy.",
                        field.name.clone(),
                        ".len)))"
                    );
                }
                AstType::Vec(_) => {
                    let ty = SwiftMapping::map_swift_sig_type(&field.ty);
                    push!(constructor2.body, |t| {
                        push!(t, "self.", field.name.clone(), " = {");
                        nested!(t, |tt| {
                            push!(
                                tt,
                                "let tmpStr = String(cString: proxy.",
                                field.name.clone(),
                                "!)"
                            );
                            push!(tt, "return autoreleasepool { () -> ", ty, " in");
                            nested!(tt, |ttt| {
                                push!(ttt, "let tmpJson = tmpStr.data(using: .utf8)!");
                                push!(ttt, "let decoder = JSONDecoder()");
                                push!(
                                    ttt,
                                    "return try! decoder.decode(",
                                    ty,
                                    ".self, from: tmpJson)"
                                )
                            });
                            push!(tt, "}");
                        });
                        push!(t, "}()");
                    });
                }
                AstType::Callback(_) => {}
                AstType::Struct(_) => {}
            }
        }

        constructor2
    }
}
