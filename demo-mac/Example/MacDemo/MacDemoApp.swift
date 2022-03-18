//
//  MacDemoApp.swift
//  MacDemo
//
//  Created by sidney on 2022/3/8.
//

import SwiftUI
import rustlib

@main
struct MacDemoApp: App {
    private var demoTrait: DemoTrait = RustLib.newDemoTrait()
    @Environment(\.scenePhase) var scenePhase

    var body: some Scene {
        WindowGroup {
            ContentView().onAppear(perform: {
//                while true {
                    print("run once")
                    demoTrait.testU81(arg: 100, arg2: 101)
                    demoTrait.testI82(arg: 100, arg2: 101)
                    demoTrait.testI163(arg: 100, arg2: 101)
                    demoTrait.testU164(arg: 100, arg2: 101)
                    demoTrait.testI325(arg: 100, arg2: 101)
                    demoTrait.testU326(arg: 100, arg2: 101)
                    demoTrait.testF3230(arg: 100.0, arg2: 101.0)
                    demoTrait.testF6431(arg: 100.0, arg2: 101.0)
                    demoTrait.testBoolFalse(arg_true: true, arg2_false: false)
                    demoTrait.testStr(arg: "Hello world")
                    demoTrait.testArgVecStr7(arg: ["Hello world"])
                    demoTrait.testArgVecU8True(arg: [100])
                    demoTrait.testArgVecI86(arg: [100])
                    demoTrait.testArgVecI169(arg: [100])
                    demoTrait.testArgVecU1610(arg: [100])
                    demoTrait.testArgVecI3211(arg: [100])
                    demoTrait.testArgVecU3212(arg: [100])
                    demoTrait.testArgVecBool13(arg_true: [true])
                    demoTrait.testTwoVecArg15(arg: [100], arg1: [101])
                    let demoStruct = demoTrait.testReturnStruct()
                    demoTrait.testArgStruct(arg: demoStruct)
                    demoTrait.testArgVecStruct14(arg: [demoStruct])

                    demoTrait.testReturnVecStr()
                    demoTrait.testReturnVecU8()
                    demoTrait.testReturnVecI8()
                    demoTrait.testReturnVecI16()
                    demoTrait.testReturnVecU16()
                    demoTrait.testReturnVecI32()
                    demoTrait.testReturnVecU32()
                    demoTrait.testReturnVecBoolTrue()
                    demoTrait.testTwoVecU8(input: [100])
                    demoTrait.testReturnVecStruct()
                    
                    demoTrait.testArgCallback16(arg: createCallback())
                    demoTrait.testTwoArgCallback20(arg: createCallback(), arg1: createCallback())
//                }
            })
        }
    }
    
    
    private func createCallback() -> DemoCallback {
        class AssertDemoCallback : DemoCallback {
            func testF3230(arg: Float, arg2: Float) -> Float {
                return 30.0
            }
            
            func testF6431(arg: Double, arg2: Double) -> Double {
                return 31.0
            }
            
            
            func testU81(arg: Int8, arg2: Int8) -> Int8 {
                return 1
            }
            
            func testI82(arg: Int8, arg2: Int8) -> Int8 {
                return 2
            }
            
            func testI163(arg: Int32, arg2: Int32) -> Int32 {
                return 3
            }
            
            func testU164(arg: Int32, arg2: Int32) -> Int32 {
                return 4
            }
            
            func testI325(arg: Int32, arg2: Int32) -> Int32 {
                return 5
            }
            
            func testU326(arg: Int32, arg2: Int32) -> Int32 {
                return 6
            }
            
            func testBoolFalse(arg_true: Bool, arg_false: Bool) -> Bool {
                return false
            }
            
            func testArgVecStr18(arg: [String]) -> Int32 {
                return 18
            }
            
            func testArgVecU87(arg: [Int8]) -> Int32 {
                return 7
            }
            
            func testArgVecI88(arg: [Int8]) -> Int32 {
                return 8
            }
            
            func testArgVecI169(arg: [Int32]) -> Int32 {
                return 9
            }
            
            func testArgVecU1610(arg: [Int32]) -> Int32 {
                return 10
            }
            
            func testArgVecI3211(arg: [Int32]) -> Int32 {
                return 11
            }
            
            func testArgVecU3212(arg: [Int32]) -> Int32 {
                return 12
            }
            
            func testArgVecBoolTrue(arg_true: [Bool]) -> Bool {
                return true
            }
            
            func testArgVecStruct17(arg: [DemoStruct]) -> Int32 {
                return 17
            }
            
            func testTwoVecArg13(arg: [Int32], arg1: [Int32]) -> Int32 {
                return 13
            }
            
            func testArgStruct14(arg: DemoStruct) -> Int32 {
                return 14
            }
            
            func testTwoArgStruct15(arg: DemoStruct, arg1: DemoStruct) -> Int32 {
                return 15
            }
            
            func testNoReturn() {
                
            }
            
            func testStr(arg : String) -> String {
//                print("arg = \(arg)")
                return "Hello world"
            }
        }
        
        return AssertDemoCallback()
    }
}
