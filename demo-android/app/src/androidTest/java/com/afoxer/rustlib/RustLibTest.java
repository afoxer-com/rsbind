package com.afoxer.rustlib;

import android.util.Log;

import androidx.test.filters.LargeTest;
import androidx.test.runner.AndroidJUnit4;

import com.afoxer.xxx.ffi.*;

import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

@RunWith(AndroidJUnit4.class)
@LargeTest
public class RustLibTest {
    private static DemoTrait demoTrait;

    @Before
    public void setup() {
        demoTrait = RustLib.newDemoTrait();
        demoTrait.init();
    }

    @Test
    public void testRustLibBase() {
        Assert.assertEquals(demoTrait.testU81((byte) 100, (byte) 101), 1);
        Assert.assertEquals(demoTrait.testI82((byte) 100, (byte) 101), 2);
        Assert.assertEquals(demoTrait.testI163(100, 101), 3);
        Assert.assertEquals(demoTrait.testU164(100, 101), 4);
        Assert.assertEquals(demoTrait.testI325(100, 101), 5);
        Assert.assertEquals(demoTrait.testU326(100, 101), 6);
        Assert.assertTrue(demoTrait.testF3230(100.0f, 101.0f) > 29.0);
        Assert.assertTrue(demoTrait.testF6431(100.0, 101.0) > 30.0);
        Assert.assertEquals(demoTrait.testBoolFalse(true, false), false);
        demoTrait.testNoReturn();
    }

    @Test
    public void testRustLibString() {
        Assert.assertEquals(demoTrait.testStr("Hello world"), "Hello world");
    }

    @Test
    public void testRustLibArray() {
        Assert.assertEquals(demoTrait.testArgVecStr7(new String[]{"Hello world"}), 7);
        Assert.assertEquals(demoTrait.testArgVecU8True(new byte[]{(byte) 100}), true);
        Assert.assertEquals(demoTrait.testArgVecI169(new Integer[]{100}), 9);

        Assert.assertEquals(demoTrait.testArgVecU1610(new Integer[]{100}), 10);
        Assert.assertEquals(demoTrait.testArgVecI3211(new Integer[]{100}), 11);
        Assert.assertEquals(demoTrait.testArgVecU3212(new Integer[]{100}), 12);

        Assert.assertEquals(demoTrait.testArgVecBool13(new Boolean[]{true}), 13);
        Assert.assertEquals(demoTrait.testTwoVecArg15(new Integer[]{100}, new Integer[]{101}), 15);
    }

    @Test
    public void testRustLibReturnArray() {
        Assert.assertArrayEquals(demoTrait.testReturnVecStr(), new String[]{"Hello world"});
        Assert.assertArrayEquals(demoTrait.testReturnVecU8(), new byte[]{100});
        Assert.assertArrayEquals(demoTrait.testReturnVecI8(), new byte[]{100});
        Assert.assertArrayEquals(demoTrait.testReturnVecI16(), new Integer[]{100});
        Assert.assertArrayEquals(demoTrait.testReturnVecU16(), new Integer[]{100});
        Assert.assertArrayEquals(demoTrait.testReturnVecI32(), new Integer[]{100});
        Assert.assertArrayEquals(demoTrait.testReturnVecU32(), new Integer[]{100});
        Assert.assertArrayEquals(demoTrait.testReturnVecBoolTrue(), new Boolean[]{true});
        Assert.assertArrayEquals(demoTrait.testTwoVecU8(new byte[]{(byte)100}), new byte[]{100});
    }

    public void testRustLibStruct() {
        DemoStruct demoStruct = demoTrait.testReturnStruct();
        assertStruct(demoStruct);
        demoTrait.testArgStruct(demoStruct);
        int result = demoTrait.testArgVecStruct14(new DemoStruct[]{demoStruct});
        Assert.assertEquals(result, 14);
    }

    @Test
    public void testRustLibCallback() {
        int result = demoTrait.testArgCallback16(createAssertCallback());
        Assert.assertEquals(result, 16);
        int result2 = demoTrait.testTwoArgCallback20(createAssertCallback(), createAssertCallback());
        Assert.assertEquals(result2, 20);
    }

    private DemoCallback createAssertCallback() {
        return new DemoCallback() {
            @Override
            public byte testU81(byte arg, byte arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 1;
            }

            @Override
            public byte testI82(byte arg, byte arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 2;
            }

            @Override
            public int testI163(int arg, int arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 3;
            }

            @Override
            public int testU164(int arg, int arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 4;
            }

            @Override
            public int testI325(int arg, int arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 5;
            }

            @Override
            public int testU326(int arg, int arg2) {
                Assert.assertEquals(arg, 100);
                Assert.assertEquals(arg2, 101);
                return 6;
            }

            @Override
            public boolean testBoolFalse(boolean argTrue, boolean argFalse) {
                Assert.assertEquals(argTrue, true);
                Assert.assertEquals(argFalse, false);
                return false;
            }

            @Override
            public float testF3230(float arg, float arg2) {
                Log.e("MainActivity", "arg = " + arg + ", arg2 = " + arg2);
                Assert.assertTrue("we assert arg > 99.0", arg > 99.0f);
                Assert.assertTrue("we assert arg2 > 100.0", arg2 > 100.0f);
                return 30.0f;
            }

            @Override
            public double testF6431(double arg, double arg2) {
                Assert.assertTrue(arg > 99.0);
                Assert.assertTrue(arg2 >100.0);
                return 31.0;
            }

            @Override
            public int testArgVecStr18(String[] arg) {
                Assert.assertArrayEquals(arg, new String[]{"Hello world"});
                return 18;
            }

            @Override
            public int testArgVecU87(byte[] arg) {
                Assert.assertArrayEquals(arg, new byte[]{100});
                return 7;
            }

            @Override
            public int testArgVecI88(byte[] arg) {
                Assert.assertArrayEquals(arg, new byte[]{100});
                return 8;
            }

            @Override
            public int testArgVecI169(Integer[] arg) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                return 9;
            }

            @Override
            public int testArgVecU1610(Integer[] arg) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                return 10;
            }

            @Override
            public int testArgVecI3211(Integer[] arg) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                return 11;
            }

            @Override
            public int testArgVecU3212(Integer[] arg) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                return 12;
            }

            @Override
            public boolean testArgVecBoolTrue(Boolean[] argTrue) {
                Assert.assertArrayEquals(argTrue, new Boolean[]{true});
                return true;
            }

            @Override
            public int testArgVecStruct17(DemoStruct[] arg) {
                assertStruct(arg[0]);
                return 17;
            }

            @Override
            public int testTwoVecArg13(Integer[] arg, Integer[] arg1) {
                Assert.assertArrayEquals(arg, new Integer[]{100});
                Assert.assertArrayEquals(arg1, new Integer[]{101});
                return 13;
            }

            @Override
            public int testArgStruct14(DemoStruct arg) {
                assertStruct(arg);
                return 14;
            }

            @Override
            public int testTwoArgStruct15(DemoStruct arg, DemoStruct arg1) {
                assertStruct(arg);
                assertStruct(arg1);
                return 15;
            }

            @Override
            public void testNoReturn() {

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
