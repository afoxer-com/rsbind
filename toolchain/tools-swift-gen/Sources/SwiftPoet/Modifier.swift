//
//  Modifier.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/10/15.
//
//

import Foundation

open class Modifier: NSObject {

    open let rawString: String

    public init(rawString: String) {
        self.rawString = rawString
    }

    open static let Open = Modifier(rawString: "open")
    open static let Public = Modifier(rawString: "public")
    open static let Internal = Modifier(rawString: "internal")
    open static let Fileprivate = Modifier(rawString: "fileprivate")
    open static let Private = Modifier(rawString: "private")

    open static let Static = Modifier(rawString: "static")
    open static let Final = Modifier(rawString: "final")
    open static let Klass = Modifier(rawString: "class")

    open static let Mutating = Modifier(rawString: "mutating")
    open static let Throws = Modifier(rawString: "throws")
    open static let Convenience = Modifier(rawString: "convenience")
    open static let Override = Modifier(rawString: "override")
    open static let Required = Modifier(rawString: "required")

    open override var hash: Int {
        return rawString.hashValue
    }

    //    case DidSet
    //    case Lazy
    //    case WillSet
    //    case Weak
    //    case Optional

    open static func equivalentAccessLevel(parentModifiers pm: Set<Modifier>, childModifiers cm: Set<Modifier>)
        -> Bool
    {
        let parentAccessLevel = Modifier.accessLevel(pm)
        let childAccessLevel = Modifier.accessLevel(cm)

        // TODO: fix this algorithm; checking childAccessLevel should be <= not !=
        // therefore, we should split out access modifiers to its own type, and
        // implement comparison operations
        if parentAccessLevel == .Private {
            return true
        }
        else if parentAccessLevel == .Fileprivate && childAccessLevel != .Private {
            return true
        }
        else if parentAccessLevel == .Internal && childAccessLevel != .Fileprivate {
            return true
        }
        else if parentAccessLevel == .Public && childAccessLevel != .Internal {
            return true
        }
        else if parentAccessLevel == .Open && childAccessLevel == .Open {
            return true
        }
        return false
    }

    open static func accessLevel(_ modifiers: Set<Modifier>)
        -> Modifier
    {
        if modifiers.contains(.Private) {
            return .Private
        }
        else if modifiers.contains(.Fileprivate) {
            return .Fileprivate
        }
        else if modifiers.contains(.Public) {
            return .Public
        }
        else if modifiers.contains(.Open) {
            return .Open
        }
        else {
            return .Internal
        }
    }
}

public func ==(lhs: Modifier, rhs: Modifier) -> Bool {
    return lhs.rawString == rhs.rawString
}
