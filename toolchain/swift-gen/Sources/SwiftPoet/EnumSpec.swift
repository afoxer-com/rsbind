//
//  EnumSpec.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/10/15.
//
//

import Foundation

open class EnumSpec: TypeSpec {
    open static let fieldModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private, .Static]
    open static let methodModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private, .Static, .Throws]
    open static let asMemberModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private]

    fileprivate init(builder: EnumSpecBuilder) {
        super.init(builder: builder as TypeSpecBuilder)
    }

    open static func builder(for name: String) -> EnumSpecBuilder {
        return EnumSpecBuilder(name: name)
    }
}

open class EnumSpecBuilder: TypeSpecBuilder, Builder {
    public typealias Result = EnumSpec
    open static let defaultConstruct: Construct = .enum

    fileprivate init(name: String) {
        super.init(name: name, construct: EnumSpecBuilder.defaultConstruct)
    }

    open func build() -> Result {
        return EnumSpec(builder: self)
    }
}

// MARK: Chaining
extension EnumSpecBuilder {

    @discardableResult
    public func add(method toAdd: MethodSpec) -> Self {
        mutatingAdd(method: toAdd)
        return self
    }

    @discardableResult
    public func add(methods toAdd: [MethodSpec]) -> Self {
        toAdd.forEach { mutatingAdd(method: $0) }
        return self
    }

    @discardableResult
    public func add(field toAdd: FieldSpec) -> Self {
        mutatingAdd(field: toAdd)
        return self
    }

    @discardableResult
    public func add(fields toAdd: [FieldSpec]) -> Self {
        toAdd.forEach { mutatingAdd(field: $0) }
        return self
    }

    @discardableResult
    public func add(protocol toAdd: TypeName) -> Self {
        mutatingAdd(protocol: toAdd)
        return self
    }

    @discardableResult
    public func add(protocols toAdd: [TypeName]) -> Self {
        mutatingAdd(protocols: toAdd)
        return self
    }

    @discardableResult
    public func add(superType toAdd: TypeName) -> Self {
        mutatingAdd(superType: toAdd)
        return self
    }

    @discardableResult
    public func add(modifier toAdd: Modifier) -> Self {
        guard EnumSpec.asMemberModifiers.contains(toAdd) else {
            return self
        }
        mutatingAdd(modifier: toAdd)
        return self
    }

    @discardableResult
    public func add(modifiers toAdd: [Modifier]) -> Self {
        toAdd.forEach { _ = add(modifier: $0) }
        return self
    }

    @discardableResult
    public func add(description toAdd: String?) -> Self {
        mutatingAdd(description: toAdd)
        return self
    }

    @discardableResult
    public func add(generatorInfo toAdd: String?) -> Self {
        mutatingAdd(generatorInfo: toAdd)
        return self
    }

    @discardableResult
    public func add(framework toAdd: String?) -> Self {
        mutatingAdd(framework: toAdd)
        return self
    }

    @discardableResult
    public func add(import toAdd: String) -> Self {
        mutatingAdd(import: toAdd)
        return self
    }

    @discardableResult
    public func add(imports toAdd: [String]) -> Self {
        mutatingAdd(imports: toAdd)
        return self
    }

    @discardableResult
    public func add(nestedType toAdd: TypeSpec) -> Self {
        mutatingAdd(nestedType: toAdd)
        return self
    }
}
