package com.afoxer.rsbind;

import org.scijava.nativelib.NativeLoader;
import java.io.IOException;

public class Common {
    public static void loadLibrary(String libName) {
        try {
            NativeLoader.loadLibrary(libName);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}