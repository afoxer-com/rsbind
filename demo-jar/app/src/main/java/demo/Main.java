package demo;

import com.afoxer.xxx.ffi.DemoTrait;
import com.afoxer.xxx.ffi.RustLib;

public class Main {
    private static DemoTrait demoTrait = RustLib.newDemoTrait();
    public static void main(String[] args) {
        demoTrait.init();
        demoTrait.testU81((byte) 100, (byte) 101);
    }
}
