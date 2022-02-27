#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct test_contract1_DemoCallback_Model {
  int8_t (*test_u8_1)(int64_t, int8_t, int8_t);
  int8_t (*test_i8_2)(int64_t, int8_t, int8_t);
  int32_t (*test_i16_3)(int64_t, int32_t, int32_t);
  int32_t (*test_u16_4)(int64_t, int32_t, int32_t);
  int32_t (*test_i32_5)(int64_t, int32_t, int32_t);
  int32_t (*test_u32_6)(int64_t, int32_t, int32_t);
  int32_t (*test_bool_false)(int64_t, int32_t, int32_t);
  int32_t (*test_arg_vec_str_18)(int64_t, const char*);
  int32_t (*test_arg_vec_u8_7)(int64_t, const char*);
  int32_t (*test_arg_vec_i8_8)(int64_t, const char*);
  int32_t (*test_arg_vec_i16_9)(int64_t, const char*);
  int32_t (*test_arg_vec_u16_10)(int64_t, const char*);
  int32_t (*test_arg_vec_i32_11)(int64_t, const char*);
  int32_t (*test_arg_vec_u32_12)(int64_t, const char*);
  int32_t (*test_arg_vec_bool_true)(int64_t, const char*);
  int32_t (*test_arg_vec_struct_17)(int64_t, const char*);
  int32_t (*test_two_vec_arg_13)(int64_t, const char*, const char*);
  int32_t (*test_arg_struct_14)(int64_t, const char*);
  int32_t (*test_two_arg_struct_15)(int64_t, const char*, const char*);
  void (*test_no_return)(int64_t);
  void (*free_callback)(int64_t);
  int64_t index;
} test_contract1_DemoCallback_Model;

void test_contract1_setup(void);

int8_t test_contract1_test_u8_1(int8_t arg, int8_t arg2);

int8_t test_contract1_test_i8_2(int8_t arg, int8_t arg2);

int32_t test_contract1_test_i16_3(int32_t arg, int32_t arg2);

int32_t test_contract1_test_u16_4(int32_t arg, int32_t arg2);

int32_t test_contract1_test_i32_5(int32_t arg, int32_t arg2);

int32_t test_contract1_test_u32_6(int32_t arg, int32_t arg2);

int32_t test_contract1_test_bool_false(int32_t arg_true, int32_t arg2_false);

char *test_contract1_test_str(const char *arg);

int32_t test_contract1_test_arg_vec_str_7(const char *arg);

int32_t test_contract1_test_arg_vec_u8_true(const char *arg);

int32_t test_contract1_test_arg_vec_i8_6(const char *arg);

int32_t test_contract1_test_arg_vec_i16_9(const char *arg);

int32_t test_contract1_test_arg_vec_u16_10(const char *arg);

int32_t test_contract1_test_arg_vec_i32_11(const char *arg);

int32_t test_contract1_test_arg_vec_u32_12(const char *arg);

int32_t test_contract1_test_arg_vec_bool_13(const char *arg_true);

int32_t test_contract1_test_two_vec_arg_15(const char *arg, const char *arg1);

char *test_contract1_test_return_vec_str(void);

char *test_contract1_test_return_vec_u8(void);

char *test_contract1_test_return_vec_i8(void);

char *test_contract1_test_return_vec_i16(void);

char *test_contract1_test_return_vec_u16(void);

char *test_contract1_test_return_vec_i32(void);

char *test_contract1_test_return_vec_u32(void);

char *test_contract1_test_return_vec_bool_true(void);

char *test_contract1_test_two_vec_u8(const char *input);

char *test_contract1_test_return_vec_struct(void);

int8_t test_contract1_test_arg_callback_16(struct test_contract1_DemoCallback_Model arg);

int8_t test_contract1_test_two_arg_callback_20(struct test_contract1_DemoCallback_Model arg,
                                               struct test_contract1_DemoCallback_Model arg1);

char *test_contract1_test_return_struct(void);

void test_contract1_test_no_return(void);

void demo_free_rust(uint8_t *ptr, uint32_t length);

void demo_free_str(char *ptr);