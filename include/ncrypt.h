#pragma once

/* Generated with cbindgen:0.26.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct String String;

/**
 * NCString - An owned NUL-terminated text string with FFI interop.
 */
typedef struct NCString {
  struct String _0;
} NCString;

/**
 * Metadata for encrypted data fields
 * (delivered encrypted with the same KEK as the data key)
 */
typedef struct DataHeader {
  /**
   * Whether the data is empty/null.
   * This field enables opaque nullability
   */
  bool empty;
  /**
   * Data encryption algorithm ID
   */
  struct NCString algorithm;
} DataHeader;

/**
 * Metadata for KEK-encrypted data-encryption keys
 * (delivered as plaintext)
 */
typedef struct KeyHeader {
  /**
   * Key encryption algorithm ID
   */
  struct NCString algorithm;
} KeyHeader;

typedef struct EncryptionHeader {
  struct DataHeader data_header;
  struct KeyHeader key_header;
  /**
   * Data-encryption key, encrypted using user-KEK
   */
  struct NCString key;
} EncryptionHeader;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void NCString_free(struct NCString *v);

char *NCString_get(struct NCString *v);

struct NCString *NCString_malloc(const char *str, size_t len);

size_t NCstring_get_len(const struct NCString *v);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus