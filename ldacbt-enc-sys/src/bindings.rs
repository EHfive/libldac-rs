/* automatically generated by rust-bindgen 0.69.1 */

pub const LDACBT_ENC_LSU: u32 = 128;
pub const LDACBT_MAX_LSU: u32 = 512;
pub const LDAC_CCI_MONO: u32 = 0;
pub const LDAC_CCI_DUAL_CHANNEL: u32 = 1;
pub const LDAC_CCI_STEREO: u32 = 2;
pub const LDACBT_MAX_NBYTES: u32 = 1024;
pub const LDAC_PRCNCH: u32 = 2;
pub const LDACBT_EQMID_INC_QUALITY: u32 = 1;
pub const LDACBT_EQMID_INC_CONNECTION: i32 = -1;
pub const LDACBT_ERR_NONE: u32 = 0;
pub const LDACBT_ERR_NON_FATAL: u32 = 1;
pub const LDACBT_ERR_BIT_ALLOCATION: u32 = 5;
pub const LDACBT_ERR_NOT_IMPLEMENTED: u32 = 128;
pub const LDACBT_ERR_NON_FATAL_ENCODE: u32 = 132;
pub const LDACBT_ERR_FATAL: u32 = 256;
pub const LDACBT_ERR_SYNTAX_BAND: u32 = 260;
pub const LDACBT_ERR_SYNTAX_GRAD_A: u32 = 261;
pub const LDACBT_ERR_SYNTAX_GRAD_B: u32 = 262;
pub const LDACBT_ERR_SYNTAX_GRAD_C: u32 = 263;
pub const LDACBT_ERR_SYNTAX_GRAD_D: u32 = 264;
pub const LDACBT_ERR_SYNTAX_GRAD_E: u32 = 265;
pub const LDACBT_ERR_SYNTAX_IDSF: u32 = 266;
pub const LDACBT_ERR_SYNTAX_SPEC: u32 = 267;
pub const LDACBT_ERR_BIT_PACKING: u32 = 280;
pub const LDACBT_ERR_ALLOC_MEMORY: u32 = 300;
pub const LDACBT_ERR_FATAL_HANDLE: u32 = 512;
pub const LDACBT_ERR_ILL_SYNCWORD: u32 = 516;
pub const LDACBT_ERR_ILL_SMPL_FORMAT: u32 = 517;
pub const LDACBT_ERR_ILL_PARAM: u32 = 518;
pub const LDACBT_ERR_ASSERT_SAMPLING_FREQ: u32 = 530;
pub const LDACBT_ERR_ASSERT_SUP_SAMPLING_FREQ: u32 = 531;
pub const LDACBT_ERR_CHECK_SAMPLING_FREQ: u32 = 532;
pub const LDACBT_ERR_ASSERT_CHANNEL_CONFIG: u32 = 533;
pub const LDACBT_ERR_CHECK_CHANNEL_CONFIG: u32 = 534;
pub const LDACBT_ERR_ASSERT_FRAME_LENGTH: u32 = 535;
pub const LDACBT_ERR_ASSERT_SUP_FRAME_LENGTH: u32 = 536;
pub const LDACBT_ERR_ASSERT_FRAME_STATUS: u32 = 537;
pub const LDACBT_ERR_ASSERT_NSHIFT: u32 = 538;
pub const LDACBT_ERR_ASSERT_CHANNEL_MODE: u32 = 539;
pub const LDACBT_ERR_ENC_INIT_ALLOC: u32 = 550;
pub const LDACBT_ERR_ENC_ILL_GRADMODE: u32 = 551;
pub const LDACBT_ERR_ENC_ILL_GRADPAR_A: u32 = 552;
pub const LDACBT_ERR_ENC_ILL_GRADPAR_B: u32 = 553;
pub const LDACBT_ERR_ENC_ILL_GRADPAR_C: u32 = 554;
pub const LDACBT_ERR_ENC_ILL_GRADPAR_D: u32 = 555;
pub const LDACBT_ERR_ENC_ILL_NBANDS: u32 = 556;
pub const LDACBT_ERR_PACK_BLOCK_FAILED: u32 = 557;
pub const LDACBT_ERR_DEC_INIT_ALLOC: u32 = 570;
pub const LDACBT_ERR_INPUT_BUFFER_SIZE: u32 = 571;
pub const LDACBT_ERR_UNPACK_BLOCK_FAILED: u32 = 572;
pub const LDACBT_ERR_UNPACK_BLOCK_ALIGN: u32 = 573;
pub const LDACBT_ERR_UNPACK_FRAME_ALIGN: u32 = 574;
pub const LDACBT_ERR_FRAME_LENGTH_OVER: u32 = 575;
pub const LDACBT_ERR_FRAME_ALIGN_OVER: u32 = 576;
pub const LDACBT_ERR_ALTER_EQMID_LIMITED: u32 = 21;
pub const LDACBT_ERR_HANDLE_NOT_INIT: u32 = 1000;
pub const LDACBT_ERR_ILL_EQMID: u32 = 1024;
pub const LDACBT_ERR_ILL_SAMPLING_FREQ: u32 = 1025;
pub const LDACBT_ERR_ILL_NUM_CHANNEL: u32 = 1026;
pub const LDACBT_ERR_ILL_MTU_SIZE: u32 = 1027;
pub const LDACBT_ERR_DEC_CONFIG_UPDATED: u32 = 40;
pub const LDACBT_MEDIA_CODEC_SC_SZ: u32 = 12;
pub const LDACBT_VENDOR_ID0: u32 = 45;
pub const LDACBT_VENDOR_ID1: u32 = 1;
pub const LDACBT_VENDOR_ID2: u32 = 0;
pub const LDACBT_VENDOR_ID3: u32 = 0;
pub const LDACBT_CODEC_ID0: u32 = 170;
pub const LDACBT_CODEC_ID1: u32 = 0;
pub const LDACBT_SAMPLING_FREQ_044100: u32 = 32;
pub const LDACBT_SAMPLING_FREQ_048000: u32 = 16;
pub const LDACBT_SAMPLING_FREQ_088200: u32 = 8;
pub const LDACBT_SAMPLING_FREQ_096000: u32 = 4;
pub const LDACBT_SAMPLING_FREQ_176400: u32 = 2;
pub const LDACBT_SAMPLING_FREQ_192000: u32 = 1;
pub const LDACBT_CHANNEL_MODE_MONO: u32 = 4;
pub const LDACBT_CHANNEL_MODE_DUAL_CHANNEL: u32 = 2;
pub const LDACBT_CHANNEL_MODE_STEREO: u32 = 1;
pub const LDACBT_SMPL_FMT_T_LDACBT_SMPL_FMT_S16: LDACBT_SMPL_FMT_T = 2;
pub const LDACBT_SMPL_FMT_T_LDACBT_SMPL_FMT_S24: LDACBT_SMPL_FMT_T = 3;
pub const LDACBT_SMPL_FMT_T_LDACBT_SMPL_FMT_S32: LDACBT_SMPL_FMT_T = 4;
pub const LDACBT_SMPL_FMT_T_LDACBT_SMPL_FMT_F32: LDACBT_SMPL_FMT_T = 5;
pub type LDACBT_SMPL_FMT_T = ::core::ffi::c_uint;
pub const LDACBT_EQMID_HQ: _bindgen_ty_1 = 0;
pub const LDACBT_EQMID_SQ: _bindgen_ty_1 = 1;
pub const LDACBT_EQMID_MQ: _bindgen_ty_1 = 2;
pub const LDACBT_EQMID_NUM: _bindgen_ty_1 = 3;
pub const LDACBT_EQMID_ABR: _bindgen_ty_1 = 127;
pub type _bindgen_ty_1 = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _st_ldacbt_handle {
    _unused: [u8; 0],
}
pub type HANDLE_LDAC_BT = *mut _st_ldacbt_handle;
extern "C" {
    pub fn ldacBT_get_handle() -> HANDLE_LDAC_BT;
}
extern "C" {
    pub fn ldacBT_free_handle(hLdacBt: HANDLE_LDAC_BT);
}
extern "C" {
    pub fn ldacBT_close_handle(hLdacBt: HANDLE_LDAC_BT);
}
extern "C" {
    pub fn ldacBT_get_version() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ldacBT_get_sampling_freq(hLdacBt: HANDLE_LDAC_BT) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ldacBT_get_bitrate(hLdacBt: HANDLE_LDAC_BT) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ldacBT_init_handle_encode(
        hLdacBt: HANDLE_LDAC_BT,
        mtu: ::core::ffi::c_int,
        eqmid: ::core::ffi::c_int,
        cm: ::core::ffi::c_int,
        fmt: LDACBT_SMPL_FMT_T,
        sf: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ldacBT_set_eqmid(
        hLdacBt: HANDLE_LDAC_BT,
        eqmid: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ldacBT_get_eqmid(hLdacBt: HANDLE_LDAC_BT) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ldacBT_alter_eqmid_priority(
        hLdacBt: HANDLE_LDAC_BT,
        priority: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ldacBT_encode(
        hLdacBt: HANDLE_LDAC_BT,
        p_pcm: *mut ::core::ffi::c_void,
        pcm_used: *mut ::core::ffi::c_int,
        p_stream: *mut ::core::ffi::c_uchar,
        stream_sz: *mut ::core::ffi::c_int,
        frame_num: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn ldacBT_get_error_code(hLdacBt: HANDLE_LDAC_BT) -> ::core::ffi::c_int;
}
