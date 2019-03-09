//
//  CallbackGenerator.swift
//  SwiftGen
//
//  Created by wangxin.sidney on 2019/2/5.
//

import Foundation
import SwiftPoet
class CallbackGenerator {
    private let traitDesc: TraitDesc
    private let libModName: String
    
    init(desc: TraitDesc, libModName: String) {
        self.traitDesc = desc
        self.libModName = libModName
    }
    
    public func generate() -> ProtocolSpecBuilder {
        let classBuilder = ProtocolSpec.builder(for: "\(self.traitDesc.name)")
            .add(import: self.libModName)
            .add(modifier: .Public)
        
        for method in self.traitDesc.methods {
            let methodSpec = quoteMethodSig(method: method)
            classBuilder.add(method: methodSpec.build())
        }

        return classBuilder
    }
    
    func quoteMethodSig(method: MethodDesc) -> MethodSpecBuilder {
        let methodSpec = MethodSpec.builder(for: method.name)
            .add(returnType: method.return_type.toTypeName())
        
        method.args.forEach({ arg in
            switch arg.ty {
            case .VEC(let base):
                if base == AstBaseType.STRUCT {
                    let structName = arg.origin_ty.replacingOccurrences(of: "Vec<", with: "").replacingOccurrences(of: ">", with: "")
                    let argSpec = ParameterSpec.builder(for: arg.name, type: TypeName.init(keyword: "[\(structName)]")).build()
                    methodSpec.add(parameter: argSpec)
                } else {
                    let argSpec = ParameterSpec.builder(for: arg.name, type: arg.ty.toTypeName()).build()
                    methodSpec.add(parameter: argSpec)
                }
            default:
                let argSpec = ParameterSpec.builder(for: arg.name, type: arg.ty.toTypeName()).build()
                methodSpec.add(parameter: argSpec)
            }
        })
        
        return methodSpec
    }
}
