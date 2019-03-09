package com.bytedance.ee.bear.ffi;

import java.io.Serializable;

public class Common {
    public static class CallbackModel implements Serializable {
        public long index;
        public String class_name;

        public CallbackModel(long index, String class_name) {
            this.index = index;
            this.class_name = class_name;
        }

        public CallbackModel() {
        }

        public long getIndex() {
            return index;
        }

        public void setIndex(long index) {
            this.index = index;
        }

        public String getClass_name() {
            return class_name;
        }

        public void setClass_name(String class_name) {
            this.class_name = class_name;
        }
    }
}