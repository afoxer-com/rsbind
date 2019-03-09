//
//  EmitType.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/12/15.
//
//

import Foundation

public enum EmitType {
    case literal
    case increaseIndentation
    case decreaseIndentation
    case beginStatement
    case endStatement
    case newLine    // uses a simple newline character ("\n") to advance to next line at leftmost column
    case nextLine   // advances to next line at same indentation as the current line
    case codeLine
    case emitter
}
