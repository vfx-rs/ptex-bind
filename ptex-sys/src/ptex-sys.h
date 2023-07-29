/// Bridge Ptex's C++ API to Rust.
/// This module provides functions that call into C++ static class methods
/// that are not accessible by the cxx crate.
///
/// https://github.com/dtolnay/cxx/issues/464#issuecomment-725909931
#pragma once

#include <Ptexture.h>
#include <string>
#include <rust/cxx.h>

namespace Ptex {
namespace sys {

/// Entry point into static PtexWriter::open().
inline PtexWriter*
writer_open(
    rust::Str filename,
    MeshType meshtype,
    DataType datatype,
    int numchannels,
    int alphachan,
    int numfaces,
    bool genmipmaps,
    std::string* error)
{
    // c_str() ensures that a NULL terminator is present.
    return Ptex::PtexWriter::open(
        std::string(filename).c_str(),
        meshtype,
        datatype,
        numchannels,
        alphachan,
        numfaces,
        *error,
        genmipmaps
    );
}

inline Res res_default()
{
    return Res();
}

inline Res res_from_uv(int8_t u, int8_t v)
{
    return Res(u, v);
}

inline Res res_from_value(uint16_t value)
{
    return Res(value);
}

inline FaceInfo faceinfo_default()
{
    return FaceInfo();
}

inline FaceInfo faceinfo_from_res(Res res)
{
    return FaceInfo(res);
}

inline FaceInfo faceinfo_from_res_and_adjacency(
    Res res,
    int32_t adjacent_faces[4],
    int32_t adjacent_edges[4],
    bool is_subface
)
{
    return FaceInfo(res, adjacent_faces, adjacent_edges, is_subface);
}

}  // namespace sys

}  // namespace Ptex
