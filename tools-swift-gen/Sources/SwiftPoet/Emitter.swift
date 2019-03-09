//
//  Emitter.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/10/15.
//
//

import Foundation

public protocol Emitter {
    @discardableResult
    func emit(to writer: CodeWriter) -> CodeWriter

    func toString() -> String
}
