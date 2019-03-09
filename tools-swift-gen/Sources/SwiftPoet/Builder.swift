//
//  Builder.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/10/15.
//
//

import Foundation

public protocol Builder {
    associatedtype Result

    func build() -> Result
}
