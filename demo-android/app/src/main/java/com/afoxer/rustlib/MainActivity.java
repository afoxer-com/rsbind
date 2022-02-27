package com.afoxer.rustlib;

import android.app.Activity;
import android.os.Bundle;
import android.util.Log;

import com.afoxer.xxx.ffi.*;


public class MainActivity extends Activity {
    private static final String TAG = "MainActivity";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        DemoTrait.init();
        DemoTrait.test_arg_callback_16(new DemoCallback() {
            @Override
            public byte test_u8_1(byte arg, byte arg2) {
                Log.e(TAG, "test_u8_1 -> arg: " + arg + " arg2: " + arg2);
                return 1;
            }

            @Override
            public byte test_i8_2(byte arg, byte arg2) {
                Log.e(TAG, "test_i8_2 -> arg: " + arg + " arg2: " + arg2);
                return 2;
            }

            @Override
            public int test_i16_3(int arg, int arg2) {
                Log.e(TAG, "test_i16_3 -> arg: " + arg + " arg2: " + arg2);
                return 3;
            }

            @Override
            public int test_u16_4(int arg, int arg2) {
                Log.e(TAG, "test_u16_4 -> arg: " + arg + " arg2: " + arg2);
                return 4;
            }

            @Override
            public int test_i32_5(int arg, int arg2) {
                Log.e(TAG, "test_i32_5 -> arg: " + arg + " arg2: " + arg2);
                return 5;
            }

            @Override
            public int test_u32_6(int arg, int arg2) {
                Log.e(TAG, "test_u32_6 -> arg: " + arg + " arg2: " + arg2);
                return 6;
            }

            @Override
            public boolean test_bool_false(boolean arg_true, boolean arg_false) {
                Log.e(TAG, "test_bool_false -> arg_true: " + arg_true + " arg_false: " + arg_false);
                return false;
            }

            @Override
            public int test_arg_vec_str_18(String[] arg) {
                Log.e(TAG, "test_arg_vec_str_18 -> arg: " + arg[0]);
                return 18;
            }

            @Override
            public int test_arg_vec_u8_7(byte[] arg) {
                Log.e(TAG, "test_arg_vec_u8_7 -> arg: " + arg[0]);
                return 7;
            }

            @Override
            public int test_arg_vec_i8_8(byte[] arg) {
                Log.e(TAG, "test_arg_vec_i8_8 -> arg: " + arg[0]);
                return 8;
            }

            @Override
            public int test_arg_vec_i16_9(Integer[] arg) {
                Log.e(TAG, "test_arg_vec_i8_8 -> arg: " + arg[0]);
                return 9;
            }

            @Override
            public int test_arg_vec_u16_10(Integer[] arg) {
                Log.e(TAG, "test_arg_vec_u16_10 -> arg: " + arg[0]);
                return 10;
            }

            @Override
            public int test_arg_vec_i32_11(Integer[] arg) {
                Log.e(TAG, "test_arg_vec_i32_11 -> arg: " + arg[0]);
                return 11;
            }

            @Override
            public int test_arg_vec_u32_12(Integer[] arg) {
                Log.e(TAG, "test_arg_vec_u32_12 -> arg: " + arg[0]);
                return 12;
            }

            @Override
            public boolean test_arg_vec_bool_true(Boolean[] arg_true) {
                Log.e(TAG, "test_arg_vec_bool_true -> arg_true: " + arg_true[0]);
                return true;
            }

            @Override
            public int test_arg_vec_struct_17(DemoStruct[] arg) {
                Log.e(TAG, "test_arg_vec_struct_17 -> arg: " + arg[0]);
                return 17;
            }

            @Override
            public int test_two_vec_arg_13(Integer[] arg, Integer[] arg1) {
                Log.e(TAG, "test_two_vec_arg_13 -> arg: " + arg[0]);
                return 13;
            }

            @Override
            public int test_arg_struct_14(DemoStruct arg) {
                Log.e(TAG, "test_arg_struct_14 -> arg: " + arg);
                return 14;
            }

            @Override
            public int test_two_arg_struct_15(DemoStruct arg, DemoStruct arg1) {
                Log.e(TAG, "test_two_arg_struct_15 -> arg: " + arg + " arg1: " + arg1);
                return 15;
            }

            @Override
            public void test_no_return() {
                Log.e(TAG, "test_no_return");
            }
        });
    }
}
