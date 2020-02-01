/* automatically generated by rust-bindgen */

pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SentencePieceProcessor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SentencePieceText {
    _unused: [u8; 0],
}
extern "C" {
    pub fn spp_encode_as_serialized_proto(
        spp: *mut SentencePieceProcessor,
        sentence: *const ::std::os::raw::c_char,
        len: *mut usize,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn spp_new() -> *mut SentencePieceProcessor;
}
extern "C" {
    pub fn spp_from_serialized_proto(
        spp: *mut SentencePieceProcessor,
        data: *const ::std::os::raw::c_char,
        len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn spp_load(
        spp: *mut SentencePieceProcessor,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn spp_free(spp: *mut SentencePieceProcessor);
}