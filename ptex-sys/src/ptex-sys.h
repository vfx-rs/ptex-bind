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

inline int res_u(Res& res)
{
    return res.u();
}

inline int res_v(Res& res)
{
    return res.v();
}

// struct FaceInfo
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

/// class PtexCache

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

inline PtexTexture* ptexcache_get(PtexCache* cache, rust::Str filename, std::string& error_string)
{
    return cache->get(std::string(filename).c_str(), error_string);
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

inline bool faceinfo_has_edits(FaceInfo* info)
{
    return info && info->hasEdits();
}

inline bool faceinfo_is_constant(FaceInfo* info)
{
    return info && info->isConstant();
}

inline bool faceinfo_is_neighborhood_constant(FaceInfo* info)
{
    return info && info->isNeighborhoodConstant();
}

inline bool faceinfo_is_subface(FaceInfo* info)
{
    return info && info->isSubface();
}

inline EdgeId faceinfo_adjacent_edge(FaceInfo* info, int edge_id)
{
    return info->adjedge(edge_id);
}

inline void faceinfo_set_adjacent_edges(FaceInfo* info, EdgeId e1, EdgeId e2, EdgeId e3, EdgeId e4)
{
    info->setadjedges(e1, e2, e3, e4);
}

inline int faceinfo_adjacent_face(FaceInfo* info, int face_id)
{
    return info->adjface(face_id);
}

inline void faceinfo_set_adjacent_faces(FaceInfo* info, int f1, int f2, int f3, int f4)
{
    info->setadjfaces(f1, f2, f3, f4);
}

/// class PtexTexture
inline void ptextexture_release(PtexTexture* texture)
{
    if (texture) {
        texture->release();
    }
}

inline bool ptextexture_has_edits(PtexTexture* texture)
{
    return texture && texture->hasEdits();
}

inline bool ptextexture_has_mipmaps(PtexTexture* texture)
{
    return texture && texture->hasMipMaps();
}

inline int ptextexture_get_alpha_channel(PtexTexture* texture)
{
    return texture ? texture->alphaChannel() : -1;
}

inline int ptextexture_get_num_channels(PtexTexture* texture)
{
    return texture ? texture->numChannels() : 0;
}

inline int ptextexture_get_num_faces(PtexTexture* texture)
{
    return texture ? texture->numFaces() : 0;
}

inline rust::String ptextexture_get_path(PtexTexture* texture)
{
    if (texture) {
        return rust::String(texture->path());
    }
    return rust::String();
}

inline MeshType ptextexture_get_meshtype(PtexTexture* texture)
{
    if (texture) {
        return texture->meshType();
    }
    return MeshType::mt_quad;
}

inline DataType ptextexture_get_datatype(PtexTexture* texture)
{
    if (texture) {
        return texture->dataType();
    }
    return DataType::dt_uint8;
}

inline BorderMode ptextexture_get_border_mode_u(PtexTexture* texture)
{
    if (texture) {
        return texture->uBorderMode();
    }
    return BorderMode::m_clamp;
}

inline BorderMode ptextexture_get_border_mode_v(PtexTexture* texture)
{
    if (texture) {
        return texture->vBorderMode();
    }
    return BorderMode::m_clamp;
}

inline EdgeFilterMode ptextexture_get_edge_filter_mode(PtexTexture* texture)
{
    if (texture) {
        return texture->edgeFilterMode();
    }
    return EdgeFilterMode::efm_none;
}

inline const FaceInfo& ptextexture_get_face_info(PtexTexture* texture, int faceid)
{
    return texture->getFaceInfo(faceid);
}

inline float ptextexture_get_pixel(
    PtexTexture* texture,
    int faceid,
    int u,
    int v,
    int first_channel,
    int num_channels)
{
    float result;
    texture->getPixel(faceid, u, v, &result, first_channel, num_channels);
    return result;
}

}  // namespace sys
}  // namespace Ptex
