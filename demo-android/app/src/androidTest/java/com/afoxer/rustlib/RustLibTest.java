package com.afoxer.rustlib;

import android.util.Log;

import androidx.test.filters.LargeTest;
import androidx.test.runner.AndroidJUnit4;

import com.afoxer.xxx.ffi.DemoCallback;
import com.afoxer.xxx.ffi.DemoStruct;
import com.afoxer.xxx.ffi.DemoTrait;

import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

@RunWith(AndroidJUnit4.class)
@LargeTest
public class RustLibTest {
    @Before
    public void setup() {
        DemoTrait.init();
    }

    @Test
    public void testRustLibBase() {
        Assert.assertEquals(DemoTrait.test_u8_1((byte) 100, (byte) 101), 1);
        Assert.assertEquals(DemoTrait.test_i8_2((byte) 100, (byte) 101), 2);
        Assert.assertEquals(DemoTrait.test_i16_3(100, 101), 3);
        Assert.assertEquals(DemoTrait.test_u16_4(100, 101), 4);
        Assert.assertEquals(DemoTrait.test_i32_5(100, 101), 5);
        Assert.assertEquals(DemoTrait.test_u32_6(100, 101), 6);
        Assert.assertTrue(DemoTrait.test_f32_30(100.0f, 101.0f) > 29.0);
        Assert.assertTrue(DemoTrait.test_f64_31(100.0, 101.0) > 30.0);
        Assert.assertEquals(DemoTrait.test_bool_false(true, false), false);
        DemoTrait.test_no_return();
    }

    @Test
    public void testRustLibString() {
        Assert.assertEquals(DemoTrait.test_str("Hello world"), "Hello world");
    }

    @Test
    public void testRustLibArray() {
        Assert.assertEquals(DemoTrait.test_arg_vec_str_7(new String[]{"Hello world"}), 7);
        Assert.assertEquals(DemoTrait.test_arg_vec_u8_true(new byte[]{(byte) 100}), true);
        Assert.assertEquals(DemoTrait.test_arg_vec_i16_9(new Integer[]{100}), 9);

        Assert.assertEquals(DemoTrait.test_arg_vec_u16_10(new Integer[]{100}), 10);
        Assert.assertEquals(DemoTrait.test_arg_vec_i32_11(new Integer[]{100}), 11);
        Assert.assertEquals(DemoTrait.test_arg_vec_u32_12(new Integer[]{100}), 12);

        Assert.assertEquals(DemoTrait.test_arg_vec_bool_13(new Boolean[]{true}), 13);
        Assert.assertEquals(DemoTrait.test_two_vec_arg_15(new Integer[]{100}, new Integer[]{101}), 15);
    }

    @Test
    public void testRustLibReturnArray() {
        Assert.assertArrayEquals(DemoTrait.test_return_vec_str(), new String[]{"Hello world"});
        Assert.assertArrayEquals(DemoTrait.test_return_vec_u8(), new byte[]{100});
        Assert.assertArrayEquals(DemoTrait.test_return_vec_i8(), new byte[]{100});
        Assert.assertArrayEquals(DemoTrait.test_return_vec_i16(), new Integer[]{100});
        Assert.assertArrayEquals(DemoTrait.test_return_vec_u16(), new Integer[]{100});
        Assert.assertArrayEquals(DemoTrait.test_return_vec_i32(), new Integer[]{100});
        Assert.assertArrayEquals(DemoTrait.test_return_vec_u32(), new Integer[]{100});
        Assert.assertArrayEquals(DemoTrait.test_return_vec_bool_true(), new Boolean[]{true});
        Assert.assertArrayEquals(DemoTrait.test_two_vec_u8(new byte[]{(byte)100}), new byte[]{100});
    }

    public void testRustLibStruct() {
        DemoStruct demoStruct = DemoTrait.test_return_struct();
        assertStruct(demoStruct);
        DemoTrait.test_arg_struct(demoStruct);
        int result = DemoTrait.test_arg_vec_struct_14(new DemoStruct[]{demoStruct});
        Assert.assertEquals(result, 14);
    }

    @Test
    public void testRustLibCallback() {
        int result = DemoTrait.test_arg_callback_16(createAssertCallback());
        Assert.assertEquals(result, 16);
        int result2 = DemoTrait.test_two_arg_callback_20(createAssertCallback(), createAssertCallback());
        Assert.assertEquals(result2, 20);
    }

    private DemoCallback createAssertCallback() {
        return new DemoCallback() {
            @Override
            public byte test_u8_1(byte arg, byte arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 1;
            }

            @Override
            public byte test_i8_2(byte arg, byte arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 2;
            }

            @Override
            public int test_i16_3(int arg, int arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 3;
            }

            @Override
            public int test_u16_4(int arg, int arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 4;
            }

            @Override
            public int test_i32_5(int arg, int arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 5;
            }

            @Override
            public int test_u32_6(int arg, int arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 6;
            }

            @Override
            public boolean test_bool_false(boolean arg_true, boolean arg_false) {
                Assert.assertEquals(arg_true, true);
                Assert.assertEquals(arg_false, false);
                return false;
            }

            @Override
            public float test_f32_30(float arg, float arg2) {
                Log.e("MainActivity", "arg = " + arg + ", arg2 = " + arg2);
                Assert.assertTrue("we assert arg > 99.0", arg > 99.0f);
                Assert.assertTrue("we assert arg2 > 100.0", arg2 > 100.0f);
                return 30.0f;
            }

            @Override
            public double test_f64_31(double arg, double arg2) {
                Assert.assertTrue(arg > 99.0);
                Assert.assertTrue(arg2 >100.0);
                return 31.0;
            }

            @Override
            public int test_arg_vec_str_18(String[] arg) {
                Assert.assertArrayEquals(arg, new String[]{"Hello world"});
                return 18;
            }

            @Override
            public int test_arg_vec_u8_7(byte[] arg) {
                Assert.assertArrayEquals(arg, new byte[]{100});
                return 7;
            }

            @Override
            public int test_arg_vec_i8_8(byte[] arg) {
                Assert.assertArrayEquals(arg, new byte[]{100});
                return 8;
            }

            @Override
            public int test_arg_vec_i16_9(Integer[] arg) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                return 9;
            }

            @Override
            public int test_arg_vec_u16_10(Integer[] arg) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                return 10;
            }

            @Override
            public int test_arg_vec_i32_11(Integer[] arg) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                return 11;
            }

            @Override
            public int test_arg_vec_u32_12(Integer[] arg) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                return 12;
            }

            @Override
            public boolean test_arg_vec_bool_true(Boolean[] arg_true) {
                Assert.assertArrayEquals(arg_true, new Boolean[]{true});
                return true;
            }

            @Override
            public int test_arg_vec_struct_17(DemoStruct[] arg) {
                assertStruct(arg[0]);
                return 17;
            }

            @Override
            public int test_two_vec_arg_13(Integer[] arg, Integer[] arg1) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                Assert.assertArrayEquals(arg1, new Integer[]{101});
                return 13;
            }

            @Override
            public int test_arg_struct_14(DemoStruct arg) {
                assertStruct(arg);
                return 14;
            }

            @Override
            public int test_two_arg_struct_15(DemoStruct arg, DemoStruct arg1) {
                assertStruct(arg);
                assertStruct(arg1);
                return 15;
            }

            @Override
            public void test_no_return() {

            }
        };
    }

    private void assertStruct(DemoStruct demoStruct) {
        Assert.assertEquals(demoStruct.arg1, 1);
        Assert.assertEquals(demoStruct.arg2, 2);
        Assert.assertEquals(demoStruct.arg3, 3);
        Assert.assertEquals(demoStruct.arg4, 4);
        Assert.assertEquals(demoStruct.arg5, 5);
        Assert.assertEquals(demoStruct.arg6, 6);
        Assert.assertEquals(demoStruct.arg7_str, "Hello world");
        Assert.assertEquals(demoStruct.arg8_false, false);
        Assert.assertTrue(demoStruct.arg9 > 0);
        Assert.assertTrue(demoStruct.arg10 > 0);
    }
}
