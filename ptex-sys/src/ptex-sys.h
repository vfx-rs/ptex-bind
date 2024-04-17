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

/// Write a face worth of Data
inline bool ptexwriter_write_face(
    PtexWriter *writer,
    int32_t face_id,
    FaceInfo &face_info,
    unsigned char const *data,
    int32_t stride)
{
    return writer->writeFace(face_id, face_info, (void*)data, stride);
}

/// Set border modes for writer
inline void ptexwriter_set_border_modes(
    PtexWriter *writer,
    BorderMode u_border_mode,
    BorderMode v_border_mode)
{
    writer->setBorderModes(u_border_mode, v_border_mode);
}

/// Set edge filter mode for writer.
inline void ptexwriter_set_edge_filter_mode(
    PtexWriter *writer,
    EdgeFilterMode edge_filter_mode)
{
    writer->setEdgeFilterMode(edge_filter_mode);
}

/// Write meta data.
inline bool ptexwriter_write_meta_data(
		PtexWriter *writer,
		const char *key,
		MetaDataType metadata_kind,
		char const *data,
		size_t count) {
	switch (metadata_kind) {
		case Ptex::mdt_string:
			// Assume null terminated.
			writer->writeMeta(key, data);
			return true;
		case Ptex::mdt_int8:
			writer->writeMeta(key, (int8_t*)data, count);
			return true;
		case Ptex::mdt_int16:
			writer->writeMeta(key, (int16_t*)data, count);
			return true;
		case Ptex::mdt_int32:
			writer->writeMeta(key, (int32_t*)data, count);
			return true;
		case Ptex::mdt_float:
			writer->writeMeta(key, (float*)data, count);
			return true;
		case Ptex::mdt_double:
			writer->writeMeta(key, (double*)data, count);
			return true;
		default:
			return false;
	}
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
inline int32_t res_u(Res &res)
{
    return res.u();
}

/// Get the log2 resolution in the V direction.
inline int32_t res_v(Res &res)
{
    return res.v();
}

/// Return the size for the FaceInfo.
inline int32_t res_size(Res const &res)
{
    return res.size();
}

/// Return the size for the FaceInfo.
inline Res res_swappeduv(Res const &res)
{
    return res.swappeduv();
}

/// Return the size for the FaceInfo.
inline void res_swapuv(Res &res)
{
    res.swapuv();
}

/// Return the size for the FaceInfo.
inline void res_clamp(Res &res, Res const &clamp_res)
{
    res.clamp(clamp_res);
}

/// Determine the number of tiles in the u direction for the given tile res.
inline int32_t res_ntilesu(Res &res, Res tile_res)
{
    return res.ntilesu(tile_res);
}

/// Determine the number of tiles in the v direction for the given tile res.
inline int32_t res_ntilesv(Res &res, Res tile_res)
{
    return res.ntilesv(tile_res);
}

/// Determine the number of tiles in the v direction for the given tile res.
inline int32_t res_ntiles(Res &res, Res tile_res)
{
    return res.ntiles(tile_res);
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
    int32_t adjacent_faces[4],
    int32_t adjacent_edges[4],
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
inline PtexCache* ptexcache_create(int32_t max_files, size_t max_mem, bool premultiply)
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
inline EdgeId faceinfo_adjacent_edge(FaceInfo *info, int32_t edge_id)
{
    return info->adjedge(edge_id);
}

/// Set the adjacent edges for the specified FaceInfo.
inline void faceinfo_set_adjacent_edges(FaceInfo *info, EdgeId e1, EdgeId e2, EdgeId e3, EdgeId e4)
{
    info->setadjedges(e1, e2, e3, e4);
}

/// Get the adjacent face for the specified FaceInfo and face ID.
inline int32_t faceinfo_adjacent_face(FaceInfo *info, int32_t face_id)
{
    return info->adjface(face_id);
}

/// Set the adjacent faces for the specified FaceInfo.
inline void faceinfo_set_adjacent_faces(FaceInfo *info, int32_t f1, int32_t f2, int32_t f3, int32_t f4)
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

inline int32_t ptextexture_get_alpha_channel(PtexTexture const *texture)
{
    return texture ? const_cast<PtexTexture *>(texture)->alphaChannel() : -1;
}

inline int32_t ptextexture_get_num_channels(PtexTexture const *texture)
{
    return texture ? const_cast<PtexTexture *>(texture)->numChannels() : 0;
}

inline int32_t ptextexture_get_num_faces(PtexTexture const *texture)
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

inline PtexMetaData* ptextexture_get_meta_data(PtexTexture *texture)
{
	return const_cast<PtexTexture *>(texture)->getMetaData();
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
        BorderMode mode = const_cast<PtexTexture *>(texture)->uBorderMode();
        if (mode > BorderMode::m_periodic) {
            return BorderMode::m_clamp;
        }
        return mode;
    }
    return BorderMode::m_clamp;
}

inline BorderMode ptextexture_get_border_mode_v(PtexTexture const *texture)
{
    if (texture) {
        BorderMode mode = const_cast<PtexTexture *>(texture)->uBorderMode();
        if (mode > BorderMode::m_periodic) {
            return BorderMode::m_clamp;
        }
        return mode;
    }
    return BorderMode::m_clamp;
}

inline EdgeFilterMode ptextexture_get_edge_filter_mode(PtexTexture const *texture)
{
    if (texture) {
        EdgeFilterMode mode = const_cast<PtexTexture *>(texture)->edgeFilterMode();
        if (mode > EdgeFilterMode::efm_tanvec) {
            return EdgeFilterMode::efm_none;
        }
        return mode;
    }
    return EdgeFilterMode::efm_none;
}

inline const FaceInfo& ptextexture_get_face_info(PtexTexture const *texture, int32_t faceid)
{
    return const_cast<PtexTexture *>(texture)->getFaceInfo(faceid);
}

inline float ptextexture_get_pixel(
    PtexTexture const *texture,
    int32_t faceid,
    int32_t u,
    int32_t v,
    int32_t first_channel,
    int32_t num_channels)
{
    float result;
    const_cast<PtexTexture *>(texture)->getPixel(faceid, u, v, &result, first_channel, num_channels);
    return result;
}


// struct PtexMetaData
inline int32_t ptexmetadata_num_keys(PtexMetaData *metadata)  {
    return (int32_t) metadata->numKeys();
}

inline void ptexmetadata_get_key(PtexMetaData *metadata, int index, const char *& key, MetaDataType& typ) {
    if (metadata) {
        metadata->getKey(index, key, typ);
    }
}

inline bool ptexmetadata_find_key(PtexMetaData *metadata, const char *key, int &index, MetaDataType& typ) {
    if (metadata) {
        return metadata->findKey(key, index, typ);
    } else {
	return false;
    }
}

inline void ptexmetadata_get_value_at_index(PtexMetaData *metadata, int index, MetaDataType typ, char const *&value, int &count) {
    if (metadata && &value != 0) {
        switch (typ) {
            case Ptex::mdt_string:
                metadata->getValue(index, value);
                count = 0;
                break;
            case Ptex::mdt_int8:
                metadata->getValue(index, (const int8_t *&)value, count);
                break;
            case Ptex::mdt_int16:
                metadata->getValue(index, (const int16_t *&)value, count);
                break;
            case Ptex::mdt_int32:
                metadata->getValue(index, (const int32_t *&)value, count);
                break;
            case Ptex::mdt_float:
                metadata->getValue(index, (const float *&)value, count);
                break;
            case Ptex::mdt_double:
                metadata->getValue(index, (const double *&)value, count);
                break;
            default:
                value = NULL;
                count = 0;
                break;
        }
    }
}

inline void ptexmetadata_get_value_for_key(PtexMetaData *metadata, const char *key, MetaDataType typ, char const *&value, int &count) {
    if (metadata && value) {
        switch (typ) {
            case Ptex::mdt_string:
                metadata->getValue(key, value);
                count = 0;
                break;
            case Ptex::mdt_int8:
                metadata->getValue(key, (const int8_t *&)value, count);
                break;
            case Ptex::mdt_int16:
                metadata->getValue(key, (const int16_t *&)value, count);
                break;
            case Ptex::mdt_int32:
                metadata->getValue(key, (const int32_t *&)value, count);
                break;
            case Ptex::mdt_float:
                metadata->getValue(key, (const float *&)value, count);
                break;
            case Ptex::mdt_double:
                metadata->getValue(key, (const double *&)value, count);
                break;
            default:
                value = NULL;
                count = 0;
                break;
        }
    }
}

inline void ptexmetadata_release(PtexMetaData *metadata)  {
    if (metadata) {
        metadata->release();
    }
}

}  // namespace sys
}  // namespace Ptex
