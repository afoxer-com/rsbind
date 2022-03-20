#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CInt8Array {
  const int8_t *ptr;
  int32_t len;
} CInt8Array;

typedef struct CInt16Array {
  const int16_t *ptr;
  int32_t len;
} CInt16Array;

typedef struct CInt32Array {
  const int32_t *ptr;
  int32_t len;
} CInt32Array;

typedef struct CInt64Array {
  const int64_t *ptr;
  int32_t len;
} CInt64Array;

typedef struct test_contract1_DemoCallback_Model {
  int8_t (*test_u8_1)(int64_t, int8_t, int8_t);
  int8_t (*test_i8_2)(int64_t, int8_t, int8_t);
  int16_t (*test_i16_3)(int64_t, int16_t, int16_t);
  int16_t (*test_u16_4)(int64_t, int16_t, int16_t);
  int32_t (*test_i32_5)(int64_t, int32_t, int32_t);
  int32_t (*test_u32_6)(int64_t, int32_t, int32_t);
  int32_t (*test_bool_false)(int64_t, int32_t, int32_t);
  float (*test_f32_30)(int64_t, float, float);
  double (*test_f64_31)(int64_t, double, double);
  int64_t (*test_i64_7)(int64_t, int64_t, int64_t);
  int64_t (*test_u64_7)(int64_t, int64_t, int64_t);
  const char *(*test_str)(int64_t, const char*);
  int32_t (*test_arg_vec_str_18)(int64_t, const char*);
  int32_t (*test_arg_vec_u8_7)(int64_t, struct CInt8Array);
  int32_t (*test_arg_vec_i8_8)(int64_t, struct CInt8Array);
  int32_t (*test_arg_vec_i16_9)(int64_t, struct CInt16Array);
  int32_t (*test_arg_vec_u16_10)(int64_t, struct CInt16Array);
  int32_t (*test_arg_vec_i32_11)(int64_t, struct CInt32Array);
  int32_t (*test_arg_vec_u32_12)(int64_t, struct CInt32Array);
  int64_t (*test_arg_vec_i64_11)(int64_t, struct CInt64Array);
  int64_t (*test_arg_vec_u64_12)(int64_t, struct CInt64Array);
  int32_t (*test_arg_vec_bool_true)(int64_t, const char*);
  int32_t (*test_arg_vec_struct_17)(int64_t, const char*);
  int32_t (*test_two_vec_arg_13)(int64_t, struct CInt32Array, struct CInt32Array);
  struct CInt8Array (*test_return_vec_u8)(int64_t);
  struct CInt8Array (*test_return_vec_i8)(int64_t);
  struct CInt16Array (*test_return_vec_i16)(int64_t);
  struct CInt16Array (*test_return_vec_u16)(int64_t);
  struct CInt32Array (*test_return_vec_i32)(int64_t);
  struct CInt32Array (*test_return_vec_u32)(int64_t);
  struct CInt64Array (*test_return_vec_i64)(int64_t);
  struct CInt64Array (*test_return_vec_u64)(int64_t);
  struct CInt8Array (*test_two_vec_u8)(int64_t, struct CInt8Array);
  int32_t (*test_arg_struct_14)(int64_t, const char*);
  int32_t (*test_two_arg_struct_15)(int64_t, const char*, const char*);
  void (*test_no_return)(int64_t);
  void (*free_callback)(int64_t);
  void (*free_ptr)(int8_t*, int32_t);
  int64_t index;
} test_contract1_DemoCallback_Model;

void test_contract1_setup(void);

int8_t test_contract1_test_u8_1(int8_t arg, int8_t arg2);

int8_t test_contract1_test_i8_2(int8_t arg, int8_t arg2);

int16_t test_contract1_test_i16_3(int16_t arg, int16_t arg2);

int16_t test_contract1_test_u16_4(int16_t arg, int16_t arg2);

int32_t test_contract1_test_i32_5(int32_t arg, int32_t arg2);

int32_t test_contract1_test_u32_6(int32_t arg, int32_t arg2);

int64_t test_contract1_test_i64_7(int64_t arg, int64_t arg2);

int64_t test_contract1_test_u64_7(int64_t arg, int64_t arg2);

int32_t test_contract1_test_bool_false(int32_t arg_true, int32_t arg2_false);

float test_contract1_test_f32_30(float arg, float arg2);

double test_contract1_test_f64_31(double arg, double arg2);

char *test_contract1_test_str(const char *arg);

int32_t test_contract1_test_arg_vec_str_7(const char *arg);

int32_t test_contract1_test_arg_vec_u8_true(struct CInt8Array arg);

int32_t test_contract1_test_arg_vec_i8_6(struct CInt8Array arg);

int32_t test_contract1_test_arg_vec_i16_9(struct CInt16Array arg);

int32_t test_contract1_test_arg_vec_u16_10(struct CInt16Array arg);

int32_t test_contract1_test_arg_vec_i32_11(struct CInt32Array arg);

int32_t test_contract1_test_arg_vec_u32_12(struct CInt32Array arg);

int64_t test_contract1_test_arg_vec_i64_11(struct CInt64Array arg);

int64_t test_contract1_test_arg_vec_u64_12(struct CInt64Array arg);

int32_t test_contract1_test_arg_vec_bool_13(const char *arg_true);

int32_t test_contract1_test_arg_vec_struct_14(const char *arg);

int32_t test_contract1_test_two_vec_arg_15(struct CInt32Array arg, struct CInt32Array arg1);

char *test_contract1_test_return_vec_str(void);

struct CInt8Array test_contract1_test_return_vec_u8(void);

struct CInt8Array test_contract1_test_return_vec_i8(void);

struct CInt16Array test_contract1_test_return_vec_i16(void);

struct CInt16Array test_contract1_test_return_vec_u16(void);

struct CInt32Array test_contract1_test_return_vec_i32(void);

struct CInt32Array test_contract1_test_return_vec_u32(void);

struct CInt64Array test_contract1_test_return_vec_i64(void);

struct CInt64Array test_contract1_test_return_vec_u64(void);

char *test_contract1_test_return_vec_bool_true(void);

struct CInt8Array test_contract1_test_two_vec_u8(struct CInt8Array input);

char *test_contract1_test_return_vec_struct(void);

int8_t test_contract1_test_arg_callback_16(struct test_contract1_DemoCallback_Model arg);

int8_t test_contract1_test_two_arg_callback_20(struct test_contract1_DemoCallback_Model arg,
                                               struct test_contract1_DemoCallback_Model arg1);

char *test_contract1_test_return_struct(void);

void test_contract1_test_arg_struct(const char *arg);

void test_contract1_test_no_return(void);

void demo_free_rust(int8_t *ptr, uint32_t length);

void demo_free_str(char *ptr);
