//
//  Construct.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/11/15.
//
//

import Foundation

public enum Construct: Int {
    case param = 0
    case mutableParam
    case field
    case mutableField
    case method
    case `enum`
    case `struct`
    case `class`
    case `protocol`
    case typeAlias
    case `extension`

    public var stringValue: String {
        switch self {
        case .param: return ""
        case .mutableParam: return "var"
        case .field: return "let"
        case .mutableField: return "var"
        case .method: return "func"
        case .enum: return "enum"
        case .struct: return "struct"
        case .class: return "class"
        case .protocol: return "protocol"
        case .typeAlias: return "typealias"
        case .extension: return "extension"
        }
    }
}

extension Construct: Literal {
    public func literalValue() -> String {
        return self.stringValue
    }
}
