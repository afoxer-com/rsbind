//
//  FieldSpec.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/10/15.
//
//

import Foundation

public protocol FieldSpecType {
    var type: TypeName? { get }
    var initializer: CodeBlock? { get }
    var parentType: Construct? { get set }
    var associatedValues: [TypeName]? { get }
    var nameCase: String.Case? { get }
}

open class FieldSpec: PoetSpec, FieldSpecType {
    open let type: TypeName?
    open let initializer: CodeBlock?
    open var parentType: Construct?
    open var associatedValues: [TypeName]?
    open var nameCase: String.Case?

    fileprivate init(builder: FieldSpecBuilder) {
        self.type = builder.type
        self.initializer = builder.initializer
        self.parentType = builder.parentType
        self.associatedValues = builder.associatedValues
        self.nameCase = builder.nameCase
        super.init(name: builder.name, construct: builder.construct,
                   modifiers: builder.modifiers, description: builder.description,
                   generatorInfo: builder.generatorInfo, framework: builder.framework,
                   imports: builder.imports)
    }

    open static func builder(for name: String, type: TypeName? = nil, construct: Construct? = nil) -> FieldSpecBuilder {
        return FieldSpecBuilder(name: name, type: type, construct: construct)
    }

    open override func collectImports() -> Set<String> {
        guard let nestedImports = type?.collectImports() else {
            return imports
        }
        return imports.union(nestedImports)
    }

    @discardableResult
    open override func emit(to writer: CodeWriter) -> CodeWriter {
        writer.emit(documentationFor: self)

        assert(parentType != nil)

        guard let parentType = parentType else {
            return writer
        }

        switch parentType {
        case .enum where construct != .mutableParam:
            emit(enumType: writer)

        case .struct, .class, .extension:
            emit(classType: writer)

        case .protocol:
            emit(protocolType: writer)

        default:
            emit(classType: writer)
        }

        return writer
    }

    fileprivate func emit(enumType codeWriter: CodeWriter) {
        let cleanName = name.cleaned(nameCase ?? .camelCaseName)
        let cbBuilder = CodeBlock.builder()
                    .add(literal: "case")
                    .add(literal: cleanName)
        
        if let associatedValues = associatedValues {
            cbBuilder.add(literal: "(")
            cbBuilder.add(literal: associatedValues.map {
                return $0.toString()
            }.joined(separator: ","))
            cbBuilder.add(literal: ")")
        }

        if let initializer = initializer {
            cbBuilder.add(literal: "=")
            cbBuilder.add(objects: initializer.emittableObjects)
        }

        codeWriter.emit(codeBlock: cbBuilder.build(), withIndentation: true)
    }

    fileprivate func emit(classType codeWriter: CodeWriter) {
        let defaultNameCase: String.Case = construct == .typeAlias ? .typeName : .camelCaseName
        let cleanName = name.cleaned(nameCase ?? defaultNameCase)
        codeWriter.emit(modifiers: modifiers)
        let cbBuilder = CodeBlock.builder()
            .add(literal: construct)
            .add(literal: cleanName)

        if let type = type {
            cbBuilder.add(literal: ":", trimString: true)
            cbBuilder.add(literal: type)
        }

        if let initializer = initializer {
            if construct == .field || construct == .mutableField {
                cbBuilder.add(literal: "=")
                cbBuilder.add(objects: initializer.emittableObjects)
            } else if construct == .mutableParam {
                cbBuilder.add(type: .beginStatement)
                cbBuilder.add(codeBlock: initializer)
                cbBuilder.add(type: .endStatement)
            } else if construct == .typeAlias && parentType != nil && parentType! == .protocol {
                cbBuilder.add(literal: ":", trimString: true)
                cbBuilder.add(objects: initializer.emittableObjects)
            } else if construct == .typeAlias {
                cbBuilder.add(literal: "=")
                cbBuilder.add(objects: initializer.emittableObjects)
            } else {
                fatalError()
            }
        }

        codeWriter.emit(codeBlock: cbBuilder.build())
    }

    fileprivate func emit(protocolType codeWriter: CodeWriter) {
        let defaultNameCase: String.Case = parentType == .enum || construct == .typeAlias ? .typeName : .camelCaseName
        let cleanName = name.cleaned(nameCase ?? defaultNameCase)
        codeWriter.emit(modifiers: modifiers)
        let cbBuilder = CodeBlock.builder()
            .add(literal: construct)
            .add(literal: cleanName)
            .add(literal: ":", trimString: true)
            .add(literal: type!)

        if construct == .mutableField {
            cbBuilder.add(literal: "{get set}")
        } else {
            cbBuilder.add(literal: "{ get }")
        }

        codeWriter.emit(codeBlock: cbBuilder.build())
    }
}

open class FieldSpecBuilder: PoetSpecBuilder, Builder, FieldSpecType {
    fileprivate static let defaultConstruct: Construct = .field
    
    public typealias Result = FieldSpec

    open let type: TypeName?
    open fileprivate(set) var initializer: CodeBlock? = nil
    open var parentType: Construct?
    open var associatedValues: [TypeName]?
    open var nameCase: String.Case?

    fileprivate init(name: String, type: TypeName? = nil, construct: Construct? = nil) {
        self.type = type
        let requiredConstruct = construct == nil ? FieldSpecBuilder.defaultConstruct : construct!
        // clean name before using
        super.init(name: name, construct: requiredConstruct)
    }

    open func build() -> Result {
        return FieldSpec(builder: self)
    }
}

// MARK: Add field specific info
extension FieldSpecBuilder {
    @discardableResult
    public func add(initializer toAdd: CodeBlock) -> Self {
        self.initializer = toAdd
        return self
    }

    @discardableResult
    public func add(parentType toAdd: Construct) -> Self {
        self.parentType = toAdd
        return self
    }

    @discardableResult
    public func add(nameCase toAdd: String.Case) -> Self {
        self.nameCase = toAdd
        return self
    }
}

// MARK: Chaining
extension FieldSpecBuilder {

    @discardableResult
    public func add(modifier toAdd: Modifier) -> Self {
        mutatingAdd(modifier: toAdd)
        return self
    }

    @discardableResult
    public func add(modifiers toAdd: [Modifier]) -> Self {
        toAdd.forEach { mutatingAdd(modifier: $0) }
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
}

// MARK: Add enum specific info
extension FieldSpecBuilder {
    @discardableResult
    public func add(associatedValues toAdd: [TypeName]) -> Self {
        self.associatedValues = toAdd
        return self
    }
}
