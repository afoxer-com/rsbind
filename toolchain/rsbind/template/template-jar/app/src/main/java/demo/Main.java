package demo;

import com.afoxer.xxx.ffi.DemoTrait;

public class Main {
    public static void main(String[] args) {
        DemoTrait.init();
        DemoTrait.test_u8_1((byte) 100, (byte) 101);
    }
}
