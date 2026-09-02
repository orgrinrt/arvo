// a01: does C++ have arvo's bridge problem?
//
// The question arvo's bridge asks: turn a WRITTEN width (a value) into a machine
// container (a type), at compile time, without one declaration per width.
//
// C++ answer: a non-type template parameter may be operated on with arbitrary
// constant expressions in template-argument position. No table, no enumeration,
// no per-width declaration. One generic definition covers every width.
//
// Compile: clang++ -std=c++20 -O2 -c a01_cpp_nttp_derivation.cpp -o out/a01.o

#include <cstdint>
#include <cstddef>
#include <array>
#include <type_traits>

// --- the ladder, written once, generic over the width ---------------------

// arbitrary arithmetic on N in TYPE position. This is the whole thing.
template <unsigned N>
using Words = std::array<std::uint64_t, (N + 63u) / 64u>;

template <unsigned N>
using Pick = std::conditional_t<(N <=  8), std::uint8_t,
             std::conditional_t<(N <= 16), std::uint16_t,
             std::conditional_t<(N <= 32), std::uint32_t,
             std::conditional_t<(N <= 64), std::uint64_t,
                                           Words<N>>>>>;

// --- the surface ----------------------------------------------------------

template <unsigned I, unsigned F>
struct Fixed {
    using Raw = Pick<I + F>;   // arithmetic on TWO parameters, in type position
    Raw raw;
};

// --- assertions: the derivation lands where it should ---------------------

static_assert(sizeof(Fixed<3, 0>)     == 1,  "3 bits -> u8");
static_assert(sizeof(Fixed<13, 3>)    == 2,  "16 bits -> u16");
static_assert(sizeof(Fixed<13, 13>)   == 4,  "26 bits -> u32");
static_assert(sizeof(Fixed<40, 20>)   == 8,  "60 bits -> u64");
static_assert(sizeof(Fixed<100, 100>) == 32, "200 bits -> 4 words");
// a width nobody wrote down anywhere. No declaration exists for 4712 bits.
static_assert(sizeof(Fixed<4711, 1>)  == 8 * ((4712 + 63) / 64), "arbitrary width");

// --- erasure: the width is not in the object file -------------------------

extern "C" std::uint16_t arvo16(Fixed<13, 3> a, Fixed<13, 3> b) {
    return static_cast<std::uint16_t>(a.raw + b.raw);
}
extern "C" std::uint16_t native16(std::uint16_t a, std::uint16_t b) {
    return static_cast<std::uint16_t>(a + b);
}
