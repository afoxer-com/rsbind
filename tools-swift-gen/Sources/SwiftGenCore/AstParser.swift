//
//  AstParser.swift
//  SwiftGenCore
//
//  Created by wangxin.sidney on 2018/6/22.
//

import Foundation

class AstParser {
    let astPath: String
    
    init(path: String) {
        self.astPath = path
    }
    
    public func parseAst() -> AstResult {
        let dic = loadAstJson()
        var traits: [TraitDesc] = []
        var structs: [StructDesc] = []
        
        let ty = dic["ty"] as! String
        if ty == "struct" {
            let struct_ = parseStruct(dic: dic)
            structs.append(struct_)
        } else if ty == "trait" {
            let trait = parseTrait(dic: dic)
            traits.append(trait)
        }
        
        return AstResult(traits: traits, structs: structs)
    }
    
    private func parseStruct(dic: [String:Any]) -> StructDesc {
        let structName = dic["name"] as! String
        let modName = dic["mod_name"] as! String
        let crateName = dic["crate_name"] as! String
        let fields = dic["fields"] as! [Any]
        let args = parseArgs(args: fields)
        
        return StructDesc(name: structName, mod_name: modName, crate_name:crateName,  fields: args)
    }
    
    private func parseTrait(dic: [String:Any]) -> TraitDesc {
        let traitName = dic["name"] as! String
        let modName = dic["mod_name"] as! String
        let crateName = dic["crate_name"] as! String
        let methods = dic["methods"] as! [Any]
        let isCallback = dic["is_callback"] as! Bool
        
        var methodDescs: [MethodDesc] = []
        for method in methods {
            let method = method as! [String: Any]
            
            let methodName = method["name"] as! String
            let orginReturnType = method["origin_return_ty"] as! String
            let args = method["args"] as! [Any]
            
            var returnType = AstType.VOID
            if let returnTy = method["return_type"] as? String {
                returnType = AstType.fromStr(ty: returnTy, originTy: orginReturnType)
            } else {
                if let vecType = method["return_type"] as? [String: String] {
                    let baseType = vecType["Vec"]!
                    returnType = AstType.VEC(AstBaseType.fromStr(str: baseType))
                }
            }
            
            var argDescs:[ArgDesc] = parseArgs(args: args)
            let methodDesc = MethodDesc(name: methodName, return_type: returnType, origin_return_ty: orginReturnType, args: argDescs)
            methodDescs.append(methodDesc)
        }
        let traitDesc = TraitDesc(name: traitName, mod_name: modName, crate_name:crateName, is_callback: isCallback,  methods: methodDescs)
        return traitDesc
    }
    
    private func parseArgs(args: [Any]) -> [ArgDesc] {
        var argDescs:[ArgDesc] = []
        for arg in args {
            let arg = arg as! [String: Any]
            let argName = arg["name"] as! String
            let (argType, argOriginType) = parseType(arg: arg)
            let argDesc = ArgDesc(name: argName, ty: argType, origin_ty: argOriginType)
            argDescs.append(argDesc)
        }
        
        return argDescs
    }
    
    private func parseType(arg: [String: Any]) -> (AstType, String) {
        let argOriginType = arg["origin_ty"] as! String
        var argType = AstType.VOID
        if let argTy = arg["ty"] as? String  {
            argType = AstType.fromStr(ty: argTy, originTy: argOriginType)
        } else {
            if let vecType = arg["ty"] as? [String: String] {
                let baseType = vecType["Vec"]!
                argType = AstType.VEC(AstBaseType.fromStr(str: baseType))
            }
        }
        
        return (argType, argOriginType)
    }
    
    private func loadAstJson() -> [String:Any] {
        print("begin load ast json for \(self.astPath).")
        
        let fileManager = FileManager.default
        if let jsonData = fileManager.contents(atPath: self.astPath) {
            do {
                if let dic = try JSONSerialization.jsonObject(with: jsonData, options: JSONSerialization.ReadingOptions(rawValue: 0)) as? [String: Any] {
                    return dic
                } else {
                    print("parse ast json to empty!")
                    return [:]
                }
            } catch {
                print("parse ast json wrong!")
            }
        } else {
            print("obtain data from file \(self.astPath) error.")
        }
        
        return [:]
    }
}
