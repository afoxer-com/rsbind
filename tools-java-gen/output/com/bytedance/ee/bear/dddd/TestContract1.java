package com.bytedance.ee.bear.dddd;

import com.alibaba.fastjson.JSON;
import java.io.Serializable;
import java.lang.Integer;
import java.lang.Long;
import java.lang.Object;
import java.lang.String;
import java.lang.Void;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;

public class TestContract1 implements Serializable {
  private static AtomicLong globalIndex = new AtomicLong(0);

  private static ConcurrentHashMap<Long, Object> globalCallbacks = new ConcurrentHashMap<>();

  static {
    System.loadLibrary("lark");
    System.loadLibrary("");
  }

  public static void free_callback(long index) {
    globalCallbacks.remove(index);
  }

  public static int test_arg_vec(String[] arg) {
    String r_arg = JSON.toJSONString(arg);
    int ret = native_test_arg_vec(r_arg);
    return ret;
  }

  public static Integer[] test_return_vec(int arg) {
    int r_arg = arg;
    String ret = native_test_return_vec(r_arg);
    List<Integer> list = JSON.parseArray(ret, Integer.class);
    Integer[] array = new Integer[list.size()];
    return list.toArray(array);
  }

  public static int test_arg_callback(Callback arg) {
    long arg_callback_index = globalIndex.incrementAndGet();
    globalCallbacks.put(arg_callback_index, arg);
    long r_arg = arg_callback_index;
    int ret = native_test_arg_callback(r_arg);
    return ret;
  }

  public static boolean test_bool(boolean arg1) {
    int r_arg1 = arg1 ? 1 : 0;
    int ret = native_test_bool(r_arg1);
    return ret > 0 ? true : false;
  }

  public static StructSimple test_struct() {
    String ret = native_test_struct();
    return JSON.parseObject(ret, StructSimple.class);
  }

  public static StructSimple[] test_struct_vec() {
    String ret = native_test_struct_vec();
    List<StructSimple> list = JSON.parseArray(ret, StructSimple.class);
    StructSimple[] array = new StructSimple[list.size()];
    return list.toArray(array);
  }

  public static int invoke_Callback_on_callback(long index, int arg1, String arg2, int arg3,
      float arg4, double arg5) {
    int j_arg1 = arg1;
    String j_arg2 = arg2;
    boolean j_arg3 = arg3 > 0 ? true : false;
    float j_arg4 = arg4;
    double j_arg5 = arg5;
    Callback callback = (Callback) globalCallbacks.get(index);
    int result = callback.on_callback(j_arg1,j_arg2,j_arg3,j_arg4,j_arg5);
    return result;
  }

  public static int invoke_Callback_on_callback2(long index, int arg1) {
    boolean j_arg1 = arg1 > 0 ? true : false;
    Callback callback = (Callback) globalCallbacks.get(index);
    boolean result = callback.on_callback2(j_arg1);
    return result ? 1 : 0;
  }

  public static int invoke_Callback_on_callback_complex(long index, String arg1) {
    StructSimple j_arg1 = JSON.parseObject(arg1, StructSimple.class);
    Callback callback = (Callback) globalCallbacks.get(index);
    boolean result = callback.on_callback_complex(j_arg1);
    return result ? 1 : 0;
  }

  public static int invoke_Callback_on_callback_arg_vec(long index, String arg1) {
    List<Void> arg1_list = JSON.parseArray(ret, Void.class);
    Void[] arg1_array = new Void[arg1_list.size()];
    Void[] j_arg1 = list.toArray(array);
    Callback callback = (Callback) globalCallbacks.get(index);
    boolean result = callback.on_callback_arg_vec(j_arg1);
    return result ? 1 : 0;
  }

  public static int invoke_Callback_on_callback_arg_vec_simple(long index, String arg1) {
    List<String> arg1_list = JSON.parseArray(ret, String.class);
    String[] arg1_array = new String[arg1_list.size()];
    String[] j_arg1 = list.toArray(array);
    Callback callback = (Callback) globalCallbacks.get(index);
    boolean result = callback.on_callback_arg_vec_simple(j_arg1);
    return result ? 1 : 0;
  }

  private static native int native_test_arg_vec(String arg);

  private static native String native_test_return_vec(int arg);

  private static native int native_test_arg_callback(long arg);

  private static native int native_test_bool(int arg1);

  private static native String native_test_struct();

  private static native String native_test_struct_vec();
}
