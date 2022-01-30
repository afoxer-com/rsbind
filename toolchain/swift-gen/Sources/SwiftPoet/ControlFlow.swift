//
//  ControlFlow.swift
//  SwiftPoet
//
//  Created by Kyle Dorman on 11/18/15.
//
//

import Foundation

public enum ControlFlow: String {
    case guardStatement = "guard"
    case ifStatement = "if"
    case elseIfClause = "else if"
    case elseClause = "else"
    case whileStatement = "while"
    case repeatWhileStatement = "repeat"
    case forInClause = "in"
    case forStatement = "for"
    case switchStatement = "switch"

    public static var guardControlFlow: (ComparisonList?, () -> CodeBlock) -> CodeBlock = ControlFlow.fnGenerator(.guardStatement)

    public static var ifControlFlow: (ComparisonList?, () -> CodeBlock) -> CodeBlock = ControlFlow.fnGenerator(.ifStatement)

    public static var elseIfControlFlow: (ComparisonList?, () -> CodeBlock) -> CodeBlock = ControlFlow.fnGenerator(.elseIfClause)

    public static var elseControlFlow: (ComparisonList?, () -> CodeBlock) -> CodeBlock = ControlFlow.fnGenerator(.elseClause)

    public static var whileControlFlow: (ComparisonList?, () -> CodeBlock) -> CodeBlock = ControlFlow.fnGenerator(.whileStatement)

    public static func repeatWhileControlFlow(_ comparisonList: ComparisonList, bodyFn: () -> CodeBlock) -> CodeBlock {
        return CodeBlock.builder()
            .add(literal: ControlFlow.repeatWhileStatement.rawValue)
            .add(type: .beginStatement)
            .add(codeBlock: bodyFn())
            .add(type: .endStatement)
            .add(literal: ControlFlow.whileStatement.rawValue)
            .add(type: .emitter, data: comparisonList)
            .build()
    }

    public static func forInControlFlow(_ iterator: Literal, iterable: Literal, bodyFn: () -> CodeBlock) -> CodeBlock {
        return CodeBlock.builder()
            .add(literal: ControlFlow.forStatement.rawValue)
            .add(literal: iterator)
            .add(literal: ControlFlow.forInClause.rawValue)
            .add(literal: iterable)
            .add(type: .beginStatement)
            .add(codeBlock: bodyFn())
            .add(type: .endStatement)
            .build()
    }

    public static func closure(parameterList: Literal, canThrow: Bool, returnType: Literal? , bodyFn: () -> CodeBlock) -> CodeBlock {
        let cb = CodeBlock.builder()
        let closureBlock = CodeBlock.builder()

        cb.add(type: .beginStatement)

        closureBlock.add(literal: "(")
        closureBlock.add(literal: parameterList, trimString: true)
        closureBlock.add(literal: ")", trimString: true)
        if canThrow {
            closureBlock.add(literal: "throws")
        }
        closureBlock.add(literal: "->")
        if let returnType = returnType {
            closureBlock.add(literal: returnType)
        } else {
            closureBlock.add(literal: "Void")
        }
        closureBlock.add(literal: ControlFlow.forInClause.rawValue)

        closureBlock.add(type: .increaseIndentation)
        closureBlock.add(codeBlock: bodyFn())
        closureBlock.add(type: .decreaseIndentation)

        cb.add(codeBlock: closureBlock.build())
        cb.add(type: .endStatement)

        return cb.build()
    }

    public static func forControlFlow(_ iterator: CodeBlock, iterable: CodeBlock, execution: CodeBlock) -> CodeBlock {
        fatalError("So many loops so little time")
    }

    public static func doCatchControlFlow(_ doFn: () -> CodeBlock, catchFn: () -> CodeBlock) -> CodeBlock {
        let doCB = CodeBlock.builder()
        doCB.add(literal: "do")
        doCB.add(type: .beginStatement)
        doCB.add(codeBlock: doFn())
        doCB.add(type: .endStatement)

        let catchCB = CodeBlock.builder()
        catchCB.add(literal: "catch")
        catchCB.add(type: .beginStatement)
        catchCB.add(codeBlock: catchFn())
        catchCB.add(type: .endStatement)

        return doCB.add(codeBlock: catchCB.build()).build()
    }

    public static func switchControlFlow(_ switchValue: String, cases: [(String, CodeBlock)], defaultCase: CodeBlock? = nil) -> CodeBlock {
        let cb = CodeBlock.builder()
        cb.add(literal: ControlFlow.switchStatement.rawValue)
        cb.add(literal: switchValue)
        cb.add(type: .beginStatement)

        cases.forEach { caseItem in
            cb.add(codeBlock: ControlFlow.switchCase(caseItem.0, execution: caseItem.1))
        }

        if let defaultCase = defaultCase {
            cb.add(codeBlock: ControlFlow.switchCase(nil, execution: defaultCase))
        }

        cb.add(type: .endStatement)
        return cb.build()
    }

    fileprivate static func switchCase(_ caseLine: String?, execution: CodeBlock)
        -> CodeBlock
    {
        let cbCase = CodeBlock.builder()

        cbCase.add(literal: "case")
        if let caseLine = caseLine {
            cbCase.add(literal: caseLine)
        } else {
            cbCase.add(literal: "_")
        }
        cbCase.add(literal: ":")

        let cbCaseLineTwo = CodeBlock.builder()
        cbCaseLineTwo.add(type: .increaseIndentation)
        cbCaseLineTwo.add(codeBlock: execution)
        cbCaseLineTwo.add(type: .decreaseIndentation)

        cbCase.add(codeBlock: cbCaseLineTwo.build())

        return cbCase.build()
    }

    fileprivate static func fnGenerator(_ type: ControlFlow) -> (ComparisonList?, () -> CodeBlock) -> CodeBlock {
        return { (comparisonList: ComparisonList?, bodyFn: () -> CodeBlock) -> CodeBlock in
            let cb = CodeBlock.builder()
                .add(literal: type.rawValue)

            if type != .elseClause && comparisonList != nil {
                cb.add(type: .emitter, data: comparisonList!)
            }

            if type == .guardStatement {
                cb.add(literal: "else")
            }

            cb.add(type: .beginStatement)
            cb.add(codeBlock: bodyFn())
            cb.add(type: .endStatement)
            return cb.build()
        }
    }
}

open class ComparisonList: Emitter {
    fileprivate let requirement: Requirement?
    fileprivate let list: [Either<ComparisonListItem, ComparisonList>]

    public init(lhs: CodeBlock, comparator: Comparator, rhs: CodeBlock) {
        let comparison = Comparison(lhs: lhs, comparator: comparator, rhs: rhs)
        let listItem = ComparisonListItem(comparison: comparison)
        self.list = [Either.left(listItem)]
        self.requirement = nil
    }

    public init(list: [ComparisonListItem], requirement: Requirement? = nil) {
        self.list = list.map { item in
            return Either.left(item)
        }
        self.requirement = requirement
    }

    public init(list: [Either<ComparisonListItem, ComparisonList>], requirement: Requirement? = nil) {
        self.list = list
        self.requirement = requirement
    }

    @discardableResult
    open func emit(to writer: CodeWriter) -> CodeWriter {
        if requirement != nil {
            writer.emit(literal: requirement!)
        }

        list.forEach { listItem in
            switch listItem {
            case .left(let item):
                item.emit(to: writer)
            case .right(let cList):
                writer.emit(type: .literal, data: "(")
                cList.emit(to: writer)
                writer.emit(type: .literal, data: ")")
            }
        }

        return writer
    }

    open func toString() -> String {
        return emit(to: CodeWriter()).out
    }
}

open class ComparisonListItem: Emitter {
    let comparison: Comparison
    let requirement: Requirement?

    public init(comparison: Comparison, requirement: Requirement? = nil) {
        self.comparison = comparison
        self.requirement = requirement
    }

    @discardableResult
    open func emit(to writer: CodeWriter) -> CodeWriter {
        if requirement != nil {
            writer.emit(type: .literal, data: requirement!.rawValue)
        }
        return comparison.emit(to: writer)
    }

    open func toString() -> String {
        return emit(to: CodeWriter()).out
    }
}

open class Comparison: Emitter {
    let lhs: CodeBlock
    let comparator: Comparator
    let rhs: CodeBlock

    public init(lhs: CodeBlock, comparator: Comparator, rhs: CodeBlock) {
        self.lhs = lhs
        self.comparator = comparator
        self.rhs = rhs
    }

    open func emit(to writer: CodeWriter) -> CodeWriter {
        let cbBuilder = CodeBlock.builder()
        cbBuilder.add(objects: lhs.emittableObjects)
        cbBuilder.add(literal: comparator.rawValue)
        cbBuilder.add(objects: rhs.emittableObjects)
        writer.emit(codeBlock: cbBuilder.build())

        return writer
    }

    open func toString() -> String {
        return emit(to: CodeWriter()).out
    }
}

public enum Comparator: String {
    case Equals = "=="
    case NotEquals = "!="
    case LessThan = "<"
    case GreaterThan = ">"
    case LessThanOrEqualTo = "<="
    case GreaterThanOrEqualTo = ">="
    case OptionalCheck = "="
}

public enum Requirement: String {
    case And = "&&"
    case Or = "||"
    case OptionalList = ", "
}
