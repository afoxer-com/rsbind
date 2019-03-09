//
//  StructSpec.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/11/15.
//
//

import Foundation

open class StructSpec: TypeSpec {
    open static let fieldModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private, .Static]
    open static let methodModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private, .Static, .Mutating, .Throws]
    open static let asMemberModifiers: [Modifier] = [.Public, .Internal, .Fileprivate, .Private]

    fileprivate init(builder: StructSpecBuilder) {
        super.init(builder: builder as TypeSpecBuilder)
    }

    open static func builder(for name: String) -> StructSpecBuilder {
        return StructSpecBuilder(name: name)
    }
}

open class StructSpecBuilder: TypeSpecBuilder, Builder {
    public typealias Result = StructSpec
    open static let defaultConstruct: Construct = .struct
    fileprivate var includeInit: Bool = false

    public init(name: String) {
        super.init(name: name, construct: StructSpecBuilder.defaultConstruct)
    }

    open func build() -> Result {
        if (methods.contains { $0.name == "init" }) || includeInit {
            addInitMethod()
        }
        return StructSpec(builder: self)
    }

    @discardableResult
    fileprivate func addInitMethod() -> Self {
        var mb = MethodSpec.builder(for: "init")
        let cb = CodeBlock.builder()

        fields.forEach { spec in

            if Modifier.equivalentAccessLevel(parentModifiers: modifiers, childModifiers: spec.modifiers)
                && !spec.modifiers.contains(.Static) {

                mb.add(parameter: ParameterSpec.builder(for: spec.name, type: spec.type!)
                    .add(modifiers: Array(spec.modifiers))
                    .add(description: spec.description)
                    .build()
                )
                let cleanName = spec.name.cleaned(.camelCaseName)

                cb.add(codeBlock: "self.\(cleanName) = \(cleanName)".toCodeBlock())
            }
        }

        mb.add(codeBlock: cb.build())

        mb = mb.add(modifier: Modifier.accessLevel(modifiers))

        return add(method: mb.build())
    }

    @discardableResult
    open func includeDefaultInit() -> StructSpecBuilder {
        includeInit = true
        return self;
    }
}

// MARK: Chaining
extension StructSpecBuilder {

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
        guard StructSpec.asMemberModifiers.contains(toAdd) else {
            return self
        }
        mutatingAdd(modifier: toAdd)
        return self
    }

    @discardableResult
    public func add(modifiers toAdd: [Modifier]) -> Self {
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
