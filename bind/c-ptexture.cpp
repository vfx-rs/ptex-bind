#include <Ptexture.h>
#include <cppmm_bind.hpp>

namespace cppmm_bind {

namespace Ptex {

namespace v2_4 {

namespace Ptex = ::Ptex::v2_4;

enum MeshType {
    mt_triangle = 0,
    mt_quad = 1,
} CPPMM_ENUM_PREFIX(mt_) CPPMM_RUSTIFY_ENUM;

enum DataType {
    dt_uint8 = 0,
    dt_uint16 = 1,
    dt_half = 2,
    dt_float = 3,
} CPPMM_ENUM_PREFIX(dt_) CPPMM_RUSTIFY_ENUM;

enum EdgeFilterMode {
    efm_none = 0,
    efm_tanvec = 1,
} CPPMM_ENUM_PREFIX(efm_) CPPMM_RUSTIFY_ENUM;

enum BorderMode {
    m_clamp = 0,
    m_black = 1,
    m_periodic = 2,
} CPPMM_ENUM_PREFIX(m_) CPPMM_RUSTIFY_ENUM;

enum EdgeId {
    e_bottom = 0,
    e_right = 1,
    e_top = 2,
    e_left = 3,
} CPPMM_ENUM_PREFIX(e_) CPPMM_RUSTIFY_ENUM;

enum MetaDataType {
    mdt_string = 0,
    mdt_int8 = 1,
    mdt_int16 = 2,
    mdt_int32 = 3,
    mdt_float = 4,
    mdt_double = 5,
} CPPMM_ENUM_PREFIX(mdt_) CPPMM_RUSTIFY_ENUM;

auto MeshTypeName(Ptex::MeshType mt) -> const char*;

auto DataTypeName(Ptex::DataType dt) -> const char*;

auto BorderModeName(Ptex::BorderMode m) -> const char*;

auto EdgeFilterModeName(Ptex::EdgeFilterMode m) -> const char*;

auto EdgeIdName(Ptex::EdgeId eid) -> const char*;

auto MetaDataTypeName(Ptex::MetaDataType mdt) -> const char*;

auto DataSize(Ptex::DataType dt) -> int;

auto OneValue(Ptex::DataType dt) -> float;

auto OneValueInv(Ptex::DataType dt) -> float;

auto ConvertToFloat(float* dst, const void* src, Ptex::DataType dt,
                    int numChannels) -> void;

auto ConvertFromFloat(void* dst, const float* src, Ptex::DataType dt,
                      int numChannels) -> void;

struct Res {
    using BoundType = Ptex::Res;

    Res();

    CPPMM_RENAME(from_uv_log2)
    Res(signed char ulog2_, signed char vlog2_);

    CPPMM_RENAME(from_value)
    Res(unsigned short value);
    auto u() const -> int;
    auto v() const -> int;
    auto val() const -> unsigned short;
    auto size() const -> int;
    auto operator==(const Ptex::Res& r) const -> bool;
    auto operator!=(const Ptex::Res& r) const -> bool;
    CPPMM_RENAME(_op_ge)
    auto operator>=(const Ptex::Res& r) const -> bool;
    auto swappeduv() const -> Ptex::Res;
    auto swapuv() -> void;
    auto clamp(const Ptex::Res& r) -> void;
    auto ntilesu(Ptex::Res tileres) const -> int;
    auto ntilesv(Ptex::Res tileres) const -> int;
    auto ntiles(Ptex::Res tileres) const -> int;
    Res(const Ptex::Res&);

    CPPMM_IGNORE
    Res(Ptex::Res&&);
    ~Res();

    CPPMM_RENAME(assign)
    auto operator=(const Ptex::Res&) -> Ptex::Res&;

} CPPMM_VALUETYPE CPPMM_TRIVIALLY_COPYABLE; // struct Res

struct FaceInfo {
    using BoundType = Ptex::FaceInfo;

    CPPMM_RENAME(default)
    FaceInfo();

    CPPMM_RENAME(from_res)
    FaceInfo(Ptex::Res res_);

    CPPMM_RENAME(from_res_and_adjacency)
    FaceInfo(Ptex::Res res_, int adjfaces_[4], int adjedges_[4], bool isSubface_);

    auto adjedge(int eid) const -> Ptex::EdgeId;
    auto adjface(int eid) const -> int;
    auto isConstant() const -> bool;
    auto isNeighborhoodConstant() const -> bool;
    auto hasEdits() const -> bool;
    auto isSubface() const -> bool;
    auto setadjfaces(int f0, int f1, int f2, int f3) -> void;
    auto setadjedges(int e0, int e1, int e2, int e3) -> void;

    enum Flag {
        flag_constant = 1,
        flag_hasedits = 2,
        flag_nbconstant = 4,
        flag_subface = 8,
    } CPPMM_ENUM_PREFIX(flag_) CPPMM_RUSTIFY_ENUM;
} CPPMM_OPAQUEBYTES; // struct FaceInfo

struct PtexMetaData {
    using BoundType = Ptex::PtexMetaData;

    auto release() -> void;
    auto numKeys() -> int;
    auto getKey(int index, const char*& key, Ptex::MetaDataType& type) -> void;
    auto findKey(const char* key, int& index, Ptex::MetaDataType& type) -> bool;

    CPPMM_RENAME(getValueFromKeyChar)
    auto getValue(const char* key, const char*& value) -> void;

    CPPMM_RENAME(getValueFromIndexChar)
    auto getValue(int index, const char*& value) -> void;

    CPPMM_RENAME(getValueFromKeySChar)
    auto getValue(const char* key, const signed char*& value, int& count)
        -> void;

    CPPMM_RENAME(getValueFromIndexSChar)
    auto getValue(int index, const signed char*& value, int& count) -> void;

    CPPMM_RENAME(getValueFromKeyShort)
    auto getValue(const char* key, const short*& value, int& count) -> void;

    CPPMM_RENAME(getValueFromIndexShort)
    auto getValue(int index, const short*& value, int& count) -> void;

    CPPMM_RENAME(getValueFromKeyInt)
    auto getValue(const char* key, const int*& value, int& count) -> void;

    CPPMM_RENAME(getValueFromIndexInt)
    auto getValue(int index, const int*& value, int& count) -> void;

    CPPMM_RENAME(getValueFromKeyFloat)
    auto getValue(const char* key, const float*& value, int& count) -> void;

    CPPMM_RENAME(getValueFromIndexFloat)
    auto getValue(int index, const float*& value, int& count) -> void;

    CPPMM_RENAME(getValueFromKeyDouble)
    auto getValue(const char* key, const double*& value, int& count) -> void;

    CPPMM_RENAME(getValueFromIndexDouble)
    auto getValue(int index, const double*& value, int& count) -> void;

    auto operator=(const Ptex::PtexMetaData&) -> Ptex::PtexMetaData&;
} CPPMM_OPAQUEPTR; // struct PtexMetaData

struct PtexFaceData {
    using BoundType = Ptex::PtexFaceData;

    auto release() -> void;
    auto isConstant() -> bool;
    auto res() -> Ptex::Res;
    auto getPixel(int u, int v, void* result) -> void;
    auto getData() -> void*;
    auto isTiled() -> bool;
    auto tileRes() -> Ptex::Res;
    auto getTile(int tile) -> Ptex::PtexFaceData*;
    auto operator=(const Ptex::PtexFaceData&) -> Ptex::PtexFaceData&;
} CPPMM_OPAQUEPTR; // struct PtexFaceData

struct PtexTexture {
    using BoundType = Ptex::PtexTexture;

    static auto open(const char* path, std::string& error, bool premultiply)
        -> Ptex::PtexTexture*;
    auto release() -> void;
    auto path() -> const char*;
    auto getInfo() -> Ptex::PtexTexture::Info;
    auto meshType() -> Ptex::MeshType;
    auto dataType() -> Ptex::DataType;
    auto uBorderMode() -> Ptex::BorderMode;
    auto vBorderMode() -> Ptex::BorderMode;
    auto edgeFilterMode() -> Ptex::EdgeFilterMode;
    auto alphaChannel() -> int;
    auto numChannels() -> int;
    auto numFaces() -> int;
    auto hasEdits() -> bool;
    auto hasMipMaps() -> bool;
    auto getMetaData() -> Ptex::PtexMetaData*;
    auto getFaceInfo(int faceid) -> const Ptex::FaceInfo&;

    CPPMM_RENAME(getDataInto)
    auto getData(int faceid, void* buffer, int stride) -> void;

    CPPMM_RENAME(getDataIntoWithRes)
    auto getData(int faceid, void* buffer, int stride, Ptex::Res res) -> void;

    auto getData(int faceid) -> Ptex::PtexFaceData*;

    CPPMM_RENAME(getDataWithRes)
    auto getData(int faceid, Ptex::Res res) -> Ptex::PtexFaceData*;

    auto getPixel(int faceid, int u, int v, float* result, int firstchan,
                  int nchannels) -> void;

    CPPMM_RENAME(getPixelWithRes)
    auto getPixel(int faceid, int u, int v, float* result, int firstchan,
                  int nchannels, Ptex::Res res) -> void;
    auto operator=(const Ptex::PtexTexture&) -> Ptex::PtexTexture&;

    struct Info {
        using BoundType = Ptex::PtexTexture::Info;

    } CPPMM_OPAQUEBYTES; // struct Info

} CPPMM_OPAQUEPTR; // struct PtexTexture

struct PtexInputHandler {
    using BoundType = Ptex::PtexInputHandler;

    auto open(const char* path) -> void*;
    auto seek(void* handle, long long pos) -> void;
    auto read(void* buffer, unsigned long size, void* handle) -> unsigned long;
    auto close(void* handle) -> bool;
    auto lastError() -> const char*;
    auto operator=(const Ptex::PtexInputHandler&) -> Ptex::PtexInputHandler&;
} CPPMM_OPAQUEPTR; // struct PtexInputHandler

struct PtexErrorHandler {
    using BoundType = Ptex::PtexErrorHandler;

    auto reportError(const char* error) -> void;
    auto operator=(const Ptex::PtexErrorHandler&) -> Ptex::PtexErrorHandler&;
} CPPMM_OPAQUEPTR; // struct PtexErrorHandler

struct PtexCache {
    using BoundType = Ptex::PtexCache;

    static auto create(int maxFiles, size_t maxMem, bool premultiply,
                       Ptex::PtexInputHandler* inputHandler,
                       Ptex::PtexErrorHandler* errorHandler)
        -> Ptex::PtexCache*;
    auto release() -> void;
    auto setSearchPath(const char* path) -> void;
    auto getSearchPath() -> const char*;
    auto get(const char* path, std::string& error) -> Ptex::PtexTexture*;
    auto purge(Ptex::PtexTexture* texture) -> void;

    CPPMM_RENAME(purgePath)
    auto purge(const char* path) -> void;
    auto purgeAll() -> void;
    auto getStats(Ptex::PtexCache::Stats& stats) -> void;
    auto operator=(const Ptex::PtexCache&) -> Ptex::PtexCache&;

    struct Stats {
        using BoundType = Ptex::PtexCache::Stats;

    } CPPMM_OPAQUEBYTES; // struct Stats

} CPPMM_OPAQUEPTR; // struct PtexCache

struct PtexWriter {
    using BoundType = Ptex::PtexWriter;

    static auto open(const char* path, Ptex::MeshType mt, Ptex::DataType dt,
                     int nchannels, int alphachan, int nfaces,
                     std::string& error, bool genmipmaps) -> Ptex::PtexWriter*;
    static auto edit(const char* path, bool incremental, Ptex::MeshType mt,
                     Ptex::DataType dt, int nchannels, int alphachan,
                     int nfaces, std::string& error, bool genmipmaps)
        -> Ptex::PtexWriter*;
    static auto applyEdits(const char* path, std::string& error) -> bool;
    auto release() -> void;
    auto setBorderModes(Ptex::BorderMode uBorderMode,
                        Ptex::BorderMode vBorderMode) -> void;
    auto setEdgeFilterMode(Ptex::EdgeFilterMode edgeFilterMode) -> void;

    CPPMM_RENAME(writeMetaString)
    auto writeMeta(const char* key, const char* string) -> void;

    CPPMM_RENAME(writeMetaChar)
    auto writeMeta(const char* key, const signed char* value, int count)
        -> void;

    CPPMM_RENAME(writeMetaShort)
    auto writeMeta(const char* key, const short* value, int count) -> void;

    CPPMM_RENAME(writeMetaInt)
    auto writeMeta(const char* key, const int* value, int count) -> void;

    CPPMM_RENAME(writeMetaFloat)
    auto writeMeta(const char* key, const float* value, int count) -> void;

    CPPMM_RENAME(writeMetaDouble)
    auto writeMeta(const char* key, const double* value, int count) -> void;

    auto writeMeta(Ptex::PtexMetaData* data) -> void;

    auto writeFace(int faceid, const Ptex::FaceInfo& info, const void* data,
                   int stride) -> bool;
    auto writeConstantFace(int faceid, const Ptex::FaceInfo& info,
                           const void* data) -> bool;
    auto close(std::string& error) -> bool;
    auto operator=(const Ptex::PtexWriter&) -> Ptex::PtexWriter&;
} CPPMM_OPAQUEPTR; // struct PtexWriter

struct PtexFilter {
    using BoundType = Ptex::PtexFilter;

    static auto getFilter(Ptex::PtexTexture* tx,
                          const Ptex::PtexFilter::Options& opts)
        -> Ptex::PtexFilter*;
    auto release() -> void;
    auto eval(float* result, int firstchan, int nchannels, int faceid, float u,
              float v, float uw1, float vw1, float uw2, float vw2, float width,
              float blur) -> void;
    auto operator=(const Ptex::PtexFilter&) -> Ptex::PtexFilter&;

    enum FilterType {
        f_point = 0,
        f_bilinear = 1,
        f_box = 2,
        f_gaussian = 3,
        f_bicubic = 4,
        f_bspline = 5,
        f_catmullrom = 6,
        f_mitchell = 7,
    } CPPMM_ENUM_PREFIX(f_) CPPMM_RUSTIFY_ENUM;

    struct Options {
        using BoundType = Ptex::PtexFilter::Options;

        Options(Ptex::PtexFilter::FilterType filter_, bool lerp_,
                float sharpness_, bool noedgeblend_);
    } CPPMM_OPAQUEBYTES; // struct Options

} CPPMM_OPAQUEPTR; // struct PtexFilter

template <class T> struct PtexPtr {
    using BoundType = Ptex::PtexPtr<T>;

    PtexPtr<T>(T* ptr);
    ~PtexPtr<T>();

    CPPMM_IGNORE
    operator T*();

    CPPMM_IGNORE
    auto operator->() -> T*;
    auto get() -> T*;
    auto swap(Ptex::PtexPtr<T>& p) -> void;
    auto reset(T* ptr) -> void;
} CPPMM_OPAQUEBYTES; // struct PtexPtr

// PtexPtr explicit instantiations
//template class PtexPtr<Ptex::PtexCache>;
using PtexCachePtr = PtexPtr<Ptex::PtexCache>;

//template class PtexPtr<Ptex::PtexMetaData>;
using PtexMetaDataPtr = PtexPtr<Ptex::PtexMetaData>;

//template class PtexPtr<Ptex::PtexTexture>;
using PtexTexturePtr = PtexPtr<Ptex::PtexTexture>;

} // namespace v2_x

} // namespace Ptex

} // namespace cppmm_bind

// PtexPtr explicit instantiations
template class Ptex::v2_4::PtexPtr<Ptex::v2_4::PtexCache>;
template class Ptex::v2_4::PtexPtr<Ptex::v2_4::PtexMetaData>;
template class Ptex::v2_4::PtexPtr<Ptex::v2_4::PtexTexture>;
