//
//  ReservedWords.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/10/15.
//
//

import Foundation

public struct ReservedWords {
    fileprivate static let all: Set = [
        "class", "deinit", "enum", "extension", "func", "import", "init",
        "inout", "internal", "let", "operator", "private", "protocol", "public",
        "static", "struct", "subscript", "typealias", "var", "break", "case",
        "continue", "default", "defer", "do", "else", "fallthrough", "for",
        "guard", "if", "in", "repeat", "return", "switch", "where", "while",
        "as", "catch", "dynamicType", "false", "is", "nil", "rethrows", "super",
        "self", "Self", "throw", "throws", "true", "try", "__COLUMN__",
        "__FILE__", "__FUNCTION__", "__LINE__", "_", "associativity",
        "convenience", "dynamic", "didSet", "final", "get", "infix", "indirect",
        "lazy", "left", "mutating", "none", "nonmutating", "optional", "override",
        "postfix", "precedence", "prefix", "protocol", "required",
        "right", "set", "Type", "unowned", "weak", "willSet"
    ]

    public static func contains(_ word: String) -> Bool {
        return ReservedWords.all.contains(word)
    }

    public static func safeWord(_ word: String) -> String {
        guard ReservedWords.contains(word) == false else {
            return "`\(word)`"
        }
        return word
    }
}
