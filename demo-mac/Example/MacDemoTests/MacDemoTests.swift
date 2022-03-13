//
//  MacDemoTests.swift
//  MacDemoTests
//
//  Created by sidney on 2022/3/8.
//


import XCTest
import rustlib

class demo_ios_ExampleTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
        DemoTrait.setup();
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testBase() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct results.
        XCTAssertEqual(DemoTrait.test_u8_1(arg: 100, arg2: 101), 1)
        XCTAssertEqual(DemoTrait.test_i8_2(arg: 100, arg2: 101), 2)
        XCTAssertEqual(DemoTrait.test_i16_3(arg: 100, arg2: 101), 3)
        XCTAssertEqual(DemoTrait.test_u16_4(arg: 100, arg2: 101), 4)
        XCTAssertEqual(DemoTrait.test_i32_5(arg: 100, arg2: 101), 5)
        XCTAssertEqual(DemoTrait.test_u32_6(arg: 100, arg2: 101), 6)
        XCTAssertEqual(DemoTrait.test_f32_30(arg: 100.0, arg2: 101.0), 30)
        XCTAssertEqual(DemoTrait.test_f64_31(arg: 100.0, arg2: 101.0), 31)
        XCTAssertEqual(DemoTrait.test_bool_false(arg_true: true, arg2_false: false), false)
        XCTAssertEqual(DemoTrait.test_str(arg: "Hello world"), "Hello world")
        XCTAssertEqual(DemoTrait.test_arg_vec_str_7(arg: ["Hello world"]), 7)
        XCTAssertEqual(DemoTrait.test_arg_vec_u8_true(arg: [100]), true)
        XCTAssertEqual(DemoTrait.test_arg_vec_i8_6(arg: [100]), 6)
        XCTAssertEqual(DemoTrait.test_arg_vec_i16_9(arg: [100]), 9)
        XCTAssertEqual(DemoTrait.test_arg_vec_u16_10(arg: [100]), 10)
        XCTAssertEqual(DemoTrait.test_arg_vec_i32_11(arg: [100]), 11)
        XCTAssertEqual(DemoTrait.test_arg_vec_u32_12(arg: [100]), 12)
        XCTAssertEqual(DemoTrait.test_arg_vec_bool_13(arg_true: [true]), 13)
        XCTAssertEqual(DemoTrait.test_two_vec_arg_15(arg: [100], arg1: [101]), 15)
        let demoStruct = DemoTrait.test_return_struct()
        assertStruct(demoStruct: demoStruct)
        DemoTrait.test_arg_struct(arg: demoStruct)
        XCTAssertEqual(DemoTrait.test_arg_vec_struct_14(arg: [demoStruct]), 14)
    }
    
    func testReturnVec() throws {
        XCTAssertEqual(DemoTrait.test_return_vec_str(), ["Hello world"])
        XCTAssertEqual(DemoTrait.test_return_vec_u8(), [100])
        XCTAssertEqual(DemoTrait.test_return_vec_i8(), [100])
        XCTAssertEqual(DemoTrait.test_return_vec_i16(), [100])
        XCTAssertEqual(DemoTrait.test_return_vec_u16(), [100])
        XCTAssertEqual(DemoTrait.test_return_vec_i32(), [100])
        XCTAssertEqual(DemoTrait.test_return_vec_u32(), [100])
        XCTAssertEqual(DemoTrait.test_return_vec_bool_true(), [true])
        XCTAssertEqual(DemoTrait.test_two_vec_u8(input: [100]), [100])
        let demoStruct = DemoTrait.test_return_vec_struct()
        assertStruct(demoStruct: demoStruct[0])
    }
    
    func testCallback() throws {
        XCTAssertEqual(DemoTrait.test_arg_callback_16(arg: createCallback(demoTest: self)), 16)
        XCTAssertEqual(DemoTrait.test_two_arg_callback_20(arg: createCallback(demoTest: self), arg1: createCallback(demoTest: self)), 20)
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
            func test_f32_30(arg: Float, arg2: Float) -> Float {
                XCTAssertEqual(arg, 100.0)
                XCTAssertEqual(arg2, 101.0)
                return 30.0
            }
            
            func test_f64_31(arg: Double, arg2: Double) -> Double {
                XCTAssertEqual(arg, 100.0)
                XCTAssertEqual(arg2, 101.0)
                return 31.0
            }
            
            private var demoTest: demo_ios_ExampleTests
            
            init(demoTest: demo_ios_ExampleTests) {
                self.demoTest = demoTest
            }
            
            func test_u8_1(arg: Int8, arg2: Int8) -> Int8 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 1
            }
            
            func test_i8_2(arg: Int8, arg2: Int8) -> Int8 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 2
            }
            
            func test_i16_3(arg: Int32, arg2: Int32) -> Int32 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 3
            }
            
            func test_u16_4(arg: Int32, arg2: Int32) -> Int32 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 4
            }
            
            func test_i32_5(arg: Int32, arg2: Int32) -> Int32 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 5
            }
            
            func test_u32_6(arg: Int32, arg2: Int32) -> Int32 {
                XCTAssertEqual(arg, 100)
                XCTAssertEqual(arg2, 101)
                return 6
            }
            
            func test_bool_false(arg_true: Bool, arg_false: Bool) -> Bool {
                XCTAssertEqual(arg_true, true)
                XCTAssertEqual(arg_false, false)
                return false
            }
            
            func test_arg_vec_str_18(arg: [String]) -> Int32 {
                XCTAssertEqual(arg, ["Hello world"])
                return 18
            }
            
            func test_arg_vec_u8_7(arg: [Int8]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 7
            }
            
            func test_arg_vec_i8_8(arg: [Int8]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 8
            }
            
            func test_arg_vec_i16_9(arg: [Int32]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 9
            }
            
            func test_arg_vec_u16_10(arg: [Int32]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 10
            }
            
            func test_arg_vec_i32_11(arg: [Int32]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 11
            }
            
            func test_arg_vec_u32_12(arg: [Int32]) -> Int32 {
                XCTAssertEqual(arg, [100])
                return 12
            }
            
            func test_arg_vec_bool_true(arg_true: [Bool]) -> Bool {
                XCTAssertEqual(arg_true, [true])
                return true
            }
            
            func test_arg_vec_struct_17(arg: [DemoStruct]) -> Int32 {
                demoTest.assertStruct(demoStruct: arg[0])
                return 17
            }
            
            func test_two_vec_arg_13(arg: [Int32], arg1: [Int32]) -> Int32 {
                XCTAssertEqual(arg, [100])
                XCTAssertEqual(arg1, [101])
                return 13
            }
            
            func test_arg_struct_14(arg: DemoStruct) -> Int32 {
                demoTest.assertStruct(demoStruct: arg)
                return 14
            }
            
            func test_two_arg_struct_15(arg: DemoStruct, arg1: DemoStruct) -> Int32 {
                demoTest.assertStruct(demoStruct: arg)
                demoTest.assertStruct(demoStruct: arg1)
                return 15
            }
            
            func test_no_return() {
                
            }
        }
        
        return AssertDemoCallback(demoTest: demoTest)
    }
}
