//
//  globalIndex.swift
//
//  Contains:
//  var globalIndex
//  var globalCallbacks
//  protocol Callback
//  class TestContract1
//  struct StructSimple
//
//  Generated by SwiftPoet on 2022/2/18
//

private var globalIndex: Int64 = 0

private var globalCallbacks: Dictionary<Int64,Any> = [Int64: Any]()

public protocol Callback {


    /**
        :param:    arg1
    */
    func on_callback_u8(arg1: Int8) -> Int8

    /**
        :param:    arg1
    */
    func on_callback_i8(arg1: Int8) -> Int8

    /**
        :param:    arg1

        :param:    arg2

        :param:    arg3

        :param:    arg4

        :param:    arg5
    */
    func on_callback(arg1: Int, arg2: String, arg3: Bool, arg4: Double, arg5: Double) -> Int

    /**
        :param:    arg1
    */
    func on_callback2(arg1: Bool) -> Bool

    /**
        :param:    arg1
    */
    func on_callback_complex(arg1: StructSimple) -> Bool

    /**
        :param:    arg1
    */
    func on_callback_arg_vec(arg1: Array<StructSimple>) -> Bool

    /**
        :param:    arg1
    */
    func on_callback_arg_vec_simple(arg1: Array<String>) -> Bool

    func on_empty_callback()

}

public class TestContract1 {


    /**
        :param:    arg
    */
    public static func test_byte(arg: Int8) -> Int8 {
        
        let s_arg = Int8(arg)
        let result = test_contract1_test_byte(s_arg)
        let s_result = Int8(result)
        return s_result
    }

    /**
        :param:    arg
    */
    public static func test_byte_i8(arg: Int8) -> Int8 {
        
        let s_arg = Int8(arg)
        let result = test_contract1_test_byte_i8(s_arg)
        let s_result = Int8(result)
        return s_result
    }

    /**
        :param:    arg
    */
    public static func test_arg_vec(arg: Array<String>) -> Int {
        
        let encoder = JSONEncoder()
        let data_arg = try! encoder.encode(arg)
        let s_arg = String(data: data_arg, encoding: .utf8)!
        let result = test_contract1_test_arg_vec(s_arg)
        let s_result = Int(result)
        return s_result
    }

    /**
        :param:    arg
    */
    static public func test_return_vec(arg: Int8) -> Array<Int> {
        
        let s_arg = Int8(arg)
        let result = test_contract1_test_return_vec(s_arg)
        let ret_str = String(cString:result!)
        demo_free_str(result!)
        var s_tmp_result: [Int]?
        autoreleasepool {
        let ret_str_json = ret_str.data(using: .utf8)!
        let decoder = JSONDecoder()
        s_tmp_result = try! decoder.decode([Int].self, from: ret_str_json)
        }
        let s_result = s_tmp_result!
        return s_result
    }

    /**
        :param:    arg
    */
    static public func test_arg_callback(arg: Callback) -> Int8 {
        
        let arg_index = globalIndex + 1
        globalIndex = arg_index
        globalCallbacks[arg_index] = arg
        let arg_on_callback_u8 : @convention(c) (Int64, Int8) -> Int8 = { 
        
        (index, arg1) -> Int8 in
        let arg_callback = globalCallbacks[index] as! Callback
        let c_arg1 = Int8(arg1)
        let result = arg_callback.on_callback_u8(arg1:c_arg1)
        return Int8(result)
        }
        let arg_on_callback_i8 : @convention(c) (Int64, Int8) -> Int8 = { 
        
        (index, arg1) -> Int8 in
        let arg_callback = globalCallbacks[index] as! Callback
        let c_arg1 = Int8(arg1)
        let result = arg_callback.on_callback_i8(arg1:c_arg1)
        return Int8(result)
        }
        let arg_on_callback : @convention(c) (Int64, Int32, UnsafePointer<Int8>?, Int32, Float32, Float64) -> Int32 = { 
        
        (index, arg1, arg2, arg3, arg4, arg5) -> Int32 in
        let arg_callback = globalCallbacks[index] as! Callback
        let c_arg1 = Int(arg1)
        let c_arg2 = String(cString: arg2!)
        let c_arg3: Bool = arg3 > 0 ? true : false
        let c_arg4 = Double(arg4)
        let c_arg5 = Double(arg5)
        let result = arg_callback.on_callback(arg1:c_arg1,arg2:c_arg2,arg3:c_arg3,arg4:c_arg4,arg5:c_arg5)
        return Int32(result)
        }
        let arg_on_callback2 : @convention(c) (Int64, Int32) -> Int32 = { 
        
        (index, arg1) -> Int32 in
        let arg_callback = globalCallbacks[index] as! Callback
        let c_arg1: Bool = arg1 > 0 ? true : false
        let result = arg_callback.on_callback2(arg1:c_arg1)
        return result ? 1 : 0
        }
        let arg_on_callback_complex : @convention(c) (Int64, UnsafePointer<Int8>?) -> Int32 = { 
        
        (index, arg1) -> Int32 in
        let arg_callback = globalCallbacks[index] as! Callback
        let c_tmp_arg1 = String(cString:arg1!)
        var c_option_arg1: StructSimple?
        autoreleasepool {
        let c_tmp_json_arg1 = c_tmp_arg1.data(using: .utf8)!
        let decoder = JSONDecoder()
        c_option_arg1 = try! decoder.decode(StructSimple.self, from: c_tmp_json_arg1)
        }
        let c_arg1 = c_option_arg1!
        let result = arg_callback.on_callback_complex(arg1:c_arg1)
        return result ? 1 : 0
        }
        let arg_on_callback_arg_vec : @convention(c) (Int64, UnsafePointer<Int8>?) -> Int32 = { 
        
        (index, arg1) -> Int32 in
        let arg_callback = globalCallbacks[index] as! Callback
        let c_tmp_arg1 = String(cString:arg1!)
        var c_option_arg1: Array<StructSimple>?
        autoreleasepool {
        let c_tmp_json_arg1 = c_tmp_arg1.data(using: .utf8)!
        let decoder = JSONDecoder()
        c_option_arg1 = try! decoder.decode(Array<StructSimple>.self, from: c_tmp_json_arg1)
        }
        let c_arg1 = c_option_arg1!
        let result = arg_callback.on_callback_arg_vec(arg1:c_arg1)
        return result ? 1 : 0
        }
        let arg_on_callback_arg_vec_simple : @convention(c) (Int64, UnsafePointer<Int8>?) -> Int32 = { 
        
        (index, arg1) -> Int32 in
        let arg_callback = globalCallbacks[index] as! Callback
        let c_tmp_arg1 = String(cString:arg1!)
        var c_option_arg1: [String]?
        autoreleasepool {
        let c_tmp_json_arg1 = c_tmp_arg1.data(using: .utf8)!
        let decoder = JSONDecoder()
        c_option_arg1 = try! decoder.decode([String].self, from: c_tmp_json_arg1)
        }
        let c_arg1 = c_option_arg1!
        let result = arg_callback.on_callback_arg_vec_simple(arg1:c_arg1)
        return result ? 1 : 0
        }
        let arg_on_empty_callback : @convention(c) (Int64) -> () = { 
        
        (index) -> () in
        let arg_callback = globalCallbacks[index] as! Callback
        arg_callback.on_empty_callback()
        }
        let callback_free : @convention(c)(Int64) -> () = {
        (index) in
        globalCallbacks.removeValue(forKey: index)
        }
        let s_arg = test_contract1_Callback_Model(on_callback_u8:arg_on_callback_u8,on_callback_i8:arg_on_callback_i8,on_callback:arg_on_callback,on_callback2:arg_on_callback2,on_callback_complex:arg_on_callback_complex,on_callback_arg_vec:arg_on_callback_arg_vec,on_callback_arg_vec_simple:arg_on_callback_arg_vec_simple,on_empty_callback:arg_on_empty_callback,free_callback: callback_free, index: arg_index)
        let result = test_contract1_test_arg_callback(s_arg)
        let s_result = Int8(result)
        return s_result
    }

    /**
        :param:    arg1
    */
    public static func test_bool(arg1: Bool) -> Bool {
        
        let s_arg1: Int32 = arg1 ? 1 : 0
        let result = test_contract1_test_bool(s_arg1)
        let s_result = result > 0 ? true : false
        return s_result
    }

    public static func test_struct() -> StructSimple {
        
        let result = test_contract1_test_struct()
        let ret_str = String(cString:result!)
        demo_free_str(result!)
        var s_tmp_result: StructSimple?
        autoreleasepool {
        let ret_str_json = ret_str.data(using: .utf8)!
        let decoder = JSONDecoder()
        s_tmp_result = try! decoder.decode(StructSimple.self, from: ret_str_json)
        }
        let s_result = s_tmp_result!
        return s_result
    }

    public static func test_struct_vec() -> Array<StructSimple> {
        
        let result = test_contract1_test_struct_vec()
        let ret_str = String(cString:result!)
        demo_free_str(result!)
        var s_tmp_result: Array<StructSimple>?
        autoreleasepool {
        let ret_str_json = ret_str.data(using: .utf8)!
        let decoder = JSONDecoder()
        s_tmp_result = try! decoder.decode(Array<StructSimple>.self, from: ret_str_json)
        }
        let s_result = s_tmp_result!
        return s_result
    }

    /**
        :param:    arg1

        :param:    arg2
    */
    static public func test_two_string(arg1: String, arg2: String) -> String {
        
        let s_arg1 = arg1
        let s_arg2 = arg2
        let result = test_contract1_test_two_string(s_arg1,s_arg2)
        let s_result = String(cString:result!)
        demo_free_str(result!)
        return s_result
    }

    /**
        :param:    input
    */
    static public func test_return_vec_u8(input: Array<Int8>) -> Array<Int8> {
        
        let encoder = JSONEncoder()
        let data_input = try! encoder.encode(input)
        let s_input = String(data: data_input, encoding: .utf8)!
        let result = test_contract1_test_return_vec_u8(s_input)
        let ret_str = String(cString:result!)
        demo_free_str(result!)
        var s_tmp_result: [Int8]?
        autoreleasepool {
        let ret_str_json = ret_str.data(using: .utf8)!
        let decoder = JSONDecoder()
        s_tmp_result = try! decoder.decode([Int8].self, from: ret_str_json)
        }
        let s_result = s_tmp_result!
        return s_result
    }

    /**
        :param:    input
    */
    static public func test_return_vec_i8(input: Array<Int8>) -> Array<Int8> {
        
        let encoder = JSONEncoder()
        let data_input = try! encoder.encode(input)
        let s_input = String(data: data_input, encoding: .utf8)!
        let result = test_contract1_test_return_vec_i8(s_input)
        let ret_str = String(cString:result!)
        demo_free_str(result!)
        var s_tmp_result: [Int8]?
        autoreleasepool {
        let ret_str_json = ret_str.data(using: .utf8)!
        let decoder = JSONDecoder()
        s_tmp_result = try! decoder.decode([Int8].self, from: ret_str_json)
        }
        let s_result = s_tmp_result!
        return s_result
    }

    public static func test_no_return() {
        
        test_contract1_test_no_return()
    }

}

public struct StructSimple: Codable {
    public let arg1: Int
    public let arg2: Int8
    public let arg3: String
    public let arg4: Bool
    public let arg5: Double
    public let art6: Double
}
