//
//  CodeBlock.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/10/15.
//
//

import Foundation

public struct CodeBlock
{
    private let codeBlockBuilder: CodeBlockBuilder

    public var emittableObjects: [Either<EmitObject, CodeBlock>] {
        return codeBlockBuilder.emittableObjects
    }

    public var isEmpty: Bool {
        return emittableObjects.isEmpty
    }

    fileprivate init(builder: CodeBlockBuilder) {
        self.codeBlockBuilder = builder
    }

    public func toString() -> String {
        let codeWriter = CodeWriter()
        return codeWriter.emit(codeBlock: self).out
    }
    
    @discardableResult
    public func add(codeBlock toAdd: CodeBlock) -> CodeBlock {
        codeBlockBuilder.add(codeBlock: toAdd)
        return self
    }

    @discardableResult
    public func add(type: EmitType, data: Any? = nil) -> CodeBlock {
        codeBlockBuilder.add(object: EmitObject(type: type, data: data))
        return self
    }

    @discardableResult
    public func add(literal toAdd: Literal) -> CodeBlock {
        codeBlockBuilder.add(literal: toAdd)
        return self
    }
    
    @discardableResult
    public func add(codeLine toAdd: Literal) -> CodeBlock {
        codeBlockBuilder.add(type: .codeLine, data: toAdd)
        return self
    }
    
    @discardableResult
    public func add(objects toAdd: [Either<EmitObject, CodeBlock>]) -> CodeBlock {
        codeBlockBuilder.emittableObjects.append(contentsOf: toAdd)
        return self
    }

    public static func builder() -> CodeBlockBuilder {
        return CodeBlockBuilder()
    }
}

extension CodeBlock: Equatable {}

public func ==(lhs: CodeBlock, rhs: CodeBlock) -> Bool {
    return lhs.toString() == rhs.toString()
}

extension CodeBlock: Hashable {
    public var hashValue: Int {
        return toString().hashValue
    }
}


open class CodeBlockBuilder: Builder {
    public typealias Result = CodeBlock

    open fileprivate(set) var emittableObjects = [Either<EmitObject, CodeBlock>]()

    fileprivate init () {}

    open func build()
        -> CodeBlock
    {
        return CodeBlock(builder: self)
    }

    @discardableResult
    internal func add(object toAdd: EmitObject)
        -> CodeBlockBuilder
    {
        emittableObjects.append(Either.left(toAdd))
        return self
    }

    @discardableResult
    open func add(type: EmitType, data: Any? = nil, trimString: Bool = false)
        -> CodeBlockBuilder
    {
        return self.add(object: EmitObject(type: type, data: data, trimString: trimString))
    }

    @discardableResult
    open func add(literal toAdd: Literal, trimString: Bool = false)
        -> CodeBlockBuilder
    {
        return add(type: .literal, data: toAdd, trimString: trimString)
    }

    @discardableResult
    open func add(codeLine toAdd: Literal)
        -> CodeBlockBuilder
    {
        return add(type: .codeLine, data: toAdd)
    }

    @discardableResult
    open func add(objects toAdd: [Either<EmitObject, CodeBlock>])
        -> CodeBlockBuilder
    {
        emittableObjects.append(contentsOf: toAdd)
        return self
    }

    @discardableResult
    open func add(codeBlock toAdd: CodeBlock)
        -> CodeBlockBuilder
    {
        emittableObjects.append(Either.right(toAdd))
        return self
    }
}

