#ifndef NCRYPT_H
#define NCRYPT_H

/* Generated with cbindgen:0.26.0 */

/* WARNING: This header was generated with the assumption that sizeof(size_t) == sizeof(uintptr_t). Use of this library on systems where sizeof(size_t) != sizeof(uintptr_t) is undefined behavior. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


/**
 * NBString - An owned binary string with associated length
 */
typedef struct NBString NBString;

/**
 * NCString - An owned NUL-terminated text string with associated length.
 *
 * SAFETY: An NCString used via FFI must be initialized with `NCString_init()` before use,
 * and deinitialized with `NCString_deinit()` after use
 */
typedef struct NCString NCString;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Deinitialize an NBString (unneccessary to call from Rust - RAII accomplishes deinitialization)
 */
void NBString_deinit(struct NBString *self);

/**
 * Initialize an NBString (copies to the contents of data)
 * (uneccessary to call from Rust - NBString implements [From]<[Vec<u8>]> and [From]<[\[u8\]]>)
 */
void NBString_init(struct NBString *self, unsigned char *data, size_t len);

/**
 * Deinitialize an NCString (unecessary to call from Rust - RAII accomplishes deinitialization)
 */
void NCString_deinit(struct NCString *self);

/**
 * Initialize an NCString (copies the contents of str)
 * (uneccessary to call from Rust - NCString implements [From]<[String]>)
 *
 * Returns 0 on success or 1 if *str contains inner NUL characters
 */
int NCString_init(struct NCString *self, char *str, size_t len);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* NCRYPT_H */
