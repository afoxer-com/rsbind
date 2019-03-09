//
//  ExtensionSpec.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 1/25/16.
//
//

import Foundation

open class ExtensionSpec: TypeSpec {
    open static let fieldModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private, .Static, .Final, .Override, .Required]
    open static let methodModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private, .Static, .Final, .Klass, .Throws, .Convenience, .Override, .Required]
    open static let asMemberModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private]

    fileprivate init(builder: ExtensionSpecBuilder) {
        super.init(builder: builder as TypeSpecBuilder)
    }

    open static func builder(for name: String) -> ExtensionSpecBuilder {
        return ExtensionSpecBuilder(name: name)
    }
}

open class ExtensionSpecBuilder: TypeSpecBuilder, Builder {
    public typealias Result = ExtensionSpec
    open static let defaultConstruct: Construct = .extension

    public init(name: String) {
        super.init(name: name, construct: ExtensionSpecBuilder.defaultConstruct)
    }

    open func build() -> Result {
        return ExtensionSpec(builder: self)
    }
}

// MARK: Chaining
extension ExtensionSpecBuilder {

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
        guard ExtensionSpec.asMemberModifiers.contains(toAdd) else {
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
