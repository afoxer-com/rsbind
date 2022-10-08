#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct test_contract1_Future_Model {
  int32_t (*get)(int64_t);
  void (*free_callback)(int64_t);
  void (*free_ptr)(int8_t*, int32_t, int32_t);
  int64_t index;
} test_contract1_Future_Model;

typedef struct CInt8Array {
  const int8_t *ptr;
  int32_t len;
  int32_t cap;
  void (*free_ptr)(int8_t*, int32_t, int32_t);
} CInt8Array;

typedef struct test_contract1_LoginService_Model {
  struct test_contract1_Future_Model (*login)(int64_t, struct CInt8Array, struct CInt8Array);
  void (*free_callback)(int64_t);
  void (*free_ptr)(int8_t*, int32_t, int32_t);
  int64_t index;
} test_contract1_LoginService_Model;

typedef struct test_contract1_UploadProgress_Model {
  void (*on_progress)(int64_t, int64_t, int64_t, int64_t);
  void (*free_callback)(int64_t);
  void (*free_ptr)(int8_t*, int32_t, int32_t);
  int64_t index;
} test_contract1_UploadProgress_Model;

typedef struct test_contract1_UploadService_Model {
  int64_t (*upload)(int64_t, struct CInt8Array, struct test_contract1_UploadProgress_Model);
  void (*free_callback)(int64_t);
  void (*free_ptr)(int8_t*, int32_t, int32_t);
  int64_t index;
} test_contract1_UploadService_Model;

struct test_contract1_LoginService_Model test_contract1_Services_get_login_service(void);

struct test_contract1_UploadService_Model test_contract1_Services_get_upload_service(void);

void free_i8_array(int8_t *ptr, int32_t length, int32_t cap);

void free_i16_array(int16_t *ptr, int32_t length, int32_t cap);

void free_i32_array(int32_t *ptr, int32_t length, int32_t cap);

void free_i64_array(int64_t *ptr, int32_t length, int32_t cap);

void free_str(int8_t *ptr, int32_t length, int32_t cap);
