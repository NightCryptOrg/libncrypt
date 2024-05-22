#ifndef NCRYPT_H
#define NCRYPT_H

/* Generated with cbindgen:0.26.0 */

/* WARNING: This header was generated with the assumption that sizeof(size_t) == sizeof(uintptr_t). Use of this library on systems where sizeof(size_t) != sizeof(uintptr_t) is undefined behavior. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct String String;

/**
 * NCString - An owned NUL-terminated text string with FFI interop.
 */
typedef struct String NCString;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void NCString_free(NCString *v);

char *NCString_get(NCString *v);

NCString *NCString_malloc(const char *str, size_t len);

size_t NCstring_get_len(const NCString *v);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* NCRYPT_H */
