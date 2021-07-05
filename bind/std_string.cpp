#include <string>

#include <cppmm_bind.hpp>

namespace cppmm_bind {

namespace std {

#if defined(_GLIBCXX_USE_CXX11_ABI)
#if _GLIBCXX_USE_CXX11_ABI
namespace std = ::std::__cxx11;
#else
namespace std = ::std;
#endif
#elif defined(_LIBCPP_VERSION)
namespace std = ::std::_LIBCPP_ABI_NAMESPACE;
#else
namespace std = ::std;
#endif

class basic_string {
public:
    using BoundType = std::basic_string<char>;

    basic_string(const char* s, ::std::string::size_type count);
    ::std::string& assign(const char* s, ::std::string::size_type count);
    const char* c_str() const;

} CPPMM_OPAQUEBYTES CPPMM_IGNORE_UNBOUND;

using string = ::std::string;
} // namespace std

} // namespace cppmm_bind
