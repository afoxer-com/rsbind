//
//  TraitGenerator.swift
//  SwiftGenCore
//
//  Created by wangxin.sidney on 2018/6/22.
//

import Foundation
import SwiftPoet

class TraitGenerator {
    private let traitDesc: TraitDesc
    private let libModName: String
    
    init(desc: TraitDesc, libModName: String) {
        self.traitDesc = desc
        self.libModName = libModName
    }
    
    public func generate(callbacks: [String: TraitDesc]) -> ClassSpecBuilder {
        print("generate swift begin for \(callbacks)")
        
        let classBuilder = ClassSpec.builder(for: "\(self.traitDesc.name)")
            .add(import: self.libModName)
            .add(modifier: .Public);
        
        for method in self.traitDesc.methods {
            print("generate swift codes for \(method.name)")

            let methodSpec = quoteMethodSig(method: method)

            let codeBlockBuilder = CodeBlock.builder()
            let _ = quoteArgConvert(builder: codeBlockBuilder, methodDesc: method, callbacks: callbacks)
            print("66666")

            let impMethodName = "\(self.traitDesc.mod_name)_\(method.name)"
            print("444")

            var argCalls = ""
            var index = 0
            for arg in method.args {
                if index != method.args.count - 1 {
                    argCalls += "s_\(arg.name),"
                } else {
                    argCalls += "s_\(arg.name)"
                }
                index = index + 1
            }
            print("999999")

            codeBlockBuilder
                .add(codeLine: "let result = \(impMethodName)(\(argCalls))")
            
            let _ = quoteResultConvert(builder: codeBlockBuilder, methodDesc: method)
            print("33333333eeeee")

            methodSpec.add(codeBlock: codeBlockBuilder.build())
            classBuilder.add(method: methodSpec.build())
            print("74534536")

        }
        
        return classBuilder;
    }
    
    func quoteMethodSig(method: MethodDesc) -> MethodSpecBuilder {
        let methodSpec = MethodSpec.builder(for: method.name)
            .add(modifier: .Public)
            .add(modifier: .Static)
        
        var return_type = method.return_type.toTypeName()
        switch method.return_type {
            case AstType.VEC(let base):
                if base == AstBaseType.STRUCT {
                    let return_ty_str = method.origin_return_ty.replacingOccurrences(of: "Vec", with: "Array")
                    return_type = TypeName.init(keyword: return_ty_str)
                }
            default:
                {}()
        }
            
        methodSpec.add(returnType: return_type)
        
        method.args.forEach({ arg in
            let argSpec = ParameterSpec.builder(for: arg.name, type: arg.ty.toTypeName()).build()
            methodSpec.add(parameter: argSpec)
        })
        
        return methodSpec
    }
    
    func quoteArgConvert(builder: CodeBlockBuilder, methodDesc: MethodDesc, callbacks: [String: TraitDesc]) -> CodeBlockBuilder {
        let crateName = self.traitDesc.crate_name.replacingOccurrences(of: "-", with: "_")
    
        print("quoteArgConvert start\(strlen("123456"))")
        for arg in methodDesc.args {
            print("quoteArgConvert for \(arg.name)")
            switch arg.ty {
                case AstType.BOOLEAN:
                     builder.add(codeLine: "let s_\(arg.name): Int32 = \(arg.name) ? 1 : 0")
                case AstType.INT:
                    builder.add(codeLine: "let s_\(arg.name) = Int32(\(arg.name))")
                case AstType.LONG:
                    builder.add(codeLine: "let s_\(arg.name) = Int64(\(arg.name))")
                case AstType.FLOAT:
                    builder.add(codeLine: "let s_\(arg.name) = Float32(\(arg.name))")
                case AstType.DOUBLE:
                    builder.add(codeLine: "let s_\(arg.name) = Float64(\(arg.name))")
                case AstType.STRING:
                    builder.add(codeLine: "let s_\(arg.name) = \(arg.name)")
                case AstType.CALLBACK(let str):
                    print("found callback \(str)")
                    builder.add(codeLine: "let \(arg.name)_index = globalIndex + 1")
                        .add(codeLine: "globalIndex = \(arg.name)_index")
                        .add(codeLine: "globalCallbacks[\(arg.name)_index] = \(arg.name)")
                
                    var modelArgs = ""
                    var methodIndex = 0
                    let callback = callbacks[str]
                    print("callbacks = \(callbacks), str = \(str)")

                    for method in callback!.methods {
                        var closue = ""
                        print("578457856785")

                        var index = 0
                        var arg_params = "(index, "
                        var args_str = "(Int64, "
                        for arg in method.args {
                            print("546435634573546")

                            let isLast = index == method.args.count - 1
                            let arg_type = mapCallbackType(type: arg.ty)
                            args_str = "\(args_str)\(arg_type)"
                            arg_params = "\(arg_params)\(arg.name)"
                            if !isLast {
                                args_str = "\(args_str), "
                                arg_params = "\(arg_params), "
                            }
                            index = index + 1
                            print("000000000")

                        }
                        
                        args_str = "\(args_str))"
                        arg_params = "\(arg_params))"
                        print("4434234533")

                        let return_type = mapCallbackType(type: method.return_type)
                        closue = "\(args_str) -> \(return_type)"
                        arg_params = "\(arg_params) -> \(return_type)"
                        print("jkfjfgjfjfj")

                        builder.add(codeLine: "let \(arg.name)_\(method.name) : @convention(c) \(closue) = { ")
                        let closureBuilder = CodeBlock.builder()
                        closureBuilder.add(codeLine: "\(arg_params) in")
                        closureBuilder.add(codeLine: "let \(arg.name)_callback = globalCallbacks[index] as! \(callback!.name)")
                        print("kdkgijjioejgeorg")

                        var method_call = "("
                        var i = 0
                        for arg in method.args {
                            print(".........")

                            switch arg.ty {
                                case AstType.BOOLEAN:
                                    closureBuilder.add(codeLine: "let c_\(arg.name): Bool = \(arg.name) > 0 ? true : false")
                                case AstType.INT:
                                    closureBuilder.add(codeLine: "let c_\(arg.name) = Int(\(arg.name))")
                                case AstType.LONG:
                                    closureBuilder.add(codeLine: "let c_\(arg.name) = Int64(\(arg.name))")
                                case AstType.FLOAT:
                                    closureBuilder.add(codeLine: "let c_\(arg.name) = Double(\(arg.name))")
                                case AstType.DOUBLE:
                                    closureBuilder.add(codeLine: "let c_\(arg.name) = Double(\(arg.name))")
                                case AstType.STRING:
                                    closureBuilder.add(codeLine: "let c_\(arg.name) = String(cString: \(arg.name)!)")
                                case AstType.STRUCT(_):
                                    closureBuilder.add(codeLine: "let c_tmp_\(arg.name) = String(cString:\(arg.name)!)")
                                        .add(codeLine: "let c_tmp_json_\(arg.name) = c_tmp_\(arg.name).data(using: .utf8)!")
                                        .add(codeLine: "let decoder = JSONDecoder()")
                                        .add(codeLine: "let c_\(arg.name) = try! decoder.decode(\(arg.ty.toStr()).self, from: c_tmp_json_\(arg.name))")
                                case AstType.VEC(let base):
                                    var vecType = arg.ty.toStr()
                                    if base == AstBaseType.STRUCT {
                                        vecType = arg.origin_ty.replacingOccurrences(of: "Vec", with: "Array")
                                    }
                                    closureBuilder.add(codeLine: "let c_tmp_\(arg.name) = String(cString:\(arg.name)!)")
                                        .add(codeLine: "let c_tmp_json_\(arg.name) = c_tmp_\(arg.name).data(using: .utf8)!")
                                        .add(codeLine: "let decoder = JSONDecoder()")
                                        .add(codeLine: "let c_\(arg.name) = try! decoder.decode(\(vecType).self, from: c_tmp_json_\(arg.name))")
                                default:
                                    print("don't support \(arg.origin_ty) in callback")
                                    assert(false)
                            }
                            print("..qwefqwer.......")

                            method_call = "\(method_call)\(arg.name):c_\(arg.name)"
                            if i != method.args.count - 1 {
                                method_call = "\(method_call),"
                            }
                            i = i + 1
                            print("..qwefqwer.......")
                        }
                        method_call = "\(method_call))"
                        
                        closureBuilder.add(codeLine: "let result = \(arg.name)_callback.\(method.name)\(method_call)")
                        print("..qertqretgewr.......")

                        switch method.return_type {
                            case AstType.BOOLEAN:
                                closureBuilder.add(codeLine: "return result ? 1 : 0")
                            case AstType.INT:
                                closureBuilder.add(codeLine: "return Int32(result)")
                            case AstType.LONG:
                                closureBuilder.add(codeLine: "return Int64(result)")
                            case AstType.FLOAT:
                                closureBuilder.add(codeLine: "return Float32(result)")
                            case AstType.DOUBLE:
                                closureBuilder.add(codeLine: "return Float64(result)")
                            case AstType.STRING:
                                closureBuilder.add(codeLine: "return result")
                            default:
                                print("wrong type in callback: \(method.return_type)")
                                assert(false)
                        }
                        
                        builder.add(codeBlock: closureBuilder.build())
                        builder.add(codeLine: "}")
                        print("..erwehgert.......")

                        modelArgs = "\(modelArgs)\(method.name):\(arg.name)_\(method.name),"
                        methodIndex = methodIndex + 1
                    }
                
                    builder.add(codeLine: "let callback_free : @convention(c)(Int64) -> () = {")
                    builder.add(codeLine: "(index) in")
                    builder.add(codeLine: "globalCallbacks.removeValue(forKey: index)")
                    builder.add(codeLine: "}")
                    print("..234523452.......")

                
                    builder.add(codeLine: "let s_\(arg.name) = \(traitDesc.mod_name)_\(callback!.name)_Model(\(modelArgs)free_callback: callback_free, index: \(arg.name)_index)")
                    print("srwegr44444")

                case AstType.VEC(_):
                    print("535353455dfff")

                    builder.add(codeLine: "let encoder = JSONEncoder()")
                    builder.add(codeLine: "let data_\(arg.name) = try! encoder.encode(\(arg.name))")
                    builder.add(codeLine: "let s_\(arg.name) = String(data: data_\(arg.name), encoding: .utf8)!")
                    print("435346345634gggh")

                case AstType.VOID:
                    {}()
                case AstType.STRUCT(_):
                    {}()
            }
        }
        
        return builder
    }
    
    func mapCallbackType(type: AstType) -> String {
        switch type {
        case AstType.BOOLEAN:
            return "Int32"
        case AstType.INT:
            return "Int32"
        case AstType.LONG:
            return "Int64"
        case AstType.FLOAT:
            return "Float32"
        case AstType.DOUBLE:
            return "Float64"
        case AstType.STRING:
            return "UnsafePointer<Int8>?"
        case AstType.VEC(_), AstType.STRUCT(_):
            return "UnsafePointer<Int8>?"
        default:
            print("don't support \(type) in callback")
            assert(false)
            return ""
        }
    }
    
    func quoteResultConvert(builder: CodeBlockBuilder, methodDesc: MethodDesc) -> CodeBlockBuilder {
        let crateName = self.traitDesc.crate_name.replacingOccurrences(of: "-", with: "_")
        switch methodDesc.return_type {
            case AstType.BOOLEAN:
                builder.add(codeLine: "let s_result = result > 0 ? true : false")
            case AstType.INT:
                builder.add(codeLine: "let s_result = Int(result)")
            case AstType.LONG:
                builder.add(codeLine: "let s_result = Long(result)")
            case AstType.FLOAT:
                builder.add(codeLine: "let s_result = Double(result)")
            case AstType.DOUBLE:
                builder.add(codeLine: "let s_result = Double(result)")
            case AstType.STRING:
                builder.add(codeLine: "let s_result = String(cString:result!)")
                    .add(codeLine: "\(crateName)_free_str(result!)")
            case AstType.VEC(let base):
                var vecType = methodDesc.return_type.toStr()
                if base == AstBaseType.STRUCT {
                    vecType = methodDesc.origin_return_ty.replacingOccurrences(of: "Vec", with: "Array")
                }
                builder.add(codeLine: "let ret_str = String(cString:result!)")
                    .add(codeLine: "let ret_str_json = ret_str.data(using: .utf8)!")
                    .add(codeLine: "let decoder = JSONDecoder()")
                    .add(codeLine: "let s_result = try! decoder.decode(\(vecType).self, from: ret_str_json)")
                    .add(codeLine: "\(crateName)_free_str(result!)")
            case AstType.STRUCT(_):
                builder.add(codeLine: "let ret_str = String(cString:result!)")
                    .add(codeLine: "let ret_str_json = ret_str.data(using: .utf8)!")
                    .add(codeLine: "let decoder = JSONDecoder()")
                    .add(codeLine: "let s_result = try! decoder.decode(\(methodDesc.return_type.toStr()).self, from: ret_str_json)")
                    .add(codeLine: "\(crateName)_free_str(result!)")
            case AstType.VOID:
                {}()
            case AstType.CALLBACK(_):
                {}()
        }

        builder.add(codeLine: "return s_result")
        return builder
    }
}
