//
//  ViewController.swift
//  demo-swift
//
//  Created by wangxin.sidney on 2018/6/29.
//  Copyright © 2018年 bytedance. All rights reserved.
//

import UIKit
import rustlib.Swift

class ViewController: UIViewController, Callback{
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
        
        let return_vec = TestContract1.test_return_vec(arg: 333)
        print("return_vec = \(return_vec)")

    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}

