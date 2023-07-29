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

inline void ptexcache_release(PtexCache* cache)
{
    if (cache) {
        cache->release();
    }
}

inline PtexCache* ptexcache_create(int max_files, size_t max_mem, bool premultiply)
{
    return PtexCache::create(max_files, max_mem, premultiply);
}

inline void ptexcache_set_search_path(PtexCache* cache, rust::Str path)
{
    cache->setSearchPath(std::string(path).c_str());
}

inline rust::String ptexcache_get_search_path(PtexCache* cache)
{
    if (cache) {
        return rust::String(cache->getSearchPath());
    }
    return rust::String();
}

}  // namespace sys
}  // namespace Ptex
