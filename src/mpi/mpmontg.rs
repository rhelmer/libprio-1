#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* Memory management       */
    #[no_mangle]
    fn mp_init(mp: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_init_size(mp: *mut mp_int, prec: mp_size) -> mp_err;
    #[no_mangle]
    fn mp_init_copy(mp: *mut mp_int, from: *const mp_int) -> mp_err;
    #[no_mangle]
    fn mp_copy(from: *const mp_int, to: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_exch(mp1: *mut mp_int, mp2: *mut mp_int);
    #[no_mangle]
    fn mp_clear(mp: *mut mp_int);
    #[no_mangle]
    fn mp_set(mp: *mut mp_int, d: mp_digit);
    #[no_mangle]
    fn mp_mul(a: *const mp_int, b: *const mp_int, c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_sqr(a: *const mp_int, b: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_div(a: *const mp_int, b: *const mp_int, q: *mut mp_int,
              r: *mut mp_int) -> mp_err;
    /* Modular arithmetic      */
    #[no_mangle]
    fn mp_mod(a: *const mp_int, m: *const mp_int, c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn s_mp_sub(a: *mut mp_int, b: *const mp_int) -> mp_err;
    #[no_mangle]
    fn s_mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    #[no_mangle]
    fn s_mp_rshd(mp: *mut mp_int, p: mp_size);
    #[no_mangle]
    fn s_mp_clamp(mp: *mut mp_int);
    #[no_mangle]
    fn s_mpv_mul_d_add_prop(a: *const mp_digit, a_len: mp_size, b: mp_digit,
                            c: *mut mp_digit);
    #[no_mangle]
    fn s_mp_pad(mp: *mut mp_int, min: mp_size) -> mp_err;
    #[no_mangle]
    fn abort() -> !;
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
    #[no_mangle]
    fn s_mp_setz(dp: *mut mp_digit, count: mp_size);
    /* end NSS_USE_COMBA */
    /* ------ mpv functions, operate on arrays of digits, not on mp_int's ------ */
    #[no_mangle]
    fn s_mpv_mul_d(a: *const mp_digit, a_len: mp_size, b: mp_digit,
                   c: *mut mp_digit);
    #[no_mangle]
    fn mpl_get_bits(a: *const mp_int, lsbNum: mp_size, numBits: mp_size)
     -> mp_err;
    #[no_mangle]
    fn s_mp_lshd(mp: *mut mp_int, p: mp_size) -> mp_err;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    /*
 * s_mpi_getProcessorLineSize() returns the size in bytes of the cache line
 * if a cache exists, or zero if there is no cache. If more than one
 * cache line exists, it should return the smallest line size (which is
 * usually the L1 cache).
 *
 * mp_modexp uses this information to make sure that private key information
 * isn't being leaked through the cache.
 *
 * see mpcpucache.c for the implementation.
 */
    #[no_mangle]
    fn s_mpi_getProcessorLineSize() -> libc::c_ulong;
    #[no_mangle]
    fn mpl_significant_bits(a: *const mp_int) -> mp_size;
    #[no_mangle]
    fn s_mp_invmod_radix(P: mp_digit) -> mp_digit;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    #[no_mangle]
    fn s_mp_exptmod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                    c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_isodd(a: *const mp_int) -> libc::c_int;
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
/* c += a * b * (MP_RADIX ** offset);  */
/* Callers of this macro should be aware that the return type might vary;
 * it should be treated as a void function. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mp_mont_modulus {
    pub N: mp_int,
    pub n0prime: mp_digit,
}
pub type ptrdiff_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn mp_exptmod(mut inBase: *const mp_int,
                                    mut exponent: *const mp_int,
                                    mut modulus: *const mp_int,
                                    mut result: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut base: *const mp_int = 0 as *const mp_int;
    let mut bits_in_exponent: mp_size = 0;
    let mut i: mp_size = 0;
    let mut window_bits: mp_size = 0;
    let mut odd_ints: mp_size = 0;
    let mut res: mp_err = 0;
    let mut nLen: libc::c_int = 0;
    let mut montBase: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut goodBase: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut mmm: mp_mont_modulus =
        mp_mont_modulus{N:
                            mp_int{sign: 0,
                                   alloc: 0,
                                   used: 0,
                                   dp: 0 as *mut mp_digit,},
                        n0prime: 0,};
    static mut max_window_bits: libc::c_uint = 0;
    if 0 == mp_isodd(modulus) {
        return s_mp_exptmod(inBase, exponent, modulus, result)
    }
    montBase.dp = 0 as *mut mp_digit;
    goodBase.dp = 0 as *mut mp_digit;
    if mp_cmp(inBase, modulus) < 0i32 {
        base = inBase;
        current_block = 4166486009154926805;
    } else {
        res = mp_init(&mut goodBase);
        if 0i32 > res {
            current_block = 12738366957046377633;
        } else {
            base = &mut goodBase;
            res = mp_mod(inBase, modulus, &mut goodBase);
            if 0i32 > res {
                current_block = 12738366957046377633;
            } else { current_block = 4166486009154926805; }
        }
    }
    match current_block {
        4166486009154926805 => {
            nLen = (*modulus).used as libc::c_int;
            res =
                mp_init_size(&mut montBase, (2i32 * nLen + 2i32) as mp_size);
            if !(0i32 > res) {
                mmm.N = *modulus;
                mmm.n0prime =
                    (0i32 as
                         libc::c_ulong).wrapping_sub(s_mp_invmod_radix(*(*modulus).dp.offset(0isize)));
                res = s_mp_to_mont(base, &mut mmm, &mut montBase);
                if !(0i32 > res) {
                    bits_in_exponent = mpl_significant_bits(exponent);
                    if 0 != mp_using_cache_safe_exp {
                        if bits_in_exponent > 780i32 as libc::c_uint {
                            window_bits = 6i32 as mp_size
                        } else if bits_in_exponent > 256i32 as libc::c_uint {
                            window_bits = 5i32 as mp_size
                        } else if bits_in_exponent > 20i32 as libc::c_uint {
                            window_bits = 4i32 as mp_size
                        } else { window_bits = 1i32 as mp_size }
                    } else if bits_in_exponent > 480i32 as libc::c_uint {
                        window_bits = 6i32 as mp_size
                    } else if bits_in_exponent > 160i32 as libc::c_uint {
                        window_bits = 5i32 as mp_size
                    } else if bits_in_exponent > 20i32 as libc::c_uint {
                        window_bits = 4i32 as mp_size
                    } else { window_bits = 1i32 as mp_size }
                    if 0 == max_window_bits {
                        let mut cache_size: libc::c_ulong =
                            s_mpi_getProcessorLineSize();
                        if cache_size == 0i32 as libc::c_ulong {
                            mp_using_cache_safe_exp = 0i32 as libc::c_uint
                        }
                        if cache_size == 0i32 as libc::c_ulong ||
                               cache_size >= 64i32 as libc::c_ulong {
                            max_window_bits = 6i32 as libc::c_uint
                        } else if cache_size >= 32i32 as libc::c_ulong {
                            max_window_bits = 5i32 as libc::c_uint
                        } else if cache_size >= 16i32 as libc::c_ulong {
                            max_window_bits = 4i32 as libc::c_uint
                        } else { max_window_bits = 1i32 as libc::c_uint }
                    }
                    if 0 != mp_using_cache_safe_exp {
                        if window_bits > max_window_bits {
                            window_bits = max_window_bits
                        }
                    }
                    odd_ints =
                        (1i32 <<
                             window_bits.wrapping_sub(1i32 as libc::c_uint))
                            as mp_size;
                    i = bits_in_exponent.wrapping_rem(window_bits);
                    if i != 0i32 as libc::c_uint {
                        bits_in_exponent =
                            (bits_in_exponent as
                                 libc::c_uint).wrapping_add(window_bits.wrapping_sub(i))
                                as mp_size as mp_size
                    }
                    if 0 != mp_using_cache_safe_exp {
                        res =
                            mp_exptmod_safe_i(&mut montBase, exponent,
                                              modulus, result, &mut mmm, nLen,
                                              bits_in_exponent, window_bits,
                                              (1i32 << window_bits) as
                                                  mp_size)
                    } else {
                        res =
                            mp_exptmod_i(&mut montBase, exponent, modulus,
                                         result, &mut mmm, nLen,
                                         bits_in_exponent, window_bits,
                                         odd_ints)
                    }
                }
            }
        }
        _ => { }
    }
    mp_clear(&mut montBase);
    mp_clear(&mut goodBase);
    memset(&mut mmm as *mut mp_mont_modulus as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<mp_mont_modulus>() as libc::c_ulong);
    return res;
}
/* Do modular exponentiation using integer multiply code. */
#[no_mangle]
pub unsafe extern "C" fn mp_exptmod_i(mut montBase: *const mp_int,
                                      mut exponent: *const mp_int,
                                      mut modulus: *const mp_int,
                                      mut result: *mut mp_int,
                                      mut mmm: *mut mp_mont_modulus,
                                      mut nLen: libc::c_int,
                                      mut bits_in_exponent: mp_size,
                                      mut window_bits: mp_size,
                                      mut odd_ints: mp_size) -> mp_err {
    let mut current_block: u64;
    let mut pa1: *mut mp_int = 0 as *mut mp_int;
    let mut pa2: *mut mp_int = 0 as *mut mp_int;
    let mut ptmp: *mut mp_int = 0 as *mut mp_int;
    let mut i: mp_size = 0;
    let mut res: mp_err = 0;
    let mut expOff: libc::c_int = 0;
    let mut accum1: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut accum2: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut power2: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut oddPowers: [mp_int; 32] =
        [mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,}; 32];
    accum1.dp = 0 as *mut mp_digit;
    accum2.dp = 0 as *mut mp_digit;
    power2.dp = 0 as *mut mp_digit;
    i = 0i32 as mp_size;
    while i < 32i32 as libc::c_uint {
        let ref mut fresh0 = (*oddPowers.as_mut_ptr().offset(i as isize)).dp;
        *fresh0 = 0 as *mut mp_digit;
        i = i.wrapping_add(1)
    }
    res = mp_init_size(&mut accum1, (3i32 * nLen + 2i32) as mp_size);
    if !(0i32 > res) {
        res = mp_init_size(&mut accum2, (3i32 * nLen + 2i32) as mp_size);
        if !(0i32 > res) {
            res = mp_init_copy(&mut oddPowers[0usize], montBase);
            if !(0i32 > res) {
                res =
                    mp_init_size(&mut power2,
                                 (nLen as
                                      libc::c_uint).wrapping_add((2i32 as
                                                                      libc::c_uint).wrapping_mul((*montBase).used)).wrapping_add(2i32
                                                                                                                                     as
                                                                                                                                     libc::c_uint));
                if !(0i32 > res) {
                    res = mp_sqr(montBase, &mut power2);
                    if !(0i32 > res) {
                        /* power2 = montBase ** 2 */
                        res = s_mp_redc(&mut power2, mmm);
                        if !(0i32 > res) {
                            i = 1i32 as mp_size;
                            loop  {
                                if !(i < odd_ints) {
                                    current_block = 1109700713171191020;
                                    break ;
                                }
                                res =
                                    mp_init_size(oddPowers.as_mut_ptr().offset(i
                                                                                   as
                                                                                   isize),
                                                 (nLen as
                                                      libc::c_uint).wrapping_add((2i32
                                                                                      as
                                                                                      libc::c_uint).wrapping_mul(power2.used)).wrapping_add(2i32
                                                                                                                                                as
                                                                                                                                                libc::c_uint));
                                if 0i32 > res {
                                    current_block = 2526973956654166816;
                                    break ;
                                }
                                res =
                                    mp_mul(oddPowers.as_mut_ptr().offset(i.wrapping_sub(1i32
                                                                                            as
                                                                                            libc::c_uint)
                                                                             as
                                                                             isize),
                                           &mut power2,
                                           oddPowers.as_mut_ptr().offset(i as
                                                                             isize));
                                if 0i32 > res {
                                    current_block = 2526973956654166816;
                                    break ;
                                }
                                res =
                                    s_mp_redc(oddPowers.as_mut_ptr().offset(i
                                                                                as
                                                                                isize),
                                              mmm);
                                if 0i32 > res {
                                    current_block = 2526973956654166816;
                                    break ;
                                }
                                i = i.wrapping_add(1)
                            }
                            match current_block {
                                2526973956654166816 => { }
                                _ => {
                                    mp_set(&mut accum1, 1i32 as mp_digit);
                                    res =
                                        s_mp_to_mont(&mut accum1, mmm,
                                                     &mut accum1);
                                    if !(0i32 > res) {
                                        pa1 = &mut accum1;
                                        pa2 = &mut accum2;
                                        expOff =
                                            bits_in_exponent.wrapping_sub(window_bits)
                                                as libc::c_int;
                                        loop  {
                                            if !(expOff >= 0i32) {
                                                current_block =
                                                    5537925605363743233;
                                                break ;
                                            }
                                            let mut smallExp: mp_size = 0;
                                            res =
                                                mpl_get_bits(exponent,
                                                             expOff as
                                                                 mp_size,
                                                             window_bits);
                                            if 0i32 > res {
                                                current_block =
                                                    2526973956654166816;
                                                break ;
                                            }
                                            smallExp = res as mp_size;
                                            if window_bits ==
                                                   1i32 as libc::c_uint {
                                                if 0 == smallExp {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  1i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      oddPowers.as_mut_ptr().offset(0isize),
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                } else { abort(); }
                                            } else if window_bits ==
                                                          4i32 as libc::c_uint
                                             {
                                                if 0 == smallExp {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                } else if 0 !=
                                                              smallExp &
                                                                  1i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa1,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(2i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa2,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  2i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(4i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  4i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa1,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(8i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa2,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  8i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(16i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else { abort(); }
                                            } else if window_bits ==
                                                          5i32 as libc::c_uint
                                             {
                                                if 0 == smallExp {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  1i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(2i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                } else if 0 !=
                                                              smallExp &
                                                                  2i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa1,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(4i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa2,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                } else if 0 !=
                                                              smallExp &
                                                                  4i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(8i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                } else if 0 !=
                                                              smallExp &
                                                                  8i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa1,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(16i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa2,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                } else if 0 !=
                                                              smallExp &
                                                                  0x10i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(32i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                } else { abort(); }
                                            } else if window_bits ==
                                                          6i32 as libc::c_uint
                                             {
                                                if 0 == smallExp {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                } else if 0 !=
                                                              smallExp &
                                                                  1i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa1,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(2i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa2,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  2i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(4i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  4i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa1,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(8i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa2,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  8i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(16i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  0x10i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa1,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(32i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa2,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else if 0 !=
                                                              smallExp &
                                                                  0x20i32 as
                                                                      libc::c_uint
                                                 {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      oddPowers.as_mut_ptr().offset(smallExp.wrapping_div(64i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                                        as
                                                                                                        isize),
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            2526973956654166816;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                } else { abort(); }
                                            } else { abort(); }
                                            expOff =
                                                (expOff as
                                                     libc::c_uint).wrapping_sub(window_bits)
                                                    as libc::c_int as
                                                    libc::c_int
                                        }
                                        match current_block {
                                            2526973956654166816 => { }
                                            _ => {
                                                res = s_mp_redc(pa1, mmm);
                                                mp_exch(pa1, result);
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
    mp_clear(&mut accum1);
    mp_clear(&mut accum2);
    mp_clear(&mut power2);
    i = 0i32 as mp_size;
    while i < odd_ints {
        mp_clear(oddPowers.as_mut_ptr().offset(i as isize));
        i = i.wrapping_add(1)
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_redc(mut T: *mut mp_int,
                                   mut mmm: *mut mp_mont_modulus) -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0;
    let mut i: mp_size = 0;
    i = ((*mmm).N.used << 1i32).wrapping_add(1i32 as libc::c_uint);
    res = s_mp_pad(T, i);
    if !(0i32 > res) {
        i = 0i32 as mp_size;
        while i < (*mmm).N.used {
            let mut m_i: mp_digit =
                (*(*T).dp.offset(i as isize)).wrapping_mul((*mmm).n0prime);
            s_mpv_mul_d_add_prop((*mmm).N.dp, (*mmm).N.used, m_i,
                                 (*T).dp.offset(i as isize));
            i = i.wrapping_add(1)
        }
        s_mp_clamp(T);
        s_mp_rshd(T, (*mmm).N.used);
        res = s_mp_cmp(T, &mut (*mmm).N);
        if res >= 0i32 {
            /* T = T - N */
            res = s_mp_sub(T, &mut (*mmm).N);
            if 0i32 > res {
                current_block = 10465758493475146957;
            } else { current_block = 7651349459974463963; }
        } else { current_block = 7651349459974463963; }
        match current_block {
            10465758493475146957 => { }
            _ => { res = 0i32 }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_mul_mont(mut a: *const mp_int,
                                       mut b: *const mp_int,
                                       mut c: *mut mp_int,
                                       mut mmm: *mut mp_mont_modulus)
 -> mp_err {
    let mut current_block: u64;
    let mut pb: *mut mp_digit = 0 as *mut mp_digit;
    let mut m_i: mp_digit = 0;
    let mut res: mp_err = 0;
    /* "index b": index of current digit of B */
    let mut ib: mp_size = 0;
    let mut useda: mp_size = 0;
    let mut usedb: mp_size = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if (*a).used < (*b).used {
        let mut xch: *const mp_int = b;
        b = a;
        a = xch
    }
    (*c).used = 1i32 as mp_size;
    *(*c).dp.offset(0isize) = 0i32 as mp_digit;
    ib = ((*mmm).N.used << 1i32).wrapping_add(1i32 as libc::c_uint);
    res = s_mp_pad(c, ib);
    if !(res != 0i32) {
        useda = (*a).used;
        pb = (*b).dp;
        let fresh1 = pb;
        pb = pb.offset(1);
        s_mpv_mul_d((*a).dp, useda, *fresh1, (*c).dp);
        s_mp_setz((*c).dp.offset(useda as isize).offset(1isize),
                  ib.wrapping_sub(useda.wrapping_add(1i32 as libc::c_uint)));
        m_i = (*(*c).dp.offset(0isize)).wrapping_mul((*mmm).n0prime);
        s_mpv_mul_d_add_prop((*mmm).N.dp, (*mmm).N.used, m_i,
                             (*c).dp.offset(0isize));
        usedb = (*b).used;
        ib = 1i32 as mp_size;
        while ib < usedb {
            let fresh2 = pb;
            pb = pb.offset(1);
            let mut b_i: mp_digit = *fresh2;
            if 0 != b_i {
                s_mpv_mul_d_add_prop((*a).dp, useda, b_i,
                                     (*c).dp.offset(ib as isize));
            }
            m_i = (*(*c).dp.offset(ib as isize)).wrapping_mul((*mmm).n0prime);
            s_mpv_mul_d_add_prop((*mmm).N.dp, (*mmm).N.used, m_i,
                                 (*c).dp.offset(ib as isize));
            ib = ib.wrapping_add(1)
        }
        if usedb < (*mmm).N.used {
            usedb = (*mmm).N.used;
            while ib < usedb {
                m_i =
                    (*(*c).dp.offset(ib as
                                         isize)).wrapping_mul((*mmm).n0prime);
                s_mpv_mul_d_add_prop((*mmm).N.dp, (*mmm).N.used, m_i,
                                     (*c).dp.offset(ib as isize));
                ib = ib.wrapping_add(1)
            }
        }
        s_mp_clamp(c);
        s_mp_rshd(c, (*mmm).N.used);
        if s_mp_cmp(c, &mut (*mmm).N) >= 0i32 {
            res = s_mp_sub(c, &mut (*mmm).N);
            if 0i32 > res {
                current_block = 12389498124789478099;
            } else { current_block = 7746103178988627676; }
        } else { current_block = 7746103178988627676; }
        match current_block {
            12389498124789478099 => { }
            _ => { res = 0i32 }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn s_mp_to_mont(mut x: *const mp_int,
                                      mut mmm: *mut mp_mont_modulus,
                                      mut xMont: *mut mp_int) -> mp_err {
    let mut res: mp_err = 0;
    /* xMont = x * R mod N   where  N is modulus */
    res = mp_copy(x, xMont);
    if !(0i32 > res) {
        res = s_mp_lshd(xMont, (*mmm).N.used);
        if !(0i32 > res) {
            /* xMont = x << b */
            res = mp_div(xMont, &mut (*mmm).N, 0 as *mut mp_int, xMont);
            0i32 > res;
        }
    }
    /*         mod N */
    return res;
}
/* Do modular exponentiation using integer multiply code. */
#[no_mangle]
pub unsafe extern "C" fn mp_exptmod_safe_i(mut montBase: *const mp_int,
                                           mut exponent: *const mp_int,
                                           mut modulus: *const mp_int,
                                           mut result: *mut mp_int,
                                           mut mmm: *mut mp_mont_modulus,
                                           mut nLen: libc::c_int,
                                           mut bits_in_exponent: mp_size,
                                           mut window_bits: mp_size,
                                           mut num_powers: mp_size)
 -> mp_err {
    let mut current_block: u64;
    let mut pa1: *mut mp_int = 0 as *mut mp_int;
    let mut pa2: *mut mp_int = 0 as *mut mp_int;
    let mut ptmp: *mut mp_int = 0 as *mut mp_int;
    let mut i: mp_size = 0;
    let mut first_window: mp_size = 0;
    let mut res: mp_err = 0;
    let mut expOff: libc::c_int = 0;
    let mut accum1: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut accum2: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut accum: [mp_int; 4] =
        [mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,}; 4];
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut powersArray: *mut mp_digit = 0 as *mut mp_digit;
    let mut powers: *mut mp_digit = 0 as *mut mp_digit;
    accum1.dp = 0 as *mut mp_digit;
    accum2.dp = 0 as *mut mp_digit;
    accum[0usize].dp = 0 as *mut mp_digit;
    accum[1usize].dp = 0 as *mut mp_digit;
    accum[2usize].dp = 0 as *mut mp_digit;
    accum[3usize].dp = 0 as *mut mp_digit;
    tmp.dp = 0 as *mut mp_digit;
    /* grab the first window value. This allows us to preload accumulator1
   * and save a conversion, some squares and a multiple*/
    res =
        mpl_get_bits(exponent, bits_in_exponent.wrapping_sub(window_bits),
                     window_bits);
    if !(0i32 > res) {
        first_window = res as mp_size;
        res = mp_init_size(&mut accum1, (3i32 * nLen + 2i32) as mp_size);
        if !(0i32 > res) {
            res = mp_init_size(&mut accum2, (3i32 * nLen + 2i32) as mp_size);
            if !(0i32 > res) {
                /* build the first WEAVE_WORD powers inline */
    /* if WEAVE_WORD_SIZE is not 4, this code will have to change */
                if num_powers > 2i32 as libc::c_uint {
                    res =
                        mp_init_size(&mut accum[0usize],
                                     (3i32 * nLen + 2i32) as mp_size);
                    if 0i32 > res {
                        current_block = 13848196454260914387;
                    } else {
                        res =
                            mp_init_size(&mut accum[1usize],
                                         (3i32 * nLen + 2i32) as mp_size);
                        if 0i32 > res {
                            current_block = 13848196454260914387;
                        } else {
                            res =
                                mp_init_size(&mut accum[2usize],
                                             (3i32 * nLen + 2i32) as mp_size);
                            if 0i32 > res {
                                current_block = 13848196454260914387;
                            } else {
                                res =
                                    mp_init_size(&mut accum[3usize],
                                                 (3i32 * nLen + 2i32) as
                                                     mp_size);
                                if 0i32 > res {
                                    current_block = 13848196454260914387;
                                } else {
                                    mp_set(&mut accum[0usize],
                                           1i32 as mp_digit);
                                    res =
                                        s_mp_to_mont(&mut accum[0usize], mmm,
                                                     &mut accum[0usize]);
                                    if 0i32 > res {
                                        current_block = 13848196454260914387;
                                    } else {
                                        res =
                                            mp_copy(montBase,
                                                    &mut accum[1usize]);
                                        if 0i32 > res {
                                            current_block =
                                                13848196454260914387;
                                        } else {
                                            res =
                                                mp_sqr(montBase,
                                                       &mut accum[2usize]);
                                            if 0i32 > res {
                                                current_block =
                                                    13848196454260914387;
                                            } else {
                                                res =
                                                    s_mp_redc(&mut accum[2usize],
                                                              mmm);
                                                if 0i32 > res {
                                                    current_block =
                                                        13848196454260914387;
                                                } else {
                                                    res =
                                                        s_mp_mul_mont(&mut accum[2usize],
                                                                      montBase,
                                                                      &mut accum[3usize],
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                    } else {
                                                        powersArray =
                                                            malloc((num_powers
                                                                        as
                                                                        libc::c_ulong).wrapping_mul((nLen
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                                                         as
                                                                                                                                         libc::c_ulong).wrapping_add(1i32
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_ulong)))
                                                                as
                                                                *mut mp_digit;
                                                        if powersArray.is_null()
                                                           {
                                                            res = -2i32;
                                                            current_block =
                                                                13848196454260914387;
                                                        } else {
                                                            powers =
                                                                (powersArray
                                                                     as
                                                                     ptrdiff_t
                                                                     +
                                                                     num_powers.wrapping_sub(1i32
                                                                                                 as
                                                                                                 libc::c_uint)
                                                                         as
                                                                         libc::c_long
                                                                     &
                                                                     0i32 as
                                                                         ptrdiff_t
                                                                         -
                                                                         num_powers
                                                                             as
                                                                             libc::c_long)
                                                                    as
                                                                    *mut mp_digit;
                                                            res =
                                                                mpi_to_weave(accum.as_mut_ptr(),
                                                                             powers,
                                                                             nLen
                                                                                 as
                                                                                 mp_size,
                                                                             num_powers);
                                                            if 0i32 > res {
                                                                current_block
                                                                    =
                                                                    13848196454260914387;
                                                            } else if first_window
                                                                          <
                                                                          4i32
                                                                              as
                                                                              libc::c_uint
                                                             {
                                                                res =
                                                                    mp_copy(&mut accum[first_window
                                                                                           as
                                                                                           usize],
                                                                            &mut accum1);
                                                                if 0i32 > res
                                                                   {
                                                                    current_block
                                                                        =
                                                                        13848196454260914387;
                                                                } else {
                                                                    first_window
                                                                        =
                                                                        num_powers;
                                                                    current_block
                                                                        =
                                                                        721385680381463314;
                                                                }
                                                            } else {
                                                                current_block
                                                                    =
                                                                    721385680381463314;
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
                } else if first_window == 0i32 as libc::c_uint {
                    mp_set(&mut accum1, 1i32 as mp_digit);
                    res = s_mp_to_mont(&mut accum1, mmm, &mut accum1);
                    if 0i32 > res {
                        current_block = 13848196454260914387;
                    } else { current_block = 721385680381463314; }
                } else {
                    /* assert first_window == 1? */
                    res = mp_copy(montBase, &mut accum1);
                    if 0i32 > res {
                        current_block = 13848196454260914387;
                    } else { current_block = 721385680381463314; }
                }
                match current_block {
                    13848196454260914387 => { }
                    _ => {
                        /*
     * calculate all the powers in the powers array.
     * this adds 2**(k-1)-2 square operations over just calculating the
     * odd powers where k is the window size in the two other mp_modexpt
     * implementations in this file. We will get some of that
     * back by not needing the first 'k' squares and one multiply for the
     * first window.
     * Given the value of 4 for WEAVE_WORD_SIZE, this loop will only execute if
     * num_powers > 2, in which case powers will have been allocated.
     */
                        i = 4i32 as mp_size;
                        loop  {
                            if !(i < num_powers) {
                                current_block = 777662472977924419;
                                break ;
                            }
                            /* i % WEAVE_WORD_SIZE */
                            let mut acc_index: libc::c_int =
                                (i & (4i32 - 1i32) as libc::c_uint) as
                                    libc::c_int;
                            if 0 != i & 1i32 as libc::c_uint {
                                res =
                                    s_mp_mul_mont(&mut accum[(acc_index -
                                                                  1i32) as
                                                                 usize],
                                                  montBase,
                                                  &mut accum[acc_index as
                                                                 usize], mmm);
                                if 0i32 > res {
                                    current_block = 13848196454260914387;
                                    break ;
                                }
                                /* we've filled the array do our 'per array' processing */
                                if acc_index == 4i32 - 1i32 {
                                    res =
                                        mpi_to_weave(accum.as_mut_ptr(),
                                                     powers.offset(i as
                                                                       isize).offset(-((4i32
                                                                                            -
                                                                                            1i32)
                                                                                           as
                                                                                           isize)),
                                                     nLen as mp_size,
                                                     num_powers);
                                    if 0i32 > res {
                                        current_block = 13848196454260914387;
                                        break ;
                                    }
                                    if first_window <= i {
                                        res =
                                            mp_copy(&mut accum[(first_window &
                                                                    (4i32 -
                                                                         1i32)
                                                                        as
                                                                        libc::c_uint)
                                                                   as usize],
                                                    &mut accum1);
                                        if 0i32 > res {
                                            current_block =
                                                13848196454260914387;
                                            break ;
                                        }
                                        first_window = num_powers
                                    }
                                }
                            } else if i > (2i32 * 4i32) as libc::c_uint {
                                res =
                                    weave_to_mpi(&mut accum2, powers,
                                                 i.wrapping_div(2i32 as
                                                                    libc::c_uint),
                                                 nLen as mp_size, num_powers);
                                if 0i32 > res {
                                    current_block = 13848196454260914387;
                                    break ;
                                }
                                res =
                                    mp_sqr(&mut accum2,
                                           &mut accum[acc_index as usize]);
                                if 0i32 > res {
                                    current_block = 13848196454260914387;
                                    break ;
                                }
                                res =
                                    s_mp_redc(&mut accum[acc_index as usize],
                                              mmm);
                                if 0i32 > res {
                                    current_block = 13848196454260914387;
                                    break ;
                                }
                            } else {
                                let mut half_power_index: libc::c_int =
                                    (i.wrapping_div(2i32 as libc::c_uint) &
                                         (4i32 - 1i32) as libc::c_uint) as
                                        libc::c_int;
                                if half_power_index == acc_index {
                                    /* copy is cheaper than weave_to_mpi */
                                    res =
                                        mp_copy(&mut accum[half_power_index as
                                                               usize],
                                                &mut accum2);
                                    if 0i32 > res {
                                        current_block = 13848196454260914387;
                                        break ;
                                    }
                                    res =
                                        mp_sqr(&mut accum2,
                                               &mut accum[acc_index as
                                                              usize]);
                                    if 0i32 > res {
                                        current_block = 13848196454260914387;
                                        break ;
                                    }
                                    res =
                                        s_mp_redc(&mut accum[acc_index as
                                                                 usize], mmm);
                                    if 0i32 > res {
                                        current_block = 13848196454260914387;
                                        break ;
                                    }
                                } else {
                                    res =
                                        mp_sqr(&mut accum[half_power_index as
                                                              usize],
                                               &mut accum[acc_index as
                                                              usize]);
                                    if 0i32 > res {
                                        current_block = 13848196454260914387;
                                        break ;
                                    }
                                    res =
                                        s_mp_redc(&mut accum[acc_index as
                                                                 usize], mmm);
                                    if 0i32 > res {
                                        current_block = 13848196454260914387;
                                        break ;
                                    }
                                }
                            }
                            i = i.wrapping_add(1)
                        }
                        match current_block {
                            13848196454260914387 => { }
                            _ => {
                                pa1 = &mut accum1;
                                pa2 = &mut accum2;
                                /* tmp is not used if window_bits == 1. */
                                if window_bits != 1i32 as libc::c_uint {
                                    res =
                                        mp_init_size(&mut tmp,
                                                     (3i32 * nLen + 2i32) as
                                                         mp_size);
                                    if 0i32 > res {
                                        current_block = 13848196454260914387;
                                    } else {
                                        current_block = 11052029508375673978;
                                    }
                                } else {
                                    current_block = 11052029508375673978;
                                }
                                match current_block {
                                    13848196454260914387 => { }
                                    _ => {
                                        expOff =
                                            bits_in_exponent.wrapping_sub(window_bits.wrapping_mul(2i32
                                                                                                       as
                                                                                                       libc::c_uint))
                                                as libc::c_int;
                                        loop  {
                                            if !(expOff >= 0i32) {
                                                current_block =
                                                    15460309861373144675;
                                                break ;
                                            }
                                            let mut smallExp: mp_size = 0;
                                            res =
                                                mpl_get_bits(exponent,
                                                             expOff as
                                                                 mp_size,
                                                             window_bits);
                                            if 0i32 > res {
                                                current_block =
                                                    13848196454260914387;
                                                break ;
                                            }
                                            smallExp = res as mp_size;
                                            /* handle unroll the loops */
                                            match window_bits {
                                                1 => {
                                                    if 0 == smallExp {
                                                        res =
                                                            mp_sqr(pa1, pa2);
                                                        if 0i32 > res {
                                                            current_block =
                                                                13848196454260914387;
                                                            break ;
                                                        }
                                                        res =
                                                            s_mp_redc(pa2,
                                                                      mmm);
                                                        if 0i32 > res {
                                                            current_block =
                                                                13848196454260914387;
                                                            break ;
                                                        }
                                                        ptmp = pa1;
                                                        pa1 = pa2;
                                                        pa2 = ptmp
                                                    } else if 0 !=
                                                                  smallExp &
                                                                      1i32 as
                                                                          libc::c_uint
                                                     {
                                                        res =
                                                            mp_sqr(pa1, pa2);
                                                        if 0i32 > res {
                                                            current_block =
                                                                13848196454260914387;
                                                            break ;
                                                        }
                                                        res =
                                                            s_mp_redc(pa2,
                                                                      mmm);
                                                        if 0i32 > res {
                                                            current_block =
                                                                13848196454260914387;
                                                            break ;
                                                        }
                                                        res =
                                                            s_mp_mul_mont(pa2,
                                                                          montBase,
                                                                          pa1,
                                                                          mmm);
                                                        if 0i32 > res {
                                                            current_block =
                                                                13848196454260914387;
                                                            break ;
                                                        }
                                                    } else { abort(); }
                                                    current_block =
                                                        5793491756164225964;
                                                }
                                                6 => {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    /* fall through */
                                                    current_block =
                                                        15118377098007030281;
                                                }
                                                4 => {
                                                    current_block =
                                                        15118377098007030281;
                                                }
                                                5 => {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res =
                                                        weave_to_mpi(&mut tmp,
                                                                     powers,
                                                                     smallExp,
                                                                     nLen as
                                                                         mp_size,
                                                                     num_powers);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa2,
                                                                      &mut tmp,
                                                                      pa1,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    current_block =
                                                        5793491756164225964;
                                                }
                                                _ => { abort(); }
                                            }
                                            match current_block {
                                                15118377098007030281 => {
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa1, pa2);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa2, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = mp_sqr(pa2, pa1);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res = s_mp_redc(pa1, mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res =
                                                        weave_to_mpi(&mut tmp,
                                                                     powers,
                                                                     smallExp,
                                                                     nLen as
                                                                         mp_size,
                                                                     num_powers);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    res =
                                                        s_mp_mul_mont(pa1,
                                                                      &mut tmp,
                                                                      pa2,
                                                                      mmm);
                                                    if 0i32 > res {
                                                        current_block =
                                                            13848196454260914387;
                                                        break ;
                                                    }
                                                    ptmp = pa1;
                                                    pa1 = pa2;
                                                    pa2 = ptmp
                                                }
                                                _ => { }
                                            }
                                            expOff =
                                                (expOff as
                                                     libc::c_uint).wrapping_sub(window_bits)
                                                    as libc::c_int as
                                                    libc::c_int
                                        }
                                        match current_block {
                                            13848196454260914387 => { }
                                            _ => {
                                                res = s_mp_redc(pa1, mmm);
                                                mp_exch(pa1, result);
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
    mp_clear(&mut accum1);
    mp_clear(&mut accum2);
    mp_clear(&mut accum[0usize]);
    mp_clear(&mut accum[1usize]);
    mp_clear(&mut accum[2usize]);
    mp_clear(&mut accum[3usize]);
    mp_clear(&mut tmp);
    free(powersArray as *mut libc::c_void);
    return res;
}
/*
 * These functions return 0xffffffff if the output is true, and 0 otherwise.
 */
/* Reverse the operation above for one mp_int.
 * Reconstruct one mp_int from its column in the weaved array.
 * Every read accesses every element of the weaved array, in order to
 * avoid timing attacks based on patterns of memory accesses.
 */
#[no_mangle]
pub unsafe extern "C" fn weave_to_mpi(mut a: *mut mp_int,
                                      mut weaved: *const mp_digit,
                                      mut index: mp_size,
                                      mut nDigits: mp_size,
                                      mut nBignums: mp_size) -> mp_err {
    /* these are indices, but need to be the same size as mp_digit
     * because of the CONST_TIME operations */
    let mut i: mp_digit = 0;
    let mut j: mp_digit = 0;
    let mut d: mp_digit = 0;
    let mut pDest: *mut mp_digit = (*a).dp;
    (*a).sign = 0i32 as mp_sign;
    (*a).used = nDigits;
    if 0 != weaved.is_null() as libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                               &[libc::c_char; 13]>(b"weave_to_mpi\x00")).as_ptr(),
                     b"build/mpi/mpmontg.c\x00" as *const u8 as
                         *const libc::c_char, 783i32,
                     b"weaved != NULL\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    i = 0i32 as mp_digit;
    while i < nDigits as libc::c_ulong {
        d = 0i32 as mp_digit;
        j = 0i32 as mp_digit;
        while j < nBignums as libc::c_ulong {
            d |=
                *weaved.offset(i.wrapping_mul(nBignums as
                                                  libc::c_ulong).wrapping_add(j)
                                   as isize) &
                    (0i64 as
                         libc::c_ulong).wrapping_sub((!(j ^
                                                            index as
                                                                libc::c_ulong)
                                                          &
                                                          (j ^
                                                               index as
                                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_ulong))
                                                         >>
                                                         (8i32 as
                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ulong>()
                                                                                              as
                                                                                              libc::c_ulong).wrapping_sub(1i32
                                                                                                                              as
                                                                                                                              libc::c_ulong));
            j = j.wrapping_add(1)
        }
        *pDest.offset(i as isize) = d;
        i = i.wrapping_add(1)
    }
    s_mp_clamp(a);
    return 0i32;
}
/*
 * mpi_to_weave takes an array of bignums, a matrix in which each bignum
 * occupies all the columns of a row, and transposes it into a matrix in
 * which each bignum occupies a column of every row.  The first row of the
 * input matrix becomes the first column of the output matrix.  The n'th
 * row of input becomes the n'th column of output.  The input data is said
 * to be "interleaved" or "woven" into the output matrix.
 *
 * The array of bignums is left in this woven form.  Each time a single
 * bignum value is needed, it is recreated by fetching the n'th column,
 * forming a single row which is the new bignum.
 *
 * The purpose of this interleaving is make it impossible to determine which
 * of the bignums is being used in any one operation by examining the pattern
 * of cache misses.
 *
 * The weaving function does not transpose the entire input matrix in one call.
 * It transposes 4 rows of mp_ints into their respective columns of output.
 *
 * This implementation treats each mp_int bignum as an array of mp_digits,
 * It stores those bytes as a column of mp_digits in the output matrix.  It
 * doesn't care if the machine uses big-endian or little-endian byte ordering
 * within mp_digits.
 *
 * "bignums" is an array of mp_ints.
 * It points to four rows, four mp_ints, a subset of a larger array of mp_ints.
 *
 * "weaved" is the weaved output matrix.
 * The first byte of bignums[0] is stored in weaved[0].
 *
 * "nBignums" is the total number of bignums in the array of which "bignums"
 * is a part.
 *
 * "nDigits" is the size in mp_digits of each mp_int in the "bignums" array.
 * mp_ints that use less than nDigits digits are logically padded with zeros
 * while being stored in the weaved array.
 */
#[no_mangle]
pub unsafe extern "C" fn mpi_to_weave(mut bignums: *const mp_int,
                                      mut weaved: *mut mp_digit,
                                      mut nDigits: mp_size,
                                      mut nBignums: mp_size) -> mp_err {
    let mut i: mp_size = 0;
    let mut endDest: *mut mp_digit =
        weaved.offset(nDigits.wrapping_mul(nBignums) as isize);
    i = 0i32 as mp_size;
    while i < 4i32 as libc::c_uint {
        let mut used: mp_size = (*bignums.offset(i as isize)).used;
        let mut pSrc: *mut mp_digit = (*bignums.offset(i as isize)).dp;
        let mut endSrc: *mut mp_digit = pSrc.offset(used as isize);
        let mut pDest: *mut mp_digit = weaved.offset(i as isize);
        if !((*bignums.offset(i as isize)).sign == 0i32 as libc::c_uint) {
            return -4i32
        }
        if !(used <= nDigits) { return -4i32 }
        while pSrc < endSrc {
            *pDest = *pSrc;
            pDest = pDest.offset(nBignums as isize);
            pSrc = pSrc.offset(1isize)
        }
        while pDest < endDest {
            *pDest = 0i32 as mp_digit;
            pDest = pDest.offset(nBignums as isize)
        }
        i = i.wrapping_add(1)
    }
    return 0i32;
}
#[no_mangle]
pub static mut mp_using_cache_safe_exp: libc::c_uint = 1i32 as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn mp_set_safe_modexp(mut value: libc::c_int)
 -> mp_err {
    mp_using_cache_safe_exp = value as libc::c_uint;
    return 0i32;
}