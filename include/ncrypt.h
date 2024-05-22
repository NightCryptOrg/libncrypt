#pragma once

/* Generated with cbindgen:0.26.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


#define V0 0

#define CURRENT_VERSION V0

typedef struct String String;

typedef struct Vec_u8 Vec_u8;

/**
 * NCString - An NUL-terminated text string with FFI interop via [NString].
 */
typedef struct String NCString;

/**
 * NBString - A byte string with FFI interop via [NString]
 */
typedef struct Vec_u8 NBString;

/**
 * Metadata for encrypted data fields
 * (delivered encrypted with the same KEK as the data key)
 */
typedef struct DataHeader {
  uint16_t version;
  /**
   * Whether the data is empty/null.
   * This field enables opaque nullability
   */
  bool empty;
  /**
   * Data encryption algorithm ID
   */
  NCString *algorithm;
} DataHeader;

/**
 * Metadata for KEK-encrypted data-encryption keys
 * (delivered as plaintext)
 */
typedef struct KeyHeader {
  uint16_t version;
  /**
   * Key encryption algorithm ID
   */
  NCString *algorithm;
} KeyHeader;

typedef struct EncryptionHeader {
  uint16_t version;
  struct DataHeader data_header;
  struct KeyHeader key_header;
  /**
   * Data-encryption key, encrypted using user-KEK
   */
  NBString *key;
} EncryptionHeader;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

NCString *NCString_malloc(const char *str, size_t len);

void NCString_free(NCString *v);

char *NCString_get(NCString *v);

size_t NCString_get_len(const NCString *v);

NBString *NBString_malloc(const uint8_t *data, size_t len);

void NBString_free(NBString *v);

uint8_t *NBString_get(NBString *v);

size_t NBString_get_len(const NBString *v);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
