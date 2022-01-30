//
//  StringExtension.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 1/29/16.
//
//

import Foundation

extension String {
    public func toCodeBlock() -> CodeBlock {
        return CodeBlock.builder().add(literal: self).build()
    }
}

extension String
{
    public enum Case {
        case typeName

        /*  Swift allows dot notation when referencing types, e.g. `let value: Type.NestedType`
            but you have to make sure escape any of the combined type from dots by yourself */
        case typeReferenceName

        case camelCaseName          // Variables, method declarations, arguments
        case unescapedCamelCaseName // Can be used for unescaped param names in methods
        case uppercasedName
    }
}

extension StringProtocol {
    public func cleaned(_ stringCase: String.Case) -> String {
        return self as! String;
//        switch stringCase {
//        case .typeName:
//            return ReservedWords.safeWord(PoetUtil.stripSpaceAndPunctuation(self).joined(separator: ""))
//        case .typeReferenceName:
//            return ReservedWords.safeWord(PoetUtil.stripSpaceAndPunctuation(self, escapeDots: false).joined(separator: ""))
//        case .camelCaseName, .unescapedCamelCaseName:
//            let cleanedNameChars = PoetUtil.stripSpaceAndPunctuation(self, escapeUppercase: true).joined(separator: "")
//            if cleanedNameChars == cleanedNameChars.uppercased() {
//                return cleanedNameChars.lowercased()
//            }
//            if case .unescapedCamelCaseName = stringCase {
//                return PoetUtil.lowercaseFirstChar(cleanedNameChars)
//            }
//            return ReservedWords.safeWord(PoetUtil.lowercaseFirstChar(cleanedNameChars))
//        case .uppercasedName:
//            let cleanedNameChars = PoetUtil.stripSpaceAndPunctuation(self).joined(separator: "")
//            return ReservedWords.safeWord(cleanedNameChars.uppercased())
//        }
    }
}
