#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct test_contract1_Future_Model {
  int32_t (*get)(int64_t);
  void (*free_callback)(int64_t);
  void (*free_ptr)(int8_t*, int32_t);
  int64_t index;
} test_contract1_Future_Model;

typedef struct test_contract1_LoginService_Model {
  struct test_contract1_Future_Model (*login)(int64_t, const char*, const char*);
  void (*free_callback)(int64_t);
  void (*free_ptr)(int8_t*, int32_t);
  int64_t index;
} test_contract1_LoginService_Model;

typedef struct test_contract1_UploadProgress_Model {
  void (*on_progress)(int64_t, int64_t, int64_t, int64_t);
  void (*free_callback)(int64_t);
  void (*free_ptr)(int8_t*, int32_t);
  int64_t index;
} test_contract1_UploadProgress_Model;

typedef struct test_contract1_UploadService_Model {
  int64_t (*upload)(int64_t, const char*, struct test_contract1_UploadProgress_Model);
  void (*free_callback)(int64_t);
  void (*free_ptr)(int8_t*, int32_t);
  int64_t index;
} test_contract1_UploadService_Model;

struct test_contract1_LoginService_Model test_contract1_Services_get_login_service(void);

struct test_contract1_UploadService_Model test_contract1_Services_get_upload_service(void);

void demo_free_rust(int8_t *ptr, uint32_t length);

void demo_free_str(char *ptr);
