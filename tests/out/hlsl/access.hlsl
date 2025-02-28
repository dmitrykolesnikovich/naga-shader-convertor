
typedef struct { float2 _0; float2 _1; } __mat2x2;
float2 __get_col_of_mat2x2(__mat2x2 mat, uint idx) {
    switch(idx) {
    case 0: { return mat._0; }
    case 1: { return mat._1; }
    default: { return (float2)0; }
    }
}
void __set_col_of_mat2x2(__mat2x2 mat, uint idx, float2 value) {
    switch(idx) {
    case 0: { mat._0 = value; break; }
    case 1: { mat._1 = value; break; }
    }
}
void __set_el_of_mat2x2(__mat2x2 mat, uint idx, uint vec_idx, float value) {
    switch(idx) {
    case 0: { mat._0[vec_idx] = value; break; }
    case 1: { mat._1[vec_idx] = value; break; }
    }
}

typedef struct { float2 _0; float2 _1; float2 _2; float2 _3; } __mat4x2;
float2 __get_col_of_mat4x2(__mat4x2 mat, uint idx) {
    switch(idx) {
    case 0: { return mat._0; }
    case 1: { return mat._1; }
    case 2: { return mat._2; }
    case 3: { return mat._3; }
    default: { return (float2)0; }
    }
}
void __set_col_of_mat4x2(__mat4x2 mat, uint idx, float2 value) {
    switch(idx) {
    case 0: { mat._0 = value; break; }
    case 1: { mat._1 = value; break; }
    case 2: { mat._2 = value; break; }
    case 3: { mat._3 = value; break; }
    }
}
void __set_el_of_mat4x2(__mat4x2 mat, uint idx, uint vec_idx, float value) {
    switch(idx) {
    case 0: { mat._0[vec_idx] = value; break; }
    case 1: { mat._1[vec_idx] = value; break; }
    case 2: { mat._2[vec_idx] = value; break; }
    case 3: { mat._3[vec_idx] = value; break; }
    }
}

struct GlobalConst {
    uint a;
    int _pad1_0;
    int _pad1_1;
    int _pad1_2;
    uint3 b;
    int c;
};

struct AlignedWrapper {
    int value;
    int _end_pad_0;
};

struct Baz {
    float2 m_0; float2 m_1; float2 m_2;
};

struct MatCx2InArray {
    __mat4x2 am[2];
};

GlobalConst ConstructGlobalConst(uint arg0, uint3 arg1, int arg2) {
    GlobalConst ret = (GlobalConst)0;
    ret.a = arg0;
    ret.b = arg1;
    ret.c = arg2;
    return ret;
}

typedef float4x2 ret_Constructarray2_float4x2_[2];
ret_Constructarray2_float4x2_ Constructarray2_float4x2_(float4x2 arg0, float4x2 arg1) {
    float4x2 ret[2] = { arg0, arg1 };
    return ret;
}

typedef float ret_Constructarray10_float_[10];
ret_Constructarray10_float_ Constructarray10_float_(float arg0, float arg1, float arg2, float arg3, float arg4, float arg5, float arg6, float arg7, float arg8, float arg9) {
    float ret[10] = { arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 };
    return ret;
}

typedef float ret_Constructarray5_array10_float__[5][10];
ret_Constructarray5_array10_float__ Constructarray5_array10_float__(float arg0[10], float arg1[10], float arg2[10], float arg3[10], float arg4[10]) {
    float ret[5][10] = { arg0, arg1, arg2, arg3, arg4 };
    return ret;
}

static GlobalConst global_const = ConstructGlobalConst(0u, uint3(0u, 0u, 0u), 0);
RWByteAddressBuffer bar : register(u0);
cbuffer baz : register(b1) { Baz baz; }
RWByteAddressBuffer qux : register(u2);
cbuffer nested_mat_cx2_ : register(b3) { MatCx2InArray nested_mat_cx2_; }
groupshared uint val;

float3x2 GetMatmOnBaz(Baz obj) {
    return float3x2(obj.m_0, obj.m_1, obj.m_2);
}

void SetMatmOnBaz(Baz obj, float3x2 mat) {
    obj.m_0 = mat[0];
    obj.m_1 = mat[1];
    obj.m_2 = mat[2];
}

void SetMatVecmOnBaz(Baz obj, float2 vec, uint mat_idx) {
    switch(mat_idx) {
    case 0: { obj.m_0 = vec; break; }
    case 1: { obj.m_1 = vec; break; }
    case 2: { obj.m_2 = vec; break; }
    }
}

void SetMatScalarmOnBaz(Baz obj, float scalar, uint mat_idx, uint vec_idx) {
    switch(mat_idx) {
    case 0: { obj.m_0[vec_idx] = scalar; break; }
    case 1: { obj.m_1[vec_idx] = scalar; break; }
    case 2: { obj.m_2[vec_idx] = scalar; break; }
    }
}

Baz ConstructBaz(float3x2 arg0) {
    Baz ret = (Baz)0;
    ret.m_0 = arg0[0];
    ret.m_1 = arg0[1];
    ret.m_2 = arg0[2];
    return ret;
}

void test_matrix_within_struct_accesses()
{
    int idx = (int)0;
    Baz t = (Baz)0;

    idx = 1;
    int _expr2 = idx;
    idx = (_expr2 - 1);
    float3x2 unnamed = GetMatmOnBaz(baz);
    float2 unnamed_1 = GetMatmOnBaz(baz)[0];
    int _expr15 = idx;
    float2 unnamed_2 = GetMatmOnBaz(baz)[_expr15];
    float unnamed_3 = GetMatmOnBaz(baz)[0].y;
    int _expr29 = idx;
    float unnamed_4 = GetMatmOnBaz(baz)[0][_expr29];
    int _expr34 = idx;
    float unnamed_5 = GetMatmOnBaz(baz)[_expr34].y;
    int _expr41 = idx;
    int _expr43 = idx;
    float unnamed_6 = GetMatmOnBaz(baz)[_expr41][_expr43];
    t = ConstructBaz(float3x2((1.0).xx, (2.0).xx, (3.0).xx));
    int _expr55 = idx;
    idx = (_expr55 + 1);
    SetMatmOnBaz(t, float3x2((6.0).xx, (5.0).xx, (4.0).xx));
    t.m_0 = (9.0).xx;
    int _expr72 = idx;
    SetMatVecmOnBaz(t, (90.0).xx, _expr72);
    t.m_0[1] = 10.0;
    int _expr85 = idx;
    t.m_0[_expr85] = 20.0;
    int _expr89 = idx;
    SetMatScalarmOnBaz(t, 30.0, _expr89, 1);
    int _expr95 = idx;
    int _expr97 = idx;
    SetMatScalarmOnBaz(t, 40.0, _expr95, _expr97);
    return;
}

MatCx2InArray ConstructMatCx2InArray(float4x2 arg0[2]) {
    MatCx2InArray ret = (MatCx2InArray)0;
    ret.am = (__mat4x2[2])arg0;
    return ret;
}

void test_matrix_within_array_within_struct_accesses()
{
    int idx_1 = (int)0;
    MatCx2InArray t_1 = (MatCx2InArray)0;

    idx_1 = 1;
    int _expr2 = idx_1;
    idx_1 = (_expr2 - 1);
    float4x2 unnamed_7[2] = ((float4x2[2])nested_mat_cx2_.am);
    float4x2 unnamed_8 = ((float4x2)nested_mat_cx2_.am[0]);
    float2 unnamed_9 = nested_mat_cx2_.am[0]._0;
    int _expr24 = idx_1;
    float2 unnamed_10 = __get_col_of_mat4x2(nested_mat_cx2_.am[0], _expr24);
    float unnamed_11 = nested_mat_cx2_.am[0]._0.y;
    int _expr42 = idx_1;
    float unnamed_12 = nested_mat_cx2_.am[0]._0[_expr42];
    int _expr49 = idx_1;
    float unnamed_13 = __get_col_of_mat4x2(nested_mat_cx2_.am[0], _expr49).y;
    int _expr58 = idx_1;
    int _expr60 = idx_1;
    float unnamed_14 = __get_col_of_mat4x2(nested_mat_cx2_.am[0], _expr58)[_expr60];
    t_1 = ConstructMatCx2InArray(Constructarray2_float4x2_(float4x2(float2(0.0, 0.0), float2(0.0, 0.0), float2(0.0, 0.0), float2(0.0, 0.0)), float4x2(float2(0.0, 0.0), float2(0.0, 0.0), float2(0.0, 0.0), float2(0.0, 0.0))));
    int _expr66 = idx_1;
    idx_1 = (_expr66 + 1);
    t_1.am = (__mat4x2[2])Constructarray2_float4x2_(float4x2(float2(0.0, 0.0), float2(0.0, 0.0), float2(0.0, 0.0), float2(0.0, 0.0)), float4x2(float2(0.0, 0.0), float2(0.0, 0.0), float2(0.0, 0.0), float2(0.0, 0.0)));
    t_1.am[0] = (__mat4x2)float4x2((8.0).xx, (7.0).xx, (6.0).xx, (5.0).xx);
    t_1.am[0]._0 = (9.0).xx;
    int _expr93 = idx_1;
    __set_col_of_mat4x2(t_1.am[0], _expr93, (90.0).xx);
    t_1.am[0]._0.y = 10.0;
    int _expr110 = idx_1;
    t_1.am[0]._0[_expr110] = 20.0;
    int _expr116 = idx_1;
    __set_el_of_mat4x2(t_1.am[0], _expr116, 1, 30.0);
    int _expr124 = idx_1;
    int _expr126 = idx_1;
    __set_el_of_mat4x2(t_1.am[0], _expr124, _expr126, 40.0);
    return;
}

float read_from_private(inout float foo_1)
{
    float _expr1 = foo_1;
    return _expr1;
}

float test_arr_as_arg(float a[5][10])
{
    return a[4][9];
}

void assign_through_ptr_fn(inout uint p)
{
    p = 42u;
    return;
}

typedef float4 ret_Constructarray2_float4_[2];
ret_Constructarray2_float4_ Constructarray2_float4_(float4 arg0, float4 arg1) {
    float4 ret[2] = { arg0, arg1 };
    return ret;
}

void assign_array_through_ptr_fn(inout float4 foo_2[2])
{
    foo_2 = Constructarray2_float4_((1.0).xxxx, (2.0).xxxx);
    return;
}

uint NagaBufferLengthRW(RWByteAddressBuffer buffer)
{
    uint ret;
    buffer.GetDimensions(ret);
    return ret;
}

typedef int ret_Constructarray5_int_[5];
ret_Constructarray5_int_ Constructarray5_int_(int arg0, int arg1, int arg2, int arg3, int arg4) {
    int ret[5] = { arg0, arg1, arg2, arg3, arg4 };
    return ret;
}

float4 foo_vert(uint vi : SV_VertexID) : SV_Position
{
    float foo = (float)0;
    int c2_[5] = (int[5])0;

    foo = 0.0;
    float baz_1 = foo;
    foo = 1.0;
    test_matrix_within_struct_accesses();
    test_matrix_within_array_within_struct_accesses();
    float4x3 _matrix = float4x3(asfloat(bar.Load3(0+0)), asfloat(bar.Load3(0+16)), asfloat(bar.Load3(0+32)), asfloat(bar.Load3(0+48)));
    uint2 arr_1[2] = {asuint(bar.Load2(144+0)), asuint(bar.Load2(144+8))};
    float b = asfloat(bar.Load(0+48+0));
    int a_1 = asint(bar.Load(0+(((NagaBufferLengthRW(bar) - 160) / 8) - 2u)*8+160));
    int2 c = asint(qux.Load2(0));
    const float _e34 = read_from_private(foo);
    c2_ = Constructarray5_int_(a_1, int(b), 3, 4, 5);
    c2_[(vi + 1u)] = 42;
    int value = c2_[vi];
    const float _e48 = test_arr_as_arg(Constructarray5_array10_float__(Constructarray10_float_(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), Constructarray10_float_(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), Constructarray10_float_(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), Constructarray10_float_(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), Constructarray10_float_(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)));
    return float4(mul(float4((value).xxxx), _matrix), 2.0);
}

typedef uint2 ret_Constructarray2_uint2_[2];
ret_Constructarray2_uint2_ Constructarray2_uint2_(uint2 arg0, uint2 arg1) {
    uint2 ret[2] = { arg0, arg1 };
    return ret;
}

float4 foo_frag() : SV_Target0
{
    bar.Store(8+16+0, asuint(1.0));
    {
        float4x3 _value2 = float4x3((0.0).xxx, (1.0).xxx, (2.0).xxx, (3.0).xxx);
        bar.Store3(0+0, asuint(_value2[0]));
        bar.Store3(0+16, asuint(_value2[1]));
        bar.Store3(0+32, asuint(_value2[2]));
        bar.Store3(0+48, asuint(_value2[3]));
    }
    {
        uint2 _value2[2] = Constructarray2_uint2_((0u).xx, (1u).xx);
        bar.Store2(144+0, asuint(_value2[0]));
        bar.Store2(144+8, asuint(_value2[1]));
    }
    bar.Store(0+8+160, asuint(1));
    qux.Store2(0, asuint(int2(0, 0)));
    return (0.0).xxxx;
}

[numthreads(1, 1, 1)]
void atomics()
{
    int tmp = (int)0;

    int value_1 = asint(bar.Load(96));
    int _e7; bar.InterlockedAdd(96, 5, _e7);
    tmp = _e7;
    int _e11; bar.InterlockedAdd(96, -5, _e11);
    tmp = _e11;
    int _e15; bar.InterlockedAnd(96, 5, _e15);
    tmp = _e15;
    int _e19; bar.InterlockedOr(96, 5, _e19);
    tmp = _e19;
    int _e23; bar.InterlockedXor(96, 5, _e23);
    tmp = _e23;
    int _e27; bar.InterlockedMin(96, 5, _e27);
    tmp = _e27;
    int _e31; bar.InterlockedMax(96, 5, _e31);
    tmp = _e31;
    int _e35; bar.InterlockedExchange(96, 5, _e35);
    tmp = _e35;
    bar.Store(96, asuint(value_1));
    return;
}

[numthreads(1, 1, 1)]
void assign_through_ptr(uint3 __global_invocation_id : SV_DispatchThreadID)
{
    if (all(__global_invocation_id == uint3(0u, 0u, 0u))) {
        val = (uint)0;
    }
    GroupMemoryBarrierWithGroupSync();
    float4 arr[2] = (float4[2])0;

    arr = Constructarray2_float4_((6.0).xxxx, (7.0).xxxx);
    assign_through_ptr_fn(val);
    assign_array_through_ptr_fn(arr);
    return;
}
