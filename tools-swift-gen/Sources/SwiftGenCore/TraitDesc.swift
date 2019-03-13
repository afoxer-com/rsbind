//
//  File.swift
//  SwiftGenCore
//
//  Created by wangxin.sidney on 2018/6/22.
//

import Foundation
import SwiftPoet

enum AstBaseType: String {
    case VOID = "Void"
    case BYTE = "Int8"
    case INT = "Int"
    case LONG = "Long"
    case FLOAT = "Float"
    case DOUBLE = "Double"
    case STRING = "String"
    case BOOLEAN = "Boolean"
    case STRUCT = "Struct"
}

extension AstBaseType {
    static func fromStr(str: String) -> AstBaseType {
        switch str {
        case "Void":
            return AstBaseType.VOID
        case "Byte":
            return AstBaseType.BYTE
        case "Int":
            return AstBaseType.INT
        case "Long":
            return AstBaseType.LONG
        case "Float":
            return AstBaseType.FLOAT
        case "Double":
            return AstBaseType.DOUBLE
        case "String":
            return AstBaseType.STRING
        case "Boolean":
            return AstBaseType.BOOLEAN
        case "Struct":
            return AstBaseType.STRUCT
        default:
            return AstBaseType.VOID
        }
    }
}

enum AstType {
    case VOID
    case BYTE
    case INT
    case LONG
    case FLOAT
    case DOUBLE
    case STRING
    case BOOLEAN
    case VEC(AstBaseType)
    case CALLBACK(String)
    case STRUCT(String)
}

extension AstType {
    static func fromStr(ty: String, originTy: String) -> AstType {
        switch ty {
        case "Void":
            return AstType.VOID
        case "Byte":
            return AstType.BYTE
        case "Int":
            return AstType.INT
        case "Long":
            return AstType.LONG
        case "Float":
            return AstType.FLOAT
        case "Double":
            return AstType.DOUBLE
        case "String":
            return AstType.STRING
        case "Boolean":
            return AstType.BOOLEAN
        case "Struct":
            return AstType.STRUCT(originTy)
        case "Callback":
            return AstType.CALLBACK(originTy)
        default:
            return AstType.VOID
        }
    }
    
    func toStr() -> String {
        switch self {
        case AstType.VOID:
            return "Void"
        case AstType.BYTE:
            return "Byte"
        case AstType.INT:
            return "Int"
        case AstType.LONG:
            return "Long"
        case AstType.FLOAT:
            return "Float"
        case AstType.DOUBLE:
            return "Double"
        case AstType.STRING:
            return "String"
        case AstType.BOOLEAN:
            return "Boolean"
        case AstType.VEC(let baseType):
            return "[\(baseType.rawValue)]"
        case AstType.CALLBACK(let str):
            return "\(str)"
        case AstType.STRUCT(let str):
            return "\(str)"
        }
    }
    
    func toTypeName() -> TypeName {
        switch self {
            case .BOOLEAN:
                return TypeName.BooleanType
            case .BYTE:
                return TypeName.init(keyword: "Int8")
            case .INT:
                return TypeName.IntegerType
            case .LONG:
                return TypeName.LongType
            case .FLOAT:
                return TypeName.DoubleType
            case .DOUBLE:
                return TypeName.DoubleType
            case .STRING:
                return TypeName.StringType
            case .VOID:
                return TypeName.AnyType
            case .VEC(_):
                return TypeName.init(keyword: self.toStr())
            case .CALLBACK(let str):
                return TypeName.init(keyword: str)
            case .STRUCT(let str):
                return TypeName.init(keyword: str)
        }
    }
}

struct ArgDesc {
    let name: String
    let ty: AstType
    let origin_ty: String
}

struct MethodDesc {
    let name: String
    let return_type: AstType
    let origin_return_ty: String
    let args: [ArgDesc]
}

struct TraitDesc {
    let name: String
    let mod_name: String
    let crate_name: String
    let is_callback: Bool
    let methods: [MethodDesc]
}

struct StructDesc {
    let name: String
    let mod_name: String
    let crate_name: String
    let fields:[ArgDesc]
}

struct AstResult {
    let traits: [TraitDesc]
    let structs: [StructDesc]
}
