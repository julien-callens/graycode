#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// Fill `dest` with a single Gray-code bit-plane.
/// Safety: `dest` must be WIDTH*HEIGHT bytes long.
void gc_bit_plane(uint8_t *dest, uint32_t width, uint32_t height, uintptr_t bit);

}  // extern "C"
