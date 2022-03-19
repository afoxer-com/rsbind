//
//  ViewController.swift
//  demo-swift
//
//  Created by sidney.wang on 2018/6/29.
//  Copyright © 2018年 sidney.wang. All rights reserved.
//

import UIKit
import rustlib

class ViewController: UIViewController, DemoCallback {
    private var demoTrait = RustLib.newDemoTrait();
    
    func testI647(arg: Int64, arg2: Int64) -> Int64 {
        7
    }
    
    func testU647(arg: Int64, arg2: Int64) -> Int64 {
        7
    }
    
    func testStr(arg: String) -> String {
        "Hello world"
    }
    
    func testArgVecI6411(arg: [Int64]) -> Int64 {
        11
    }
    
    func testArgVecU6412(arg: [Int64]) -> Int64 {
        12
    }
    
    func testF3230(arg: Float, arg2: Float) -> Float {
        30.0
    }
    
    func testF6431(arg: Double, arg2: Double) -> Double {
        31.0
    }
    
    func testU81(arg: Int8, arg2: Int8) -> Int8 {
        1
    }
    
    func testI82(arg: Int8, arg2: Int8) -> Int8 {
        2
    }
    
    func testI163(arg: Int16, arg2: Int16) -> Int16 {
        3
    }
    
    func testU164(arg: Int16, arg2: Int16) -> Int16 {
        4
    }
    
    func testI325(arg: Int32, arg2: Int32) -> Int32 {
        5
    }
    
    func testU326(arg: Int32, arg2: Int32) -> Int32 {
        6
    }
    
    func testBoolFalse(arg_true: Bool, arg_false: Bool) -> Bool {
        false
    }
    
    func testArgVecStr18(arg: [String]) -> Int32 {
        18
    }
    
    func testArgVecU87(arg: [Int8]) -> Int32 {
        7
    }
    
    func testArgVecI88(arg: [Int8]) -> Int32 {
        8
    }
    
    func testArgVecI169(arg: [Int16]) -> Int32 {
        9
    }
    
    func testArgVecU1610(arg: [Int16]) -> Int32 {
        10
    }
    
    func testArgVecI3211(arg: [Int32]) -> Int32 {
        11
    }
    
    func testArgVecU3212(arg: [Int32]) -> Int32 {
        12
    }
    
    func testArgVecBoolTrue(arg_true: [Bool]) -> Bool {
        true
    }
    
    func testArgVecStruct17(arg: [DemoStruct]) -> Int32 {
        17
    }
    
    func testTwoVecArg13(arg: [Int32], arg1: [Int32]) -> Int32 {
        13
    }
    
    func testArgStruct14(arg: DemoStruct) -> Int32 {
        14
    }
    
    func testTwoArgStruct15(arg: DemoStruct, arg1: DemoStruct) -> Int32 {
        15
    }
    
    func testNoReturn() {
        
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        demoTrait.setup()
        demoTrait.testArgCallback16(arg: self)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}
