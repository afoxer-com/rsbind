//
//  File.swift
//  SwiftGenCore
//
//  Created by wangxin.sidney on 2019/2/17.
//

import Foundation
import SwiftPoet

class StructGenerator {
    private let structDesc: StructDesc
    private let libModName: String
    
    init(desc: StructDesc, libModName: String) {
        self.structDesc = desc
        self.libModName = libModName
    }
    
    public func generate() -> StructSpecBuilder {
        let classBuilder = StructSpecBuilder.init(name: "\(self.structDesc.name)")
//             .add(import: self.libModName)
            .add(modifier: .Public)
            .add(superType: TypeName.init(keyword: "Codable"))
        
        for field in self.structDesc.fields {
            classBuilder.add(field: FieldSpec.builder(for: field.name, type: field.ty.toTypeName()).add(modifier: Modifier.Public).build())
        }
        
        return classBuilder
    }
}
