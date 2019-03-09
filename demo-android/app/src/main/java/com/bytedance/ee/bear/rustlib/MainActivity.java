package com.bytedance.ee.bear.rustlib;

import android.os.Bundle;
import android.support.v7.app.AppCompatActivity;
import android.util.Log;

import com.bytedance.ee.bear.ffi.Callback;
import com.bytedance.ee.bear.ffi.StructSimple;
import com.bytedance.ee.bear.ffi.TestContract1;

import java.util.Arrays;

public class MainActivity extends AppCompatActivity {
    private static final String TAG = "MainActivity";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        StructSimple[] structSimple = TestContract1.test_struct_vec();
        Log.i(TAG, "xxxxx onCreate: result = " + structSimple[1].arg3);

        TestContract1.test_arg_callback(new Callback() {
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
        });
    }
}
