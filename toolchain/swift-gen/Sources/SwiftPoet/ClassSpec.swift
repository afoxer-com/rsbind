//
//  ClassSpec.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/11/15.
//
//

import Foundation

public class ClassSpec: TypeSpec {
    public static let fieldModifiers: [Modifier] = [.Open, .Public, .Internal, .Fileprivate, .Private, .Static, .Final, .Klass, .Override, .Required]
    public static let methodModifiers: [Modifier] = [.Open, .Public, .Internal, .Fileprivate, .Private, .Static, .Final, .Klass, .Throws, .Convenience, .Override, .Required]
    public static let asMemberModifiers: [Modifier] = [.Open, .Public, .Internal, .Fileprivate, .Private]

    fileprivate init(builder: ClassSpecBuilder) {
        super.init(builder: builder as TypeSpecBuilder)
    }

    public static func builder(for name: String) -> ClassSpecBuilder {
        return ClassSpecBuilder(name: name)
    }
}

public class ClassSpecBuilder: TypeSpecBuilder, Builder {
    public typealias Result = ClassSpec
    public static let defaultConstruct: Construct = .class

    public init(name: String) {
        super.init(name: name, construct: ClassSpecBuilder.defaultConstruct)
    }

    public func build() -> Result {
        return ClassSpec(builder: self)
    }
}

// MARK: Chaining
extension ClassSpecBuilder {

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
        guard ClassSpec.asMemberModifiers.contains(toAdd) else {
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
