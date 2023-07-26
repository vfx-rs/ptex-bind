/// Bridge Ptex's C++ API to Rust.
/// This module provides functions that call into C++ static class methods
/// that are not accessible by the cxx crate.
///
/// https://github.com/dtolnay/cxx/issues/464#issuecomment-725909931
#pragma once

#include <Ptexture.h>
#include <string>

namespace Ptex {
namespace bind {

/// Entry point into static PtexWriter::open().
inline PtexWriter*
writer_open(
    const char* path,
    MeshType mt,
    DataType dt,
    int nchannels,
    int alphachan,
    int nfaces,
    bool genmipmaps,
    std::string* error)
{
    return Ptex::PtexWriter::open(path, mt, dt, nchannels, alphachan, nfaces, *error, genmipmaps);
}

}  // namespace bind
}  // namespace Ptex
