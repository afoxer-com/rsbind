package com.bytedance.ee.bear.dddd;

import java.io.Serializable;
import java.lang.String;

public interface Callback extends Serializable {
  int on_callback(int arg1, String arg2, boolean arg3, float arg4, double arg5);

  boolean on_callback2(boolean arg1);

  boolean on_callback_complex(StructSimple arg1);

  boolean on_callback_arg_vec(StructSimple[] arg1);

  boolean on_callback_arg_vec_simple(String[] arg1);
}
