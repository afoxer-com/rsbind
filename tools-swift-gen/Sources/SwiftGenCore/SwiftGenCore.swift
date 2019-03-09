import Foundation
import SwiftPoet

enum RunError : Error {
    case ArgumentError
}

public final class SwiftGen {
    private let arguments : [String]
    private let astDir: String
    private let libModuleName: String
    private let outputDir: String
    
    public init(arguments: [String] = CommandLine.arguments) throws {
        self.arguments = arguments
        if arguments.count < 4 {
            print("please support enough arguments.")
            print("usage: cmd astPath libmoduleName outputDir")
            throw RunError.ArgumentError
        }
        
        self.astDir = arguments[1]
        self.libModuleName = arguments[2]
        self.outputDir = arguments[3]
    }
    
    public func run() throws {
        print("Swift gen run begin...")
        prepareDirs()
        
        let filemanager = FileManager.default
        let enumerator = filemanager.enumerator(atPath: self.astDir)
        
        let poetFile = PoetFile(list: [], generatorInfo: nil)
        
        var callback_dic = [String: TraitDesc]()
        var allTraitDescs = [TraitDesc]()
        var allStructDescs = [StructDesc]()
        
        let globalIndexSpec = FieldSpec.builder(for: "globalIndex", type: TypeName.init(keyword: "Int64"), construct: .mutableField)
            .add(parentType: .mutableField)
            .add(modifier: Modifier.Private)
            .add(initializer: CodeBlock.builder().add(literal: "0").build());
        poetFile.append(globalIndexSpec.build())
        
        let globalCallbacksSpec = FieldSpec.builder(for: "globalCallbacks", type: TypeName.init(keyword: "[Int64: Any]"), construct: .mutableField)
            .add(parentType: .mutableField)
            .add(modifier: Modifier.Private)
            .add(initializer: CodeBlock.builder().add(literal: "[Int64: Any]()").build());
        poetFile.append(globalCallbacksSpec.build())
        
        print("appedning global callbacks over")

        enumerator?.forEach({ file in
            let filePath = (self.astDir as NSString).strings(byAppendingPaths: [file as! String])[0]
            
            let parser = AstParser(path: filePath)
            let astResult = parser.parseAst()
            let traitDescs = astResult.traits
            for desc in traitDescs {
                allTraitDescs.append(desc)
            }
            
            print("found contract => \(traitDescs)")
            
            let _ = traitDescs.filter { desc -> Bool in
                desc.is_callback
            }.map { desc -> (String, TraitDesc) in
                print("2.1.0")
                callback_dic[desc.name] = desc
                print("2.1.1")
                return (desc.name, desc)
            }
            print("2.1 callbacks = \(callback_dic)")
            
            let structs = astResult.structs
            for struct_ in structs {
                allStructDescs.append(struct_)
            }
            
        })
        
        for desc in allTraitDescs {
            print("2.2.0")
            if desc.is_callback {
                let traitGenerator = CallbackGenerator(desc: desc, libModName: self.libModuleName)
                let cls = traitGenerator.generate()
                poetFile.append(cls.build())
                print("2.2")
                
            } else {
                let traitGenerator = TraitGenerator(desc: desc, libModName: self.libModuleName)
                let cls = traitGenerator.generate(callbacks: callback_dic)
                poetFile.append(cls.build())
                print("2.3")
            }
        }
        print("3")
        
        for struct_ in allStructDescs {
            let structGenerator = StructGenerator(desc: struct_, libModName: self.libModuleName)
            let result = structGenerator.generate()
            poetFile.append(result.build())
        }

        
        let outFilePath = (self.outputDir as NSString).strings(byAppendingPaths: ["ffi.swift"])[0]
        let url = URL(fileURLWithPath: outFilePath)
        do {
            if !filemanager.fileExists(atPath: url.path) {
                filemanager.createFile(atPath: url.path, contents: nil, attributes: nil)
            }
            
            let fileHandle = try FileHandle(forWritingTo: url)
            fileHandle.seekToEndOfFile()
            fileHandle.write(Data(bytes: Array(poetFile.fileContents.utf8)))
            fileHandle.closeFile()
        } catch let err as NSError {
            print("write code to file error => \(err)")
        }
        print("code => \(poetFile.fileContents)")
    }
    
    func prepareDirs() {
        let filemanager = FileManager.default
        let exist = filemanager.fileExists(atPath: self.outputDir)
        
        do {
            if exist {
               try filemanager.removeItem(atPath: self.outputDir)
               try filemanager.createDirectory(atPath: self.outputDir, withIntermediateDirectories: true, attributes: nil)
            }
        } catch let err as NSError {
            print("prepare directories erro => \(err)")
        }
    }
}
