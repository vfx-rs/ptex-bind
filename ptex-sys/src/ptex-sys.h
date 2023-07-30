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
ptexwriter_open(
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

/// Release a PtexWriter instance.
inline void ptexwriter_release(PtexWriter *writer)
{
    if (writer) {
        writer->release();
    }
}

/// Close a PtexWriter instance.
inline rust::String ptexwriter_close(PtexWriter *writer)
{
    std::string error_message;
    if (writer) {
        writer->close(error_message);
    }
    return rust::String(error_message);
}

// struct Res

/// Create a default-constructed Res.
inline Res res_default()
{
    return Res();
}

/// Create Res from u and v log2 values.
inline Res res_from_uv(int8_t u, int8_t v)
{
    return Res(u, v);
}

/// Create Res from a packed u16 value.
inline Res res_from_value(uint16_t value)
{
    return Res(value);
}

/// Get the log2 resolution in the U direction.
inline int res_u(Res &res)
{
    return res.u();
}

/// Get the log2 resolution in the V direction.
inline int res_v(Res &res)
{
    return res.v();
}

/// Return the size for the FaceInfo.
inline int res_size(Res const &res)
{
    return res.size();
}

// struct FaceInfo

/// Create a default-constructed FaceInfo.
inline FaceInfo faceinfo_default()
{
    return FaceInfo();
}

/// Create a FaceInfo from a Res.
inline FaceInfo faceinfo_from_res(Res res)
{
    return FaceInfo(res);
}

/// Create a FaceInfo from a Res and adjacency information.
inline FaceInfo faceinfo_from_res_and_adjacency(
    Res res,
    int adjacent_faces[4],
    int adjacent_edges[4],
    bool is_subface
)
{
    return FaceInfo(res, adjacent_faces, adjacent_edges, is_subface);
}

// class PtexCache

/// Release a PtexCache pointer.
inline void ptexcache_release(PtexCache *cache)
{
    if (cache) {
        cache->release();
    }
}

/// Create a new PtexCache.
inline PtexCache* ptexcache_create(int max_files, size_t max_mem, bool premultiply)
{
    return PtexCache::create(max_files, max_mem, premultiply);
}

/// Create a PtexTexture reader for a filename or return an existing one if it already exists.
inline PtexTexture* ptexcache_get(PtexCache *cache, rust::Str filename, std::string &error_string)
{
    return cache->get(std::string(filename).c_str(), error_string);
}

/// Set the search path on a PtexCache instance.
inline void ptexcache_set_search_path(PtexCache *cache, rust::Str path)
{
    cache->setSearchPath(std::string(path).c_str());
}

/// Get the PtexCache search path.
inline rust::String ptexcache_get_search_path(PtexCache const *cache)
{
    if (cache) {
        return rust::String(const_cast<PtexCache *>(cache)->getSearchPath());
    }
    return rust::String();
}

/// Return true if the FaceInfo instance contains edits.
inline bool faceinfo_has_edits(FaceInfo *info)
{
    return info && info->hasEdits();
}

/// Return true if the FaceInfo contains constant data.
inline bool faceinfo_is_constant(FaceInfo *info)
{
    return info && info->isConstant();
}

/// Return true if the FaceInfo is in a neighborhood of constant faces.
inline bool faceinfo_is_neighborhood_constant(FaceInfo *info)
{
    return info && info->isNeighborhoodConstant();
}

/// Return true if the FaceInfo is a subface.
inline bool faceinfo_is_subface(FaceInfo *info)
{
    return info && info->isSubface();
}

/// Return the adjacent edge ID for the specified FaceInfo and edge.
inline EdgeId faceinfo_adjacent_edge(FaceInfo *info, int edge_id)
{
    return info->adjedge(edge_id);
}

/// Set the adjacent edges for the specified FaceInfo.
inline void faceinfo_set_adjacent_edges(FaceInfo *info, EdgeId e1, EdgeId e2, EdgeId e3, EdgeId e4)
{
    info->setadjedges(e1, e2, e3, e4);
}

/// Get the adjacent face for the specified FaceInfo and face ID.
inline int faceinfo_adjacent_face(FaceInfo *info, int face_id)
{
    return info->adjface(face_id);
}

/// Set the adjacent faces for the specified FaceInfo.
inline void faceinfo_set_adjacent_faces(FaceInfo *info, int f1, int f2, int f3, int f4)
{
    info->setadjfaces(f1, f2, f3, f4);
}

// class PtexTexture

/// Release a PtexTexture instance.
inline void ptextexture_release(PtexTexture *texture)
{
    if (texture) {
        texture->release();
    }
}

inline bool ptextexture_has_edits(PtexTexture const *texture)
{
    return texture && const_cast<PtexTexture *>(texture)->hasEdits();
}

inline bool ptextexture_has_mipmaps(PtexTexture const *texture)
{
    return texture && const_cast<PtexTexture *>(texture)->hasMipMaps();
}

inline int ptextexture_get_alpha_channel(PtexTexture const *texture)
{
    return texture ? const_cast<PtexTexture *>(texture)->alphaChannel() : -1;
}

inline int ptextexture_get_num_channels(PtexTexture const *texture)
{
    return texture ? const_cast<PtexTexture *>(texture)->numChannels() : 0;
}

inline int ptextexture_get_num_faces(PtexTexture const *texture)
{
    return texture ? const_cast<PtexTexture *>(texture)->numFaces() : 0;
}

inline rust::String ptextexture_get_path(PtexTexture const *texture)
{
    if (texture) {
        return rust::String(const_cast<PtexTexture *>(texture)->path());
    }
    return rust::String();
}

inline MeshType ptextexture_get_meshtype(PtexTexture const *texture)
{
    if (texture) {
        return const_cast<PtexTexture *>(texture)->meshType();
    }
    return MeshType::mt_quad;
}

inline DataType ptextexture_get_datatype(PtexTexture const *texture)
{
    if (texture) {
        return const_cast<PtexTexture *>(texture)->dataType();
    }
    return DataType::dt_uint8;
}

inline BorderMode ptextexture_get_border_mode_u(PtexTexture const *texture)
{
    if (texture) {
        return const_cast<PtexTexture *>(texture)->uBorderMode();
    }
    return BorderMode::m_clamp;
}

inline BorderMode ptextexture_get_border_mode_v(PtexTexture const *texture)
{
    if (texture) {
        return const_cast<PtexTexture *>(texture)->vBorderMode();
    }
    return BorderMode::m_clamp;
}

inline EdgeFilterMode ptextexture_get_edge_filter_mode(PtexTexture const *texture)
{
    if (texture) {
        return const_cast<PtexTexture *>(texture)->edgeFilterMode();
    }
    return EdgeFilterMode::efm_none;
}

inline const FaceInfo& ptextexture_get_face_info(PtexTexture const *texture, int faceid)
{
    return const_cast<PtexTexture *>(texture)->getFaceInfo(faceid);
}

inline float ptextexture_get_pixel(
    PtexTexture const *texture,
    int faceid,
    int u,
    int v,
    int first_channel,
    int num_channels)
{
    float result;
    const_cast<PtexTexture *>(texture)->getPixel(faceid, u, v, &result, first_channel, num_channels);
    return result;
}

}  // namespace sys
}  // namespace Ptex
