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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fputc(_: libc::c_int, _: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
    #[no_mangle]
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    #[no_mangle]
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn labs(_: libc::c_long) -> libc::c_long;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *const libc::c_char)
                               -> __darwin_rune_t>,
    pub __sputrune: Option<unsafe extern "C" fn(_: __darwin_rune_t,
                                                _: *mut libc::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *mut libc::c_char)
                               -> libc::c_int>,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
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
unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
    return (_c & !0x7fi32 == 0i32) as libc::c_int;
}
unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t,
                              mut _f: libc::c_ulong) -> libc::c_int {
    return if 0 != isascii(_c) {
               (0 !=
                    _DefaultRuneLocale.__runetype[_c as usize] as
                        libc::c_ulong & _f) as libc::c_int
           } else { (0 != __maskrune(_c, _f)) as libc::c_int };
}
unsafe extern "C" fn __isctype(mut _c: __darwin_ct_rune_t,
                               mut _f: libc::c_ulong) -> __darwin_ct_rune_t {
    return if _c < 0i32 || _c >= 1i32 << 8i32 {
               0i32
           } else {
               (0 !=
                    _DefaultRuneLocale.__runetype[_c as usize] as
                        libc::c_ulong & _f) as libc::c_int
           };
}
unsafe extern "C" fn isdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x400i64 as libc::c_ulong);
}
unsafe extern "C" fn islower(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x1000i64 as libc::c_ulong);
}
unsafe extern "C" fn isupper(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x8000i64 as libc::c_ulong);
}
unsafe extern "C" fn tolower(mut _c: libc::c_int) -> libc::c_int {
    return __tolower(_c);
}
unsafe extern "C" fn toupper(mut _c: libc::c_int) -> libc::c_int {
    return __toupper(_c);
}
/* Default precision       */
#[no_mangle]
pub unsafe extern "C" fn mp_get_prec() -> mp_size { return s_mp_defprec; }
/* }}} */
/* {{{ Default precision manipulation */
/* Default precision for newly created mp_int's      */
static mut s_mp_defprec: mp_size = 64i32 as mp_size;
#[no_mangle]
pub unsafe extern "C" fn mp_set_prec(mut prec: mp_size) {
    if prec == 0i32 as libc::c_uint {
        s_mp_defprec = 64i32 as mp_size
    } else { s_mp_defprec = prec };
}
/* Memory management       */
#[no_mangle]
pub unsafe extern "C" fn mp_init(mut mp: *mut mp_int) -> mp_err {
    return mp_init_size(mp, s_mp_defprec);
}
#[no_mangle]
pub unsafe extern "C" fn mp_init_size(mut mp: *mut mp_int, mut prec: mp_size)
 -> mp_err {
    if !(!mp.is_null() && prec > 0i32 as libc::c_uint) { return -4i32 }
    prec =
        prec.wrapping_add(s_mp_defprec).wrapping_sub(1i32 as
                                                         libc::c_uint).wrapping_div(s_mp_defprec).wrapping_mul(s_mp_defprec);
    (*mp).dp =
        s_mp_alloc(prec as size_t,
                   ::std::mem::size_of::<mp_digit>() as libc::c_ulong) as
            *mut mp_digit;
    if (*mp).dp.is_null() { return -2i32 }
    (*mp).sign = 0i32 as mp_sign;
    (*mp).used = 1i32 as mp_size;
    (*mp).alloc = prec;
    return 0i32;
}
/* general allocator     */
#[no_mangle]
pub unsafe extern "C" fn s_mp_alloc(mut nb: size_t, mut ni: size_t)
 -> *mut libc::c_void {
    return calloc(nb, ni);
}
#[no_mangle]
pub unsafe extern "C" fn mp_init_copy(mut mp: *mut mp_int,
                                      mut from: *const mp_int) -> mp_err {
    if !(!mp.is_null() && !from.is_null()) { return -4i32 }
    if mp == from as *mut mp_int { return 0i32 }
    (*mp).dp =
        s_mp_alloc((*from).alloc as size_t,
                   ::std::mem::size_of::<mp_digit>() as libc::c_ulong) as
            *mut mp_digit;
    if (*mp).dp.is_null() { return -2i32 }
    s_mp_copy((*from).dp, (*mp).dp, (*from).used);
    (*mp).used = (*from).used;
    (*mp).alloc = (*from).alloc;
    (*mp).sign = (*from).sign;
    return 0i32;
}
/* copy */
#[no_mangle]
pub unsafe extern "C" fn s_mp_copy(mut sp: *const mp_digit,
                                   mut dp: *mut mp_digit,
                                   mut count: mp_size) {
    memcpy(dp as *mut libc::c_void, sp as *const libc::c_void,
           (count as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn mp_copy(mut from: *const mp_int, mut to: *mut mp_int)
 -> mp_err {
    if !(!from.is_null() && !to.is_null()) { return -4i32 }
    if from == to { return 0i32 }
    let mut tmp: *mut mp_digit = 0 as *mut mp_digit;
    if (*to).alloc >= (*from).used {
        s_mp_setz((*to).dp.offset((*from).used as isize),
                  (*to).alloc.wrapping_sub((*from).used));
        s_mp_copy((*from).dp, (*to).dp, (*from).used);
    } else {
        tmp =
            s_mp_alloc((*from).alloc as size_t,
                       ::std::mem::size_of::<mp_digit>() as libc::c_ulong) as
                *mut mp_digit;
        if tmp.is_null() { return -2i32 }
        s_mp_copy((*from).dp, tmp, (*from).used);
        if !(*to).dp.is_null() {
            s_mp_setz((*to).dp, (*to).alloc);
            s_mp_free((*to).dp as *mut libc::c_void);
        }
        (*to).dp = tmp;
        (*to).alloc = (*from).alloc
    }
    (*to).used = (*from).used;
    (*to).sign = (*from).sign;
    return 0i32;
}
/* general free function */
#[no_mangle]
pub unsafe extern "C" fn s_mp_free(mut ptr: *mut libc::c_void) {
    if !ptr.is_null() { free(ptr); };
}
/* if MP_LOGTAB */
/* }}} */
/* {{{ Digit arithmetic macros */
/*
  When adding and multiplying digits, the results can be larger than
  can be contained in an mp_digit.  Thus, an mp_word is used.  These
  macros mask off the upper and lower digits of the mp_word (the
  mp_word may be more than 2 mp_digits wide, but we only concern
  ourselves with the low-order 2 mp_digits)
 */
/* }}} */
/* {{{ Comparison constants */
/* }}} */
/* {{{ private function declarations */
/* zero digits           */
#[no_mangle]
pub unsafe extern "C" fn s_mp_setz(mut dp: *mut mp_digit,
                                   mut count: mp_size) {
    memset(dp as *mut libc::c_void, 0i32,
           (count as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn mp_exch(mut mp1: *mut mp_int, mut mp2: *mut mp_int) {
    if mp1.is_null() || mp2.is_null() { return }
    s_mp_exch(mp1, mp2);
}
/* swap a and b in place   */
#[no_mangle]
pub unsafe extern "C" fn s_mp_exch(mut a: *mut mp_int, mut b: *mut mp_int) {
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    if a.is_null() || b.is_null() { return }
    tmp = *a;
    *a = *b;
    *b = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn mp_clear(mut mp: *mut mp_int) {
    if mp.is_null() { return }
    if !(*mp).dp.is_null() {
        s_mp_setz((*mp).dp, (*mp).alloc);
        s_mp_free((*mp).dp as *mut libc::c_void);
        (*mp).dp = 0 as *mut mp_digit
    }
    (*mp).used = 0i32 as mp_size;
    (*mp).alloc = 0i32 as mp_size;
}
#[no_mangle]
pub unsafe extern "C" fn mp_zero(mut mp: *mut mp_int) {
    if mp.is_null() { return }
    s_mp_setz((*mp).dp, (*mp).alloc);
    (*mp).used = 1i32 as mp_size;
    (*mp).sign = 0i32 as mp_sign;
}
#[no_mangle]
pub unsafe extern "C" fn mp_set(mut mp: *mut mp_int, mut d: mp_digit) {
    if mp.is_null() { return }
    mp_zero(mp);
    *(*mp).dp.offset(0isize) = d;
}
#[no_mangle]
pub unsafe extern "C" fn mp_set_int(mut mp: *mut mp_int, mut z: libc::c_long)
 -> mp_err {
    let mut ix: libc::c_int = 0;
    let mut v: libc::c_ulong = labs(z) as libc::c_ulong;
    let mut res: mp_err = 0;
    if mp.is_null() { return -4i32 }
    mp_zero(mp);
    if z == 0i32 as libc::c_long { return 0i32 }
    if ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong <=
           ::std::mem::size_of::<mp_digit>() as libc::c_ulong {
        *(*mp).dp.offset(0isize) = v
    } else {
        ix =
            (::std::mem::size_of::<libc::c_long>() as
                 libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as
                libc::c_int;
        while ix >= 0i32 {
            res = s_mp_mul_d(mp, (127i32 * 2i32 + 1i32 + 1i32) as mp_digit);
            if res != 0i32 { return res }
            res =
                s_mp_add_d(mp,
                           v >> ix * 8i32 &
                               (127i32 * 2i32 + 1i32) as libc::c_ulong);
            if res != 0i32 { return res }
            ix -= 1
        }
    }
    if z < 0i32 as libc::c_long { (*mp).sign = 1i32 as mp_sign }
    return 0i32;
}
/* normalize for division  */
/* unsigned digit addition */
#[no_mangle]
pub unsafe extern "C" fn s_mp_add_d(mut mp: *mut mp_int, mut d: mp_digit)
 -> mp_err {
    let mut pmp: *mut mp_digit = (*mp).dp;
    let mut sum: mp_digit = 0;
    let mut mp_i: mp_digit = 0;
    let mut carry: mp_digit = 0i32 as mp_digit;
    let mut res: mp_err = 0i32;
    let mut used: libc::c_int = (*mp).used as libc::c_int;
    mp_i = *pmp;
    let fresh0 = pmp;
    pmp = pmp.offset(1);
    sum = d.wrapping_add(mp_i);
    *fresh0 = sum;
    carry = (sum < d) as libc::c_int as mp_digit;
    while 0 != carry && { used -= 1; used > 0i32 } {
        mp_i = *pmp;
        let fresh1 = pmp;
        pmp = pmp.offset(1);
        sum = carry.wrapping_add(mp_i);
        *fresh1 = sum;
        carry = (0 == sum) as libc::c_int as mp_digit
    }
    if 0 != carry && 0 == used {
        used = (*mp).used as libc::c_int;
        res = s_mp_pad(mp, (used + 1i32) as mp_size);
        if !(0i32 > res) { *(*mp).dp.offset(used as isize) = carry }
    }
    return res;
}
/* left pad with zeroes    */
#[no_mangle]
pub unsafe extern "C" fn s_mp_pad(mut mp: *mut mp_int, mut min: mp_size)
 -> mp_err {
    if min > (*mp).used {
        let mut res: mp_err = 0;
        if min > (*mp).alloc {
            res = s_mp_grow(mp, min);
            if res != 0i32 { return res }
        } else {
            s_mp_setz((*mp).dp.offset((*mp).used as isize),
                      min.wrapping_sub((*mp).used));
        }
        (*mp).used = min
    }
    return 0i32;
}
/* increase allocated size */
#[no_mangle]
pub unsafe extern "C" fn s_mp_grow(mut mp: *mut mp_int, mut min: mp_size)
 -> mp_err {
    if min > (*mp).alloc {
        let mut tmp: *mut mp_digit = 0 as *mut mp_digit;
        min =
            min.wrapping_add(s_mp_defprec).wrapping_sub(1i32 as
                                                            libc::c_uint).wrapping_div(s_mp_defprec).wrapping_mul(s_mp_defprec);
        tmp =
            s_mp_alloc(min as size_t,
                       ::std::mem::size_of::<mp_digit>() as libc::c_ulong) as
                *mut mp_digit;
        if tmp.is_null() { return -2i32 }
        s_mp_copy((*mp).dp, tmp, (*mp).used);
        s_mp_setz((*mp).dp, (*mp).alloc);
        s_mp_free((*mp).dp as *mut libc::c_void);
        (*mp).dp = tmp;
        (*mp).alloc = min
    }
    return 0i32;
}
/* unsigned digit multiply */
#[no_mangle]
pub unsafe extern "C" fn s_mp_mul_d(mut a: *mut mp_int, mut d: mp_digit)
 -> mp_err {
    let mut res: mp_err = 0;
    let mut used: mp_size = 0;
    let mut pow: libc::c_int = 0;
    if 0 == d { mp_zero(a); return 0i32 }
    if d == 1i32 as libc::c_ulong { return 0i32 }
    pow = s_mp_ispow2d(d);
    if 0i32 <= pow { return s_mp_mul_2d(a, pow as mp_digit) }
    used = (*a).used;
    res = s_mp_pad(a, used.wrapping_add(1i32 as libc::c_uint));
    if !(0i32 > res) {
        s_mpv_mul_d((*a).dp, used, d, (*a).dp);
        s_mp_clamp(a);
    }
    return res;
}
/* clip leading zeroes     */
#[no_mangle]
pub unsafe extern "C" fn s_mp_clamp(mut mp: *mut mp_int) {
    let mut used: mp_size = (*mp).used;
    while used > 1i32 as libc::c_uint &&
              *(*mp).dp.offset(used.wrapping_sub(1i32 as libc::c_uint) as
                                   isize) == 0i32 as libc::c_ulong {
        used = used.wrapping_sub(1)
    }
    (*mp).used = used;
}
/* end NSS_USE_COMBA */
/* ------ mpv functions, operate on arrays of digits, not on mp_int's ------ */
#[no_mangle]
pub unsafe extern "C" fn s_mpv_mul_d(mut a: *const mp_digit,
                                     mut a_len: mp_size, mut b: mp_digit,
                                     mut c: *mut mp_digit) {
    let mut carry: mp_digit = 0i32 as mp_digit;
    loop  {
        let fresh2 = a_len;
        a_len = a_len.wrapping_sub(1);
        if !(0 != fresh2) { break ; }
        let fresh3 = a;
        a = a.offset(1);
        let mut a_i: mp_digit = *fresh3;
        let mut a0b0: mp_digit = 0;
        let mut a1b1: mp_digit = 0;
        let mut a0b1: mp_digit = 0;
        let mut a1b0: mp_digit = 0;
        a0b0 =
            (a_i &
                 (2147483647i32 as
                      libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                     libc::c_ulong).wrapping_mul(b &
                                                     (2147483647i32 as
                                                          libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                         as libc::c_ulong);
        a1b1 =
            (a_i >>
                 (8i32 as
                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                      as
                                                      libc::c_ulong).wrapping_div(2i32
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_mul(b
                                                                                                                       >>
                                                                                                                       (8i32
                                                                                                                            as
                                                                                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                                                                            as
                                                                                                                                                            libc::c_ulong).wrapping_div(2i32
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_ulong));
        a0b1 =
            (a_i &
                 (2147483647i32 as
                      libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                     libc::c_ulong).wrapping_mul(b >>
                                                     (8i32 as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_div(2i32
                                                                                                                          as
                                                                                                                          libc::c_ulong));
        a1b0 =
            (a_i >>
                 (8i32 as
                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                      as
                                                      libc::c_ulong).wrapping_div(2i32
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_mul(b
                                                                                                                       &
                                                                                                                       (2147483647i32
                                                                                                                            as
                                                                                                                            libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                                                                                           as
                                                                                                                           libc::c_ulong);
        a1b0 =
            (a1b0 as libc::c_ulong).wrapping_add(a0b1) as mp_digit as
                mp_digit;
        a1b1 =
            (a1b1 as
                 libc::c_ulong).wrapping_add(a1b0 >>
                                                 (8i32 as
                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_div(2i32
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                as mp_digit as mp_digit;
        if a1b0 < a0b1 {
            a1b1 =
                (a1b1 as
                     libc::c_ulong).wrapping_add((1i32 as
                                                      libc::c_ulong).wrapping_add((2147483647i32
                                                                                       as
                                                                                       libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                                                      as
                                                                                      mp_digit))
                    as mp_digit as mp_digit
        }
        a1b0 <<=
            (8i32 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                 as
                                                 libc::c_ulong).wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_ulong);
        a0b0 =
            (a0b0 as libc::c_ulong).wrapping_add(a1b0) as mp_digit as
                mp_digit;
        if a0b0 < a1b0 { a1b1 = a1b1.wrapping_add(1) }
        a0b0 =
            (a0b0 as libc::c_ulong).wrapping_add(carry) as mp_digit as
                mp_digit;
        if a0b0 < carry { a1b1 = a1b1.wrapping_add(1) }
        let fresh4 = c;
        c = c.offset(1);
        *fresh4 = a0b0;
        carry = a1b1
    }
    *c = carry;
}
/* multiply by 2^d in place */
#[no_mangle]
pub unsafe extern "C" fn s_mp_mul_2d(mut mp: *mut mp_int, mut d: mp_digit)
 -> mp_err {
    let mut res: mp_err = 0;
    let mut dshift: mp_digit = 0;
    let mut bshift: mp_digit = 0;
    let mut mask: mp_digit = 0;
    if mp.is_null() { return -4i32 }
    dshift =
        d.wrapping_div((8i32 as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                            as
                                                            libc::c_ulong));
    bshift =
        d.wrapping_rem((8i32 as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                            as
                                                            libc::c_ulong));
    if 0 != bshift {
        mask =
            (!0i32 as mp_digit) <<
                (8i32 as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(bshift);
        mask &=
            *(*mp).dp.offset((*mp).used.wrapping_sub(1i32 as libc::c_uint) as
                                 isize)
    } else { mask = 0i32 as mp_digit }
    res =
        s_mp_pad(mp,
                 ((*mp).used as
                      libc::c_ulong).wrapping_add(dshift).wrapping_add((mask
                                                                            !=
                                                                            0i32
                                                                                as
                                                                                libc::c_ulong)
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                     as mp_size);
    if 0i32 != res { return res }
    if 0 != dshift && { res = s_mp_lshd(mp, dshift as mp_size); 0i32 != res }
       {
        return res
    }
    if 0 != bshift {
        let mut pa: *mut mp_digit = (*mp).dp;
        let mut alim: *mut mp_digit = pa.offset((*mp).used as isize);
        let mut prev: mp_digit = 0i32 as mp_digit;
        pa = pa.offset(dshift as isize);
        while pa < alim {
            let mut x: mp_digit = *pa;
            let fresh5 = pa;
            pa = pa.offset(1);
            *fresh5 = x << bshift | prev;
            prev =
                x >>
                    (8i32 as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(bshift)
        }
    }
    s_mp_clamp(mp);
    return 0i32;
}
/* left-shift by p digits  */
#[no_mangle]
pub unsafe extern "C" fn s_mp_lshd(mut mp: *mut mp_int, mut p: mp_size)
 -> mp_err {
    let mut res: mp_err = 0;
    let mut ix: libc::c_uint = 0;
    if p == 0i32 as libc::c_uint { return 0i32 }
    if (*mp).used == 1i32 as libc::c_uint &&
           *(*mp).dp.offset(0isize) == 0i32 as libc::c_ulong {
        return 0i32
    }
    res = s_mp_pad(mp, (*mp).used.wrapping_add(p));
    if res != 0i32 { return res }
    ix = (*mp).used.wrapping_sub(p);
    loop  {
        let fresh6 = ix;
        ix = ix.wrapping_sub(1);
        if !(fresh6 > 0i32 as libc::c_uint) { break ; }
        *(*mp).dp.offset(ix.wrapping_add(p) as isize) =
            *(*mp).dp.offset(ix as isize)
    }
    ix = 0i32 as libc::c_uint;
    while ix < p {
        *(*mp).dp.offset(ix as isize) = 0i32 as mp_digit;
        ix = ix.wrapping_add(1)
    }
    return 0i32;
}
/* is d a power of 2?      */
#[no_mangle]
pub unsafe extern "C" fn s_mp_ispow2d(mut d: mp_digit) -> libc::c_int {
    if d != 0i32 as libc::c_ulong &&
           d & d.wrapping_sub(1i32 as libc::c_ulong) == 0i32 as libc::c_ulong
       {
        let mut pow: libc::c_int = 0i32;
        if 0 != d & 0xffffffff00000000u64 { pow += 32i32 }
        if 0 != d & 0xffff0000ffff0000u64 { pow += 16i32 }
        if 0 != d & 0xff00ff00ff00ff00u64 { pow += 8i32 }
        if 0 != d & 0xf0f0f0f0f0f0f0f0u64 { pow += 4i32 }
        if 0 != d & 0xccccccccccccccccu64 { pow += 2i32 }
        if 0 != d & 0xaaaaaaaaaaaaaaaau64 { pow += 1i32 }
        return pow
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_set_ulong(mut mp: *mut mp_int,
                                      mut z: libc::c_ulong) -> mp_err {
    let mut ix: libc::c_int = 0;
    let mut res: mp_err = 0;
    if mp.is_null() { return -4i32 }
    mp_zero(mp);
    if z == 0i32 as libc::c_ulong { return 0i32 }
    if ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong <=
           ::std::mem::size_of::<mp_digit>() as libc::c_ulong {
        *(*mp).dp.offset(0isize) = z
    } else {
        ix =
            (::std::mem::size_of::<libc::c_long>() as
                 libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as
                libc::c_int;
        while ix >= 0i32 {
            res = s_mp_mul_d(mp, (127i32 * 2i32 + 1i32 + 1i32) as mp_digit);
            if res != 0i32 { return res }
            res =
                s_mp_add_d(mp,
                           z >> ix * 8i32 &
                               (127i32 * 2i32 + 1i32) as libc::c_ulong);
            if res != 0i32 { return res }
            ix -= 1
        }
    }
    return 0i32;
}
/* Single digit arithmetic */
#[no_mangle]
pub unsafe extern "C" fn mp_add_d(mut a: *const mp_int, mut d: mp_digit,
                                  mut b: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null()) { return -4i32 }
    res = mp_init_copy(&mut tmp, a);
    if res != 0i32 { return res }
    if tmp.sign == 0i32 as libc::c_uint {
        res = s_mp_add_d(&mut tmp, d);
        if res != 0i32 {
            current_block = 6643007740976466712;
        } else { current_block = 17860125682698302841; }
    } else if s_mp_cmp_d(&mut tmp, d) >= 0i32 {
        res = s_mp_sub_d(&mut tmp, d);
        if res != 0i32 {
            current_block = 6643007740976466712;
        } else { current_block = 17860125682698302841; }
    } else {
        mp_neg(&mut tmp, &mut tmp);
        *tmp.dp.offset(0isize) = d.wrapping_sub(*tmp.dp.offset(0isize));
        current_block = 17860125682698302841;
    }
    match current_block {
        17860125682698302841 => {
            if s_mp_cmp_d(&mut tmp, 0i32 as mp_digit) == 0i32 {
                tmp.sign = 0i32 as mp_sign
            }
            s_mp_exch(&mut tmp, b);
        }
        _ => { }
    }
    mp_clear(&mut tmp);
    return res;
}
/* magnitude digit compare */
#[no_mangle]
pub unsafe extern "C" fn s_mp_cmp_d(mut a: *const mp_int, mut d: mp_digit)
 -> libc::c_int {
    if (*a).used > 1i32 as libc::c_uint { return 1i32 }
    if *(*a).dp.offset(0isize) < d {
        return -1i32
    } else if *(*a).dp.offset(0isize) > d {
        return 1i32
    } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn mp_neg(mut a: *const mp_int, mut b: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null()) { return -4i32 }
    res = mp_copy(a, b);
    if res != 0i32 { return res }
    if s_mp_cmp_d(b, 0i32 as mp_digit) == 0i32 {
        (*b).sign = 0i32 as mp_sign
    } else {
        (*b).sign =
            (if (*b).sign == 1i32 as libc::c_uint { 0i32 } else { 1i32 }) as
                mp_sign
    }
    return 0i32;
}
/* unsigned digit subtract */
#[no_mangle]
pub unsafe extern "C" fn s_mp_sub_d(mut mp: *mut mp_int, mut d: mp_digit)
 -> mp_err {
    let mut pmp: *mut mp_digit = (*mp).dp;
    let mut mp_i: mp_digit = 0;
    let mut diff: mp_digit = 0;
    let mut borrow: mp_digit = 0;
    let mut used: mp_size = (*mp).used;
    mp_i = *pmp;
    let fresh7 = pmp;
    pmp = pmp.offset(1);
    diff = mp_i.wrapping_sub(d);
    *fresh7 = diff;
    borrow = (diff > mp_i) as libc::c_int as mp_digit;
    while 0 != borrow && { used = used.wrapping_sub(1); 0 != used } {
        mp_i = *pmp;
        let fresh8 = pmp;
        pmp = pmp.offset(1);
        diff = mp_i.wrapping_sub(borrow);
        *fresh8 = diff;
        borrow = (diff > mp_i) as libc::c_int as mp_digit
    }
    s_mp_clamp(mp);
    return if 0 != borrow && 0 == used { -3i32 } else { 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn mp_sub_d(mut a: *const mp_int, mut d: mp_digit,
                                  mut b: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null()) { return -4i32 }
    res = mp_init_copy(&mut tmp, a);
    if res != 0i32 { return res }
    if tmp.sign == 1i32 as libc::c_uint {
        res = s_mp_add_d(&mut tmp, d);
        if res != 0i32 {
            current_block = 18052238657935791076;
        } else { current_block = 13056961889198038528; }
    } else if s_mp_cmp_d(&mut tmp, d) >= 0i32 {
        res = s_mp_sub_d(&mut tmp, d);
        if res != 0i32 {
            current_block = 18052238657935791076;
        } else { current_block = 13056961889198038528; }
    } else {
        mp_neg(&mut tmp, &mut tmp);
        *tmp.dp.offset(0isize) = d.wrapping_sub(*tmp.dp.offset(0isize));
        tmp.sign = 1i32 as mp_sign;
        current_block = 13056961889198038528;
    }
    match current_block {
        13056961889198038528 => {
            if s_mp_cmp_d(&mut tmp, 0i32 as mp_digit) == 0i32 {
                tmp.sign = 0i32 as mp_sign
            }
            s_mp_exch(&mut tmp, b);
        }
        _ => { }
    }
    mp_clear(&mut tmp);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_mul_d(mut a: *const mp_int, mut d: mp_digit,
                                  mut b: *mut mp_int) -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null()) { return -4i32 }
    if d == 0i32 as libc::c_ulong { mp_zero(b); return 0i32 }
    res = mp_copy(a, b);
    if res != 0i32 { return res }
    res = s_mp_mul_d(b, d);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_mul_2(mut a: *const mp_int, mut c: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !c.is_null()) { return -4i32 }
    res = mp_copy(a, c);
    if res != 0i32 { return res }
    return s_mp_mul_2(c);
}
/* multiply by 2 in place  */
#[no_mangle]
pub unsafe extern "C" fn s_mp_mul_2(mut mp: *mut mp_int) -> mp_err {
    let mut pd: *mut mp_digit = 0 as *mut mp_digit;
    let mut ix: libc::c_uint = 0;
    let mut used: libc::c_uint = 0;
    let mut kin: mp_digit = 0i32 as mp_digit;
    used = (*mp).used;
    pd = (*mp).dp;
    ix = 0i32 as libc::c_uint;
    while ix < used {
        let mut d: mp_digit = *pd;
        let fresh9 = pd;
        pd = pd.offset(1);
        *fresh9 = d << 1i32 | kin;
        kin =
            d >>
                (8i32 as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong);
        ix = ix.wrapping_add(1)
    }
    if 0 != kin {
        if ix >= (*mp).alloc {
            let mut res: mp_err = 0;
            res =
                s_mp_grow(mp, (*mp).alloc.wrapping_add(1i32 as libc::c_uint));
            if res != 0i32 { return res }
        }
        *(*mp).dp.offset(ix as isize) = kin;
        (*mp).used =
            ((*mp).used as libc::c_uint).wrapping_add(1i32 as libc::c_uint) as
                mp_size as mp_size
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_div_d(mut a: *const mp_int, mut d: mp_digit,
                                  mut q: *mut mp_int, mut r: *mut mp_digit)
 -> mp_err {
    let mut res: mp_err = 0;
    let mut qp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut rem: mp_digit = 0i32 as mp_digit;
    let mut pow: libc::c_int = 0;
    if a.is_null() { return -4i32 }
    if d == 0i32 as libc::c_ulong { return -3i32 }
    pow = s_mp_ispow2d(d);
    if pow >= 0i32 {
        let mut mask: mp_digit = 0;
        mask =
            ((1i32 as mp_digit) << pow).wrapping_sub(1i32 as libc::c_ulong);
        rem = *(*a).dp.offset(0isize) & mask;
        if !q.is_null() {
            res = mp_copy(a, q);
            if res != 0i32 { return res }
            s_mp_div_2d(q, pow as mp_digit);
        }
        if !r.is_null() { *r = rem }
        return 0i32
    }
    res = mp_init_copy(&mut qp, a);
    if res != 0i32 { return res }
    res = s_mp_div_d(&mut qp, d, &mut rem);
    if s_mp_cmp_d(&mut qp, 0i32 as mp_digit) == 0i32 {
        (*q).sign = 0i32 as mp_sign
    }
    if !r.is_null() { *r = rem }
    if !q.is_null() { s_mp_exch(&mut qp, q); }
    mp_clear(&mut qp);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_div_d(mut mp: *mut mp_int, mut d: mp_digit,
                                    mut r: *mut mp_digit) -> mp_err {
    let mut current_block: u64;
    let mut w: mp_digit = 0i32 as mp_digit;
    let mut q: mp_digit = 0;
    let mut ix: libc::c_int = 0;
    let mut res: mp_err = 0;
    let mut quot: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut rem: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    if d == 0i32 as libc::c_ulong { return -3i32 }
    if d == 1i32 as libc::c_ulong {
        if !r.is_null() { *r = 0i32 as mp_digit }
        return 0i32
    }
    if (*mp).used == 1i32 as libc::c_uint {
        let mut n: mp_digit = *(*mp).dp.offset(0isize);
        let mut rem_0: mp_digit = 0;
        q = n.wrapping_div(d);
        rem_0 = n.wrapping_rem(d);
        *(*mp).dp.offset(0isize) = q;
        if !r.is_null() { *r = rem_0 }
        return 0i32
    }
    rem.dp = 0 as *mut mp_digit;
    quot.dp = 0 as *mut mp_digit;
    /* Make room for the quotient */
    res = mp_init_size(&mut quot, (*mp).used);
    if !(0i32 > res) {
        let mut p: mp_digit = 0;
        let mut norm: mp_digit = 0;
        res = mp_init_copy(&mut rem, mp);
        if !(0i32 > res) {
            *quot.dp.offset(0isize) = d;
            res = s_mp_norm(&mut rem, &mut quot, &mut norm);
            if !(0i32 > res) {
                if 0 != norm { d <<= norm }
                *quot.dp.offset(0isize) = 0i32 as mp_digit;
                p = 0i32 as mp_digit;
                ix =
                    rem.used.wrapping_sub(1i32 as libc::c_uint) as
                        libc::c_int;
                loop  {
                    if !(ix >= 0i32) {
                        current_block = 4567019141635105728;
                        break ;
                    }
                    w = *rem.dp.offset(ix as isize);
                    if 0 != p {
                        res = s_mpv_div_2dx1d(p, w, d, &mut q, &mut w);
                        if 0i32 > res {
                            current_block = 2541140235787719656;
                            break ;
                        }
                    } else if w >= d {
                        q = w.wrapping_div(d);
                        w = w.wrapping_rem(d)
                    } else { q = 0i32 as mp_digit }
                    res = s_mp_lshd(&mut quot, 1i32 as mp_size);
                    if 0i32 > res {
                        current_block = 2541140235787719656;
                        break ;
                    }
                    *quot.dp.offset(0isize) = q;
                    p = w;
                    ix -= 1
                }
                match current_block {
                    2541140235787719656 => { }
                    _ => {
                        if 0 != norm { w >>= norm }
                        if !r.is_null() { *r = w }
                        s_mp_clamp(&mut quot);
                        mp_exch(&mut quot, mp);
                    }
                }
            }
        }
    }
    mp_clear(&mut quot);
    mp_clear(&mut rem);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn s_mpv_div_2dx1d(mut Nhi: mp_digit, mut Nlo: mp_digit,
                                         mut divisor: mp_digit,
                                         mut qp: *mut mp_digit,
                                         mut rp: *mut mp_digit) -> mp_err {
    let mut d1: mp_digit = 0;
    let mut d0: mp_digit = 0;
    let mut q1: mp_digit = 0;
    let mut q0: mp_digit = 0;
    let mut r1: mp_digit = 0;
    let mut r0: mp_digit = 0;
    let mut m: mp_digit = 0;
    d1 =
        divisor >>
            (8i32 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                 as
                                                 libc::c_ulong).wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_ulong);
    d0 =
        divisor &
            (2147483647i32 as
                 libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                libc::c_ulong;
    r1 = Nhi.wrapping_rem(d1);
    q1 = Nhi.wrapping_div(d1);
    m = q1.wrapping_mul(d0);
    r1 =
        r1 <<
            (8i32 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                 as
                                                 libc::c_ulong).wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_ulong)
            |
            Nlo >>
                (8i32 as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                     as
                                                     libc::c_ulong).wrapping_div(2i32
                                                                                     as
                                                                                     libc::c_ulong);
    if r1 < m {
        q1 = q1.wrapping_sub(1);
        r1 =
            (r1 as libc::c_ulong).wrapping_add(divisor) as mp_digit as
                mp_digit;
        if r1 >= divisor && r1 < m {
            q1 = q1.wrapping_sub(1);
            r1 =
                (r1 as libc::c_ulong).wrapping_add(divisor) as mp_digit as
                    mp_digit
        }
    }
    r1 = (r1 as libc::c_ulong).wrapping_sub(m) as mp_digit as mp_digit;
    r0 = r1.wrapping_rem(d1);
    q0 = r1.wrapping_div(d1);
    m = q0.wrapping_mul(d0);
    r0 =
        r0 <<
            (8i32 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                 as
                                                 libc::c_ulong).wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_ulong)
            |
            Nlo &
                (2147483647i32 as
                     libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                    libc::c_ulong;
    if r0 < m {
        q0 = q0.wrapping_sub(1);
        r0 =
            (r0 as libc::c_ulong).wrapping_add(divisor) as mp_digit as
                mp_digit;
        if r0 >= divisor && r0 < m {
            q0 = q0.wrapping_sub(1);
            r0 =
                (r0 as libc::c_ulong).wrapping_add(divisor) as mp_digit as
                    mp_digit
        }
    }
    if !qp.is_null() {
        *qp =
            q1 <<
                (8i32 as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                     as
                                                     libc::c_ulong).wrapping_div(2i32
                                                                                     as
                                                                                     libc::c_ulong)
                | q0
    }
    if !rp.is_null() { *rp = r0.wrapping_sub(m) }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_norm(mut a: *mut mp_int, mut b: *mut mp_int,
                                   mut pd: *mut mp_digit) -> mp_err {
    let mut current_block: u64;
    let mut d: mp_digit = 0;
    let mut mask: mp_digit = 0;
    let mut b_msd: mp_digit = 0;
    let mut res: mp_err = 0i32;
    d = 0i32 as mp_digit;
    mask =
        (9223372036854775807i64 as
             libc::c_ulong).wrapping_mul(2u64).wrapping_add(1u64) &
            !((9223372036854775807i64 as
                   libc::c_ulong).wrapping_mul(2u64).wrapping_add(1u64) >>
                  1i32);
    b_msd =
        *(*b).dp.offset((*b).used.wrapping_sub(1i32 as libc::c_uint) as
                            isize);
    while 0 == b_msd & mask { b_msd <<= 1i32; d = d.wrapping_add(1) }
    if 0 != d {
        res = s_mp_mul_2d(a, d);
        if 0i32 > res {
            current_block = 18031509829004894457;
        } else {
            res = s_mp_mul_2d(b, d);
            if 0i32 > res {
                current_block = 18031509829004894457;
            } else { current_block = 1841672684692190573; }
        }
    } else { current_block = 1841672684692190573; }
    match current_block { 1841672684692190573 => { *pd = d } _ => { } }
    return res;
}
/* divide by 2^d in place  */
#[no_mangle]
pub unsafe extern "C" fn s_mp_div_2d(mut mp: *mut mp_int, mut d: mp_digit) {
    let mut ix: libc::c_int = 0;
    let mut save: mp_digit = 0;
    let mut next: mp_digit = 0;
    let mut mask: mp_digit = 0;
    s_mp_rshd(mp,
              d.wrapping_div((8i32 as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                  as
                                                                  libc::c_ulong))
                  as mp_size);
    d =
        (d as
             libc::c_ulong).wrapping_rem((8i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                              as
                                                                              libc::c_ulong))
            as mp_digit as mp_digit;
    if 0 != d {
        mask = ((1i32 as mp_digit) << d).wrapping_sub(1i32 as libc::c_ulong);
        save = 0i32 as mp_digit;
        ix = (*mp).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
        while ix >= 0i32 {
            next = *(*mp).dp.offset(ix as isize) & mask;
            *(*mp).dp.offset(ix as isize) =
                *(*mp).dp.offset(ix as isize) >> d |
                    save <<
                        (8i32 as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                             as
                                                             libc::c_ulong).wrapping_sub(d);
            save = next;
            ix -= 1
        }
    }
    s_mp_clamp(mp);
}
/* right-shift by p digits */
#[no_mangle]
pub unsafe extern "C" fn s_mp_rshd(mut mp: *mut mp_int, mut p: mp_size) {
    let mut ix: mp_size = 0;
    let mut src: *mut mp_digit = 0 as *mut mp_digit;
    let mut dst: *mut mp_digit = 0 as *mut mp_digit;
    if p == 0i32 as libc::c_uint { return }
    if p >= (*mp).used {
        s_mp_setz((*mp).dp, (*mp).alloc);
        (*mp).used = 1i32 as mp_size;
        (*mp).sign = 0i32 as mp_sign;
        return
    }
    dst = (*mp).dp;
    src = dst.offset(p as isize);
    ix = (*mp).used.wrapping_sub(p);
    while ix > 0i32 as libc::c_uint {
        let fresh11 = dst;
        dst = dst.offset(1);
        let fresh10 = src;
        src = src.offset(1);
        *fresh11 = *fresh10;
        ix = ix.wrapping_sub(1)
    }
    (*mp).used =
        ((*mp).used as libc::c_uint).wrapping_sub(p) as mp_size as mp_size;
    loop  {
        let fresh12 = p;
        p = p.wrapping_sub(1);
        if !(fresh12 > 0i32 as libc::c_uint) { break ; }
        let fresh13 = dst;
        dst = dst.offset(1);
        *fresh13 = 0i32 as mp_digit
    };
}
#[no_mangle]
pub unsafe extern "C" fn mp_div_2(mut a: *const mp_int, mut c: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !c.is_null()) { return -4i32 }
    res = mp_copy(a, c);
    if res != 0i32 { return res }
    s_mp_div_2(c);
    return 0i32;
}
/* divide by 2 in place    */
#[no_mangle]
pub unsafe extern "C" fn s_mp_div_2(mut mp: *mut mp_int) {
    s_mp_div_2d(mp, 1i32 as mp_digit);
}
#[no_mangle]
pub unsafe extern "C" fn mp_expt_d(mut a: *const mp_int, mut d: mp_digit,
                                   mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut s: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut x: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    if !(!a.is_null() && !c.is_null()) { return -4i32 }
    res = mp_init(&mut s);
    if res != 0i32 { return res }
    res = mp_init_copy(&mut x, a);
    if !(res != 0i32) {
        *s.dp.offset(0isize) = 1i32 as mp_digit;
        loop  {
            if !(d != 0i32 as libc::c_ulong) {
                current_block = 9606288038608642794;
                break ;
            }
            if 0 != d & 1i32 as libc::c_ulong {
                res = s_mp_mul(&mut s, &mut x);
                if res != 0i32 {
                    current_block = 8406659366743279450;
                    break ;
                }
            }
            d =
                (d as libc::c_ulong).wrapping_div(2i32 as libc::c_ulong) as
                    mp_digit as mp_digit;
            res = s_mp_sqr(&mut x);
            if res != 0i32 { current_block = 8406659366743279450; break ; }
        }
        match current_block {
            9606288038608642794 => { s_mp_exch(&mut s, c); }
            _ => { }
        }
        mp_clear(&mut x);
    }
    mp_clear(&mut s);
    return res;
}
/* magnitude square        */
#[no_mangle]
pub unsafe extern "C" fn s_mp_sqr(mut a: *mut mp_int) -> mp_err {
    let mut res: mp_err = 0;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    res =
        mp_init_size(&mut tmp,
                     (2i32 as libc::c_uint).wrapping_mul((*a).used));
    if res != 0i32 { return res }
    res = mp_sqr(a, &mut tmp);
    if res == 0i32 { s_mp_exch(&mut tmp, a); }
    mp_clear(&mut tmp);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_sqr(mut a: *const mp_int, mut sqr: *mut mp_int)
 -> mp_err {
    let mut current_block: u64;
    let mut pa: *mut mp_digit = 0 as *mut mp_digit;
    let mut d: mp_digit = 0;
    let mut res: mp_err = 0;
    let mut ix: mp_size = 0;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut count: libc::c_int = 0;
    if !(!a.is_null() && !sqr.is_null()) { return -4i32 }
    if a == sqr {
        res = mp_init_copy(&mut tmp, a);
        if res != 0i32 { return res }
        a = &mut tmp
    } else { tmp.dp = 0 as *mut mp_digit; res = 0i32 }
    ix = (2i32 as libc::c_uint).wrapping_mul((*a).used);
    if ix > (*sqr).alloc {
        (*sqr).used = 1i32 as mp_size;
        res = s_mp_grow(sqr, ix);
        if 0i32 > res {
            current_block = 13574214132429213469;
        } else { current_block = 2370887241019905314; }
    } else { current_block = 2370887241019905314; }
    match current_block {
        2370887241019905314 => {
            (*sqr).used = ix;
            *(*sqr).dp.offset(0isize) = 0i32 as mp_digit;
            pa = (*a).dp;
            count =
                (*a).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
            if count > 0i32 {
                let fresh14 = pa;
                pa = pa.offset(1);
                d = *fresh14;
                s_mpv_mul_d(pa, count as mp_size, d,
                            (*sqr).dp.offset(1isize));
                ix = 3i32 as mp_size;
                loop  {
                    count -= 1;
                    if !(count > 0i32) { break ; }
                    let fresh15 = pa;
                    pa = pa.offset(1);
                    d = *fresh15;
                    s_mpv_mul_d_add(pa, count as mp_size, d,
                                    (*sqr).dp.offset(ix as isize));
                    ix =
                        (ix as
                             libc::c_uint).wrapping_add(2i32 as libc::c_uint)
                            as mp_size as mp_size
                }
                *(*sqr).dp.offset((*sqr).used.wrapping_sub(1i32 as
                                                               libc::c_uint)
                                      as isize) = 0i32 as mp_digit;
                s_mp_mul_2(sqr);
            } else { *(*sqr).dp.offset(1isize) = 0i32 as mp_digit }
            s_mpv_sqr_add_prop((*a).dp, (*a).used, (*sqr).dp);
            (*sqr).sign = 0i32 as mp_sign;
            s_mp_clamp(sqr);
        }
        _ => { }
    }
    mp_clear(&mut tmp);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn s_mpv_sqr_add_prop(mut pa: *const mp_digit,
                                            mut a_len: mp_size,
                                            mut ps: *mut mp_digit) {
    let mut carry: mp_digit = 0i32 as mp_digit;
    loop  {
        let fresh16 = a_len;
        a_len = a_len.wrapping_sub(1);
        if !(0 != fresh16) { break ; }
        let fresh17 = pa;
        pa = pa.offset(1);
        let mut a_i: mp_digit = *fresh17;
        let mut a0a0: mp_digit = 0;
        let mut a1a1: mp_digit = 0;
        let mut Pmid: mp_digit = 0;
        a0a0 =
            (a_i &
                 (2147483647i32 as
                      libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                     libc::c_ulong).wrapping_mul(a_i &
                                                     (2147483647i32 as
                                                          libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                         as libc::c_ulong);
        a1a1 =
            (a_i >>
                 (8i32 as
                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                      as
                                                      libc::c_ulong).wrapping_div(2i32
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_mul(a_i
                                                                                                                       >>
                                                                                                                       (8i32
                                                                                                                            as
                                                                                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                                                                            as
                                                                                                                                                            libc::c_ulong).wrapping_div(2i32
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_ulong));
        Pmid =
            (a_i &
                 (2147483647i32 as
                      libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                     libc::c_ulong).wrapping_mul(a_i >>
                                                     (8i32 as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_div(2i32
                                                                                                                          as
                                                                                                                          libc::c_ulong));
        a1a1 =
            (a1a1 as
                 libc::c_ulong).wrapping_add(Pmid >>
                                                 (8i32 as
                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_div(2i32
                                                                                                                      as
                                                                                                                      libc::c_ulong).wrapping_sub(1i32
                                                                                                                                                      as
                                                                                                                                                      libc::c_ulong))
                as mp_digit as mp_digit;
        Pmid <<=
            (8i32 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                 as
                                                 libc::c_ulong).wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(1i32
                                                                                                                 as
                                                                                                                 libc::c_ulong);
        a0a0 =
            (a0a0 as libc::c_ulong).wrapping_add(Pmid) as mp_digit as
                mp_digit;
        if a0a0 < Pmid { a1a1 = a1a1.wrapping_add(1) }
        a0a0 =
            (a0a0 as libc::c_ulong).wrapping_add(carry) as mp_digit as
                mp_digit;
        if a0a0 < carry { a1a1 = a1a1.wrapping_add(1) }
        a_i = *ps;
        a0a0 =
            (a0a0 as libc::c_ulong).wrapping_add(a_i) as mp_digit as mp_digit;
        if a0a0 < a_i { a1a1 = a1a1.wrapping_add(1) }
        let fresh18 = ps;
        ps = ps.offset(1);
        *fresh18 = a0a0;
        a_i = *ps;
        a1a1 =
            (a1a1 as libc::c_ulong).wrapping_add(a_i) as mp_digit as mp_digit;
        carry = (a1a1 < a_i) as libc::c_int as mp_digit;
        let fresh19 = ps;
        ps = ps.offset(1);
        *fresh19 = a1a1
    }
    while 0 != carry {
        let mut s_i: mp_digit = *ps;
        carry =
            (carry as libc::c_ulong).wrapping_add(s_i) as mp_digit as
                mp_digit;
        let fresh20 = ps;
        ps = ps.offset(1);
        *fresh20 = carry;
        carry = (carry < s_i) as libc::c_int as mp_digit
    };
}
#[no_mangle]
pub unsafe extern "C" fn s_mpv_mul_d_add(mut a: *const mp_digit,
                                         mut a_len: mp_size, mut b: mp_digit,
                                         mut c: *mut mp_digit) {
    let mut carry: mp_digit = 0i32 as mp_digit;
    loop  {
        let fresh21 = a_len;
        a_len = a_len.wrapping_sub(1);
        if !(0 != fresh21) { break ; }
        let fresh22 = a;
        a = a.offset(1);
        let mut a_i: mp_digit = *fresh22;
        let mut a0b0: mp_digit = 0;
        let mut a1b1: mp_digit = 0;
        let mut a0b1: mp_digit = 0;
        let mut a1b0: mp_digit = 0;
        a0b0 =
            (a_i &
                 (2147483647i32 as
                      libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                     libc::c_ulong).wrapping_mul(b &
                                                     (2147483647i32 as
                                                          libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                         as libc::c_ulong);
        a1b1 =
            (a_i >>
                 (8i32 as
                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                      as
                                                      libc::c_ulong).wrapping_div(2i32
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_mul(b
                                                                                                                       >>
                                                                                                                       (8i32
                                                                                                                            as
                                                                                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                                                                            as
                                                                                                                                                            libc::c_ulong).wrapping_div(2i32
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_ulong));
        a0b1 =
            (a_i &
                 (2147483647i32 as
                      libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                     libc::c_ulong).wrapping_mul(b >>
                                                     (8i32 as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_div(2i32
                                                                                                                          as
                                                                                                                          libc::c_ulong));
        a1b0 =
            (a_i >>
                 (8i32 as
                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                      as
                                                      libc::c_ulong).wrapping_div(2i32
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_mul(b
                                                                                                                       &
                                                                                                                       (2147483647i32
                                                                                                                            as
                                                                                                                            libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                                                                                           as
                                                                                                                           libc::c_ulong);
        a1b0 =
            (a1b0 as libc::c_ulong).wrapping_add(a0b1) as mp_digit as
                mp_digit;
        a1b1 =
            (a1b1 as
                 libc::c_ulong).wrapping_add(a1b0 >>
                                                 (8i32 as
                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_div(2i32
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                as mp_digit as mp_digit;
        if a1b0 < a0b1 {
            a1b1 =
                (a1b1 as
                     libc::c_ulong).wrapping_add((1i32 as
                                                      libc::c_ulong).wrapping_add((2147483647i32
                                                                                       as
                                                                                       libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                                                      as
                                                                                      mp_digit))
                    as mp_digit as mp_digit
        }
        a1b0 <<=
            (8i32 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                 as
                                                 libc::c_ulong).wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_ulong);
        a0b0 =
            (a0b0 as libc::c_ulong).wrapping_add(a1b0) as mp_digit as
                mp_digit;
        if a0b0 < a1b0 { a1b1 = a1b1.wrapping_add(1) }
        a0b0 =
            (a0b0 as libc::c_ulong).wrapping_add(carry) as mp_digit as
                mp_digit;
        if a0b0 < carry { a1b1 = a1b1.wrapping_add(1) }
        a_i = *c;
        a0b0 =
            (a0b0 as libc::c_ulong).wrapping_add(a_i) as mp_digit as mp_digit;
        if a0b0 < a_i { a1b1 = a1b1.wrapping_add(1) }
        let fresh23 = c;
        c = c.offset(1);
        *fresh23 = a0b0;
        carry = a1b1
    }
    *c = carry;
}
/* a += b * RADIX^offset   */
/* magnitude multiply      */
#[no_mangle]
pub unsafe extern "C" fn s_mp_mul(mut a: *mut mp_int, mut b: *const mp_int)
 -> mp_err {
    return mp_mul(a, b, a);
}
#[no_mangle]
pub unsafe extern "C" fn mp_mul(mut a: *const mp_int, mut b: *const mp_int,
                                mut c: *mut mp_int) -> mp_err {
    let mut pb: *mut mp_digit = 0 as *mut mp_digit;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    let mut ib: mp_size = 0;
    let mut useda: mp_size = 0;
    let mut usedb: mp_size = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if a == c {
        res = mp_init_copy(&mut tmp, a);
        if res != 0i32 { return res }
        if a == b { b = &mut tmp }
        a = &mut tmp
    } else if b == c {
        res = mp_init_copy(&mut tmp, b);
        if res != 0i32 { return res }
        b = &mut tmp
    } else { tmp.dp = 0 as *mut mp_digit }
    if (*a).used < (*b).used {
        let mut xch: *const mp_int = b;
        b = a;
        a = xch
    }
    (*c).used = 1i32 as mp_size;
    *(*c).dp.offset(0isize) = 0i32 as mp_digit;
    res = s_mp_pad(c, (*a).used.wrapping_add((*b).used));
    if !(res != 0i32) {
        pb = (*b).dp;
        let fresh24 = pb;
        pb = pb.offset(1);
        s_mpv_mul_d((*a).dp, (*a).used, *fresh24, (*c).dp);
        useda = (*a).used;
        usedb = (*b).used;
        ib = 1i32 as mp_size;
        while ib < usedb {
            let fresh25 = pb;
            pb = pb.offset(1);
            let mut b_i: mp_digit = *fresh25;
            if 0 != b_i {
                s_mpv_mul_d_add((*a).dp, useda, b_i,
                                (*c).dp.offset(ib as isize));
            } else { *(*c).dp.offset(ib.wrapping_add(useda) as isize) = b_i }
            ib = ib.wrapping_add(1)
        }
        s_mp_clamp(c);
        if (*a).sign == (*b).sign || s_mp_cmp_d(c, 0i32 as mp_digit) == 0i32 {
            (*c).sign = 0i32 as mp_sign
        } else { (*c).sign = 1i32 as mp_sign }
    }
    mp_clear(&mut tmp);
    return res;
}
/* Sign manipulations      */
#[no_mangle]
pub unsafe extern "C" fn mp_abs(mut a: *const mp_int, mut b: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null()) { return -4i32 }
    res = mp_copy(a, b);
    if res != 0i32 { return res }
    (*b).sign = 0i32 as mp_sign;
    return 0i32;
}
/* Full arithmetic         */
#[no_mangle]
pub unsafe extern "C" fn mp_add(mut a: *const mp_int, mut b: *const mp_int,
                                mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if (*a).sign == (*b).sign {
        /* same sign:  add values, keep sign */
        res = s_mp_add_3arg(a, b, c);
        if 0i32 > res {
            current_block = 10927387433881084995;
        } else { current_block = 5399440093318478209; }
    } else if s_mp_cmp(a, b) >= 0i32 {
        /* different sign: |a| >= |b|   */
        res = s_mp_sub_3arg(a, b, c);
        if 0i32 > res {
            current_block = 10927387433881084995;
        } else { current_block = 5399440093318478209; }
    } else {
        /* different sign: |a|  < |b|   */
        res = s_mp_sub_3arg(b, a, c);
        if 0i32 > res {
            current_block = 10927387433881084995;
        } else { current_block = 5399440093318478209; }
    }
    match current_block {
        5399440093318478209 => {
            if s_mp_cmp_d(c, 0i32 as mp_digit) == 0i32 {
                (*c).sign = 0i32 as mp_sign
            }
        }
        _ => { }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_sub_3arg(mut a: *const mp_int,
                                       mut b: *const mp_int,
                                       mut c: *mut mp_int) -> mp_err {
    let mut pa: *mut mp_digit = 0 as *mut mp_digit;
    let mut pb: *mut mp_digit = 0 as *mut mp_digit;
    let mut pc: *mut mp_digit = 0 as *mut mp_digit;
    let mut d: mp_digit = 0;
    let mut diff: mp_digit = 0;
    let mut borrow: mp_digit = 0i32 as mp_digit;
    let mut ix: libc::c_int = 0;
    let mut limit: libc::c_int = 0;
    let mut res: mp_err = 0;
    (*c).sign = (*a).sign;
    res = s_mp_pad(c, (*a).used);
    if 0i32 != res { return res }
    pa = (*a).dp;
    pb = (*b).dp;
    pc = (*c).dp;
    limit = (*b).used as libc::c_int;
    ix = 0i32;
    while ix < limit {
        let fresh26 = pa;
        pa = pa.offset(1);
        d = *fresh26;
        let fresh27 = pb;
        pb = pb.offset(1);
        diff = d.wrapping_sub(*fresh27);
        d = (diff > d) as libc::c_int as mp_digit;
        if 0 != borrow &&
               {
                   diff = diff.wrapping_sub(1);
                   diff ==
                       (9223372036854775807i64 as
                            libc::c_ulong).wrapping_mul(2u64).wrapping_add(1u64)
               } {
            d = d.wrapping_add(1)
        }
        let fresh28 = pc;
        pc = pc.offset(1);
        *fresh28 = diff;
        borrow = d;
        ix += 1
    }
    limit = (*a).used as libc::c_int;
    while ix < limit {
        let fresh29 = pa;
        pa = pa.offset(1);
        d = *fresh29;
        let fresh30 = pc;
        pc = pc.offset(1);
        diff = d.wrapping_sub(borrow);
        *fresh30 = diff;
        borrow = (diff > d) as libc::c_int as mp_digit;
        ix += 1
    }
    (*c).used = ix as mp_size;
    s_mp_clamp(c);
    return if 0 != borrow { -3i32 } else { 0i32 };
}
/* magnitude comparison */
#[no_mangle]
pub unsafe extern "C" fn s_mp_cmp(mut a: *const mp_int, mut b: *const mp_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut used_a: mp_size = (*a).used;
    let mut used_b: mp_size = (*b).used;
    if !(used_a > used_b) {
        if used_a < used_b {
            current_block = 4062132495826371883;
        } else {
            let mut pa: *mut mp_digit = 0 as *mut mp_digit;
            let mut pb: *mut mp_digit = 0 as *mut mp_digit;
            let mut da: mp_digit = 0i32 as mp_digit;
            let mut db: mp_digit = 0i32 as mp_digit;
            pa = (*a).dp.offset(used_a as isize);
            pb = (*b).dp.offset(used_a as isize);
            loop  {
                if !(used_a >= 4i32 as libc::c_uint) {
                    current_block = 12039483399334584727;
                    break ;
                }
                pa = pa.offset(-4isize);
                pb = pb.offset(-4isize);
                used_a =
                    (used_a as
                         libc::c_uint).wrapping_sub(4i32 as libc::c_uint) as
                        mp_size as mp_size;
                da = *pa.offset(3isize);
                db = *pb.offset(3isize);
                if da != db { current_block = 5782813937026337957; break ; }
                da = *pa.offset(2isize);
                db = *pb.offset(2isize);
                if da != db { current_block = 5782813937026337957; break ; }
                da = *pa.offset(1isize);
                db = *pb.offset(1isize);
                if da != db { current_block = 5782813937026337957; break ; }
                da = *pa.offset(0isize);
                db = *pb.offset(0isize);
                if da != db { current_block = 5782813937026337957; break ; }
            }
            match current_block {
                12039483399334584727 => {
                    loop  {
                        let fresh31 = used_a;
                        used_a = used_a.wrapping_sub(1);
                        if !(fresh31 > 0i32 as libc::c_uint &&
                                 {
                                     pa = pa.offset(-1isize);
                                     da = *pa;
                                     pb = pb.offset(-1isize);
                                     db = *pb;
                                     da == db
                                 }) {
                            break ;
                        }
                    }
                }
                _ => { }
            }
            if da > db {
                current_block = 6700782847853571213;
            } else if da < db {
                current_block = 4062132495826371883;
            } else { return 0i32 }
        }
        match current_block {
            6700782847853571213 => { }
            _ => { return -1i32 }
        }
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_add_3arg(mut a: *const mp_int,
                                       mut b: *const mp_int,
                                       mut c: *mut mp_int) -> mp_err {
    let mut pa: *mut mp_digit = 0 as *mut mp_digit;
    let mut pb: *mut mp_digit = 0 as *mut mp_digit;
    let mut pc: *mut mp_digit = 0 as *mut mp_digit;
    let mut sum: mp_digit = 0;
    let mut carry: mp_digit = 0i32 as mp_digit;
    let mut d: mp_digit = 0;
    let mut ix: mp_size = 0;
    let mut used: mp_size = 0;
    let mut res: mp_err = 0;
    (*c).sign = (*a).sign;
    if (*a).used < (*b).used {
        let mut xch: *const mp_int = a;
        a = b;
        b = xch
    }
    res = s_mp_pad(c, (*a).used);
    if 0i32 != res { return res }
    pa = (*a).dp;
    pb = (*b).dp;
    pc = (*c).dp;
    used = (*b).used;
    ix = 0i32 as mp_size;
    while ix < used {
        let fresh32 = pa;
        pa = pa.offset(1);
        d = *fresh32;
        let fresh33 = pb;
        pb = pb.offset(1);
        sum = d.wrapping_add(*fresh33);
        d = (sum < d) as libc::c_int as mp_digit;
        let fresh34 = pc;
        pc = pc.offset(1);
        sum =
            (sum as libc::c_ulong).wrapping_add(carry) as mp_digit as
                mp_digit;
        *fresh34 = sum;
        carry = d.wrapping_add((sum < carry) as libc::c_int as libc::c_ulong);
        ix = ix.wrapping_add(1)
    }
    used = (*a).used;
    while ix < used {
        let fresh36 = pc;
        pc = pc.offset(1);
        let fresh35 = pa;
        pa = pa.offset(1);
        sum = carry.wrapping_add(*fresh35);
        *fresh36 = sum;
        carry = (sum < carry) as libc::c_int as mp_digit;
        ix = ix.wrapping_add(1)
    }
    if 0 != carry {
        res = s_mp_pad(c, used.wrapping_add(1i32 as libc::c_uint));
        if res != 0i32 { return res }
        *(*c).dp.offset(used as isize) = carry;
        used = used.wrapping_add(1)
    }
    (*c).used = used;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_sub(mut a: *const mp_int, mut b: *const mp_int,
                                mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0;
    let mut magDiff: libc::c_int = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if a == b { mp_zero(c); return 0i32 }
    if (*a).sign != (*b).sign {
        res = s_mp_add_3arg(a, b, c);
        if 0i32 > res {
            current_block = 16161006052407413037;
        } else { current_block = 4808432441040389987; }
    } else {
        magDiff = s_mp_cmp(a, b);
        if 0 == magDiff {
            mp_zero(c);
            res = 0i32;
            current_block = 4808432441040389987;
        } else if magDiff > 0i32 {
            res = s_mp_sub_3arg(a, b, c);
            if 0i32 > res {
                current_block = 16161006052407413037;
            } else { current_block = 4808432441040389987; }
        } else {
            res = s_mp_sub_3arg(b, a, c);
            if 0i32 > res {
                current_block = 16161006052407413037;
            } else {
                (*c).sign = (0 == (*a).sign) as libc::c_int as mp_sign;
                current_block = 4808432441040389987;
            }
        }
    }
    match current_block {
        4808432441040389987 => {
            if s_mp_cmp_d(c, 0i32 as mp_digit) == 0i32 {
                (*c).sign = 0i32 as mp_sign
            }
        }
        _ => { }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_div(mut a: *const mp_int, mut b: *const mp_int,
                                mut q: *mut mp_int, mut r: *mut mp_int)
 -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0;
    let mut pQ: *mut mp_int = 0 as *mut mp_int;
    let mut pR: *mut mp_int = 0 as *mut mp_int;
    let mut qtmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut rtmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut btmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut cmp: libc::c_int = 0;
    let mut signA: mp_sign = 0;
    let mut signB: mp_sign = 0;
    if !(!a.is_null() && !b.is_null()) { return -4i32 }
    signA = (*a).sign;
    signB = (*b).sign;
    if mp_cmp_z(b) == 0i32 { return -3i32 }
    qtmp.dp = 0 as *mut mp_digit;
    rtmp.dp = 0 as *mut mp_digit;
    btmp.dp = 0 as *mut mp_digit;
    /* Set up some temporaries... */
    if r.is_null() || r == a as *mut mp_int || r == b as *mut mp_int {
        res = mp_init_copy(&mut rtmp, a);
        if 0i32 > res {
            current_block = 15368055182570835334;
        } else { pR = &mut rtmp; current_block = 12124785117276362961; }
    } else {
        res = mp_copy(a, r);
        if 0i32 > res {
            current_block = 15368055182570835334;
        } else { pR = r; current_block = 12124785117276362961; }
    }
    match current_block {
        12124785117276362961 => {
            if q.is_null() || q == a as *mut mp_int || q == b as *mut mp_int {
                res = mp_init_size(&mut qtmp, (*a).used);
                if 0i32 > res {
                    current_block = 15368055182570835334;
                } else {
                    pQ = &mut qtmp;
                    current_block = 11057878835866523405;
                }
            } else {
                res = s_mp_pad(q, (*a).used);
                if 0i32 > res {
                    current_block = 15368055182570835334;
                } else {
                    pQ = q;
                    mp_zero(pQ);
                    current_block = 11057878835866523405;
                }
            }
            match current_block {
                15368055182570835334 => { }
                _ => {
                    /*
      If |a| <= |b|, we can compute the solution without division;
      otherwise, we actually do the work required.
     */
                    cmp = s_mp_cmp(a, b);
                    if cmp <= 0i32 {
                        if 0 != cmp {
                            mp_zero(pQ);
                        } else { mp_set(pQ, 1i32 as mp_digit); mp_zero(pR); }
                        current_block = 6417057564578538666;
                    } else {
                        res = mp_init_copy(&mut btmp, b);
                        if 0i32 > res {
                            current_block = 15368055182570835334;
                        } else {
                            res = s_mp_div(pR, &mut btmp, pQ);
                            if 0i32 > res {
                                current_block = 15368055182570835334;
                            } else { current_block = 6417057564578538666; }
                        }
                    }
                    match current_block {
                        15368055182570835334 => { }
                        _ => {
                            (*pR).sign = signA;
                            (*pQ).sign =
                                (if signA == signB { 0i32 } else { 1i32 }) as
                                    mp_sign;
                            if s_mp_cmp_d(pQ, 0i32 as mp_digit) == 0i32 {
                                (*pQ).sign = 0i32 as mp_sign
                            }
                            if s_mp_cmp_d(pR, 0i32 as mp_digit) == 0i32 {
                                (*pR).sign = 0i32 as mp_sign
                            }
                            if !q.is_null() && q != pQ { s_mp_exch(pQ, q); }
                            if !r.is_null() && r != pR { s_mp_exch(pR, r); }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    mp_clear(&mut btmp);
    mp_clear(&mut rtmp);
    mp_clear(&mut qtmp);
    return res;
}
/* magnitude div */
#[no_mangle]
pub unsafe extern "C" fn s_mp_div(mut rem: *mut mp_int, mut div: *mut mp_int,
                                  mut quot: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut part: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut t: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut q_msd: mp_digit = 0;
    let mut res: mp_err = 0;
    let mut d: mp_digit = 0;
    let mut div_msd: mp_digit = 0;
    let mut ix: libc::c_int = 0;
    if mp_cmp_z(div) == 0i32 { return -3i32 }
    t.dp = 0 as *mut mp_digit;
    /* Shortcut if divisor is power of two */
    ix = s_mp_ispow2(div);
    if ix >= 0i32 {
        res = mp_copy(rem, quot);
        if !(0i32 > res) {
            s_mp_div_2d(quot, ix as mp_digit);
            s_mp_mod_2d(rem, ix as mp_digit);
            return 0i32
        }
    } else {
        (*rem).sign = 0i32 as mp_sign;
        (*div).sign = 0i32 as mp_sign;
        part.sign = 0i32 as mp_sign;
        /* A working temporary for division     */
        res = mp_init_size(&mut t, (*rem).alloc);
        if !(0i32 > res) {
            /* Normalize to optimize guessing       */
            res = s_mp_norm(rem, div, &mut d);
            if !(0i32 > res) {
                (*quot).used = (*quot).alloc;
                /* Find a partial substring of rem which is at least div */
    /* If we didn't find one, we're finished dividing    */
                's_84:
                    loop  {
                        if !((*rem).used > (*div).used ||
                                 s_mp_cmp(rem, div) >= 0i32) {
                            current_block = 5892776923941496671;
                            break ;
                        }
                        let mut i: libc::c_int = 0;
                        let mut unusedRem: libc::c_int = 0;
                        /* set to true if we need to extend part */
                        let mut partExtended: libc::c_int = 0i32;
                        unusedRem =
                            (*rem).used.wrapping_sub((*div).used) as
                                libc::c_int;
                        part.dp = (*rem).dp.offset(unusedRem as isize);
                        part.alloc =
                            (*rem).alloc.wrapping_sub(unusedRem as
                                                          libc::c_uint);
                        part.used = (*div).used;
                        if s_mp_cmp(&mut part, div) < 0i32 {
                            unusedRem -= 1;
                            part.dp = part.dp.offset(-1isize);
                            part.used = part.used.wrapping_add(1);
                            part.alloc = part.alloc.wrapping_add(1);
                            partExtended = 1i32
                        }
                        q_msd =
                            *part.dp.offset(part.used.wrapping_sub(1i32 as
                                                                       libc::c_uint)
                                                as isize);
                        div_msd =
                            *(*div).dp.offset((*div).used.wrapping_sub(1i32 as
                                                                           libc::c_uint)
                                                  as isize);
                        if 0 == partExtended {
                            q_msd = 1i32 as mp_digit
                        } else if q_msd == div_msd {
                            q_msd =
                                (9223372036854775807i64 as
                                     libc::c_ulong).wrapping_mul(2u64).wrapping_add(1u64)
                        } else {
                            let mut r: mp_digit = 0;
                            res =
                                s_mpv_div_2dx1d(q_msd,
                                                *part.dp.offset(part.used.wrapping_sub(2i32
                                                                                           as
                                                                                           libc::c_uint)
                                                                    as isize),
                                                div_msd, &mut q_msd, &mut r);
                            if 0i32 > res {
                                current_block = 13062273731226422152;
                                break ;
                            }
                        }
                        if q_msd <= 0i32 as libc::c_ulong {
                            current_block = 5892776923941496671;
                            break ;
                        }
                        mp_copy(div, &mut t);
                        res = s_mp_mul_d(&mut t, q_msd);
                        if 0i32 > res {
                            current_block = 13062273731226422152;
                            break ;
                        }
                        /*
           If it's too big, back it off.  We should not have to do this
           more than once, or, in rare cases, twice.  Knuth describes a
           method by which this could be reduced to a maximum of once, but
           I didn't implement that here.
           When using s_mpv_div_2dx1d, we may have to do this 3 times.
         */
                        i = 4i32;
                        while s_mp_cmp(&mut t, &mut part) > 0i32 && i > 0i32 {
                            q_msd = q_msd.wrapping_sub(1);
                            res = s_mp_sub(&mut t, div);
                            if 0i32 > res {
                                /* t -= div */
                                current_block = 13062273731226422152;
                                break 's_84 ;
                            } else { i -= 1 }
                        }
                        if i < 0i32 {
                            res = -3i32;
                            current_block = 13062273731226422152;
                            break ;
                        } else {
                            /* At this point, q_msd should be the right next digit   */
                            res = s_mp_sub(&mut part, &mut t);
                            if 0i32 > res {
                                /* part -= t */
                                current_block = 13062273731226422152;
                                break ;
                            } else {
                                s_mp_clamp(rem);
                                *(*quot).dp.offset(unusedRem as isize) = q_msd
                            }
                        }
                    }
                match current_block {
                    13062273731226422152 => { }
                    _ => {
                        if 0 != d { s_mp_div_2d(rem, d); }
                        s_mp_clamp(quot);
                    }
                }
            }
        }
    }
    mp_clear(&mut t);
    return res;
}
/* magnitude subtract      */
#[no_mangle]
pub unsafe extern "C" fn s_mp_sub(mut a: *mut mp_int, mut b: *const mp_int)
 -> mp_err {
    let mut pa: *mut mp_digit = 0 as *mut mp_digit;
    let mut pb: *mut mp_digit = 0 as *mut mp_digit;
    let mut limit: *mut mp_digit = 0 as *mut mp_digit;
    let mut d: mp_digit = 0;
    let mut diff: mp_digit = 0;
    let mut borrow: mp_digit = 0i32 as mp_digit;
    pa = (*a).dp;
    pb = (*b).dp;
    limit = pb.offset((*b).used as isize);
    while pb < limit {
        d = *pa;
        let fresh37 = pb;
        pb = pb.offset(1);
        diff = d.wrapping_sub(*fresh37);
        d = (diff > d) as libc::c_int as mp_digit;
        if 0 != borrow &&
               {
                   diff = diff.wrapping_sub(1);
                   diff ==
                       (9223372036854775807i64 as
                            libc::c_ulong).wrapping_mul(2u64).wrapping_add(1u64)
               } {
            d = d.wrapping_add(1)
        }
        let fresh38 = pa;
        pa = pa.offset(1);
        *fresh38 = diff;
        borrow = d
    }
    limit = (*a).dp.offset((*a).used as isize);
    while 0 != borrow && pa < limit {
        d = *pa;
        let fresh39 = pa;
        pa = pa.offset(1);
        diff = d.wrapping_sub(borrow);
        *fresh39 = diff;
        borrow = (diff > d) as libc::c_int as mp_digit
    }
    s_mp_clamp(a);
    return if 0 != borrow { -3i32 } else { 0i32 };
}
/* modulo 2^d in place     */
#[no_mangle]
pub unsafe extern "C" fn s_mp_mod_2d(mut mp: *mut mp_int, mut d: mp_digit) {
    let mut ndig: mp_size =
        d.wrapping_div((8i32 as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                            as libc::c_ulong))
            as mp_size;
    let mut nbit: mp_size =
        d.wrapping_rem((8i32 as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                            as libc::c_ulong))
            as mp_size;
    let mut ix: mp_size = 0;
    let mut dmask: mp_digit = 0;
    if ndig >= (*mp).used { return }
    dmask = ((1i32 as mp_digit) << nbit).wrapping_sub(1i32 as libc::c_ulong);
    let ref mut fresh40 = *(*mp).dp.offset(ndig as isize);
    *fresh40 &= dmask;
    ix = ndig.wrapping_add(1i32 as libc::c_uint);
    while ix < (*mp).used {
        *(*mp).dp.offset(ix as isize) = 0i32 as mp_digit;
        ix = ix.wrapping_add(1)
    }
    s_mp_clamp(mp);
}
/* is v a power of 2?      */
#[no_mangle]
pub unsafe extern "C" fn s_mp_ispow2(mut v: *const mp_int) -> libc::c_int {
    let mut d: mp_digit = 0;
    let mut extra: libc::c_int = 0i32;
    let mut ix: libc::c_int = 0;
    ix = (*v).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    d = *(*v).dp.offset(ix as isize);
    extra = s_mp_ispow2d(d);
    if extra < 0i32 || ix == 0i32 { return extra }
    loop  {
        ix -= 1;
        if !(ix >= 0i32) { break ; }
        if *(*v).dp.offset(ix as isize) != 0i32 as libc::c_ulong {
            return -1i32
        }
        extra =
            (extra as
                 libc::c_ulong).wrapping_add((8i32 as
                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                  as
                                                                                  libc::c_ulong))
                as libc::c_int as libc::c_int
    }
    return extra;
}
/* MP_MODARITH */
/* Comparisons             */
#[no_mangle]
pub unsafe extern "C" fn mp_cmp_z(mut a: *const mp_int) -> libc::c_int {
    if (*a).sign == 1i32 as libc::c_uint {
        return -1i32
    } else if (*a).used == 1i32 as libc::c_uint &&
                  *(*a).dp.offset(0isize) == 0i32 as libc::c_ulong {
        return 0i32
    } else { return 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn mp_div_2d(mut a: *const mp_int, mut d: mp_digit,
                                   mut q: *mut mp_int, mut r: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    if a.is_null() { return -4i32 }
    if !q.is_null() { res = mp_copy(a, q); if res != 0i32 { return res } }
    if !r.is_null() { res = mp_copy(a, r); if res != 0i32 { return res } }
    if !q.is_null() { s_mp_div_2d(q, d); }
    if !r.is_null() { s_mp_mod_2d(r, d); }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_expt(mut a: *mut mp_int, mut b: *mut mp_int,
                                 mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut s: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut x: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    let mut d: mp_digit = 0;
    let mut dig: libc::c_uint = 0;
    let mut bit: libc::c_uint = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if mp_cmp_z(b) < 0i32 { return -3i32 }
    res = mp_init(&mut s);
    if res != 0i32 { return res }
    mp_set(&mut s, 1i32 as mp_digit);
    res = mp_init_copy(&mut x, a);
    if !(res != 0i32) {
        /* Loop over low-order digits in ascending order */
        dig = 0i32 as libc::c_uint;
        's_62:
            loop  {
                if !(dig < (*b).used.wrapping_sub(1i32 as libc::c_uint)) {
                    current_block = 1109700713171191020;
                    break ;
                }
                d = *(*b).dp.offset(dig as isize);
                /* Loop over bits of each non-maximal digit */
                bit = 0i32 as libc::c_uint;
                while (bit as libc::c_ulong) <
                          (8i32 as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                               as
                                                               libc::c_ulong)
                      {
                    if 0 != d & 1i32 as libc::c_ulong {
                        res = s_mp_mul(&mut s, &mut x);
                        if res != 0i32 {
                            current_block = 8260157620059458958;
                            break 's_62 ;
                        }
                    }
                    d >>= 1i32;
                    res = s_mp_sqr(&mut x);
                    if res != 0i32 {
                        current_block = 8260157620059458958;
                        break 's_62 ;
                    }
                    bit = bit.wrapping_add(1)
                }
                dig = dig.wrapping_add(1)
            }
        match current_block {
            1109700713171191020 => {
                d = *(*b).dp.offset(dig as isize);
                loop  {
                    if !(0 != d) {
                        current_block = 11932355480408055363;
                        break ;
                    }
                    if 0 != d & 1i32 as libc::c_ulong {
                        res = s_mp_mul(&mut s, &mut x);
                        if res != 0i32 {
                            current_block = 8260157620059458958;
                            break ;
                        }
                    }
                    d >>= 1i32;
                    res = s_mp_sqr(&mut x);
                    if res != 0i32 {
                        current_block = 8260157620059458958;
                        break ;
                    }
                }
                match current_block {
                    8260157620059458958 => { }
                    _ => {
                        if 0 != mp_iseven(b) { s.sign = (*a).sign }
                        res = mp_copy(&mut s, c)
                    }
                }
            }
            _ => { }
        }
        mp_clear(&mut x);
    }
    mp_clear(&mut s);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_iseven(mut a: *const mp_int) -> libc::c_int {
    return (0 == mp_isodd(a)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mp_isodd(mut a: *const mp_int) -> libc::c_int {
    if a.is_null() { return 0i32 }
    return (*(*a).dp.offset(0isize) & 1i32 as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mp_2expt(mut a: *mut mp_int, mut k: mp_digit)
 -> mp_err {
    if a.is_null() { return -4i32 }
    return s_mp_2expt(a, k);
}
/* a = 2^k                 */
#[no_mangle]
pub unsafe extern "C" fn s_mp_2expt(mut a: *mut mp_int, mut k: mp_digit)
 -> mp_err {
    let mut res: mp_err = 0;
    let mut dig: mp_size = 0;
    let mut bit: mp_size = 0;
    dig =
        k.wrapping_div((8i32 as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                            as libc::c_ulong))
            as mp_size;
    bit =
        k.wrapping_rem((8i32 as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                            as libc::c_ulong))
            as mp_size;
    mp_zero(a);
    res = s_mp_pad(a, dig.wrapping_add(1i32 as libc::c_uint));
    if res != 0i32 { return res }
    let ref mut fresh41 = *(*a).dp.offset(dig as isize);
    *fresh41 |= (1i32 as mp_digit) << bit;
    return 0i32;
}
/* Modular arithmetic      */
#[no_mangle]
pub unsafe extern "C" fn mp_mod(mut a: *const mp_int, mut m: *const mp_int,
                                mut c: *mut mp_int) -> mp_err {
    let mut res: mp_err = 0;
    let mut mag: libc::c_int = 0;
    if !(!a.is_null() && !m.is_null() && !c.is_null()) { return -4i32 }
    if (*m).sign == 1i32 as libc::c_uint { return -3i32 }
    mag = s_mp_cmp(a, m);
    if mag > 0i32 {
        res = mp_div(a, m, 0 as *mut mp_int, c);
        if res != 0i32 { return res }
        if (*c).sign == 1i32 as libc::c_uint {
            res = mp_add(c, m, c);
            if res != 0i32 { return res }
        }
    } else if mag < 0i32 {
        res = mp_copy(a, c);
        if res != 0i32 { return res }
        if mp_cmp_z(a) < 0i32 {
            res = mp_add(c, m, c);
            if res != 0i32 { return res }
        }
    } else { mp_zero(c); }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_mod_d(mut a: *const mp_int, mut d: mp_digit,
                                  mut c: *mut mp_digit) -> mp_err {
    let mut res: mp_err = 0;
    let mut rem: mp_digit = 0;
    if !(!a.is_null() && !c.is_null()) { return -4i32 }
    if s_mp_cmp_d(a, d) > 0i32 {
        res = mp_div_d(a, d, 0 as *mut mp_int, &mut rem);
        if res != 0i32 { return res }
    } else if (*a).sign == 1i32 as libc::c_uint {
        rem = d.wrapping_sub(*(*a).dp.offset(0isize))
    } else { rem = *(*a).dp.offset(0isize) }
    if !c.is_null() { *c = rem }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_addmod(mut a: *const mp_int, mut b: *const mp_int,
                                   mut m: *const mp_int, mut c: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null() && !m.is_null() && !c.is_null()) {
        return -4i32
    }
    res = mp_add(a, b, c);
    if res != 0i32 { return res }
    res = mp_mod(c, m, c);
    if res != 0i32 { return res }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_submod(mut a: *const mp_int, mut b: *const mp_int,
                                   mut m: *const mp_int, mut c: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null() && !m.is_null() && !c.is_null()) {
        return -4i32
    }
    res = mp_sub(a, b, c);
    if res != 0i32 { return res }
    res = mp_mod(c, m, c);
    if res != 0i32 { return res }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_mulmod(mut a: *const mp_int, mut b: *const mp_int,
                                   mut m: *const mp_int, mut c: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null() && !m.is_null() && !c.is_null()) {
        return -4i32
    }
    res = mp_mul(a, b, c);
    if res != 0i32 { return res }
    res = mp_mod(c, m, c);
    if res != 0i32 { return res }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_sqrmod(mut a: *const mp_int, mut m: *const mp_int,
                                   mut c: *mut mp_int) -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !m.is_null() && !c.is_null()) { return -4i32 }
    res = mp_sqr(a, c);
    if res != 0i32 { return res }
    res = mp_mod(c, m, c);
    if res != 0i32 { return res }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_exptmod_d(mut a: *const mp_int, mut d: mp_digit,
                                      mut m: *const mp_int,
                                      mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut s: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut x: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    if !(!a.is_null() && !c.is_null()) { return -4i32 }
    res = mp_init(&mut s);
    if res != 0i32 { return res }
    res = mp_init_copy(&mut x, a);
    if !(res != 0i32) {
        mp_set(&mut s, 1i32 as mp_digit);
        loop  {
            if !(d != 0i32 as libc::c_ulong) {
                current_block = 9606288038608642794;
                break ;
            }
            if 0 != d & 1i32 as libc::c_ulong {
                res = s_mp_mul(&mut s, &mut x);
                if res != 0i32 ||
                       { res = mp_mod(&mut s, m, &mut s); res != 0i32 } {
                    current_block = 5242414451524867936;
                    break ;
                }
            }
            d =
                (d as libc::c_ulong).wrapping_div(2i32 as libc::c_ulong) as
                    mp_digit as mp_digit;
            res = s_mp_sqr(&mut x);
            if res != 0i32 || { res = mp_mod(&mut x, m, &mut x); res != 0i32 }
               {
                current_block = 5242414451524867936;
                break ;
            }
        }
        match current_block {
            9606288038608642794 => { s_mp_exch(&mut s, c); }
            _ => { }
        }
        mp_clear(&mut x);
    }
    mp_clear(&mut s);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_cmp_d(mut a: *const mp_int, mut d: mp_digit)
 -> libc::c_int {
    if a.is_null() { return 0i32 }
    if (*a).sign == 1i32 as libc::c_uint { return -1i32 }
    return s_mp_cmp_d(a, d);
}
#[no_mangle]
pub unsafe extern "C" fn mp_cmp(mut a: *const mp_int, mut b: *const mp_int)
 -> libc::c_int {
    if !(!a.is_null() && !b.is_null()) { return 0i32 }
    if (*a).sign == (*b).sign {
        let mut mag: libc::c_int = 0;
        mag = s_mp_cmp(a, b);
        if mag == 0i32 { return 0i32 }
        if (*a).sign == 0i32 as libc::c_uint {
            return mag
        } else { return -mag }
    } else if (*a).sign == 0i32 as libc::c_uint {
        return 1i32
    } else { return -1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn mp_cmp_mag(mut a: *const mp_int,
                                    mut b: *const mp_int) -> libc::c_int {
    if !(!a.is_null() && !b.is_null()) { return 0i32 }
    return s_mp_cmp(a, b);
}
/* Number theoretic        */
#[no_mangle]
pub unsafe extern "C" fn mp_gcd(mut a: *mut mp_int, mut b: *mut mp_int,
                                mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0;
    let mut u: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut v: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut t: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut k: mp_size = 0i32 as mp_size;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if mp_cmp_z(a) == 0i32 && mp_cmp_z(b) == 0i32 { return -3i32 }
    if mp_cmp_z(a) == 0i32 {
        return mp_copy(b, c)
    } else { if mp_cmp_z(b) == 0i32 { return mp_copy(a, c) } }
    res = mp_init(&mut t);
    if res != 0i32 { return res }
    res = mp_init_copy(&mut u, a);
    if !(res != 0i32) {
        res = mp_init_copy(&mut v, b);
        if !(res != 0i32) {
            u.sign = 0i32 as mp_sign;
            v.sign = 0i32 as mp_sign;
            while 0 != mp_iseven(&mut u) && 0 != mp_iseven(&mut v) {
                s_mp_div_2(&mut u);
                s_mp_div_2(&mut v);
                k = k.wrapping_add(1)
            }
            /* Initialize t */
            if 0 != mp_isodd(&mut u) {
                res = mp_copy(&mut v, &mut t);
                if res != 0i32 {
                    current_block = 1378419234716054762;
                } else {
                    if v.sign == 0i32 as libc::c_uint {
                        t.sign = 1i32 as mp_sign
                    } else { t.sign = 0i32 as mp_sign }
                    current_block = 2569451025026770673;
                }
            } else {
                res = mp_copy(&mut u, &mut t);
                if res != 0i32 {
                    current_block = 1378419234716054762;
                } else { current_block = 2569451025026770673; }
            }
            loop  {
                match current_block {
                    1378419234716054762 => { mp_clear(&mut v); break ; }
                    _ => {
                        while 0 != mp_iseven(&mut t) { s_mp_div_2(&mut t); }
                        if mp_cmp_z(&mut t) == 1i32 {
                            res = mp_copy(&mut t, &mut u);
                            if res != 0i32 {
                                current_block = 1378419234716054762;
                                continue ;
                            }
                        } else {
                            res = mp_copy(&mut t, &mut v);
                            if res != 0i32 {
                                current_block = 1378419234716054762;
                                continue ;
                            }
                            if t.sign == 0i32 as libc::c_uint {
                                v.sign = 1i32 as mp_sign
                            } else { v.sign = 0i32 as mp_sign }
                        }
                        res = mp_sub(&mut u, &mut v, &mut t);
                        if res != 0i32 {
                            current_block = 1378419234716054762;
                            continue ;
                        }
                        if !(s_mp_cmp_d(&mut t, 0i32 as mp_digit) == 0i32) {
                            current_block = 2569451025026770673;
                            continue ;
                        }
                        s_mp_2expt(&mut v, k as mp_digit);
                        res = mp_mul(&mut u, &mut v, c);
                        current_block = 1378419234716054762;
                    }
                }
            }
        }
        mp_clear(&mut u);
    }
    mp_clear(&mut t);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_lcm(mut a: *mut mp_int, mut b: *mut mp_int,
                                mut c: *mut mp_int) -> mp_err {
    let mut gcd: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut prod: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    res = mp_init(&mut gcd);
    if res != 0i32 { return res }
    res = mp_init(&mut prod);
    if !(res != 0i32) {
        res = mp_mul(a, b, &mut prod);
        if !(res != 0i32) {
            res = mp_gcd(a, b, &mut gcd);
            if !(res != 0i32) {
                res = mp_div(&mut prod, &mut gcd, c, 0 as *mut mp_int)
            }
        }
        mp_clear(&mut prod);
    }
    mp_clear(&mut gcd);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_xgcd(mut a: *const mp_int, mut b: *const mp_int,
                                 mut g: *mut mp_int, mut x: *mut mp_int,
                                 mut y: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut gx: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut xc: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut yc: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut u: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut v: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut A: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut B: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut C: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut D: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut clean: [*mut mp_int; 9] = [0 as *mut mp_int; 9];
    let mut res: mp_err = 0;
    let mut last: libc::c_int = -1i32;
    if mp_cmp_z(b) == 0i32 { return -3i32 }
    /* Initialize all these variables we need */
    res = mp_init(&mut u);
    if !(0i32 > res) {
        last += 1;
        clean[last as usize] = &mut u;
        res = mp_init(&mut v);
        if !(0i32 > res) {
            last += 1;
            clean[last as usize] = &mut v;
            res = mp_init(&mut gx);
            if !(0i32 > res) {
                last += 1;
                clean[last as usize] = &mut gx;
                res = mp_init(&mut A);
                if !(0i32 > res) {
                    last += 1;
                    clean[last as usize] = &mut A;
                    res = mp_init(&mut B);
                    if !(0i32 > res) {
                        last += 1;
                        clean[last as usize] = &mut B;
                        res = mp_init(&mut C);
                        if !(0i32 > res) {
                            last += 1;
                            clean[last as usize] = &mut C;
                            res = mp_init(&mut D);
                            if !(0i32 > res) {
                                last += 1;
                                clean[last as usize] = &mut D;
                                res = mp_init_copy(&mut xc, a);
                                if !(0i32 > res) {
                                    last += 1;
                                    clean[last as usize] = &mut xc;
                                    mp_abs(&mut xc, &mut xc);
                                    res = mp_init_copy(&mut yc, b);
                                    if !(0i32 > res) {
                                        last += 1;
                                        clean[last as usize] = &mut yc;
                                        mp_abs(&mut yc, &mut yc);
                                        mp_set(&mut gx, 1i32 as mp_digit);
                                        /* Divide by two until at least one of them is odd */
                                        loop  {
                                            if !(0 != mp_iseven(&mut xc) &&
                                                     0 != mp_iseven(&mut yc))
                                               {
                                                current_block =
                                                    8693738493027456495;
                                                break ;
                                            }
                                            let mut nx: mp_size =
                                                mp_trailing_zeros(&mut xc);
                                            let mut ny: mp_size =
                                                mp_trailing_zeros(&mut yc);
                                            let mut n: mp_size =
                                                if nx < ny { nx } else { ny };
                                            s_mp_div_2d(&mut xc,
                                                        n as mp_digit);
                                            s_mp_div_2d(&mut yc,
                                                        n as mp_digit);
                                            res =
                                                s_mp_mul_2d(&mut gx,
                                                            n as mp_digit);
                                            if 0i32 > res {
                                                current_block =
                                                    15791482327579691142;
                                                break ;
                                            }
                                        }
                                        match current_block {
                                            15791482327579691142 => { }
                                            _ => {
                                                res =
                                                    mp_copy(&mut xc, &mut u);
                                                if !(0i32 > res) {
                                                    res =
                                                        mp_copy(&mut yc,
                                                                &mut v);
                                                    if !(0i32 > res) {
                                                        mp_set(&mut A,
                                                               1i32 as
                                                                   mp_digit);
                                                        mp_set(&mut D,
                                                               1i32 as
                                                                   mp_digit);
                                                        /* Loop through binary GCD algorithm */
                                                        's_181:
                                                            loop  {
                                                                if 0 !=
                                                                       mp_iseven(&mut u)
                                                                   {
                                                                    s_mp_div_2(&mut u);
                                                                    if 0 !=
                                                                           mp_iseven(&mut A)
                                                                           &&
                                                                           0
                                                                               !=
                                                                               mp_iseven(&mut B)
                                                                       {
                                                                        s_mp_div_2(&mut A);
                                                                        s_mp_div_2(&mut B);
                                                                    } else {
                                                                        res =
                                                                            mp_add(&mut A,
                                                                                   &mut yc,
                                                                                   &mut A);
                                                                        if 0i32
                                                                               >
                                                                               res
                                                                           {
                                                                            current_block
                                                                                =
                                                                                15791482327579691142;
                                                                            break
                                                                                ;
                                                                        }
                                                                        s_mp_div_2(&mut A);
                                                                        res =
                                                                            mp_sub(&mut B,
                                                                                   &mut xc,
                                                                                   &mut B);
                                                                        if 0i32
                                                                               >
                                                                               res
                                                                           {
                                                                            current_block
                                                                                =
                                                                                15791482327579691142;
                                                                            break
                                                                                ;
                                                                        }
                                                                        s_mp_div_2(&mut B);
                                                                    }
                                                                } else {
                                                                    while 0 !=
                                                                              mp_iseven(&mut v)
                                                                          {
                                                                        s_mp_div_2(&mut v);
                                                                        if 0
                                                                               !=
                                                                               mp_iseven(&mut C)
                                                                               &&
                                                                               0
                                                                                   !=
                                                                                   mp_iseven(&mut D)
                                                                           {
                                                                            s_mp_div_2(&mut C);
                                                                            s_mp_div_2(&mut D);
                                                                        } else {
                                                                            res
                                                                                =
                                                                                mp_add(&mut C,
                                                                                       &mut yc,
                                                                                       &mut C);
                                                                            if 0i32
                                                                                   >
                                                                                   res
                                                                               {
                                                                                current_block
                                                                                    =
                                                                                    15791482327579691142;
                                                                                break
                                                                                    's_181
                                                                                    ;
                                                                            }
                                                                            s_mp_div_2(&mut C);
                                                                            res
                                                                                =
                                                                                mp_sub(&mut D,
                                                                                       &mut xc,
                                                                                       &mut D);
                                                                            if 0i32
                                                                                   >
                                                                                   res
                                                                               {
                                                                                current_block
                                                                                    =
                                                                                    15791482327579691142;
                                                                                break
                                                                                    's_181
                                                                                    ;
                                                                            }
                                                                            s_mp_div_2(&mut D);
                                                                        }
                                                                    }
                                                                    if mp_cmp(&mut u,
                                                                              &mut v)
                                                                           >=
                                                                           0i32
                                                                       {
                                                                        res =
                                                                            mp_sub(&mut u,
                                                                                   &mut v,
                                                                                   &mut u);
                                                                        if 0i32
                                                                               >
                                                                               res
                                                                           {
                                                                            current_block
                                                                                =
                                                                                15791482327579691142;
                                                                            break
                                                                                ;
                                                                        }
                                                                        res =
                                                                            mp_sub(&mut A,
                                                                                   &mut C,
                                                                                   &mut A);
                                                                        if 0i32
                                                                               >
                                                                               res
                                                                           {
                                                                            current_block
                                                                                =
                                                                                15791482327579691142;
                                                                            break
                                                                                ;
                                                                        }
                                                                        res =
                                                                            mp_sub(&mut B,
                                                                                   &mut D,
                                                                                   &mut B);
                                                                        if 0i32
                                                                               >
                                                                               res
                                                                           {
                                                                            current_block
                                                                                =
                                                                                15791482327579691142;
                                                                            break
                                                                                ;
                                                                        }
                                                                    } else {
                                                                        res =
                                                                            mp_sub(&mut v,
                                                                                   &mut u,
                                                                                   &mut v);
                                                                        if 0i32
                                                                               >
                                                                               res
                                                                           {
                                                                            current_block
                                                                                =
                                                                                15791482327579691142;
                                                                            break
                                                                                ;
                                                                        }
                                                                        res =
                                                                            mp_sub(&mut C,
                                                                                   &mut A,
                                                                                   &mut C);
                                                                        if 0i32
                                                                               >
                                                                               res
                                                                           {
                                                                            current_block
                                                                                =
                                                                                15791482327579691142;
                                                                            break
                                                                                ;
                                                                        }
                                                                        res =
                                                                            mp_sub(&mut D,
                                                                                   &mut B,
                                                                                   &mut D);
                                                                        if 0i32
                                                                               >
                                                                               res
                                                                           {
                                                                            current_block
                                                                                =
                                                                                15791482327579691142;
                                                                            break
                                                                                ;
                                                                        }
                                                                    }
                                                                    if !(mp_cmp_z(&mut u)
                                                                             !=
                                                                             0i32)
                                                                       {
                                                                        current_block
                                                                            =
                                                                            13660591889533726445;
                                                                        break
                                                                            ;
                                                                    }
                                                                }
                                                            }
                                                        match current_block {
                                                            15791482327579691142
                                                            => {
                                                            }
                                                            _ => {
                                                                /* copy results to output */
                                                                if !x.is_null()
                                                                   {
                                                                    res =
                                                                        mp_copy(&mut C,
                                                                                x);
                                                                    if 0i32 >
                                                                           res
                                                                       {
                                                                        current_block
                                                                            =
                                                                            15791482327579691142;
                                                                    } else {
                                                                        current_block
                                                                            =
                                                                            2606304779496145856;
                                                                    }
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        2606304779496145856;
                                                                }
                                                                match current_block
                                                                    {
                                                                    15791482327579691142
                                                                    => {
                                                                    }
                                                                    _ => {
                                                                        if !y.is_null()
                                                                           {
                                                                            res
                                                                                =
                                                                                mp_copy(&mut D,
                                                                                        y);
                                                                            if 0i32
                                                                                   >
                                                                                   res
                                                                               {
                                                                                current_block
                                                                                    =
                                                                                    15791482327579691142;
                                                                            } else {
                                                                                current_block
                                                                                    =
                                                                                    7178192492338286402;
                                                                            }
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                7178192492338286402;
                                                                        }
                                                                        match current_block
                                                                            {
                                                                            15791482327579691142
                                                                            =>
                                                                            {
                                                                            }
                                                                            _
                                                                            =>
                                                                            {
                                                                                if !g.is_null()
                                                                                   {
                                                                                    res
                                                                                        =
                                                                                        mp_mul(&mut gx,
                                                                                               &mut v,
                                                                                               g);
                                                                                    0i32
                                                                                        >
                                                                                        res;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    while last >= 0i32 {
        let fresh42 = last;
        last = last - 1;
        mp_clear(clean[fresh42 as usize]);
    }
    return res;
}
/* Miscellaneous */
#[no_mangle]
pub unsafe extern "C" fn mp_trailing_zeros(mut mp: *const mp_int) -> mp_size {
    let mut d: mp_digit = 0;
    let mut n: mp_size = 0i32 as mp_size;
    let mut ix: libc::c_uint = 0;
    if mp.is_null() || (*mp).dp.is_null() || 0 == mp_cmp_z(mp) { return n }
    ix = 0i32 as libc::c_uint;
    loop  {
        d = *(*mp).dp.offset(ix as isize);
        if !(0 == d && ix < (*mp).used) { break ; }
        n =
            (n as
                 libc::c_ulong).wrapping_add((8i32 as
                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                  as
                                                                                  libc::c_ulong))
                as mp_size as mp_size;
        ix = ix.wrapping_add(1)
    }
    if 0 == d { return 0i32 as mp_size }
    if 0 == d & 0xffffffffu32 as libc::c_ulong {
        d >>= 32i32;
        n =
            (n as libc::c_uint).wrapping_add(32i32 as libc::c_uint) as mp_size
                as mp_size
    }
    if 0 == d & 0xffffu32 as libc::c_ulong {
        d >>= 16i32;
        n =
            (n as libc::c_uint).wrapping_add(16i32 as libc::c_uint) as mp_size
                as mp_size
    }
    if 0 == d & 0xffu32 as libc::c_ulong {
        d >>= 8i32;
        n =
            (n as libc::c_uint).wrapping_add(8i32 as libc::c_uint) as mp_size
                as mp_size
    }
    if 0 == d & 0xfu32 as libc::c_ulong {
        d >>= 4i32;
        n =
            (n as libc::c_uint).wrapping_add(4i32 as libc::c_uint) as mp_size
                as mp_size
    }
    if 0 == d & 0x3u32 as libc::c_ulong {
        d >>= 2i32;
        n =
            (n as libc::c_uint).wrapping_add(2i32 as libc::c_uint) as mp_size
                as mp_size
    }
    if 0 == d & 0x1u32 as libc::c_ulong {
        d >>= 1i32;
        n =
            (n as libc::c_uint).wrapping_add(1i32 as libc::c_uint) as mp_size
                as mp_size
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn mp_invmod(mut a: *const mp_int, mut m: *const mp_int,
                                   mut c: *mut mp_int) -> mp_err {
    if !(!a.is_null() && !m.is_null() && !c.is_null()) { return -4i32 }
    if mp_cmp_z(a) == 0i32 || mp_cmp_z(m) == 0i32 { return -3i32 }
    if 0 != mp_isodd(m) { return s_mp_invmod_odd_m(a, m, c) }
    if 0 != mp_iseven(a) { return -5i32 }
    return s_mp_invmod_even_m(a, m, c);
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_invmod_even_m(mut a: *const mp_int,
                                            mut m: *const mp_int,
                                            mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0;
    let mut k: mp_size = 0;
    /* factors of the modulus */
    let mut oddFactor: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut evenFactor: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    /* parts to combine via CRT. */
    let mut oddPart: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut evenPart: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut C2: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut tmp1: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut tmp2: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    res = s_mp_ispow2(m);
    if res >= 0i32 { k = res as mp_size; return s_mp_invmod_2d(a, k, c) }
    oddFactor.dp = 0 as *mut mp_digit;
    evenFactor.dp = 0 as *mut mp_digit;
    oddPart.dp = 0 as *mut mp_digit;
    evenPart.dp = 0 as *mut mp_digit;
    C2.dp = 0 as *mut mp_digit;
    tmp1.dp = 0 as *mut mp_digit;
    tmp2.dp = 0 as *mut mp_digit;
    res = mp_init_copy(&mut oddFactor, m);
    if !(0i32 > res) {
        /* oddFactor = m */
        res = mp_init(&mut evenFactor);
        if !(0i32 > res) {
            res = mp_init(&mut oddPart);
            if !(0i32 > res) {
                res = mp_init(&mut evenPart);
                if !(0i32 > res) {
                    res = mp_init(&mut C2);
                    if !(0i32 > res) {
                        res = mp_init(&mut tmp1);
                        if !(0i32 > res) {
                            res = mp_init(&mut tmp2);
                            if !(0i32 > res) {
                                k = mp_trailing_zeros(m);
                                s_mp_div_2d(&mut oddFactor, k as mp_digit);
                                res =
                                    s_mp_2expt(&mut evenFactor,
                                               k as mp_digit);
                                if !(0i32 > res) {
                                    /* compute a**-1 mod oddFactor. */
                                    res =
                                        s_mp_invmod_odd_m(a, &mut oddFactor,
                                                          &mut oddPart);
                                    if !(0i32 > res) {
                                        /* compute a**-1 mod evenFactor, where evenFactor == 2**k. */
                                        res =
                                            s_mp_invmod_2d(a, k,
                                                           &mut evenPart);
                                        if !(0i32 > res) {
                                            /* Use Chinese Remainer theorem to compute a**-1 mod m. */
    /* let m1 = oddFactor,  v1 = oddPart,
     * let m2 = evenFactor, v2 = evenPart.
     */
                                            /* Compute C2 = m1**-1 mod m2. */
                                            res =
                                                s_mp_invmod_2d(&mut oddFactor,
                                                               k, &mut C2);
                                            if !(0i32 > res) {
                                                /* compute u = (v2 - v1)*C2 mod m2 */
                                                res =
                                                    mp_sub(&mut evenPart,
                                                           &mut oddPart,
                                                           &mut tmp1);
                                                if !(0i32 > res) {
                                                    res =
                                                        mp_mul(&mut tmp1,
                                                               &mut C2,
                                                               &mut tmp2);
                                                    if !(0i32 > res) {
                                                        s_mp_mod_2d(&mut tmp2,
                                                                    k as
                                                                        mp_digit);
                                                        loop  {
                                                            if !(tmp2.sign !=
                                                                     0i32 as
                                                                         libc::c_uint)
                                                               {
                                                                current_block
                                                                    =
                                                                    18377268871191777778;
                                                                break ;
                                                            }
                                                            res =
                                                                mp_add(&mut tmp2,
                                                                       &mut evenFactor,
                                                                       &mut tmp2);
                                                            if 0i32 > res {
                                                                current_block
                                                                    =
                                                                    15615538888444589552;
                                                                break ;
                                                            }
                                                        }
                                                        match current_block {
                                                            15615538888444589552
                                                            => {
                                                            }
                                                            _ => {
                                                                /* compute answer = v1 + u*m1 */
                                                                res =
                                                                    mp_mul(&mut tmp2,
                                                                           &mut oddFactor,
                                                                           c);
                                                                if !(0i32 >
                                                                         res)
                                                                   {
                                                                    res =
                                                                        mp_add(&mut oddPart,
                                                                               c,
                                                                               c);
                                                                    if !(0i32
                                                                             >
                                                                             res)
                                                                       {
                                                                        /* not sure this is necessary, but it's low cost if not. */
                                                                        res =
                                                                            mp_mod(c,
                                                                                   m,
                                                                                   c);
                                                                        0i32 >
                                                                            res;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mp_clear(&mut oddFactor);
    mp_clear(&mut evenFactor);
    mp_clear(&mut oddPart);
    mp_clear(&mut evenPart);
    mp_clear(&mut C2);
    mp_clear(&mut tmp1);
    mp_clear(&mut tmp2);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_invmod_2d(mut a: *const mp_int, mut k: mp_size,
                                        mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0;
    let mut ix: mp_size = k.wrapping_add(4i32 as libc::c_uint);
    let mut t0: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut t1: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut val: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut two2k: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    static mut d2: mp_digit = 2i32 as mp_digit;
    static mut two: mp_int =
        unsafe {
            mp_int{sign: 0i32 as mp_sign,
                   alloc: 1i32 as mp_size,
                   used: 1i32 as mp_size,
                   dp: &d2 as *const mp_digit as *mut mp_digit,}
        };
    if 0 != mp_iseven(a) { return -5i32 }
    if k as libc::c_ulong <=
           (8i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                as libc::c_ulong) {
        let mut i: mp_digit = s_mp_invmod_radix(*(*a).dp.offset(0isize));
        if (k as libc::c_ulong) <
               (8i32 as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                    as libc::c_ulong) {
            i &= ((1i32 as mp_digit) << k).wrapping_sub(1i32 as mp_digit)
        }
        mp_set(c, i);
        return 0i32
    }
    t0.dp = 0 as *mut mp_digit;
    t1.dp = 0 as *mut mp_digit;
    val.dp = 0 as *mut mp_digit;
    tmp.dp = 0 as *mut mp_digit;
    two2k.dp = 0 as *mut mp_digit;
    res = mp_init_copy(&mut val, a);
    if !(0i32 > res) {
        s_mp_mod_2d(&mut val, k as mp_digit);
        res = mp_init_copy(&mut t0, &mut val);
        if !(0i32 > res) {
            res = mp_init_copy(&mut t1, &mut t0);
            if !(0i32 > res) {
                res = mp_init(&mut tmp);
                if !(0i32 > res) {
                    res = mp_init(&mut two2k);
                    if !(0i32 > res) {
                        res = s_mp_2expt(&mut two2k, k as mp_digit);
                        if !(0i32 > res) {
                            's_116:
                                loop  {
                                    res = mp_mul(&mut val, &mut t1, &mut tmp);
                                    if 0i32 > res {
                                        current_block = 8041589498492679283;
                                        break ;
                                    }
                                    res = mp_sub(&two, &mut tmp, &mut tmp);
                                    if 0i32 > res {
                                        current_block = 8041589498492679283;
                                        break ;
                                    }
                                    res = mp_mul(&mut t1, &mut tmp, &mut t1);
                                    if 0i32 > res {
                                        current_block = 8041589498492679283;
                                        break ;
                                    }
                                    s_mp_mod_2d(&mut t1, k as mp_digit);
                                    while t1.sign != 0i32 as libc::c_uint {
                                        res =
                                            mp_add(&mut t1, &mut two2k,
                                                   &mut t1);
                                        if 0i32 > res {
                                            current_block =
                                                8041589498492679283;
                                            break 's_116 ;
                                        }
                                    }
                                    if mp_cmp(&mut t1, &mut t0) == 0i32 {
                                        current_block = 14136749492126903395;
                                        break ;
                                    }
                                    res = mp_copy(&mut t1, &mut t0);
                                    if 0i32 > res {
                                        current_block = 8041589498492679283;
                                        break ;
                                    }
                                    ix = ix.wrapping_sub(1);
                                    if !(ix > 0i32 as libc::c_uint) {
                                        current_block = 14136749492126903395;
                                        break ;
                                    }
                                }
                            match current_block {
                                8041589498492679283 => { }
                                _ => {
                                    if 0 == ix {
                                        res = -5i32
                                    } else { mp_exch(c, &mut t1); }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mp_clear(&mut t0);
    mp_clear(&mut t1);
    mp_clear(&mut val);
    mp_clear(&mut tmp);
    mp_clear(&mut two2k);
    return res;
}
/* returns (P ** -1) mod RADIX */
#[no_mangle]
pub unsafe extern "C" fn s_mp_invmod_radix(mut P: mp_digit) -> mp_digit {
    let mut T: mp_digit = P;
    T =
        (T as
             libc::c_ulong).wrapping_mul((2i32 as
                                              libc::c_ulong).wrapping_sub(P.wrapping_mul(T)))
            as mp_digit as mp_digit;
    T =
        (T as
             libc::c_ulong).wrapping_mul((2i32 as
                                              libc::c_ulong).wrapping_sub(P.wrapping_mul(T)))
            as mp_digit as mp_digit;
    T =
        (T as
             libc::c_ulong).wrapping_mul((2i32 as
                                              libc::c_ulong).wrapping_sub(P.wrapping_mul(T)))
            as mp_digit as mp_digit;
    T =
        (T as
             libc::c_ulong).wrapping_mul((2i32 as
                                              libc::c_ulong).wrapping_sub(P.wrapping_mul(T)))
            as mp_digit as mp_digit;
    T =
        (T as
             libc::c_ulong).wrapping_mul((2i32 as
                                              libc::c_ulong).wrapping_sub(P.wrapping_mul(T)))
            as mp_digit as mp_digit;
    T =
        (T as
             libc::c_ulong).wrapping_mul((2i32 as
                                              libc::c_ulong).wrapping_sub(P.wrapping_mul(T)))
            as mp_digit as mp_digit;
    return T;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_invmod_odd_m(mut a: *const mp_int,
                                           mut m: *const mp_int,
                                           mut c: *mut mp_int) -> mp_err {
    let mut k: libc::c_int = 0;
    let mut res: mp_err = 0;
    let mut x: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    if !(!a.is_null() && !m.is_null() && !c.is_null()) { return -4i32 }
    if mp_cmp_z(a) == 0i32 || mp_cmp_z(m) == 0i32 { return -3i32 }
    if 0 != mp_iseven(m) { return -5i32 }
    x.dp = 0 as *mut mp_digit;
    if a == c {
        res = mp_init_copy(&mut x, a);
        if res != 0i32 { return res }
        if a == m { m = &mut x }
        a = &mut x
    } else if m == c {
        res = mp_init_copy(&mut x, m);
        if res != 0i32 { return res }
        m = &mut x
    } else { x.dp = 0 as *mut mp_digit }
    res = s_mp_almost_inverse(a, m, c);
    if !(0i32 > res) {
        k = res;
        res = s_mp_fixup_reciprocal(c, m, k, c);
        0i32 > res;
    }
    mp_clear(&mut x);
    return res;
}
/* Given c, k, and prime p, where a*c == 2**k (mod p),
** Compute x = (a ** -1) mod p.  This is similar to Montgomery reduction.
** This technique from the paper "Fast Modular Reciprocals" (unpublished)
** by Richard Schroeppel (a.k.a. Captain Nemo).
*/
#[no_mangle]
pub unsafe extern "C" fn s_mp_fixup_reciprocal(mut c: *const mp_int,
                                               mut p: *const mp_int,
                                               mut k: libc::c_int,
                                               mut x: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut k_orig: libc::c_int = k;
    let mut r: mp_digit = 0;
    let mut ix: mp_size = 0;
    let mut res: mp_err = 0;
    if mp_cmp_z(c) < 0i32 {
        /* c < 0 */
        res = mp_add(c, p, x);
        if 0i32 > res {
            /* x = c + p */
            current_block = 11846794541971026000;
        } else { current_block = 10879442775620481940; }
    } else {
        res = mp_copy(c, x);
        if 0i32 > res {
            /* x = c */
            current_block = 11846794541971026000;
        } else { current_block = 10879442775620481940; }
    }
    match current_block {
        10879442775620481940 => {
            ix =
                (k as
                     libc::c_ulong).wrapping_add((8i32 as
                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_sub(1i32
                                                                                                                       as
                                                                                                                       libc::c_ulong).wrapping_div((8i32
                                                                                                                                                        as
                                                                                                                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                                                                                                        as
                                                                                                                                                                                        libc::c_ulong)).wrapping_add((*p).used
                                                                                                                                                                                                                         as
                                                                                                                                                                                                                         libc::c_ulong).wrapping_add(1i32
                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                         libc::c_ulong)
                    as mp_size;
            ix = if ix > (*x).used { ix } else { (*x).used };
            res = s_mp_pad(x, ix);
            if !(0i32 > res) {
                r =
                    (0i32 as
                         libc::c_ulong).wrapping_sub(s_mp_invmod_radix(*(*p).dp.offset(0isize)));
                ix = 0i32 as mp_size;
                while k > 0i32 {
                    let mut j: libc::c_int =
                        (if (k as libc::c_ulong) <
                                (8i32 as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                     as
                                                                     libc::c_ulong)
                            {
                             k as libc::c_ulong
                         } else {
                             (8i32 as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                  as
                                                                  libc::c_ulong)
                         }) as libc::c_int;
                    let mut v: mp_digit =
                        r.wrapping_mul(*(*x).dp.offset(ix as isize));
                    if (j as libc::c_ulong) <
                           (8i32 as
                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                as
                                                                libc::c_ulong)
                       {
                        v &=
                            ((1i32 as mp_digit) <<
                                 j).wrapping_sub(1i32 as libc::c_ulong)
                    }
                    s_mpv_mul_d_add_prop((*p).dp, (*p).used, v,
                                         (*x).dp.offset(ix as isize));
                    k -= j;
                    ix = ix.wrapping_add(1)
                }
                s_mp_clamp(x);
                s_mp_div_2d(x, k_orig as mp_digit);
                res = 0i32
            }
        }
        _ => { }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn s_mpv_mul_d_add_prop(mut a: *const mp_digit,
                                              mut a_len: mp_size,
                                              mut b: mp_digit,
                                              mut c: *mut mp_digit) {
    let mut carry: mp_digit = 0i32 as mp_digit;
    loop  {
        let fresh43 = a_len;
        a_len = a_len.wrapping_sub(1);
        if !(0 != fresh43) { break ; }
        let fresh44 = a;
        a = a.offset(1);
        let mut a_i: mp_digit = *fresh44;
        let mut a0b0: mp_digit = 0;
        let mut a1b1: mp_digit = 0;
        let mut a0b1: mp_digit = 0;
        let mut a1b0: mp_digit = 0;
        a0b0 =
            (a_i &
                 (2147483647i32 as
                      libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                     libc::c_ulong).wrapping_mul(b &
                                                     (2147483647i32 as
                                                          libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                         as libc::c_ulong);
        a1b1 =
            (a_i >>
                 (8i32 as
                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                      as
                                                      libc::c_ulong).wrapping_div(2i32
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_mul(b
                                                                                                                       >>
                                                                                                                       (8i32
                                                                                                                            as
                                                                                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                                                                            as
                                                                                                                                                            libc::c_ulong).wrapping_div(2i32
                                                                                                                                                                                            as
                                                                                                                                                                                            libc::c_ulong));
        a0b1 =
            (a_i &
                 (2147483647i32 as
                      libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                     libc::c_ulong).wrapping_mul(b >>
                                                     (8i32 as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_div(2i32
                                                                                                                          as
                                                                                                                          libc::c_ulong));
        a1b0 =
            (a_i >>
                 (8i32 as
                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                      as
                                                      libc::c_ulong).wrapping_div(2i32
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_mul(b
                                                                                                                       &
                                                                                                                       (2147483647i32
                                                                                                                            as
                                                                                                                            libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                                                                                           as
                                                                                                                           libc::c_ulong);
        a1b0 =
            (a1b0 as libc::c_ulong).wrapping_add(a0b1) as mp_digit as
                mp_digit;
        a1b1 =
            (a1b1 as
                 libc::c_ulong).wrapping_add(a1b0 >>
                                                 (8i32 as
                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_div(2i32
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                as mp_digit as mp_digit;
        if a1b0 < a0b1 {
            a1b1 =
                (a1b1 as
                     libc::c_ulong).wrapping_add((1i32 as
                                                      libc::c_ulong).wrapping_add((2147483647i32
                                                                                       as
                                                                                       libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                                                      as
                                                                                      mp_digit))
                    as mp_digit as mp_digit
        }
        a1b0 <<=
            (8i32 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                 as
                                                 libc::c_ulong).wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_ulong);
        a0b0 =
            (a0b0 as libc::c_ulong).wrapping_add(a1b0) as mp_digit as
                mp_digit;
        if a0b0 < a1b0 { a1b1 = a1b1.wrapping_add(1) }
        a0b0 =
            (a0b0 as libc::c_ulong).wrapping_add(carry) as mp_digit as
                mp_digit;
        if a0b0 < carry { a1b1 = a1b1.wrapping_add(1) }
        a_i = *c;
        a0b0 =
            (a0b0 as libc::c_ulong).wrapping_add(a_i) as mp_digit as mp_digit;
        if a0b0 < a_i { a1b1 = a1b1.wrapping_add(1) }
        let fresh45 = c;
        c = c.offset(1);
        *fresh45 = a0b0;
        carry = a1b1
    }
    while 0 != carry {
        let mut c_i: mp_digit = *c;
        carry =
            (carry as libc::c_ulong).wrapping_add(c_i) as mp_digit as
                mp_digit;
        let fresh46 = c;
        c = c.offset(1);
        *fresh46 = carry;
        carry = (carry < c_i) as libc::c_int as mp_digit
    };
}
/* Given a and prime p, computes c and k such that a*c == 2**k (mod p).
** Returns k (positive) or error (negative).
** This technique from the paper "Fast Modular Reciprocals" (unpublished)
** by Richard Schroeppel (a.k.a. Captain Nemo).
*/
#[no_mangle]
pub unsafe extern "C" fn s_mp_almost_inverse(mut a: *const mp_int,
                                             mut p: *const mp_int,
                                             mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0;
    let mut k: mp_err = 0i32;
    let mut d: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut f: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut g: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    if !(!a.is_null() && !p.is_null() && !c.is_null()) { return -4i32 }
    d.dp = 0 as *mut mp_digit;
    f.dp = 0 as *mut mp_digit;
    g.dp = 0 as *mut mp_digit;
    res = mp_init(&mut d);
    if !(0i32 > res) {
        res = mp_init_copy(&mut f, a);
        if !(0i32 > res) {
            /* f = a */
            res = mp_init_copy(&mut g, p);
            if !(0i32 > res) {
                /* g = p */
                mp_set(c, 1i32 as mp_digit);
                mp_zero(&mut d);
                if mp_cmp_z(&mut f) == 0i32 {
                    res = -5i32;
                    current_block = 2543120759711851213;
                } else {
                    's_86:
                        loop  {
                            let mut diff_sign: libc::c_int = 0;
                            while 0 != mp_iseven(&mut f) {
                                let mut n: mp_size =
                                    mp_trailing_zeros(&mut f);
                                if 0 == n {
                                    res = -5i32;
                                    current_block = 13859260674977301460;
                                    break 's_86 ;
                                } else {
                                    s_mp_div_2d(&mut f, n as mp_digit);
                                    res = s_mp_mul_2d(&mut d, n as mp_digit);
                                    if 0i32 > res {
                                        current_block = 13859260674977301460;
                                        break 's_86 ;
                                    }
                                    k =
                                        (k as libc::c_uint).wrapping_add(n) as
                                            mp_err as mp_err
                                }
                            }
                            if mp_cmp_d(&mut f, 1i32 as mp_digit) == 0i32 {
                                /* f == 1 */
                                res = k;
                                current_block = 2543120759711851213;
                                break ;
                            } else {
                                diff_sign = mp_cmp(&mut f, &mut g);
                                if diff_sign < 0i32 {
                                    s_mp_exch(&mut f, &mut g);
                                    s_mp_exch(c, &mut d);
                                } else if diff_sign == 0i32 {
                                    /* f == g */
                                    res = -5i32;
                                    current_block = 2543120759711851213;
                                    break ;
                                }
                                if (*f.dp.offset(0isize)).wrapping_rem(4i32 as
                                                                           libc::c_ulong)
                                       ==
                                       (*g.dp.offset(0isize)).wrapping_rem(4i32
                                                                               as
                                                                               libc::c_ulong)
                                   {
                                    res = mp_sub(&mut f, &mut g, &mut f);
                                    if 0i32 > res {
                                        /* f = f - g */
                                        current_block = 13859260674977301460;
                                        break ;
                                    } else {
                                        res = mp_sub(c, &mut d, c);
                                        if !(0i32 > res) { continue ; }
                                        /* c = c - d */
                                        current_block = 13859260674977301460;
                                        break ;
                                    }
                                } else {
                                    res = mp_add(&mut f, &mut g, &mut f);
                                    if 0i32 > res {
                                        /* f = f + g */
                                        current_block = 13859260674977301460;
                                        break ;
                                    } else {
                                        res = mp_add(c, &mut d, c);
                                        if !(0i32 > res) { continue ; }
                                        /* c = c + d */
                                        current_block = 13859260674977301460;
                                        break ;
                                    }
                                }
                            }
                        }
                }
                match current_block {
                    13859260674977301460 => { }
                    _ => {
                        if res >= 0i32 {
                            loop  {
                                if !((*c).sign != 0i32 as libc::c_uint) {
                                    current_block = 6174974146017752131;
                                    break ;
                                }
                                res = mp_add(c, p, c);
                                if 0i32 > res {
                                    current_block = 13859260674977301460;
                                    break ;
                                }
                            }
                            match current_block {
                                13859260674977301460 => { }
                                _ => { res = k }
                            }
                        }
                    }
                }
            }
        }
    }
    mp_clear(&mut d);
    mp_clear(&mut f);
    mp_clear(&mut g);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_invmod_xgcd(mut a: *const mp_int,
                                        mut m: *const mp_int,
                                        mut c: *mut mp_int) -> mp_err {
    let mut g: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut x: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    if !(!a.is_null() && !m.is_null() && !c.is_null()) { return -4i32 }
    if mp_cmp_z(a) == 0i32 || mp_cmp_z(m) == 0i32 { return -3i32 }
    g.dp = 0 as *mut mp_digit;
    x.dp = 0 as *mut mp_digit;
    res = mp_init(&mut x);
    if !(0i32 > res) {
        res = mp_init(&mut g);
        if !(0i32 > res) {
            res = mp_xgcd(a, m, &mut g, &mut x, 0 as *mut mp_int);
            if !(0i32 > res) {
                if mp_cmp_d(&mut g, 1i32 as mp_digit) != 0i32 {
                    res = -5i32
                } else { res = mp_mod(&mut x, m, c); (*c).sign = (*a).sign }
            }
        }
    }
    mp_clear(&mut x);
    mp_clear(&mut g);
    return res;
}
/* Input and output        */
#[no_mangle]
pub unsafe extern "C" fn mp_print(mut mp: *const mp_int, mut ofp: *mut FILE) {
    let mut ix: libc::c_int = 0;
    if mp.is_null() || ofp.is_null() { return }
    fputc(if (*mp).sign == 1i32 as libc::c_uint {
              '-' as i32
          } else { '+' as i32 }, ofp);
    ix = (*mp).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    while ix >= 0i32 {
        fprintf(ofp, b"%016lX\x00" as *const u8 as *const libc::c_char,
                *(*mp).dp.offset(ix as isize));
        ix -= 1
    };
}
/* end MP_IOFUNC */
/* Base conversion         */
#[no_mangle]
pub unsafe extern "C" fn mp_read_raw(mut mp: *mut mp_int,
                                     mut str: *mut libc::c_char,
                                     mut len: libc::c_int) -> mp_err {
    let mut ix: libc::c_int = 0;
    let mut res: mp_err = 0;
    let mut ustr: *mut libc::c_uchar = str as *mut libc::c_uchar;
    if !(!mp.is_null() && !str.is_null() && len > 0i32) { return -4i32 }
    mp_zero(mp);
    if 0 != *ustr.offset(0isize) {
        (*mp).sign = 1i32 as mp_sign
    } else { (*mp).sign = 0i32 as mp_sign }
    ix = 1i32;
    while ix < len {
        res = mp_mul_d(mp, 256i32 as mp_digit, mp);
        if res != 0i32 { return res }
        res = mp_add_d(mp, *ustr.offset(ix as isize) as mp_digit, mp);
        if res != 0i32 { return res }
        ix += 1
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_raw_size(mut mp: *const mp_int) -> libc::c_int {
    if mp.is_null() { return 0i32 }
    return ((*mp).used as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                as
                                                libc::c_ulong).wrapping_add(1i32
                                                                                as
                                                                                libc::c_ulong)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mp_toraw(mut mp: *mut mp_int,
                                  mut str: *mut libc::c_char) -> mp_err {
    let mut ix: libc::c_int = 0;
    let mut jx: libc::c_int = 0;
    let mut pos: libc::c_int = 1i32;
    if !(!mp.is_null() && !str.is_null()) { return -4i32 }
    *str.offset(0isize) = (*mp).sign as libc::c_char;
    ix = (*mp).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    while ix >= 0i32 {
        let mut d: mp_digit = *(*mp).dp.offset(ix as isize);
        jx =
            (::std::mem::size_of::<mp_digit>() as
                 libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as
                libc::c_int;
        while jx >= 0i32 {
            let fresh47 = pos;
            pos = pos + 1;
            *str.offset(fresh47 as isize) = (d >> jx * 8i32) as libc::c_char;
            jx -= 1
        }
        ix -= 1
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_read_radix(mut mp: *mut mp_int,
                                       mut str: *const libc::c_char,
                                       mut radix: libc::c_int) -> mp_err {
    let mut ix: libc::c_int = 0i32;
    let mut val: libc::c_int = 0i32;
    let mut res: mp_err = 0;
    let mut sig: mp_sign = 0i32 as mp_sign;
    if !(!mp.is_null() && !str.is_null() && radix >= 2i32 && radix <= 64i32) {
        return -4i32
    }
    mp_zero(mp);
    while 0 != *str.offset(ix as isize) as libc::c_int &&
              s_mp_tovalue(*str.offset(ix as isize), radix) < 0i32 &&
              *str.offset(ix as isize) as libc::c_int != '-' as i32 &&
              *str.offset(ix as isize) as libc::c_int != '+' as i32 {
        ix += 1
    }
    if *str.offset(ix as isize) as libc::c_int == '-' as i32 {
        sig = 1i32 as mp_sign;
        ix += 1
    } else if *str.offset(ix as isize) as libc::c_int == '+' as i32 {
        sig = 0i32 as mp_sign;
        ix += 1
    }
    loop  {
        val = s_mp_tovalue(*str.offset(ix as isize), radix);
        if !(val >= 0i32) { break ; }
        res = s_mp_mul_d(mp, radix as mp_digit);
        if res != 0i32 { return res }
        res = s_mp_add_d(mp, val as mp_digit);
        if res != 0i32 { return res }
        ix += 1
    }
    if s_mp_cmp_d(mp, 0i32 as mp_digit) == 0i32 {
        (*mp).sign = 0i32 as mp_sign
    } else { (*mp).sign = sig }
    return 0i32;
}
/* convert ch to value    */
#[no_mangle]
pub unsafe extern "C" fn s_mp_tovalue(mut ch: libc::c_char,
                                      mut r: libc::c_int) -> libc::c_int {
    let mut val: libc::c_int = 0;
    let mut xch: libc::c_int = 0;
    if r > 36i32 {
        xch = ch as libc::c_int
    } else { xch = toupper(ch as libc::c_int) }
    if 0 != isdigit(xch) {
        val = xch - '0' as i32
    } else if 0 != isupper(xch) {
        val = xch - 'A' as i32 + 10i32
    } else if 0 != islower(xch) {
        val = xch - 'a' as i32 + 36i32
    } else if xch == '+' as i32 {
        val = 62i32
    } else if xch == '/' as i32 { val = 63i32 } else { return -1i32 }
    if val < 0i32 || val >= r { return -1i32 }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mp_read_variable_radix(mut a: *mut mp_int,
                                                mut str: *const libc::c_char,
                                                mut default_radix:
                                                    libc::c_int) -> mp_err {
    let mut radix: libc::c_int = default_radix;
    let mut cx: libc::c_int = 0;
    let mut sig: mp_sign = 0i32 as mp_sign;
    let mut res: mp_err = 0;
    loop  {
        cx = *str as libc::c_int;
        if !(cx != 0i32 && s_mp_tovalue(cx as libc::c_char, radix) < 0i32 &&
                 cx != '-' as i32 && cx != '+' as i32) {
            break ;
        }
        str = str.offset(1isize)
    }
    if cx == '-' as i32 {
        sig = 1i32 as mp_sign;
        str = str.offset(1isize)
    } else if cx == '+' as i32 {
        sig = 0i32 as mp_sign;
        str = str.offset(1isize)
    }
    if *str.offset(0isize) as libc::c_int == '0' as i32 {
        if *str.offset(1isize) as libc::c_int | 0x20i32 == 'x' as i32 {
            radix = 16i32;
            str = str.offset(2isize)
        } else { radix = 8i32; str = str.offset(1isize) }
    }
    res = mp_read_radix(a, str, radix);
    if res == 0i32 {
        (*a).sign =
            if s_mp_cmp_d(a, 0i32 as mp_digit) == 0i32 {
                0i32 as libc::c_uint
            } else { sig }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_radix_size(mut mp: *mut mp_int,
                                       mut radix: libc::c_int)
 -> libc::c_int {
    let mut bits: libc::c_int = 0;
    if mp.is_null() || radix < 2i32 || radix > 64i32 { return 0i32 }
    bits =
        ((*mp).used as
             libc::c_ulong).wrapping_mul((8i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                              as
                                                                              libc::c_ulong)).wrapping_sub(1i32
                                                                                                               as
                                                                                                               libc::c_ulong)
            as libc::c_int;
    return s_mp_outlen(bits, radix);
}
/* output length in bytes */
#[no_mangle]
pub unsafe extern "C" fn s_mp_outlen(mut bits: libc::c_int,
                                     mut r: libc::c_int) -> libc::c_int {
    return (bits as libc::c_double * s_logv_2[r as usize] as libc::c_double +
                1.5f64) as libc::c_int + 1i32;
}
/*
 *  mpi-priv.h  - Private header file for MPI
 *  Arbitrary precision integer arithmetic library
 *
 *  NOTE WELL: the content of this header file is NOT part of the "public"
 *  API for the MPI library, and may change at any time.
 *  Application programs that use libmpi should NOT include this header file.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* If we aren't using a wired-in logarithm table, we need to include
   the math library to get the log() function
 */
/* {{{ s_logv_2[] - log table for 2 in various bases */
/*
  A table of the logs of 2 for various bases (the 0 and 1 entries of
  this table are meaningless and should not be referenced).

  This table is used to compute output lengths for the mp_toradix()
  function.  Since a number n in radix r takes up about log_r(n)
  digits, we estimate the output size by taking the least integer
  greater than log_r(n), where:

  log_r(n) = log_2(n) * log_r(2)

  This table, therefore, is a table of log_r(2) for 2 <= r <= 36,
  which are the output bases supported.
 */
#[no_mangle]
pub static mut s_logv_2: [libc::c_float; 65] =
    [0.000000000f32, 0.000000000f32, 1.000000000f32, 0.630929754f32,
     0.500000000f32, 0.430676558f32, 0.386852807f32, 0.356207187f32,
     0.333333333f32, 0.315464877f32, 0.301029996f32, 0.289064826f32,
     0.278942946f32, 0.270238154f32, 0.262649535f32, 0.255958025f32,
     0.250000000f32, 0.244650542f32, 0.239812467f32, 0.235408913f32,
     0.231378213f32, 0.227670249f32, 0.224243824f32, 0.221064729f32,
     0.218104292f32, 0.215338279f32, 0.212746054f32, 0.210309918f32,
     0.208014598f32, 0.205846832f32, 0.203795047f32, 0.201849087f32,
     0.200000000f32, 0.198239863f32, 0.196561632f32, 0.194959022f32,
     0.193426404f32, 0.191958720f32, 0.190551412f32, 0.189200360f32,
     0.187901825f32, 0.186652411f32, 0.185449023f32, 0.184288833f32,
     0.183169251f32, 0.182087900f32, 0.181042597f32, 0.180031327f32,
     0.179052232f32, 0.178103594f32, 0.177183820f32, 0.176291434f32,
     0.175425064f32, 0.174583430f32, 0.173765343f32, 0.172969690f32,
     0.172195434f32, 0.171441601f32, 0.170707280f32, 0.169991616f32,
     0.169293808f32, 0.168613099f32, 0.167948779f32, 0.167300179f32,
     0.166666667f32];
#[no_mangle]
pub unsafe extern "C" fn mp_toradix(mut mp: *mut mp_int,
                                    mut str: *mut libc::c_char,
                                    mut radix: libc::c_int) -> mp_err {
    let mut ix: libc::c_int = 0;
    let mut pos: libc::c_int = 0i32;
    if !(!mp.is_null() && !str.is_null()) { return -4i32 }
    if !(radix > 1i32 && radix <= 64i32) { return -3i32 }
    if mp_cmp_z(mp) == 0i32 {
        *str.offset(0isize) = '0' as i32 as libc::c_char;
        *str.offset(1isize) = '\u{0}' as i32 as libc::c_char
    } else {
        let mut res: mp_err = 0;
        let mut tmp: mp_int =
            mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
        let mut sgn: mp_sign = 0;
        let mut rem: mp_digit = 0;
        let mut rdx: mp_digit = radix as mp_digit;
        let mut ch: libc::c_char = 0;
        res = mp_init_copy(&mut tmp, mp);
        if res != 0i32 { return res }
        sgn = tmp.sign;
        tmp.sign = 0i32 as mp_sign;
        while mp_cmp_z(&mut tmp) != 0i32 {
            res = mp_div_d(&mut tmp, rdx, &mut tmp, &mut rem);
            if res != 0i32 { mp_clear(&mut tmp); return res }
            ch = s_mp_todigit(rem, radix, 0i32);
            let fresh48 = pos;
            pos = pos + 1;
            *str.offset(fresh48 as isize) = ch
        }
        if sgn == 1i32 as libc::c_uint {
            let fresh49 = pos;
            pos = pos + 1;
            *str.offset(fresh49 as isize) = '-' as i32 as libc::c_char
        }
        let fresh50 = pos;
        pos = pos - 1;
        *str.offset(fresh50 as isize) = '\u{0}' as i32 as libc::c_char;
        ix = 0i32;
        while ix < pos {
            let mut tmp_0: libc::c_char = *str.offset(ix as isize);
            *str.offset(ix as isize) = *str.offset(pos as isize);
            *str.offset(pos as isize) = tmp_0;
            ix += 1;
            pos -= 1
        }
        mp_clear(&mut tmp);
    }
    return 0i32;
}
/* convert val to digit */
#[no_mangle]
pub unsafe extern "C" fn s_mp_todigit(mut val: mp_digit, mut r: libc::c_int,
                                      mut low: libc::c_int) -> libc::c_char {
    let mut ch: libc::c_char = 0;
    if val >= r as libc::c_ulong { return 0i32 as libc::c_char }
    ch = *s_dmap_1.offset(val as isize);
    if r <= 36i32 && 0 != low {
        ch = tolower(ch as libc::c_int) as libc::c_char
    }
    return ch;
}
/* say what?            */
/* MP_OKAY, MP_YES      */
/* MP_NO                */
/* MP_MEM               */
/* MP_RANGE             */
/* MP_BADARG            */
/* MP_UNDEF             */
/* Value to digit maps for radix conversion   */
/* s_dmap_1 - standard digits and letters */
static mut s_dmap_1: *const libc::c_char =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/\x00" as
        *const u8 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn mp_tovalue(mut ch: libc::c_char, mut r: libc::c_int)
 -> libc::c_int {
    return s_mp_tovalue(ch, r);
}
/* Error strings           */
#[no_mangle]
pub unsafe extern "C" fn mp_strerror(mut ec: mp_err) -> *const libc::c_char {
    let mut aec: libc::c_int = if ec < 0i32 { -ec } else { ec };
    if ec < -5i32 || ec > 0i32 {
        return mp_err_string[0usize]
    } else { return mp_err_string[(aec + 1i32) as usize] };
}
/*
 *  mpi.c
 *
 *  Arbitrary precision integer arithmetic library
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
  A table of the logs of 2 for various bases (the 0 and 1 entries of
  this table are meaningless and should not be referenced).

  This table is used to compute output lengths for the mp_toradix()
  function.  Since a number n in radix r takes up about log_r(n)
  digits, we estimate the output size by taking the least integer
  greater than log_r(n), where:

  log_r(n) = log_2(n) * log_r(2)

  This table, therefore, is a table of log_r(2) for 2 <= r <= 36,
  which are the output bases supported.
 */
/* {{{ Constant strings */
/* Constant strings returned by mp_strerror() */
static mut mp_err_string: [*const libc::c_char; 7] =
    [b"unknown result code\x00" as *const u8 as *const libc::c_char,
     b"boolean true\x00" as *const u8 as *const libc::c_char,
     b"boolean false\x00" as *const u8 as *const libc::c_char,
     b"out of memory\x00" as *const u8 as *const libc::c_char,
     b"argument out of range\x00" as *const u8 as *const libc::c_char,
     b"invalid input parameter\x00" as *const u8 as *const libc::c_char,
     b"result is undefined\x00" as *const u8 as *const libc::c_char];
/* Octet string conversion functions */
#[no_mangle]
pub unsafe extern "C" fn mp_read_unsigned_octets(mut mp: *mut mp_int,
                                                 mut str:
                                                     *const libc::c_uchar,
                                                 mut len: mp_size) -> mp_err {
    let mut count: libc::c_int = 0;
    let mut res: mp_err = 0;
    let mut d: mp_digit = 0;
    if !(!mp.is_null() && !str.is_null() && len > 0i32 as libc::c_uint) {
        return -4i32
    }
    mp_zero(mp);
    count =
        (len as
             libc::c_ulong).wrapping_rem(::std::mem::size_of::<mp_digit>() as
                                             libc::c_ulong) as libc::c_int;
    if 0 != count {
        d = 0i32 as mp_digit;
        loop  {
            let fresh51 = count;
            count = count - 1;
            if !(fresh51 > 0i32) { break ; }
            let fresh52 = str;
            str = str.offset(1);
            d = d << 8i32 | *fresh52 as libc::c_ulong;
            len = len.wrapping_sub(1)
        }
        *(*mp).dp.offset(0isize) = d
    }
    let mut current_block_21: u64;
    while len > 0i32 as libc::c_uint {
        d = 0i32 as mp_digit;
        count =
            ::std::mem::size_of::<mp_digit>() as libc::c_ulong as libc::c_int;
        while count > 0i32 {
            let fresh53 = str;
            str = str.offset(1);
            d = d << 8i32 | *fresh53 as libc::c_ulong;
            count -= 1
        }
        if 0i32 == mp_cmp_z(mp) {
            if 0 == d {
                current_block_21 = 7976072742316086414;
            } else { current_block_21 = 15768484401365413375; }
        } else {
            res = s_mp_lshd(mp, 1i32 as mp_size);
            if res != 0i32 { return res }
            current_block_21 = 15768484401365413375;
        }
        match current_block_21 {
            15768484401365413375 => { *(*mp).dp.offset(0isize) = d }
            _ => { }
        }
        len =
            (len as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<mp_digit>()
                                                 as libc::c_ulong) as mp_size
                as mp_size
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mp_unsigned_octet_size(mut mp: *const mp_int)
 -> libc::c_uint {
    let mut bytes: libc::c_uint = 0;
    let mut ix: libc::c_int = 0;
    let mut d: mp_digit = 0i32 as mp_digit;
    if mp.is_null() { return -4i32 as libc::c_uint }
    if !(0i32 as libc::c_uint == (*mp).sign) { return -4i32 as libc::c_uint }
    bytes =
        ((*mp).used as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>() as
                                             libc::c_ulong) as libc::c_uint;
    ix = (*mp).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    while ix >= 0i32 {
        d = *(*mp).dp.offset(ix as isize);
        if 0 != d { break ; }
        bytes =
            (bytes as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<mp_digit>()
                                                 as libc::c_ulong) as
                libc::c_uint as libc::c_uint;
        ix -= 1
    }
    if 0 == bytes { return 1i32 as libc::c_uint }
    ix =
        (::std::mem::size_of::<mp_digit>() as
             libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as
            libc::c_int;
    while ix >= 0i32 {
        let mut x: libc::c_uchar = (d >> ix * 8i32) as libc::c_uchar;
        if 0 != x { break ; }
        bytes = bytes.wrapping_sub(1);
        ix -= 1
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn mp_to_unsigned_octets(mut mp: *const mp_int,
                                               mut str: *mut libc::c_uchar,
                                               mut maxlen: mp_size)
 -> mp_err {
    let mut ix: libc::c_int = 0;
    let mut pos: libc::c_int = 0i32;
    let mut bytes: libc::c_uint = 0;
    if !(!mp.is_null() && !str.is_null() && 0 == (*mp).sign) { return -4i32 }
    bytes = mp_unsigned_octet_size(mp);
    if !(bytes <= maxlen) { return -4i32 }
    ix = (*mp).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    while ix >= 0i32 {
        let mut d: mp_digit = *(*mp).dp.offset(ix as isize);
        let mut jx: libc::c_int = 0;
        jx =
            (::std::mem::size_of::<mp_digit>() as
                 libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as
                libc::c_int;
        while jx >= 0i32 {
            let mut x: libc::c_uchar = (d >> jx * 8i32) as libc::c_uchar;
            /* suppress leading zeros */
            if !(0 == pos && 0 == x) {
                let fresh54 = pos;
                pos = pos + 1;
                *str.offset(fresh54 as isize) = x
            }
            jx -= 1
        }
        ix -= 1
    }
    if 0 == pos {
        let fresh55 = pos;
        pos = pos + 1;
        *str.offset(fresh55 as isize) = 0i32 as libc::c_uchar
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn mp_to_signed_octets(mut mp: *const mp_int,
                                             mut str: *mut libc::c_uchar,
                                             mut maxlen: mp_size) -> mp_err {
    let mut ix: libc::c_int = 0;
    let mut pos: libc::c_int = 0i32;
    let mut bytes: libc::c_uint = 0;
    if !(!mp.is_null() && !str.is_null() && 0 == (*mp).sign) { return -4i32 }
    bytes = mp_unsigned_octet_size(mp);
    if !(bytes <= maxlen) { return -4i32 }
    ix = (*mp).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    while ix >= 0i32 {
        let mut d: mp_digit = *(*mp).dp.offset(ix as isize);
        let mut jx: libc::c_int = 0;
        let mut current_block_23: u64;
        jx =
            (::std::mem::size_of::<mp_digit>() as
                 libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as
                libc::c_int;
        while jx >= 0i32 {
            let mut x: libc::c_uchar = (d >> jx * 8i32) as libc::c_uchar;
            if 0 == pos {
                /* suppress leading zeros */
                if 0 == x {
                    current_block_23 = 12349973810996921269;
                } else {
                    if 0 != x as libc::c_int & 0x80i32 {
                        if !(bytes.wrapping_add(1i32 as libc::c_uint) <=
                                 maxlen) {
                            return -4i32
                        }
                        if bytes.wrapping_add(1i32 as libc::c_uint) > maxlen {
                            return -4i32
                        }
                        let fresh56 = pos;
                        pos = pos + 1;
                        *str.offset(fresh56 as isize) = 0i32 as libc::c_uchar
                    }
                    current_block_23 = 14763689060501151050;
                }
            } else { current_block_23 = 14763689060501151050; }
            match current_block_23 {
                14763689060501151050 => {
                    let fresh57 = pos;
                    pos = pos + 1;
                    *str.offset(fresh57 as isize) = x
                }
                _ => { }
            }
            jx -= 1
        }
        ix -= 1
    }
    if 0 == pos {
        let fresh58 = pos;
        pos = pos + 1;
        *str.offset(fresh58 as isize) = 0i32 as libc::c_uchar
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn mp_to_fixlen_octets(mut mp: *const mp_int,
                                             mut str: *mut libc::c_uchar,
                                             mut length: mp_size) -> mp_err {
    let mut ix: libc::c_int = 0;
    let mut pos: libc::c_int = 0i32;
    let mut bytes: libc::c_uint = 0;
    if !(!mp.is_null() && !str.is_null() && 0 == (*mp).sign) { return -4i32 }
    bytes = mp_unsigned_octet_size(mp);
    if !(bytes <= length) { return -4i32 }
    while length > bytes {
        let fresh59 = str;
        str = str.offset(1);
        *fresh59 = 0i32 as libc::c_uchar;
        length = length.wrapping_sub(1)
    }
    ix = (*mp).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    while ix >= 0i32 {
        let mut d: mp_digit = *(*mp).dp.offset(ix as isize);
        let mut jx: libc::c_int = 0;
        jx =
            (::std::mem::size_of::<mp_digit>() as
                 libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as
                libc::c_int;
        while jx >= 0i32 {
            let mut x: libc::c_uchar = (d >> jx * 8i32) as libc::c_uchar;
            /* suppress leading zeros */
            if !(0 == pos && 0 == x) {
                let fresh60 = pos;
                pos = pos + 1;
                *str.offset(fresh60 as isize) = x
            }
            jx -= 1
        }
        ix -= 1
    }
    if 0 == pos {
        let fresh61 = pos;
        pos = pos + 1;
        *str.offset(fresh61 as isize) = 0i32 as libc::c_uchar
    }
    return 0i32;
}
/* unsigned digit divide   */
#[no_mangle]
pub unsafe extern "C" fn s_mp_reduce(mut x: *mut mp_int, mut m: *const mp_int,
                                     mut mu: *const mp_int) -> mp_err {
    let mut current_block: u64;
    let mut q: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    res = mp_init_copy(&mut q, x);
    if res != 0i32 { return res }
    s_mp_rshd(&mut q, (*m).used.wrapping_sub(1i32 as libc::c_uint));
    s_mp_mul(&mut q, mu);
    s_mp_rshd(&mut q, (*m).used.wrapping_add(1i32 as libc::c_uint));
    s_mp_mod_2d(x,
                (8i32 as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                     as
                                                     libc::c_ulong).wrapping_mul((*m).used.wrapping_add(1i32
                                                                                                            as
                                                                                                            libc::c_uint)
                                                                                     as
                                                                                     libc::c_ulong));
    s_mp_mul(&mut q, m);
    s_mp_mod_2d(&mut q,
                (8i32 as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                     as
                                                     libc::c_ulong).wrapping_mul((*m).used.wrapping_add(1i32
                                                                                                            as
                                                                                                            libc::c_uint)
                                                                                     as
                                                                                     libc::c_ulong));
    /* x = x - q */
    res = mp_sub(x, &mut q, x);
    if !(res != 0i32) {
        /* If x < 0, add b^(k+1) to it */
        if mp_cmp_z(x) < 0i32 {
            mp_set(&mut q, 1i32 as mp_digit);
            res =
                s_mp_lshd(&mut q,
                          (*m).used.wrapping_add(1i32 as libc::c_uint));
            if res != 0i32 {
                current_block = 767716266403053959;
            } else {
                res = mp_add(x, &mut q, x);
                if res != 0i32 {
                    current_block = 767716266403053959;
                } else { current_block = 12800627514080957624; }
            }
        } else { current_block = 12800627514080957624; }
        match current_block {
            767716266403053959 => { }
            _ => {
                while mp_cmp(x, m) >= 0i32 {
                    res = s_mp_sub(x, m);
                    if res != 0i32 { break ; }
                }
            }
        }
    }
    mp_clear(&mut q);
    return res;
}
/* Barrett reduction       */
/* magnitude addition      */
#[no_mangle]
pub unsafe extern "C" fn s_mp_add(mut a: *mut mp_int, mut b: *const mp_int)
 -> mp_err {
    let mut d: mp_digit = 0;
    let mut sum: mp_digit = 0;
    let mut carry: mp_digit = 0i32 as mp_digit;
    let mut pa: *mut mp_digit = 0 as *mut mp_digit;
    let mut pb: *mut mp_digit = 0 as *mut mp_digit;
    let mut ix: mp_size = 0;
    let mut used: mp_size = 0;
    let mut res: mp_err = 0;
    if (*b).used > (*a).used && { res = s_mp_pad(a, (*b).used); res != 0i32 }
       {
        return res
    }
    pa = (*a).dp;
    pb = (*b).dp;
    used = (*b).used;
    ix = 0i32 as mp_size;
    while ix < used {
        d = *pa;
        let fresh62 = pb;
        pb = pb.offset(1);
        sum = d.wrapping_add(*fresh62);
        d = (sum < d) as libc::c_int as mp_digit;
        let fresh63 = pa;
        pa = pa.offset(1);
        sum =
            (sum as libc::c_ulong).wrapping_add(carry) as mp_digit as
                mp_digit;
        *fresh63 = sum;
        carry = d.wrapping_add((sum < carry) as libc::c_int as libc::c_ulong);
        ix = ix.wrapping_add(1)
    }
    used = (*a).used;
    while 0 != carry && ix < used {
        sum = carry.wrapping_add(*pa);
        let fresh64 = pa;
        pa = pa.offset(1);
        *fresh64 = sum;
        carry = (0 == sum) as libc::c_int as mp_digit;
        ix = ix.wrapping_add(1)
    }
    if 0 != carry {
        res = s_mp_pad(a, used.wrapping_add(1i32 as libc::c_uint));
        if res != 0i32 { return res }
        *(*a).dp.offset(used as isize) = carry
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_add_offset(mut a: *mut mp_int,
                                         mut b: *mut mp_int,
                                         mut offset: mp_size) -> mp_err {
    let mut d: mp_digit = 0;
    let mut sum: mp_digit = 0;
    let mut carry: mp_digit = 0i32 as mp_digit;
    let mut ib: mp_size = 0;
    let mut ia: mp_size = 0;
    let mut lim: mp_size = 0;
    let mut res: mp_err = 0;
    lim = (*b).used.wrapping_add(offset);
    if lim > (*a).used && { res = s_mp_pad(a, lim); res != 0i32 } {
        return res
    }
    lim = (*b).used;
    ib = 0i32 as mp_size;
    ia = offset;
    while ib < lim {
        d = *(*a).dp.offset(ia as isize);
        sum = d.wrapping_add(*(*b).dp.offset(ib as isize));
        d = (sum < d) as libc::c_int as mp_digit;
        sum =
            (sum as libc::c_ulong).wrapping_add(carry) as mp_digit as
                mp_digit;
        *(*a).dp.offset(ia as isize) = sum;
        carry = d.wrapping_add((sum < carry) as libc::c_int as libc::c_ulong);
        ib = ib.wrapping_add(1);
        ia = ia.wrapping_add(1)
    }
    lim = (*a).used;
    while 0 != carry && ia < lim {
        d = *(*a).dp.offset(ia as isize);
        sum = d.wrapping_add(carry);
        *(*a).dp.offset(ia as isize) = sum;
        carry = (sum < d) as libc::c_int as mp_digit;
        ia = ia.wrapping_add(1)
    }
    if 0 != carry {
        res = s_mp_pad(a, lim.wrapping_add(1i32 as libc::c_uint));
        if res != 0i32 { return res }
        *(*a).dp.offset(lim as isize) = carry
    }
    s_mp_clamp(a);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_exptmod(mut a: *const mp_int,
                                      mut b: *const mp_int,
                                      mut m: *const mp_int,
                                      mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut s: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut x: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut mu: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    let mut d: mp_digit = 0;
    let mut dig: libc::c_uint = 0;
    let mut bit: libc::c_uint = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if mp_cmp_z(b) < 0i32 || mp_cmp_z(m) <= 0i32 { return -3i32 }
    res = mp_init(&mut s);
    if res != 0i32 { return res }
    res = mp_init_copy(&mut x, a);
    if !(res != 0i32 || { res = mp_mod(&mut x, m, &mut x); res != 0i32 }) {
        res = mp_init(&mut mu);
        if !(res != 0i32) {
            mp_set(&mut s, 1i32 as mp_digit);
            /* mu = b^2k / m */
            res = s_mp_add_d(&mut mu, 1i32 as mp_digit);
            if !(res != 0i32) {
                res =
                    s_mp_lshd(&mut mu,
                              (2i32 as libc::c_uint).wrapping_mul((*m).used));
                if !(res != 0i32) {
                    res = mp_div(&mut mu, m, &mut mu, 0 as *mut mp_int);
                    if !(res != 0i32) {
                        /* Loop over digits of b in ascending order, except highest order */
                        dig = 0i32 as libc::c_uint;
                        's_86:
                            loop  {
                                if !(dig <
                                         (*b).used.wrapping_sub(1i32 as
                                                                    libc::c_uint))
                                   {
                                    current_block = 7205609094909031804;
                                    break ;
                                }
                                d = *(*b).dp.offset(dig as isize);
                                /* Loop over the bits of the lower-order digits */
                                bit = 0i32 as libc::c_uint;
                                while (bit as libc::c_ulong) <
                                          (8i32 as
                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                               as
                                                                               libc::c_ulong)
                                      {
                                    if 0 != d & 1i32 as libc::c_ulong {
                                        res = s_mp_mul(&mut s, &mut x);
                                        if res != 0i32 {
                                            current_block =
                                                10016366717993617771;
                                            break 's_86 ;
                                        }
                                        res = s_mp_reduce(&mut s, m, &mut mu);
                                        if res != 0i32 {
                                            current_block =
                                                10016366717993617771;
                                            break 's_86 ;
                                        }
                                    }
                                    d >>= 1i32;
                                    res = s_mp_sqr(&mut x);
                                    if res != 0i32 {
                                        current_block = 10016366717993617771;
                                        break 's_86 ;
                                    }
                                    res = s_mp_reduce(&mut x, m, &mut mu);
                                    if res != 0i32 {
                                        current_block = 10016366717993617771;
                                        break 's_86 ;
                                    }
                                    bit = bit.wrapping_add(1)
                                }
                                dig = dig.wrapping_add(1)
                            }
                        match current_block {
                            10016366717993617771 => { }
                            _ => {
                                d = *(*b).dp.offset(dig as isize);
                                loop  {
                                    if !(0 != d) {
                                        current_block = 11743904203796629665;
                                        break ;
                                    }
                                    if 0 != d & 1i32 as libc::c_ulong {
                                        res = s_mp_mul(&mut s, &mut x);
                                        if res != 0i32 {
                                            current_block =
                                                10016366717993617771;
                                            break ;
                                        }
                                        res = s_mp_reduce(&mut s, m, &mut mu);
                                        if res != 0i32 {
                                            current_block =
                                                10016366717993617771;
                                            break ;
                                        }
                                    }
                                    d >>= 1i32;
                                    res = s_mp_sqr(&mut x);
                                    if res != 0i32 {
                                        current_block = 10016366717993617771;
                                        break ;
                                    }
                                    res = s_mp_reduce(&mut x, m, &mut mu);
                                    if res != 0i32 {
                                        current_block = 10016366717993617771;
                                        break ;
                                    }
                                }
                                match current_block {
                                    10016366717993617771 => { }
                                    _ => { s_mp_exch(&mut s, c); }
                                }
                            }
                        }
                    }
                }
            }
            mp_clear(&mut mu);
        }
        mp_clear(&mut x);
    }
    mp_clear(&mut s);
    return res;
}