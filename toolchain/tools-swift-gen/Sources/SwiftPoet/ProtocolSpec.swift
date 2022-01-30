//
//  ProtocolSpec.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/11/15.
//
//

import Foundation

open class ProtocolSpec: TypeSpec {
    open static let fieldModifiers: [Modifier] = [.Static]
    open static let methodModifiers: [Modifier] = [.Static]
    open static let asMemberModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private]

    fileprivate init(builder: ProtocolSpecBuilder) {
        super.init(builder: builder as TypeSpecBuilder)
    }

    open static func builder(for name: String) -> ProtocolSpecBuilder {
        return ProtocolSpecBuilder(name: name)
    }
}

open class ProtocolSpecBuilder: TypeSpecBuilder, Builder {
    public typealias Result = ProtocolSpec
    open static let defaultConstruct: Construct = .protocol

    public init(name: String) {
        super.init(name: name, construct: ProtocolSpecBuilder.defaultConstruct)
    }

    open func build() -> Result {
        return ProtocolSpec(builder: self)
    }
}

// MARK: Chaining
extension ProtocolSpecBuilder {

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
    public func add(_ superType: TypeName) -> Self {
        mutatingAdd(superType: superType)
        return self
    }

    @discardableResult
    public func add(modifier toAdd: Modifier) -> Self {
        guard ProtocolSpec.asMemberModifiers.contains(toAdd) else {
            return self
        }
        mutatingAdd(modifier: toAdd)
        return self
    }

    @discardableResult
    public func add(_ modifiers: [Modifier]) -> Self {
        modifiers.forEach { _ = add(modifier: $0) }
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
        mutatingAdd(framework: framework)
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
}
