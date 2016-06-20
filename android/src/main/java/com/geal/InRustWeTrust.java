package com.geal;

import android.util.Log;

public class InRustWeTrust {
    static {
        System.loadLibrary("inrustwetrust");
        Log.d("InRustWeTrust", "LOADING INRUSTWETRUST LIBRARY");

    }
    public static native int add(int v1, int v2);
}
