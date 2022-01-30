//
//  MethodSpec.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/11/15.
//
//

import Foundation

public protocol MethodSpecProtocol {
    var typeVariables: [TypeName] { get }
    var throwsError: Bool { get }
    var returnType: TypeName? { get }
    var parameters: [ParameterSpec] { get }
    var codeBlock: CodeBlock? { get }
    var parentType: Construct? { get set}
}

open class MethodSpec: PoetSpec, MethodSpecProtocol {
    open let typeVariables: [TypeName]
    open let throwsError: Bool
    open let returnType: TypeName?
    open let parameters: [ParameterSpec]
    open let codeBlock: CodeBlock?
    open var parentType: Construct?

    fileprivate init(builder: MethodSpecBuilder) {
        self.typeVariables = builder.typeVariables
        self.throwsError = builder.throwsError
        self.returnType = builder.returnType
        self.parameters = builder.parameters
        self.codeBlock = builder.codeBlock
        self.parentType = builder.parentType

        super.init(name: builder.name, construct: builder.construct, modifiers: builder.modifiers,
                   description: builder.description, generatorInfo: builder.generatorInfo, framework: builder.framework, imports: builder.imports)
    }

    open static func builder(for name: String) -> MethodSpecBuilder {
        return MethodSpecBuilder(name: name)
    }

    open override func collectImports() -> Set<String> {
        let nestedImports = [
            typeVariables.reduce(Set<String>()) { set, t in
                return set.union(t.collectImports())
            },
            parameters.reduce(Set<String>()) { set, p in
                return set.union(p.collectImports())
            },
            returnType?.collectImports()
        ]
        return nestedImports.reduce(imports) { imports, list in
            guard let list = list else {
                return imports
            }
            return imports.union(list)
        }
    }

    @discardableResult
    open override func emit(to writer: CodeWriter) -> CodeWriter {
        guard let parentType = parentType else {
            emit(generalFunction: writer)
            return writer
        }

        switch parentType {
        case .protocol:
            emit(functionSignature: writer)
        default:
            emit(generalFunction: writer)
        }

        return writer
    }

    fileprivate func emit(generalFunction emitter: CodeWriter) {
        emit(functionSignature: emitter)
        emitter.emit(type: .beginStatement)
        if let codeBlock = codeBlock {
            emitter.emit(codeBlock: codeBlock)
        }
        emitter.emit(type: .endStatement)
    }

    fileprivate func emit(functionSignature emitter: CodeWriter) {
        emitter.emit(documentationFor: self)
        emitter.emit(modifiers: modifiers)

        let cbBuilder = CodeBlock.builder()
        if name != "init" && name != "init?" {
            cbBuilder.add(literal: construct)
        }
        cbBuilder.add(literal: name)
        cbBuilder.add(literal: "(", trimString: true)
        cbBuilder.add(literal: parameters.map {
            $0.toString()
            }.joined(separator: ", "), trimString: true)
        cbBuilder.add(literal: ")", trimString: true)

        if throwsError {
            cbBuilder.add(literal: "throws")
        }

        emitter.emit(codeBlock: cbBuilder.build())

        if let returnType = returnType {
            let returnBuilder = CodeBlock.builder()
            returnBuilder.add(literal: " ->")
            returnBuilder.add(literal: returnType)
            emitter.emit(codeBlock: returnBuilder.build())
        }
    }
}

open class MethodSpecBuilder: PoetSpecBuilder, Builder, MethodSpecProtocol {
    public typealias Result = MethodSpec
    open static let defaultConstruct: Construct = .method

    open fileprivate(set) var typeVariables = [TypeName]()
    open fileprivate(set) var throwsError = false
    open fileprivate(set) var returnType: TypeName?
    open fileprivate(set) var parameters = [ParameterSpec]()
    open fileprivate(set) var codeBlock: CodeBlock?
    open var parentType: Construct?

    fileprivate init(name: String) {
        // init is a reserved word but is ok as a method name
        let cleanName = name == "init" || name == "init?"  ? name : name.cleaned(.camelCaseName)
        super.init(name: cleanName, construct: MethodSpecBuilder.defaultConstruct)
    }

    open func build() -> Result {
        return MethodSpec(builder: self)
    }
}

// MARK: Add method spcific info
extension MethodSpecBuilder {

    @discardableResult
    public func add(typeVariable toAdd: TypeName) -> Self {
        PoetUtil.addUnique(toAdd, to: &typeVariables)
        return self
    }

    @discardableResult
    public func add(typeVariables toAdd: [TypeName]) -> Self {
        toAdd.forEach { _ = add(typeVariable: $0) }
        return self
    }

    @discardableResult
    public func add(returnType toAdd: TypeName) -> Self {
        self.returnType = toAdd
        return self
    }

    @discardableResult
    public func add(parameter toAdd: ParameterSpec) -> Self {
        PoetUtil.addUnique(toAdd, to: &parameters)
        return self
    }

    @discardableResult
    public func add(parameters toAdd: [ParameterSpec]) -> Self {
        toAdd.forEach { _ = add(parameter: $0) }
        return self
    }

    @discardableResult
    public func add(codeBlock toAdd: CodeBlock) -> Self {
        self.codeBlock = CodeBlock.builder().add(codeBlock: toAdd).build()
        return self
    }

    @discardableResult
    public func add(parentType toAdd: Construct) -> Self {
        self.parentType = toAdd
        return self
    }

    @discardableResult
    public func add(throwable toAdd: Bool) -> Self {
        throwsError = toAdd
        return self
    }
}

// MARK: Chaining
extension MethodSpecBuilder {

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
