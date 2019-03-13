package com.bytedance.ee.bear.rustlib;

import android.os.Bundle;
import android.support.v7.app.AppCompatActivity;
import android.util.Log;

import com.afoxer.xxx.ffi.Callback;
import com.afoxer.xxx.ffi.StructSimple;
import com.afoxer.xxx.ffi.TestContract1;

import java.util.Arrays;


public class MainActivity extends AppCompatActivity {
    private static final String TAG = "MainActivity";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        StructSimple[] structSimple = TestContract1.test_struct_vec();
        Log.i(TAG, "xxxxx onCreate: result = " + structSimple[1].arg3);

        byte byteResult = TestContract1.test_byte((byte) 2);
        Log.i(TAG, "onCreate: byte result = " + byteResult);

        byte byteResult2 = TestContract1.test_byte_i8((byte) 3);
        Log.i(TAG, "onCreate: byteResult2 = " + byteResult2);

        TestContract1.test_arg_callback(new Callback() {
            @Override
            public byte on_callback_u8(byte b) {
                Log.i(TAG, "on_callback_u8: " + b);
                return 0;
            }

            @Override
            public byte on_callback_i8(byte b) {
                Log.i(TAG, "on_callback_i8: " + b);
                return 0;
            }

            @Override
            public int on_callback(int i, String s, boolean b, float v, double v1) {
                Log.i(TAG, "xxxxx on_callback: " + i + ", " + s + ", " + b + ", " + v + ", " + v1);
                return 0;
            }

            @Override
            public boolean on_callback2(boolean b) {
                return false;
            }

            public boolean on_callback_complex(StructSimple arg1) {
                Log.i(TAG, "xxxxx on_callback_complex: " + arg1.arg1 + arg1.arg2 + arg1.arg3 + arg1.arg4 + arg1.arg5 + arg1.art6);
                return true;
            }

            public boolean on_callback_arg_vec(StructSimple[] arg1) {
                Log.i(TAG, "xxxxx on_callback_arg_vec: " + arg1[0].arg1 + arg1[0].arg2 + arg1[0].arg3 + arg1[0].arg4 + arg1[0].arg5 + arg1[0].art6);
                return true;
            }

            public boolean on_callback_arg_vec_simple(String[] arg1) {
                Log.i(TAG, "xxxxxx on_callback_arg_vec_simple" + Arrays.toString(arg1));
                return true;
            }

            @Override
            public void on_empty_callback() {
                Log.i(TAG, "on_empty_callback: ");
            }
        });
    }
}
