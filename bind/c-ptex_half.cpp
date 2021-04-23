#include <PtexHalf.h>
#include <cppmm_bind.hpp>

namespace cppmm_bind {

namespace Ptex {

namespace v2_2 {

namespace Ptex = ::Ptex::v2_2;

struct PtexHalf {
    using BoundType = Ptex::PtexHalf;

    PtexHalf() CPPMM_RENAME(default);
    PtexHalf(float val) CPPMM_RENAME(from_f32);
    PtexHalf(double val) CPPMM_RENAME(from_f64);

    CPPMM_RENAME(as_f32)
    operator float() const;

    CPPMM_IGNORE
    auto operator=(float val) -> Ptex::PtexHalf&;
    static auto toFloat(unsigned short h) -> float;
    static auto fromFloat(float val) -> unsigned short;
} CPPMM_OPAQUEBYTES; // struct PtexHalf

} // namespace v2_2

} // namespace Ptex

} // namespace cppmm_bind
