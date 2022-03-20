package demo;

import com.afoxer.xxx.ffi.DemoCallback;
import com.afoxer.xxx.ffi.DemoStruct;
import com.afoxer.xxx.ffi.DemoTrait;
import com.afoxer.xxx.ffi.RustLib;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;


public class RustLibTest {
    private static DemoTrait demoTrait;

    @BeforeAll
    static void setup() {
        demoTrait = RustLib.newDemoTrait();
        demoTrait.setup();
    }

    @Test
    public void testRustLibBase() {
        Assertions.assertEquals(demoTrait.testU81((byte) 100, (byte) 101), 1);
        Assertions.assertEquals(demoTrait.testI82((byte) 100, (byte) 101), 2);
        Assertions.assertEquals(demoTrait.testI163((short) 100, (short) 101), (short)3);
        Assertions.assertEquals(demoTrait.testU164((short) 100, (short) 101), (short)4);
        Assertions.assertEquals(demoTrait.testI325(100, 101), 5);
        Assertions.assertEquals(demoTrait.testU326(100, 101), 6);
        Assertions.assertTrue(demoTrait.testF3230(100.0f, 101.0f) > 29.0);
        Assertions.assertTrue(demoTrait.testF6431(100.0, 101.0) > 30.0);
        Assertions.assertEquals(demoTrait.testBoolFalse(true, false), false);
        demoTrait.testNoReturn();
    }

    @Test
    public void testRustLibString() {
        Assertions.assertEquals(demoTrait.testStr("Hello world"), "Hello world");
    }

    @Test
    public void testRustLibArray() {
        Assertions.assertEquals(demoTrait.testArgVecStr7(new String[]{"Hello world"}), 7);
        Assertions.assertEquals(demoTrait.testArgVecU8True(new byte[]{(byte) 100}), true);
        Assertions.assertEquals(demoTrait.testArgVecI169(new Short[]{(short)100}), 9);

        Assertions.assertEquals(demoTrait.testArgVecU1610(new Short[]{100}), 10);
        Assertions.assertEquals(demoTrait.testArgVecI3211(new Integer[]{100}), 11);
        Assertions.assertEquals(demoTrait.testArgVecU3212(new Integer[]{100}), 12);

        Assertions.assertEquals(demoTrait.testArgVecBool13(new Boolean[]{true}), 13);
        Assertions.assertEquals(demoTrait.testTwoVecArg15(new Integer[]{100}, new Integer[]{101}), 15);
    }

    @Test
    public void testRustLibReturnArray() {
        Assertions.assertArrayEquals(demoTrait.testReturnVecStr(), new String[]{"Hello world"});
        Assertions.assertArrayEquals(demoTrait.testReturnVecU8(), new byte[]{100});
        Assertions.assertArrayEquals(demoTrait.testReturnVecI8(), new byte[]{100});
        Assertions.assertArrayEquals(demoTrait.testReturnVecI16(), new Short[]{100});
        Assertions.assertArrayEquals(demoTrait.testReturnVecU16(), new Short[]{100});
        Assertions.assertArrayEquals(demoTrait.testReturnVecI32(), new Integer[]{100});
        Assertions.assertArrayEquals(demoTrait.testReturnVecU32(), new Integer[]{100});
        Assertions.assertArrayEquals(demoTrait.testReturnVecBoolTrue(), new Boolean[]{true});
        Assertions.assertArrayEquals(demoTrait.testTwoVecU8(new byte[]{(byte)100}), new byte[]{100});
    }

    @Test
    public void testRustLibStruct() {
        DemoStruct demoStruct = demoTrait.testReturnStruct();
        assertStruct(demoStruct);
        demoTrait.testArgStruct(demoStruct);
        int result = demoTrait.testArgVecStruct14(new DemoStruct[]{demoStruct});
        Assertions.assertEquals(result, 14);
    }

    @Test
    public void testRustLibCallback() {
        int result = demoTrait.testArgCallback16(createAssertCallback());
        Assertions.assertEquals(result, 16);
        int result2 = demoTrait.testTwoArgCallback20(createAssertCallback(), createAssertCallback());
        Assertions.assertEquals(result2, 20);
    }

    private DemoCallback createAssertCallback() {
        return new DemoCallback() {
            @Override
            public byte testU81(byte arg, byte arg2) {
                Assertions.assertEquals(arg, 100);
                Assertions.assertEquals(arg2, 101);
                return 1;
            }

            @Override
            public byte testI82(byte arg, byte arg2) {
                Assertions.assertEquals(arg, 100);
                Assertions.assertEquals(arg2, 101);
                return 2;
            }

            @Override
            public short testI163(short arg, short arg2) {
                Assertions.assertEquals(arg, 100);
                Assertions.assertEquals(arg2, 101);
                return 3;
            }

            @Override
            public short testU164(short arg, short arg2) {
                Assertions.assertEquals(arg, 100);
                Assertions.assertEquals(arg2, 101);
                return 4;
            }

            @Override
            public int testI325(int arg, int arg2) {
                Assertions.assertEquals(arg, 100);
                Assertions.assertEquals(arg2, 101);
                return 5;
            }

            @Override
            public int testU326(int arg, int arg2) {
                Assertions.assertEquals(arg, 100);
                Assertions.assertEquals(arg2, 101);
                return 6;
            }

            @Override
            public boolean testBoolFalse(boolean argTrue, boolean argFalse) {
                Assertions.assertEquals(argTrue, true);
                Assertions.assertEquals(argFalse, false);
                return false;
            }

            @Override
            public float testF3230(float arg, float arg2) {
                Assertions.assertTrue( arg > 99.0f, "we Assertions arg > 99.0");
                Assertions.assertTrue(arg2 > 100.0f, "we Assertions arg2 > 100.0");
                return 30.0f;
            }

            @Override
            public double testF6431(double arg, double arg2) {
                Assertions.assertTrue(arg > 99.0);
                Assertions.assertTrue(arg2 >100.0);
                return 31.0;
            }

            @Override
            public long testI647(long arg, long arg2) {
                Assertions.assertEquals(arg, 100);
                Assertions.assertEquals(arg2, 101);
                return 7;            }

            @Override
            public long testU647(long arg, long arg2) {
                Assertions.assertEquals(arg, 100);
                Assertions.assertEquals(arg2, 101);
                return 7;
            }

            @Override
            public String testStr(String arg) {
                Assertions.assertEquals(arg, "Hello world");
                return "Hello world";
            }

            @Override
            public int testArgVecStr18(String[] arg) {
                Assertions.assertArrayEquals(arg, new String[]{"Hello world"});
                return 18;
            }

            @Override
            public int testArgVecU87(byte[] arg) {
                Assertions.assertArrayEquals(arg, new byte[]{100});
                return 7;
            }

            @Override
            public int testArgVecI88(byte[] arg) {
                Assertions.assertArrayEquals(arg, new byte[]{100});
                return 8;
            }

            @Override
            public int testArgVecI169(Short[] arg) {
                Assertions.assertArrayEquals(arg, new Short[]{100});
                return 9;
            }

            @Override
            public int testArgVecU1610(Short[] arg) {
                Assertions.assertArrayEquals(arg, new Short[]{100});
                return 10;
            }

            @Override
            public int testArgVecI3211(Integer[] arg) {
                Assertions.assertArrayEquals(arg, new Integer[]{100});
                return 11;
            }

            @Override
            public int testArgVecU3212(Integer[] arg) {
                Assertions.assertArrayEquals(arg, new Integer[]{100});
                return 12;
            }

            @Override
            public long testArgVecI6411(Long[] arg) {
                Assertions.assertArrayEquals(arg, new Long[]{100L});
                return 11;
            }

            @Override
            public long testArgVecU6412(Long[] arg) {
                Assertions.assertArrayEquals(arg, new Long[]{100L});
                return 12;            }

            @Override
            public boolean testArgVecBoolTrue(Boolean[] argTrue) {
                Assertions.assertArrayEquals(argTrue, new Boolean[]{true});
                return true;
            }

            @Override
            public int testArgVecStruct17(DemoStruct[] arg) {
                assertStruct(arg[0]);
                return 17;
            }

            @Override
            public int testTwoVecArg13(Integer[] arg, Integer[] arg1) {
                Assertions.assertArrayEquals(arg, new Integer[]{100});
                Assertions.assertArrayEquals(arg1, new Integer[]{101});
                return 13;
            }

            @Override
            public byte[] testReturnVecU8() {
                return new byte[]{100};
            }

            @Override
            public byte[] testReturnVecI8() {
                return new byte[]{100};
            }

            @Override
            public Short[] testReturnVecI16() {
                return new Short[]{100};
            }

            @Override
            public Short[] testReturnVecU16() {
                return new Short[]{100};
            }

            @Override
            public Integer[] testReturnVecI32() {
                return new Integer[]{100};
            }

            @Override
            public Integer[] testReturnVecU32() {
                return new Integer[]{100};
            }

            @Override
            public Long[] testReturnVecI64() {
                return new Long[]{100L};
            }

            @Override
            public Long[] testReturnVecU64() {
                return new Long[]{100L};
            }

            @Override
            public byte[] testTwoVecU8(byte[] input) {
                return new byte[]{100};
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
        Assertions.assertEquals(demoStruct.arg1, 1);
        Assertions.assertEquals(demoStruct.arg2, 2);
        Assertions.assertEquals(demoStruct.arg3, 3);
        Assertions.assertEquals(demoStruct.arg4, 4);
        Assertions.assertEquals(demoStruct.arg5, 5);
        Assertions.assertEquals(demoStruct.arg6, 6);
        Assertions.assertEquals(demoStruct.arg7_str, "Hello world");
        Assertions.assertEquals(demoStruct.arg8_false, false);
        Assertions.assertTrue(demoStruct.arg9 > 0);
        Assertions.assertTrue(demoStruct.arg10 > 0);
    }
}
