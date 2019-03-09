//
//  PoetFile.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 1/25/16.
//
//

import Foundation

// Represents a list of PoetSpecs in a single file
public protocol PoetFileProtocol {
    var fileName: String? { get }
    var addGenerationDate: Bool { get }
    var specList: [PoetSpec] { get }
    var fileContents: String { get }

    func append(_ item: PoetSpec)
}


open class PoetFile: PoetFileProtocol, Importable {
    open fileprivate(set) var fileName: String?
    open fileprivate(set) var specList: [PoetSpec]
    open fileprivate(set) var generatorInfo: String?
    open fileprivate(set) var addGenerationDate: Bool = true

    open var fileContents: String {
        return toFile()
    }

    open var imports: Set<String> {
        return collectImports()
    }

    fileprivate var framework: String?

    public init(list: [PoetSpec], framework: String? = nil, generatorInfo: String?, addGenerationDate: Bool = true) {
        self.specList = list
        self.fileName = list.first?.name
        self.framework = framework
        self.generatorInfo = generatorInfo
        self.addGenerationDate = addGenerationDate
    }

    public convenience init(spec: PoetSpec, framework: String? = nil, generatorInfo: String?, addGenerationDate: Bool = true) {
        self.init(list: [spec], framework: framework, generatorInfo: generatorInfo, addGenerationDate: addGenerationDate)
    }

    open func append(_ item: PoetSpec) {
        specList.append(item)
        if fileName == nil {
            fileName = item.name
        }
    }

    open func collectImports() -> Set<String> {
        return specList.reduce(Set<String>()) { set, spec in
            return set.union(spec.collectImports())
        }
    }

    fileprivate func toFile() -> String {
        let codeWriter = CodeWriter()
        codeWriter.emitFileHeader(fileName: fileName, framework: framework, generatorInfo: generatorInfo, addGenerationDate: addGenerationDate, specs: specList)
        codeWriter.emit(imports: imports)
        codeWriter.emit(specs: specList)
        return codeWriter.out
    }
}
