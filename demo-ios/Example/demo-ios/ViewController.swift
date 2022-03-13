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
    
    func test_f32_30(arg: Float, arg2: Float) -> Float {
        30.0
    }
    
    func test_f64_31(arg: Double, arg2: Double) -> Double {
        31.0
    }
    
    func test_u8_1(arg: Int8, arg2: Int8) -> Int8 {
        1
    }
    
    func test_i8_2(arg: Int8, arg2: Int8) -> Int8 {
        2
    }
    
    func test_i16_3(arg: Int32, arg2: Int32) -> Int32 {
        3
    }
    
    func test_u16_4(arg: Int32, arg2: Int32) -> Int32 {
        4
    }
    
    func test_i32_5(arg: Int32, arg2: Int32) -> Int32 {
        5
    }
    
    func test_u32_6(arg: Int32, arg2: Int32) -> Int32 {
        6
    }
    
    func test_bool_false(arg_true: Bool, arg_false: Bool) -> Bool {
        false
    }
    
    func test_arg_vec_str_18(arg: [String]) -> Int32 {
        18
    }
    
    func test_arg_vec_u8_7(arg: [Int8]) -> Int32 {
        7
    }
    
    func test_arg_vec_i8_8(arg: [Int8]) -> Int32 {
        8
    }
    
    func test_arg_vec_i16_9(arg: [Int32]) -> Int32 {
        9
    }
    
    func test_arg_vec_u16_10(arg: [Int32]) -> Int32 {
        10
    }
    
    func test_arg_vec_i32_11(arg: [Int32]) -> Int32 {
        11
    }
    
    func test_arg_vec_u32_12(arg: [Int32]) -> Int32 {
        12
    }
    
    func test_arg_vec_bool_true(arg_true: [Bool]) -> Bool {
        true
    }
    
    func test_arg_vec_struct_17(arg: [DemoStruct]) -> Int32 {
        17
    }
    
    func test_two_vec_arg_13(arg: [Int32], arg1: [Int32]) -> Int32 {
        13
    }
    
    func test_arg_struct_14(arg: DemoStruct) -> Int32 {
        14
    }
    
    func test_two_arg_struct_15(arg: DemoStruct, arg1: DemoStruct) -> Int32 {
        15
    }
    
    func test_no_return() {
        
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        demoTrait.setup()
        demoTrait.test_arg_callback_16(arg: self)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}
