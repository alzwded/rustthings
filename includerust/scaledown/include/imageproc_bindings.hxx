/* Generated with cbindgen:0.24.3 */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Resolution {
  uint32_t w;
  uint32_t h;

  Resolution(uint32_t const& w,
             uint32_t const& h)
    : w(w),
      h(h)
  {}

  bool operator==(const Resolution& other) const {
    return w == other.w &&
           h == other.h;
  }
  bool operator!=(const Resolution& other) const {
    return w != other.w ||
           h != other.h;
  }
};

extern "C" {

const char *errstr();

int32_t convertw(const char *cinpath, const char *coutpath, const Resolution *cresolution);

} // extern "C"
