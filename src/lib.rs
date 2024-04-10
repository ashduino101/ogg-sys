#![allow(missing_copy_implementations)]
#![allow(non_camel_case_types)]

use std::ffi;

pub type ogg_int64_t = i64;

#[repr(C)]
pub struct ogg_iovec_t {
    pub iov_base: *mut std::ffi::c_void,
    pub iov_len: isize,
}

#[repr(C)]
pub struct oggpack_buffer {
    pub endbyte: std::ffi::c_long,
    pub endbit: std::ffi::c_int,

    pub buffer: *mut std::ffi::c_uchar,
    pub ptr: *mut std::ffi::c_uchar,
    pub storage: std::ffi::c_long,
}

#[repr(C)]
pub struct ogg_page {
    pub header: *mut std::ffi::c_uchar,
    pub header_len: std::ffi::c_long,
    pub body: *mut std::ffi::c_uchar,
    pub body_len: std::ffi::c_long,
}

#[repr(C)]
pub struct ogg_stream_state {
    pub body_data: *mut std::ffi::c_uchar,
    pub body_storage: std::ffi::c_long,
    pub body_fill: std::ffi::c_long,
    pub body_returned: std::ffi::c_long,

    pub lacing_vals: *mut std::ffi::c_int,
    pub granule_vals: *mut ogg_int64_t,

    pub lacing_storage: std::ffi::c_long,
    pub lacing_fill: std::ffi::c_long,
    pub lacing_packet: std::ffi::c_long,
    pub lacing_returned: std::ffi::c_long,

    pub header: [std::ffi::c_uchar; 282],
    pub header_fill: std::ffi::c_int,

    pub e_o_s: std::ffi::c_int,
    pub b_o_s: std::ffi::c_int,
    pub serialno: std::ffi::c_long,
    pub pageno: std::ffi::c_long,
    pub packetno: ogg_int64_t,
    pub granulepos: ogg_int64_t,
}

#[repr(C)]
pub struct ogg_packet {
    pub packet: *mut std::ffi::c_uchar,
    pub bytes: std::ffi::c_long,
    pub b_o_s: std::ffi::c_long,
    pub e_o_s: std::ffi::c_long,

    pub granulepos: ogg_int64_t,

    pub packetno: ogg_int64_t,
}

#[repr(C)]
pub struct ogg_sync_state {
    pub data: *mut std::ffi::c_uchar,
    pub storage: std::ffi::c_int,
    pub fill: std::ffi::c_int,
    pub returned: std::ffi::c_int,

    pub unsynced: std::ffi::c_int,
    pub headerbytes: std::ffi::c_int,
    pub bodybytes: std::ffi::c_int,
}

extern {
    pub fn oggpack_writeinit(b: *mut oggpack_buffer);
    pub fn oggpack_writecheck(b: *mut oggpack_buffer) -> std::ffi::c_int;
    pub fn oggpack_writetrunc(b: *mut oggpack_buffer, bits: std::ffi::c_long);
    pub fn oggpack_writealign(b: *mut oggpack_buffer);
    pub fn oggpack_writecopy(b: *mut oggpack_buffer, source: *mut std::ffi::c_void, bits: std::ffi::c_long);
    pub fn oggpack_reset(b: *mut oggpack_buffer);
    pub fn oggpack_writeclear(b: *mut oggpack_buffer);
    pub fn oggpack_readinit(b: *mut oggpack_buffer, buf: *mut std::ffi::c_uchar, bytes: std::ffi::c_int)
        -> std::ffi::c_int;
    pub fn oggpack_write(b: *mut oggpack_buffer, value: std::ffi::c_ulong, bits: std::ffi::c_int)
        -> std::ffi::c_int;
    pub fn oggpack_look(b: *mut oggpack_buffer, bits: std::ffi::c_int) -> std::ffi::c_long;
    pub fn oggpack_look1(b: *mut oggpack_buffer) -> std::ffi::c_long;
    pub fn oggpack_adv(b: *mut oggpack_buffer, bits: std::ffi::c_int) -> std::ffi::c_int;
    pub fn oggpack_adv1(b: *mut oggpack_buffer);
    pub fn oggpack_read(b: *mut oggpack_buffer, bits: std::ffi::c_int) -> std::ffi::c_long;
    pub fn oggpack_read1(b: *mut oggpack_buffer) -> std::ffi::c_long;
    pub fn oggpack_bytes(b: *mut oggpack_buffer) -> std::ffi::c_long;
    pub fn oggpack_bits(b: *mut oggpack_buffer) -> std::ffi::c_long;
    pub fn oggpack_get_buffer(b: *mut oggpack_buffer) -> *const std::ffi::c_uchar;

    pub fn oggpackB_writeinit(b: *mut oggpack_buffer);
    pub fn oggpackB_writecheck(b: *mut oggpack_buffer) -> std::ffi::c_int;
    pub fn oggpackB_writetrunc(b: *mut oggpack_buffer, bits: std::ffi::c_long);
    pub fn oggpackB_writealign(b: *mut oggpack_buffer);
    pub fn oggpackB_writecopy(b: *mut oggpack_buffer, source: *mut std::ffi::c_void,
        bits: std::ffi::c_long);
    pub fn oggpackB_reset(b: *mut oggpack_buffer);
    pub fn oggpackB_writeclear(b: *mut oggpack_buffer);
    pub fn oggpackB_readinit(b: *mut oggpack_buffer, buf: *mut std::ffi::c_uchar, bytes: std::ffi::c_int)
        -> std::ffi::c_int;
    pub fn oggpackB_write(b: *mut oggpack_buffer, value: std::ffi::c_ulong, bits: std::ffi::c_int)
        -> std::ffi::c_int;
    pub fn oggpackB_look(b: *mut oggpack_buffer,bits: std::ffi::c_int) -> std::ffi::c_long;
    pub fn oggpackB_look1(b: *mut oggpack_buffer) -> std::ffi::c_long;
    pub fn oggpackB_adv(b: *mut oggpack_buffer, bits: std::ffi::c_int) -> std::ffi::c_int;
    pub fn oggpackB_adv1(b: *mut oggpack_buffer);
    pub fn oggpackB_read(b: *mut oggpack_buffer, bits: std::ffi::c_int) -> std::ffi::c_long;
    pub fn oggpackB_read1(b: *mut oggpack_buffer) -> std::ffi::c_long;
    pub fn oggpackB_bytes(b: *mut oggpack_buffer) -> std::ffi::c_long;
    pub fn oggpackB_bits(b: *mut oggpack_buffer) -> std::ffi::c_long;
    pub fn oggpackB_get_buffer(b: *mut oggpack_buffer) -> *const std::ffi::c_uchar;

    pub fn ogg_stream_packetin(os: *mut ogg_stream_state, op: *mut ogg_packet) -> std::ffi::c_int;
    pub fn ogg_stream_iovecin(os: *mut ogg_stream_state, iov: *mut ogg_iovec_t, count: std::ffi::c_int,
        e_o_s: std::ffi::c_long, granulepos: ogg_int64_t) -> std::ffi::c_int;
    pub fn ogg_stream_pageout(os: *mut ogg_stream_state, og: *mut ogg_page) -> std::ffi::c_int;
    pub fn ogg_stream_pageout_fill(os: *mut ogg_stream_state, og: *mut ogg_page,
        nfill: std::ffi::c_int) -> std::ffi::c_int;
    pub fn ogg_stream_flush(os: *mut ogg_stream_state, og: *mut ogg_page) -> std::ffi::c_int;
    pub fn ogg_stream_flush_fill(os: *mut ogg_stream_state, og: *mut ogg_page,
        nfill: std::ffi::c_int) -> std::ffi::c_int;

    pub fn ogg_sync_init(oy: *mut ogg_sync_state) -> std::ffi::c_int;
    pub fn ogg_sync_clear(oy: *mut ogg_sync_state) -> std::ffi::c_int;
    pub fn ogg_sync_reset(oy: *mut ogg_sync_state) -> std::ffi::c_int;
    pub fn ogg_sync_destroy(oy: *mut ogg_sync_state) -> std::ffi::c_int;
    pub fn ogg_sync_check(oy: *mut ogg_sync_state) -> std::ffi::c_int;

    pub fn ogg_sync_buffer(oy: *mut ogg_sync_state, size: std::ffi::c_long) -> *const std::ffi::c_char;
    pub fn ogg_sync_wrote(oy: *mut ogg_sync_state, bytes: std::ffi::c_long) -> std::ffi::c_int;
    pub fn ogg_sync_pageseek(oy: *mut ogg_sync_state,og: *mut ogg_page) -> std::ffi::c_long;
    pub fn ogg_sync_pageout(oy: *mut ogg_sync_state, og: *mut ogg_page) -> std::ffi::c_int;
    pub fn ogg_stream_pagein(os: *mut ogg_stream_state, og: *mut ogg_page) -> std::ffi::c_int;
    pub fn ogg_stream_packetout(os: *mut ogg_stream_state,op: *mut ogg_packet) -> std::ffi::c_int;
    pub fn ogg_stream_packetpeek(os: *mut ogg_stream_state,op: *mut ogg_packet) -> std::ffi::c_int;

    pub fn ogg_stream_init(os: *mut ogg_stream_state, serialno: std::ffi::c_int) -> std::ffi::c_int;
    pub fn ogg_stream_clear(os: *mut ogg_stream_state) -> std::ffi::c_int;
    pub fn ogg_stream_reset(os: *mut ogg_stream_state) -> std::ffi::c_int;
    pub fn ogg_stream_reset_serialno(os: *mut ogg_stream_state, serialno: std::ffi::c_int)
        -> std::ffi::c_int;
    pub fn ogg_stream_destroy(os: *mut ogg_stream_state) -> std::ffi::c_int;
    pub fn ogg_stream_check(os: *mut ogg_stream_state) -> std::ffi::c_int;
    pub fn ogg_stream_eos(os: *mut ogg_stream_state) -> std::ffi::c_int;

    pub fn ogg_page_checksum_set(og: *mut ogg_page);

    pub fn ogg_page_version(og: *const ogg_page) -> std::ffi::c_int;
    pub fn ogg_page_continued(og: *const ogg_page) -> std::ffi::c_int;
    pub fn ogg_page_bos(og: *const ogg_page) -> std::ffi::c_int;
    pub fn ogg_page_eos(og: *const ogg_page) -> std::ffi::c_int;
    pub fn ogg_page_granulepos(og: *const ogg_page) -> ogg_int64_t;
    pub fn ogg_page_serialno(og: *const ogg_page) -> std::ffi::c_int;
    pub fn ogg_page_pageno(og: *const ogg_page) -> std::ffi::c_long;
    pub fn ogg_page_packets(og: *const ogg_page) -> std::ffi::c_int;

    pub fn ogg_packet_clear(op: *mut ogg_packet);

}
