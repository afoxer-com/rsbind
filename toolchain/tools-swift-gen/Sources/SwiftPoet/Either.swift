//
//  Either.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 1/26/16.
//
//

import Foundation

public enum Either<A, B> {
    case left(A)
    case right(B)
}
