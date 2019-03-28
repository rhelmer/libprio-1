#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn atoi(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn srand(_: libc::c_uint);
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn time(_: *mut time_t) -> time_t;
    #[no_mangle]
    fn mp_init_size(mp: *mut mp_int, prec: mp_size) -> mp_err;
    #[no_mangle]
    fn mp_clear(mp: *mut mp_int);
    #[no_mangle]
    fn mp_mul(a: *const mp_int, b: *const mp_int, c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_sqr(a: *const mp_int, b: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mpp_random_size(a: *mut mp_int, prec: mp_size) -> mp_err;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_clock_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                           -> libc::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut libc::c_char,
                                           _: libc::c_int) -> libc::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: fpos_t,
                                           _: libc::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *const libc::c_char,
                                            _: libc::c_int) -> libc::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type clock_t = __darwin_clock_t;
pub type time_t = __darwin_time_t;
/*
 *  mpi.h
 *
 *  Arbitrary precision integer arithmetic library
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* #include "seccomon.h" */
/* no error, all is well */
/* yes (boolean result)  */
/* no (boolean result)   */
/* out of memory         */
/* argument out of range */
/* invalid parameter     */
/* answer is undefined   */
pub type mp_sign = libc::c_uint;
pub type mp_size = libc::c_uint;
pub type mp_err = libc::c_int;
/* C99, Solaris */
/* MP_ULONG_LONG_MAX was defined to be ULLONG_MAX */
/* HPUX */
/* We only use unsigned long for mp_digit iff long is more than 32 bits. */
pub type mp_digit = libc::c_ulong;
/* printf() format for 1 digit */
/* !defined(MP_NO_MP_WORD) */
/* MP_HALF_RADIX really ought to be called MP_SQRT_RADIX, but it's named
** MP_HALF_RADIX because it's the radix for MP_HALF_DIGITs, and it's
** consistent with the other _HALF_ names.
*/
/* Macros for accessing the mp_int internals           */
/* This defines the maximum I/O base (minimum is 2)   */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mp_int {
    pub sign: mp_sign,
    pub alloc: mp_size,
    pub used: mp_size,
    pub dp: *mut mp_digit,
}
/*
 * Test whether to include squaring code given the current settings
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* make sure squaring code is included */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut ntests: libc::c_int = 0;
    let mut prec: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut seed: libc::c_uint = 0;
    let mut start: clock_t = 0;
    let mut stop: clock_t = 0;
    let mut multime: libc::c_double = 0.;
    let mut sqrtime: libc::c_double = 0.;
    let mut a: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut c: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    seed = time(0 as *mut time_t) as libc::c_uint;
    if argc < 3i32 {
        fprintf(__stderrp,
                b"Usage: %s <ntests> <nbits>\n\x00" as *const u8 as
                    *const libc::c_char, *argv.offset(0isize));
        return 1i32
    }
    ntests = abs(atoi(*argv.offset(1isize)));
    if ntests == 0i32 {
        fprintf(__stderrp,
                b"%s: must request at least 1 test.\n\x00" as *const u8 as
                    *const libc::c_char, *argv.offset(0isize));
        return 1i32
    }
    prec = abs(atoi(*argv.offset(2isize)));
    if prec < 8i32 {
        fprintf(__stderrp,
                b"%s: must request at least %d bits.\n\x00" as *const u8 as
                    *const libc::c_char, *argv.offset(0isize), 8i32);
        return 1i32
    }
    prec =
        (prec as
             libc::c_ulong).wrapping_add((8i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                              as
                                                                              libc::c_ulong).wrapping_sub(1i32
                                                                                                              as
                                                                                                              libc::c_ulong)).wrapping_div((8i32
                                                                                                                                                as
                                                                                                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                                                                                                as
                                                                                                                                                                                libc::c_ulong))
            as libc::c_int;
    mp_init_size(&mut a, prec as mp_size);
    mp_init_size(&mut c, (2i32 * prec) as mp_size);
    srand(seed);
    start = clock();
    ix = 0i32;
    while ix < ntests {
        mpp_random_size(&mut a, prec as mp_size);
        mp_mul(&mut a, &mut a, &mut c);
        ix += 1
    }
    stop = clock();
    multime =
        stop.wrapping_sub(start) as libc::c_double /
            1000000i32 as libc::c_double;
    srand(seed);
    start = clock();
    ix = 0i32;
    while ix < ntests {
        mpp_random_size(&mut a, prec as mp_size);
        mp_sqr(&mut a, &mut c);
        ix += 1
    }
    stop = clock();
    sqrtime =
        stop.wrapping_sub(start) as libc::c_double /
            1000000i32 as libc::c_double;
    printf(b"Multiply: %.4f\n\x00" as *const u8 as *const libc::c_char,
           multime);
    printf(b"Square:   %.4f\n\x00" as *const u8 as *const libc::c_char,
           sqrtime);
    if multime < sqrtime {
        printf(b"Speedup:  %.1f%%\n\x00" as *const u8 as *const libc::c_char,
               100.0f64 * (1.0f64 - multime / sqrtime));
        printf(b"Prefer:   multiply\n\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        printf(b"Speedup:  %.1f%%\n\x00" as *const u8 as *const libc::c_char,
               100.0f64 * (1.0f64 - sqrtime / multime));
        printf(b"Prefer:   square\n\x00" as *const u8 as *const libc::c_char);
    }
    mp_clear(&mut a);
    mp_clear(&mut c);
    return 0i32;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}