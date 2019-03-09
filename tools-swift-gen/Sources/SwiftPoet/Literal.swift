//
//  Literal.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/12/15.
//
//

import Foundation

public protocol Literal {
    func literalValue() -> String
}


extension String: Literal {
    public func literalValue() -> String {
        return self
    }
}
