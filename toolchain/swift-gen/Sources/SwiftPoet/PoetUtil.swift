//
//  PoetUtil.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/12/15.
//
//

import Foundation

public struct PoetUtil {
    fileprivate static let template = "^^^^"
    fileprivate static let spacePunctuationRegexPattern = "\\s|_|-|\\[|\\]"
    fileprivate static let spaceDotsPunctuationRegexPattern = "\\s|_|\\.|-|\\[|\\]"

    fileprivate static var spaceAndPunctuationRegex: NSRegularExpression? {
        do {
            return try NSRegularExpression(pattern: PoetUtil.spacePunctuationRegexPattern, options: .anchorsMatchLines)
        } catch {
            return nil
        }
    }

    fileprivate static var spaceDotsAndPunctuationRegex: NSRegularExpression? {
        do {
            return try NSRegularExpression(pattern: PoetUtil.spaceDotsPunctuationRegexPattern, options: .anchorsMatchLines)
        } catch {
            return nil
        }
    }

    internal static func addUnique<T: Equatable>(_ data: T, to list: inout [T]) {
        if !list.contains(data) {
            list.append(data)
        }
    }

    internal static func stripSpaceAndPunctuation<T: StringProtocol>(_ name: T, escapeDots: Bool = true, escapeUppercase: Bool = false ) -> [String] {
        let nameStr = String(name)

        guard let regex = escapeDots ? spaceDotsAndPunctuationRegex : spaceAndPunctuationRegex else {
            return [nameStr]
        }

        return regex.stringByReplacingMatches(
            in: nameStr, options: [],
            range: NSMakeRange(0, nameStr.count), withTemplate: template)
            .components(separatedBy: template)
            .map { string in
                if escapeUppercase && string == string.uppercased() {
                    return capitalizeFirstChar(string.lowercased())
                }
                return capitalizeFirstChar(string)
            }
    }

    // capitalize first letter without removing cammel case on other characters
    internal static func capitalizeFirstChar(_ str: String) -> String {
        return caseFirstChar(str) {
            return $0.uppercased()
        }
    }

    // lowercase first letter without removing cammel case on other characters
    internal static func lowercaseFirstChar(_ str: String) -> String {
        return caseFirstChar(str) {
            return $0.lowercased()
        }
    }

    fileprivate static func caseFirstChar(_ str: String, caseFn: (_ str: String) -> String) -> String {
        guard !str.isEmpty else {
            return str // This does happen!
        }

        var chars = str
        let first = String(str[..<chars.index(after: chars.startIndex)])
        let range = chars.startIndex..<chars.index(after: chars.startIndex)
        chars.replaceSubrange(range, with: caseFn(first))
        return String(chars)
    }

    public static func fmap<A, B>(_ data: A?, function: (A) -> B?) -> B? {
        switch data {
        case .some(let x): return function(x)
        case .none: return .none
        }
    }

    public static func fmap<A, B>(_ data: A?, function: (A) -> B) -> B? {
        switch data {
        case .some(let x): return function(x)
        case .none: return .none
        }
    }
}
