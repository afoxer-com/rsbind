//
//  MacDemoTests.swift
//  MacDemoTests
//
//  Created by sidney on 2022/3/8.
//


import XCTest
import rustlib

class demo_ios_ExampleTests: XCTestCase {
    private var demoTrait: DemoTrait = RustLib.newDemoTrait()
    private var demoCallback: DemoCallback?
    
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
        demoTrait.setup()
        demoCallback = demoTrait.testReturnCallback()
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    func testReturnCallback() throws {

    }

    func testBase() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct results.
        XCTAssertEqual(demoTrait.testU81(arg: 100, arg2: 101), 1)
        XCTAssertEqual(demoTrait.testI82(arg: 100, arg2: 101), 2)
        XCTAssertEqual(demoTrait.testI163(arg: 100, arg2: 101), 3)
        XCTAssertEqual(demoTrait.testU164(arg: 100, arg2: 101), 4)
        XCTAssertEqual(demoTrait.testI325(arg: 100, arg2: 101), 5)
        XCTAssertEqual(demoTrait.testU326(arg: 100, arg2: 101), 6)
        XCTAssertEqual(demoTrait.testF3230(arg: 100.0, arg2: 101.0), 30)
        XCTAssertEqual(demoTrait.testF6431(arg: 100.0, arg2: 101.0), 31)
        XCTAssertEqual(demoTrait.testBoolFalse(arg_true: true, arg2_false: false), false)
        XCTAssertEqual(demoTrait.testStr(arg: "Hello world"), "Hello world")
        XCTAssertEqual(demoTrait.testArgVecStr7(arg: ["Hello world"]), 7)
        XCTAssertEqual(demoTrait.testArgVecU8True(arg: [100]), true)
        XCTAssertEqual(demoTrait.testArgVecI86(arg: [100]), 6)
        XCTAssertEqual(demoTrait.testArgVecI169(arg: [100]), 9)
        XCTAssertEqual(demoTrait.testArgVecU1610(arg: [100]), 10)
        XCTAssertEqual(demoTrait.testArgVecI3211(arg: [100]), 11)
        XCTAssertEqual(demoTrait.testArgVecU3212(arg: [100]), 12)
        XCTAssertEqual(demoTrait.testArgVecBool13(arg_true: [true]), 13)
        XCTAssertEqual(demoTrait.testTwoVecArg15(arg: [100], arg1: [101]), 15)
        let demoStruct = demoTrait.testReturnStruct()
        assertStruct(demoStruct: demoStruct)
        demoTrait.testArgStruct(arg: demoStruct)
        XCTAssertEqual(demoTrait.testArgVecStruct14(arg: [demoStruct]), 14)
        
        
        XCTAssertEqual(demoCallback?.testU81(arg: 100, arg2: 101), 1)
        XCTAssertEqual(demoCallback?.testI82(arg: 100, arg2: 101), 2)
        XCTAssertEqual(demoCallback?.testI163(arg: 100, arg2: 101), 3)
        XCTAssertEqual(demoCallback?.testU164(arg: 100, arg2: 101), 4)
        XCTAssertEqual(demoCallback?.testI325(arg: 100, arg2: 101), 5)
        XCTAssertEqual(demoCallback?.testU326(arg: 100, arg2: 101), 6)
        XCTAssertEqual(demoCallback?.testF3230(arg: 100.0, arg2: 101.0), 30)
        XCTAssertEqual(demoCallback?.testF6431(arg: 100.0, arg2: 101.0), 31)
        XCTAssertEqual(demoCallback?.testBoolFalse(arg_true: true, arg_false: false), false)
        XCTAssertEqual(demoCallback?.testStr(arg: "Hello world"), "Hello world")
        XCTAssertEqual(demoCallback?.testArgVecStr18(arg: ["Hello world"]), 18)
        XCTAssertEqual(demoCallback?.testArgVecU87(arg: [100]), 7)
        XCTAssertEqual(demoCallback?.testArgVecI88(arg: [100]), 8)
        XCTAssertEqual(demoCallback?.testArgVecI169(arg: [100]), 9)
        XCTAssertEqual(demoCallback?.testArgVecU1610(arg: [100]), 10)
        XCTAssertEqual(demoCallback?.testArgVecI3211(arg: [100]), 11)
        XCTAssertEqual(demoCallback?.testArgVecU3212(arg: [100]), 12)
        XCTAssertEqual(demoCallback?.testArgVecBoolTrue(arg_true: [true]), true)
        XCTAssertEqual(demoCallback?.testTwoVecArg13(arg: [100], arg1: [101]), 13)
    }
    
    func testReturnVec() throws {
        XCTAssertEqual(demoTrait.testReturnVecStr(), ["Hello world"])
        XCTAssertEqual(demoTrait.testReturnVecU8(), [100])
        XCTAssertEqual(demoTrait.testReturnVecI8(), [100])
        XCTAssertEqual(demoTrait.testReturnVecI16(), [100])
        XCTAssertEqual(demoTrait.testReturnVecU16(), [100])
        XCTAssertEqual(demoTrait.testReturnVecI32(), [100])
        XCTAssertEqual(demoTrait.testReturnVecU32(), [100])
        XCTAssertEqual(demoTrait.testReturnVecBoolTrue(), [true])
        XCTAssertEqual(demoTrait.testTwoVecU8(input: [100]), [100])
        let demoStruct = demoTrait.testReturnVecStruct()
        assertStruct(demoStruct: demoStruct[0])
        
        XCTAssertEqual(demoCallback?.testReturnVecU8(), [100])
        XCTAssertEqual(demoCallback?.testReturnVecI8(), [100])
        XCTAssertEqual(demoCallback?.testReturnVecI16(), [100])
        XCTAssertEqual(demoCallback?.testReturnVecU16(), [100])
        XCTAssertEqual(demoCallback?.testReturnVecI32(), [100])
        XCTAssertEqual(demoCallback?.testReturnVecU32(), [100])
        XCTAssertEqual(demoCallback?.testTwoVecU8(input: [100]), [100])
    }
    
    func testCallback() throws {
        XCTAssertEqual(demoTrait.testArgCallback16(arg: createCallback(demoTest: self)), 16)
        XCTAssertEqual(demoTrait.testTwoArgCallback20(arg: createCallback(demoTest: self), arg1: createCallback(demoTest: self)), 20)
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        measure {
            // Put the code you want to measure the time of here.
        }
    }
    
    private func assertStruct(demoStruct: DemoStruct) {
        XCTAssertEqual(demoStruct.arg1, 1)
        XCTAssertEqual(demoStruct.arg2, 2)
        XCTAssertEqual(demoStruct.arg3, 3)
        XCTAssertEqual(demoStruct.arg4, 4)
        XCTAssertEqual(demoStruct.arg5, 5)
        XCTAssertEqual(demoStruct.arg6, 6)
        XCTAssertEqual(demoStruct.arg7_str, "Hello world")
        XCTAssertEqual(demoStruct.arg8_false, false)
        XCTAssertTrue(demoStruct.arg9 > 0)
        XCTAssertTrue(demoStruct.arg10 > 0)
    }

    private func createCallback(demoTest: demo_ios_ExampleTests) -> DemoCallback {
        class AssertDemoCallback : DemoCallback {
            func testTwoVecU8(input: [Int8]) -> [Int8] {
                XCTAssertEqual(input[0], 100)
                return [100]
            }
            
            func testReturnVecI8() -> [Int8] {
                [100]
            }
            
            func testReturnVecI16() -> [Int16] {
                [100]
            }
            
            func testReturnVecU16() -> [Int16] {
                [100]
            }
            
            func testReturnVecI32() -> [Int32] {
                [100]
            }
            
            func testReturnVecU32() -> [Int32] {
                [100]
            }
            
            func testReturnVecI64() -> [Int64] {
                [100]
            }
            
            func testReturnVecU64() -> [Int64] {
                [100]
            }
            
            func testReturnVecU8() -> [Int8] {
                return [100]
            }
            
            func testI647(arg: Int64, arg2: Int64) -> Int64 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 7
            }
            
            func testU647(arg: Int64, arg2: Int64) -> Int64 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 7
            }
            
            func testArgVecI6411(arg: [Int64]) -> Int64 {
                XCTAssertEqual(arg[0], 100)
                return 11
            }
            
            func testArgVecU6412(arg: [Int64]) -> Int64 {
                XCTAssertEqual(arg[0], 100)
                return 12
            }
            
            func testI163(arg: Int16, arg2: Int16) -> Int16 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 3
            }
            
            func testU164(arg: Int16, arg2: Int16) -> Int16 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 4
            }
            
            func testArgVecI169(arg: [Int16]) -> Int32 {
                XCTAssertEqual(arg[0], 100)
                return 9
            }
            
            func testArgVecU1610(arg: [Int16]) -> Int32 {
                XCTAssertEqual(arg[0], 100)
                return 10
            }
            
            func testF3230(arg: Float, arg2: Float) -> Float {
                XCTAssertEqual(arg, 100.0)
                XCTAssertEqual(arg2, 101.0)
                return 30.0
            }
            
            func testF6431(arg: Double, arg2: Double) -> Double {
                XCTAssertEqual(arg, 100.0)
                XCTAssertEqual(arg2, 101.0)
                return 31.0
            }
            
            private var demoTest: demo_ios_ExampleTests
            
            init(demoTest: demo_ios_ExampleTests) {
                self.demoTest = demoTest
            }
            
            func testU81(arg: Int8, arg2: Int8) -> Int8 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 1
            }
            
            func testI82(arg: Int8, arg2: Int8) -> Int8 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 2
            }
            
            func testI325(arg: Int32, arg2: Int32) -> Int32 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 5
            }
            
            func testU326(arg: Int32, arg2: Int32) -> Int32 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 6
            }
            
            func testBoolFalse(arg_true: Bool, arg_false: Bool) -> Bool {
                XCTAssertEqual(arg_true, true)
                XCTAssertEqual(arg_false, false)
                return false
            }
            
            func testArgVecStr18(arg: [String]) -> Int32 {
                XCTAssertEqual(arg, ["Hello world"])
                return 18
            }
            
            func testArgVecU87(arg: [Int8]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 7
            }
            
            func testArgVecI88(arg: [Int8]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 8
            }
            
            func testArgVecI3211(arg: [Int32]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 11
            }
            
            func testArgVecU3212(arg: [Int32]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 12
            }
            
            func testArgVecBoolTrue(arg_true: [Bool]) -> Bool {
                XCTAssertEqual(arg_true, [true])
                return true
            }
            
            func testArgVecStruct17(arg: [DemoStruct]) -> Int32 {
                demoTest.assertStruct(demoStruct: arg[0])
                return 17
            }
            
            func testTwoVecArg13(arg: [Int32], arg1: [Int32]) -> Int32 {
                XCTAssertEqual(arg, [100])
                XCTAssertEqual(arg1, [101])
                return 13
            }
            
            func testArgStruct14(arg: DemoStruct) -> Int32 {
                demoTest.assertStruct(demoStruct: arg)
                return 14
            }
            
            func testTwoArgStruct15(arg: DemoStruct, arg1: DemoStruct) -> Int32 {
                demoTest.assertStruct(demoStruct: arg)
                demoTest.assertStruct(demoStruct: arg1)
                return 15
            }
            
            func testNoReturn() {
                
            }
            
            func testStr(arg : String) -> String {
                XCTAssertEqual(arg, "Hello world")
                return "Hello world"
            }
        }
        
        return AssertDemoCallback(demoTest: demoTest)
    }
}
