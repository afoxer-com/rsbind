use contract::test_contract1::Callback;
use contract::test_contract1::StructSimple;
use contract::test_contract1::TestContract1;

pub struct TestContract1Imp {}

impl TestContract1 for TestContract1Imp {
    fn test_arg_vec(arg: Vec<String>) -> i32 {
        return arg.len() as i32;
    }

    fn test_return_vec(_arg: u8) -> Vec<i32> {
        return [22, 44, 333].to_vec();
    }

    fn test_arg_callback(arg: Box<Callback>) -> u8 {
        arg.on_callback(1122i32, "helllllo".to_owned(), false, 2.333, 4.3333);
        arg.on_callback_complex(StructSimple {
            arg1: 0,
            arg2: 1,
            arg3: "2".to_string(),
            arg4: false,
            arg5: 4.0,
            art6: 5.0,
        });
        arg.on_callback_arg_vec(vec![StructSimple {
            arg1: 9,
            arg2: 8,
            arg3: "7".to_string(),
            arg4: true,
            arg5: 6.0,
            art6: 5.0,
        }]);
        arg.on_callback_arg_vec_simple(vec!["Helllo vec simple".to_owned(), "d".to_owned()]);
        33u8
    }

    fn test_bool(_arg1: bool) -> bool {
        true
    }

    fn test_struct() -> StructSimple {
        StructSimple {
            arg1: 0,
            arg2: 1,
            arg3: "2".to_string(),
            arg4: true,
            arg5: 3.0,
            art6: 4.0,
        }
    }

    fn test_struct_vec() -> Vec<StructSimple> {
        vec![
            StructSimple {
                arg1: 0,
                arg2: 1,
                arg3: "2".to_string(),
                arg4: true,
                arg5: 3.0,
                art6: 4.0,
            },
            StructSimple {
                arg1: 02,
                arg2: 14,
                arg3: "dd".to_string(),
                arg4: false,
                arg5: 3.3,
                art6: 4.20,
            },
        ]
    }

    fn test_two_string(arg1: String, arg2: String) -> String {
        "hhhhh".to_owned()
    }

    fn test_return_vec_u8(input: Vec<u8>) -> Vec<u8> {
        return vec![3, 4, 5];
    }

    fn test_no_return() {
        
    }

    //    fn test_return_callback(arg: bool) -> Box<Callback> {
    //
    //    }
}
