#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
  int8_t (*on_callback_u8)(int64_t, int8_t);
  int8_t (*on_callback_i8)(int64_t, int8_t);
  int32_t (*on_callback)(int64_t, int32_t, const char*, int32_t, float, double);
  int32_t (*on_callback2)(int64_t, int32_t);
  int32_t (*on_callback_complex)(int64_t, const char*);
  int32_t (*on_callback_arg_vec)(int64_t, const char*);
  int32_t (*on_callback_arg_vec_simple)(int64_t, const char*);
  void (*on_empty_callback)(int64_t);
  void (*free_callback)(int64_t);
  int64_t index;
} test_contract1_Callback_Model;

void demo_free_rust(uint8_t *ptr, uint32_t length);

void demo_free_str(char *ptr);

int8_t test_contract1_test_arg_callback(test_contract1_Callback_Model arg);

int32_t test_contract1_test_arg_vec(const char *arg);

int32_t test_contract1_test_bool(int32_t arg1);

int8_t test_contract1_test_byte(int8_t arg);

int8_t test_contract1_test_byte_i8(int8_t arg);

void test_contract1_test_no_return();

char *test_contract1_test_return_vec(int8_t arg);

char *test_contract1_test_return_vec_u8(const char *input);

char *test_contract1_test_return_vec_u8(const char *input);

char *test_contract1_test_struct();

char *test_contract1_test_struct_vec();

char *test_contract1_test_two_string(const char *arg1, const char *arg2);
