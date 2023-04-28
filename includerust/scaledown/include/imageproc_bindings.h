/* Generated with cbindgen:0.24.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Resolution {
  uint32_t w;
  uint32_t h;
} Resolution;

const char *errstr(void);

int32_t convertw(const char *cinpath, const char *coutpath, const struct Resolution *cresolution);
