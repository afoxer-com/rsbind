//
//  ViewController.swift
//  demo-swift
//
//  Created by sidney.wang on 2018/6/29.
//  Copyright © 2018年 sidney.wang. All rights reserved.
//

import UIKit
import rustlib.Swift

class ViewController: UIViewController, Callback{cv 
    func on_callback_u8(arg1: Int8) -> Int8 {
        print("on_callback_u8 \(arg1)")
        return 55
    }
    
    func on_callback_i8(arg1: Int8) -> Int8 {
        print("on_callback_i8 \(arg1)")
        return 3
    }
    
    func on_empty_callback() {
        print("on_empty_callback")
        }
    
    func on_callback(arg1: Int, arg2: String, arg3: Bool, arg4: Double, arg5: Double) -> Int {
        print("on_callback: \(arg1)")
        return 3
    }
    
    func on_callback2(arg1: Bool) -> Bool {
        print("on_callback2: \(arg1)")
        return true
    }
    
    func on_callback_complex(arg1: StructSimple) -> Bool {
        print("on_callback_complex: \(arg1)")
        return true
    }
    
    func on_callback_arg_vec(arg1: Array<StructSimple>) -> Bool {
        print("on_callback_arg_vec: \(arg1)")
        return true
    }
    
    func on_callback_arg_vec_simple(arg1: Array<String>) -> Bool {
        print("on_callback_arg_vec_simple: \(arg1)")
        return true
    }
    
    func update_progress(key: String, status: Int, bytes_transferred: Int64, bytes_total: Int64) -> Bool {
        print("update process \(key), \(status), \(bytes_transferred), \(bytes_total)")
        return true
    }
    

    override func viewDidLoad() {
        super.viewDidLoad()
        
        let result = TestContract1.test_struct_vec()
        print("result = \(result)")
        // Do any additional setup after loading the view, typically from a nib.
        
        TestContract1.test_arg_callback(arg: self)
        let struct_imp = TestContract1.test_struct();
        print("struct = \(struct_imp)")
        
        let struct_vec = TestContract1.test_struct_vec()
        print("struct_vec = \(struct_imp)")
        
        let return_vec = TestContract1.test_return_vec(arg: 43)
        print("return_vec = \(return_vec)")
        
        let byte_result = TestContract1.test_byte(arg: 44)
        print("byte result = \(byte_result)")
        
        let i8_result = TestContract1.test_byte_i8(arg: 55)
        print("i8 result = \(i8_result)")

    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}

