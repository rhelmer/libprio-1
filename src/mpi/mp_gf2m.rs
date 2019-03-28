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
    fn mp_init_copy(mp: *mut mp_int, from: *const mp_int) -> mp_err;
    #[no_mangle]
    fn mp_copy(from: *const mp_int, to: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_clear(mp: *mut mp_int);
    #[no_mangle]
    fn mp_zero(mp: *mut mp_int);
    #[no_mangle]
    fn mp_cmp_mag(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    #[no_mangle]
    fn mp_isodd(a: *const mp_int) -> libc::c_int;
    #[no_mangle]
    fn s_mp_clamp(mp: *mut mp_int);
    #[no_mangle]
    fn s_mp_pad(mp: *mut mp_int, min: mp_size) -> mp_err;
    /* Shift functions                   */
    #[no_mangle]
    fn mpl_rsh(a: *const mp_int, b: *mut mp_int, d: mp_digit) -> mp_err;
    /* Get & Set the value of a bit */
    #[no_mangle]
    fn mpl_set_bit(a: *mut mp_int, bitNum: mp_size, value: mp_size) -> mp_err;
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
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#[no_mangle]
pub unsafe extern "C" fn mp_badd(mut a: *const mp_int, mut b: *const mp_int,
                                 mut c: *mut mp_int) -> mp_err {
    let mut pa: *mut mp_digit = 0 as *mut mp_digit;
    let mut pb: *mut mp_digit = 0 as *mut mp_digit;
    let mut pc: *mut mp_digit = 0 as *mut mp_digit;
    let mut ix: mp_size = 0;
    let mut used_pa: mp_size = 0;
    let mut used_pb: mp_size = 0;
    let mut res: mp_err = 0i32;
    if (*a).used >= (*b).used {
        pa = (*a).dp;
        pb = (*b).dp;
        used_pa = (*a).used;
        used_pb = (*b).used
    } else {
        pa = (*b).dp;
        pb = (*a).dp;
        used_pa = (*b).used;
        used_pb = (*a).used
    }
    /* Make sure c has enough precision for the output value */
    res = s_mp_pad(c, used_pa);
    if !(0i32 > res) {
        pc = (*c).dp;
        ix = 0i32 as mp_size;
        while ix < used_pb {
            let fresh2 = pc;
            pc = pc.offset(1);
            let fresh0 = pa;
            pa = pa.offset(1);
            let fresh1 = pb;
            pb = pb.offset(1);
            *fresh2 = *fresh0 ^ *fresh1;
            ix = ix.wrapping_add(1)
        }
        while ix < used_pa {
            let fresh4 = pc;
            pc = pc.offset(1);
            let fresh3 = pa;
            pa = pa.offset(1);
            *fresh4 = *fresh3;
            ix = ix.wrapping_add(1)
        }
        (*c).used = used_pa;
        (*c).sign = 0i32 as mp_sign;
        s_mp_clamp(c);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_bmul(mut a: *const mp_int, mut b: *const mp_int,
                                 mut c: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut pb: *mut mp_digit = 0 as *mut mp_digit;
    let mut b_i: mp_digit = 0;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut ib: mp_size = 0;
    let mut a_used: mp_size = 0;
    let mut b_used: mp_size = 0;
    let mut res: mp_err = 0i32;
    tmp.dp = 0 as *mut mp_digit;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if a == c {
        res = mp_init_copy(&mut tmp, a);
        if 0i32 > res {
            current_block = 2825498391814666816;
        } else {
            if a == b { b = &mut tmp }
            a = &mut tmp;
            current_block = 8831408221741692167;
        }
    } else if b == c {
        res = mp_init_copy(&mut tmp, b);
        if 0i32 > res {
            current_block = 2825498391814666816;
        } else { b = &mut tmp; current_block = 8831408221741692167; }
    } else { current_block = 8831408221741692167; }
    match current_block {
        8831408221741692167 => {
            if (*a).used < (*b).used {
                let mut xch: *const mp_int = b;
                b = a;
                a = xch
            }
            (*c).used = 1i32 as mp_size;
            *(*c).dp.offset(0isize) = 0i32 as mp_digit;
            res = s_mp_pad(c, (*a).used.wrapping_add((*b).used));
            if !(0i32 > res) {
                pb = (*b).dp;
                let fresh5 = pb;
                pb = pb.offset(1);
                s_bmul_d((*a).dp, (*a).used, *fresh5, (*c).dp);
                a_used = (*a).used;
                b_used = (*b).used;
                (*c).used = a_used.wrapping_add(b_used);
                ib = 1i32 as mp_size;
                while ib < b_used {
                    let fresh6 = pb;
                    pb = pb.offset(1);
                    b_i = *fresh6;
                    if 0 != b_i {
                        s_bmul_d_add((*a).dp, a_used, b_i,
                                     (*c).dp.offset(ib as isize));
                    } else {
                        *(*c).dp.offset(ib.wrapping_add(a_used) as isize) =
                            b_i
                    }
                    ib = ib.wrapping_add(1)
                }
                s_mp_clamp(c);
                (*c).sign = 0i32 as mp_sign
            }
        }
        _ => { }
    }
    mp_clear(&mut tmp);
    return res;
}
/* Compute binary polynomial xor multiply accumulate d ^= a * b */
unsafe extern "C" fn s_bmul_d_add(mut a: *const mp_digit, mut a_len: mp_size,
                                  mut b: mp_digit, mut d: *mut mp_digit) {
    let mut a_i: mp_digit = 0;
    let mut a0b0: mp_digit = 0;
    let mut a1b1: mp_digit = 0;
    let mut carry: mp_digit = 0i32 as mp_digit;
    loop  {
        let fresh7 = a_len;
        a_len = a_len.wrapping_sub(1);
        if !(0 != fresh7) { break ; }
        let fresh8 = a;
        a = a.offset(1);
        a_i = *fresh8;
        s_bmul_1x1(&mut a1b1, &mut a0b0, a_i, b);
        let fresh9 = d;
        d = d.offset(1);
        *fresh9 ^= a0b0 ^ carry;
        carry = a1b1
    }
    *d ^= carry;
}
/* enable fast divide and mod operations on MP_DIGIT_BITS */
/* Platform-specific macros for fast binary polynomial squaring. */
/* Multiply two binary polynomials mp_digits a, b.
 * Result is a polynomial with degree < 2 * MP_DIGIT_BITS - 1.
 * Output in two mp_digits rh, rl.
 */
#[no_mangle]
pub unsafe extern "C" fn s_bmul_1x1(mut rh: *mut mp_digit,
                                    mut rl: *mut mp_digit, a: mp_digit,
                                    b: mp_digit) {
    let mut h: mp_digit = 0;
    let mut l: mp_digit = 0;
    let mut s: mp_digit = 0;
    let mut tab: [mp_digit; 16] = [0; 16];
    let mut top3b: mp_digit = a >> 61i32;
    let mut a1: mp_digit = 0;
    let mut a2: mp_digit = 0;
    let mut a4: mp_digit = 0;
    let mut a8: mp_digit = 0;
    a1 = (a as libc::c_ulonglong & 0x1fffffffffffffffu64) as mp_digit;
    a2 = a1 << 1i32;
    a4 = a2 << 1i32;
    a8 = a4 << 1i32;
    tab[0usize] = 0i32 as mp_digit;
    tab[1usize] = a1;
    tab[2usize] = a2;
    tab[3usize] = a1 ^ a2;
    tab[4usize] = a4;
    tab[5usize] = a1 ^ a4;
    tab[6usize] = a2 ^ a4;
    tab[7usize] = a1 ^ a2 ^ a4;
    tab[8usize] = a8;
    tab[9usize] = a1 ^ a8;
    tab[10usize] = a2 ^ a8;
    tab[11usize] = a1 ^ a2 ^ a8;
    tab[12usize] = a4 ^ a8;
    tab[13usize] = a1 ^ a4 ^ a8;
    tab[14usize] = a2 ^ a4 ^ a8;
    tab[15usize] = a1 ^ a2 ^ a4 ^ a8;
    s = tab[(b & 0xfi32 as libc::c_ulong) as usize];
    l = s;
    s = tab[(b >> 4i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 4i32;
    h = s >> 60i32;
    s = tab[(b >> 8i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 8i32;
    h ^= s >> 56i32;
    s = tab[(b >> 12i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 12i32;
    h ^= s >> 52i32;
    s = tab[(b >> 16i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 16i32;
    h ^= s >> 48i32;
    s = tab[(b >> 20i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 20i32;
    h ^= s >> 44i32;
    s = tab[(b >> 24i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 24i32;
    h ^= s >> 40i32;
    s = tab[(b >> 28i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 28i32;
    h ^= s >> 36i32;
    s = tab[(b >> 32i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 32i32;
    h ^= s >> 32i32;
    s = tab[(b >> 36i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 36i32;
    h ^= s >> 28i32;
    s = tab[(b >> 40i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 40i32;
    h ^= s >> 24i32;
    s = tab[(b >> 44i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 44i32;
    h ^= s >> 20i32;
    s = tab[(b >> 48i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 48i32;
    h ^= s >> 16i32;
    s = tab[(b >> 52i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 52i32;
    h ^= s >> 12i32;
    s = tab[(b >> 56i32 & 0xfi32 as libc::c_ulong) as usize];
    l ^= s << 56i32;
    h ^= s >> 8i32;
    s = tab[(b >> 60i32) as usize];
    l ^= s << 60i32;
    h ^= s >> 4i32;
    if 0 != top3b & 0o1i32 as libc::c_ulong {
        l ^= b << 61i32;
        h ^= b >> 3i32
    }
    if 0 != top3b & 0o2i32 as libc::c_ulong {
        l ^= b << 62i32;
        h ^= b >> 2i32
    }
    if 0 != top3b & 0o4i32 as libc::c_ulong {
        l ^= b << 63i32;
        h ^= b >> 1i32
    }
    *rh = h;
    *rl = l;
}
/* Compute binary polynomial multiply d = a * b */
unsafe extern "C" fn s_bmul_d(mut a: *const mp_digit, mut a_len: mp_size,
                              mut b: mp_digit, mut d: *mut mp_digit) {
    let mut a_i: mp_digit = 0;
    let mut a0b0: mp_digit = 0;
    let mut a1b1: mp_digit = 0;
    let mut carry: mp_digit = 0i32 as mp_digit;
    loop  {
        let fresh10 = a_len;
        a_len = a_len.wrapping_sub(1);
        if !(0 != fresh10) { break ; }
        let fresh11 = a;
        a = a.offset(1);
        a_i = *fresh11;
        s_bmul_1x1(&mut a1b1, &mut a0b0, a_i, b);
        let fresh12 = d;
        d = d.offset(1);
        *fresh12 = a0b0 ^ carry;
        carry = a1b1
    }
    *d = carry;
}
/* For modular arithmetic, the irreducible polynomial f(t) is represented
 * as an array of int[], where f(t) is of the form:
 *     f(t) = t^p[0] + t^p[1] + ... + t^p[k]
 * where m = p[0] > p[1] > ... > p[k] = 0.
 */
#[no_mangle]
pub unsafe extern "C" fn mp_bmod(mut a: *const mp_int,
                                 mut p: *const libc::c_uint,
                                 mut r: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut dN: libc::c_int = 0;
    let mut d0: libc::c_int = 0;
    let mut d1: libc::c_int = 0;
    let mut zz: mp_digit = 0;
    let mut z: *mut mp_digit = 0 as *mut mp_digit;
    let mut tmp: mp_digit = 0;
    let mut used: mp_size = 0;
    let mut res: mp_err = 0i32;
    /* The algorithm does the reduction in place in r,
     * if a != r, copy a into r first so reduction can be done in r
     */
    if a != r {
        res = mp_copy(a, r);
        if 0i32 > res {
            current_block = 5013647533129079827;
        } else { current_block = 15427931788582360902; }
    } else { current_block = 15427931788582360902; }
    match current_block {
        15427931788582360902 => {
            z = (*r).dp;
            dN = (*p.offset(0isize) >> 6i32) as libc::c_int;
            used = (*r).used;
            j = used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
            while j > dN {
                zz = *z.offset(j as isize);
                if zz == 0i32 as libc::c_ulong {
                    j -= 1
                } else {
                    *z.offset(j as isize) = 0i32 as mp_digit;
                    k = 1i32;
                    while *p.offset(k as isize) > 0i32 as libc::c_uint {
                        n =
                            (*p.offset(0isize)).wrapping_sub(*p.offset(k as
                                                                           isize))
                                as libc::c_int;
                        d0 = n & 0x3fi32;
                        d1 = 64i32 - d0;
                        n >>= 6i32;
                        let ref mut fresh13 = *z.offset((j - n) as isize);
                        *fresh13 ^= zz >> d0;
                        if 0 != d0 {
                            let ref mut fresh14 =
                                *z.offset((j - n - 1i32) as isize);
                            *fresh14 ^= zz << d1
                        }
                        k += 1
                    }
                    n = dN;
                    d0 =
                        (*p.offset(0isize) & 0x3fi32 as libc::c_uint) as
                            libc::c_int;
                    d1 = 64i32 - d0;
                    let ref mut fresh15 = *z.offset((j - n) as isize);
                    *fresh15 ^= zz >> d0;
                    if 0 != d0 {
                        let ref mut fresh16 =
                            *z.offset((j - n - 1i32) as isize);
                        *fresh16 ^= zz << d1
                    }
                }
            }
            while j == dN {
                d0 =
                    (*p.offset(0isize) & 0x3fi32 as libc::c_uint) as
                        libc::c_int;
                zz = *z.offset(dN as isize) >> d0;
                if zz == 0i32 as libc::c_ulong { break ; }
                d1 = 64i32 - d0;
                if 0 != d0 {
                    *z.offset(dN as isize) =
                        *z.offset(dN as isize) << d1 >> d1
                } else { *z.offset(dN as isize) = 0i32 as mp_digit }
                *z ^= zz;
                k = 1i32;
                while *p.offset(k as isize) > 0i32 as libc::c_uint {
                    n = (*p.offset(k as isize) >> 6i32) as libc::c_int;
                    d0 =
                        (*p.offset(k as isize) & 0x3fi32 as libc::c_uint) as
                            libc::c_int;
                    d1 = 64i32 - d0;
                    let ref mut fresh17 = *z.offset(n as isize);
                    *fresh17 ^= zz << d0;
                    tmp = zz >> d1;
                    if 0 != d0 && 0 != tmp {
                        let ref mut fresh18 = *z.offset((n + 1i32) as isize);
                        *fresh18 ^= tmp
                    }
                    k += 1
                }
            }
            s_mp_clamp(r);
        }
        _ => { }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_bmulmod(mut a: *const mp_int,
                                    mut b: *const mp_int,
                                    mut p: *const libc::c_uint,
                                    mut r: *mut mp_int) -> mp_err {
    let mut res: mp_err = 0;
    if a == b { return mp_bsqrmod(a, p, r) }
    res = mp_bmul(a, b, r);
    if res != 0i32 { return res }
    return mp_bmod(r, p, r);
}
#[no_mangle]
pub unsafe extern "C" fn mp_bsqrmod(mut a: *const mp_int,
                                    mut p: *const libc::c_uint,
                                    mut r: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut pa: *mut mp_digit = 0 as *mut mp_digit;
    let mut pr: *mut mp_digit = 0 as *mut mp_digit;
    let mut a_i: mp_digit = 0;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut ia: mp_size = 0;
    let mut a_used: mp_size = 0;
    let mut res: mp_err = 0;
    if !(!a.is_null() && !r.is_null()) { return -4i32 }
    tmp.dp = 0 as *mut mp_digit;
    if a == r {
        res = mp_init_copy(&mut tmp, a);
        if 0i32 > res {
            current_block = 11218981141702897965;
        } else { a = &mut tmp; current_block = 11650488183268122163; }
    } else { current_block = 11650488183268122163; }
    match current_block {
        11650488183268122163 => {
            (*r).used = 1i32 as mp_size;
            *(*r).dp.offset(0isize) = 0i32 as mp_digit;
            res = s_mp_pad(r, (2i32 as libc::c_uint).wrapping_mul((*a).used));
            if !(0i32 > res) {
                pa = (*a).dp;
                pr = (*r).dp;
                a_used = (*a).used;
                (*r).used = (2i32 as libc::c_uint).wrapping_mul(a_used);
                ia = 0i32 as mp_size;
                while ia < a_used {
                    let fresh19 = pa;
                    pa = pa.offset(1);
                    a_i = *fresh19;
                    let fresh20 = pr;
                    pr = pr.offset(1);
                    *fresh20 =
                        mp_gf2m_sqr_tb[(a_i >> 28i32 &
                                            0xfi32 as libc::c_ulong) as usize]
                            << 56i32 |
                            mp_gf2m_sqr_tb[(a_i >> 24i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 48i32 |
                            mp_gf2m_sqr_tb[(a_i >> 20i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 40i32 |
                            mp_gf2m_sqr_tb[(a_i >> 16i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 32i32 |
                            mp_gf2m_sqr_tb[(a_i >> 12i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 24i32 |
                            mp_gf2m_sqr_tb[(a_i >> 8i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 16i32 |
                            mp_gf2m_sqr_tb[(a_i >> 4i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 8i32 |
                            mp_gf2m_sqr_tb[(a_i & 0xfi32 as libc::c_ulong) as
                                               usize];
                    let fresh21 = pr;
                    pr = pr.offset(1);
                    *fresh21 =
                        mp_gf2m_sqr_tb[(a_i >> 60i32 &
                                            0xfi32 as libc::c_ulong) as usize]
                            << 56i32 |
                            mp_gf2m_sqr_tb[(a_i >> 56i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 48i32 |
                            mp_gf2m_sqr_tb[(a_i >> 52i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 40i32 |
                            mp_gf2m_sqr_tb[(a_i >> 48i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 32i32 |
                            mp_gf2m_sqr_tb[(a_i >> 44i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 24i32 |
                            mp_gf2m_sqr_tb[(a_i >> 40i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 16i32 |
                            mp_gf2m_sqr_tb[(a_i >> 36i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize] << 8i32 |
                            mp_gf2m_sqr_tb[(a_i >> 32i32 &
                                                0xfi32 as libc::c_ulong) as
                                               usize];
                    ia = ia.wrapping_add(1)
                }
                res = mp_bmod(r, p, r);
                if !(0i32 > res) {
                    s_mp_clamp(r);
                    (*r).sign = 0i32 as mp_sign
                }
            }
        }
        _ => { }
    }
    mp_clear(&mut tmp);
    return res;
}
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#[no_mangle]
pub static mut mp_gf2m_sqr_tb: [mp_digit; 16] =
    [0i32 as mp_digit, 1i32 as mp_digit, 4i32 as mp_digit, 5i32 as mp_digit,
     16i32 as mp_digit, 17i32 as mp_digit, 20i32 as mp_digit,
     21i32 as mp_digit, 64i32 as mp_digit, 65i32 as mp_digit,
     68i32 as mp_digit, 69i32 as mp_digit, 80i32 as mp_digit,
     81i32 as mp_digit, 84i32 as mp_digit, 85i32 as mp_digit];
#[no_mangle]
pub unsafe extern "C" fn mp_bdivmod(mut y: *const mp_int,
                                    mut x: *const mp_int,
                                    mut pp: *const mp_int,
                                    mut p: *const libc::c_uint,
                                    mut r: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut aa: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut bb: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut uu: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut a: *mut mp_int = 0 as *mut mp_int;
    let mut b: *mut mp_int = 0 as *mut mp_int;
    let mut u: *mut mp_int = 0 as *mut mp_int;
    let mut v: *mut mp_int = 0 as *mut mp_int;
    let mut res: mp_err = 0i32;
    aa.dp = 0 as *mut mp_digit;
    bb.dp = 0 as *mut mp_digit;
    uu.dp = 0 as *mut mp_digit;
    res = mp_init_copy(&mut aa, x);
    if !(0i32 > res) {
        res = mp_init_copy(&mut uu, y);
        if !(0i32 > res) {
            res = mp_init_copy(&mut bb, pp);
            if !(0i32 > res) {
                res = s_mp_pad(r, (*pp).used);
                if !(0i32 > res) {
                    (*r).used = 1i32 as mp_size;
                    *(*r).dp.offset(0isize) = 0i32 as mp_digit;
                    a = &mut aa;
                    b = &mut bb;
                    u = &mut uu;
                    v = r;
                    /* reduce x and y mod p */
                    res = mp_bmod(a, p, a);
                    if !(0i32 > res) {
                        res = mp_bmod(u, p, u);
                        if !(0i32 > res) {
                            loop  {
                                if !(0 == mp_isodd(a)) {
                                    current_block = 7172762164747879670;
                                    break ;
                                }
                                res = mpl_rsh(a, a, 1i32 as mp_digit);
                                if 0i32 > res {
                                    current_block = 17561965678190735196;
                                    break ;
                                }
                                if 0 != mp_isodd(u) {
                                    res = mp_badd(u, pp, u);
                                    if 0i32 > res {
                                        current_block = 17561965678190735196;
                                        break ;
                                    }
                                }
                                res = mpl_rsh(u, u, 1i32 as mp_digit);
                                if 0i32 > res {
                                    current_block = 17561965678190735196;
                                    break ;
                                }
                            }
                            match current_block {
                                17561965678190735196 => { }
                                _ => {
                                    's_129:
                                        loop  {
                                            if mp_cmp_mag(b, a) > 0i32 {
                                                res = mp_badd(b, a, b);
                                                if 0i32 > res {
                                                    current_block =
                                                        17561965678190735196;
                                                    break ;
                                                }
                                                res = mp_badd(v, u, v);
                                                if 0i32 > res {
                                                    current_block =
                                                        17561965678190735196;
                                                    break ;
                                                }
                                                loop  {
                                                    res =
                                                        mpl_rsh(b, b,
                                                                1i32 as
                                                                    mp_digit);
                                                    if 0i32 > res {
                                                        current_block =
                                                            17561965678190735196;
                                                        break 's_129 ;
                                                    }
                                                    if 0 != mp_isodd(v) {
                                                        res =
                                                            mp_badd(v, pp, v);
                                                        if 0i32 > res {
                                                            current_block =
                                                                17561965678190735196;
                                                            break 's_129 ;
                                                        }
                                                    }
                                                    res =
                                                        mpl_rsh(v, v,
                                                                1i32 as
                                                                    mp_digit);
                                                    if 0i32 > res {
                                                        current_block =
                                                            17561965678190735196;
                                                        break 's_129 ;
                                                    }
                                                    if !(0 == mp_isodd(b)) {
                                                        break ;
                                                    }
                                                }
                                            } else {
                                                if *(*a).dp.offset(0isize) ==
                                                       1i32 as libc::c_ulong
                                                       &&
                                                       (*a).used ==
                                                           1i32 as
                                                               libc::c_uint {
                                                    current_block =
                                                        12556861819962772176;
                                                    break ;
                                                }
                                                res = mp_badd(a, b, a);
                                                if 0i32 > res {
                                                    current_block =
                                                        17561965678190735196;
                                                    break ;
                                                }
                                                res = mp_badd(u, v, u);
                                                if 0i32 > res {
                                                    current_block =
                                                        17561965678190735196;
                                                    break ;
                                                }
                                                loop  {
                                                    res =
                                                        mpl_rsh(a, a,
                                                                1i32 as
                                                                    mp_digit);
                                                    if 0i32 > res {
                                                        current_block =
                                                            17561965678190735196;
                                                        break 's_129 ;
                                                    }
                                                    if 0 != mp_isodd(u) {
                                                        res =
                                                            mp_badd(u, pp, u);
                                                        if 0i32 > res {
                                                            current_block =
                                                                17561965678190735196;
                                                            break 's_129 ;
                                                        }
                                                    }
                                                    res =
                                                        mpl_rsh(u, u,
                                                                1i32 as
                                                                    mp_digit);
                                                    if 0i32 > res {
                                                        current_block =
                                                            17561965678190735196;
                                                        break 's_129 ;
                                                    }
                                                    if !(0 == mp_isodd(a)) {
                                                        break ;
                                                    }
                                                }
                                            }
                                        }
                                    match current_block {
                                        17561965678190735196 => { }
                                        _ => {
                                            res = mp_copy(u, r);
                                            0i32 > res;
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
    mp_clear(&mut aa);
    mp_clear(&mut bb);
    mp_clear(&mut uu);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mp_bpoly2arr(mut a: *const mp_int,
                                      mut p: *mut libc::c_uint,
                                      mut max: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut top_bit: mp_digit = 0;
    let mut mask: mp_digit = 0;
    top_bit = 1i32 as mp_digit;
    top_bit <<=
        (8i32 as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>() as
                                             libc::c_ulong).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_ulong);
    k = 0i32;
    while k < max { *p.offset(k as isize) = 0i32 as libc::c_uint; k += 1 }
    k = 0i32;
    i = (*a).used.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
    while i >= 0i32 {
        mask = top_bit;
        j =
            (8i32 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)
                as libc::c_int;
        while j >= 0i32 {
            if 0 != *(*a).dp.offset(i as isize) & mask {
                if k < max {
                    *p.offset(k as isize) =
                        (8i32 as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(i
                                                                                             as
                                                                                             libc::c_ulong).wrapping_add(j
                                                                                                                             as
                                                                                                                             libc::c_ulong)
                            as libc::c_uint
                }
                k += 1
            }
            mask >>= 1i32;
            j -= 1
        }
        i -= 1
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn mp_barr2poly(mut p: *const libc::c_uint,
                                      mut a: *mut mp_int) -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0i32;
    let mut i: libc::c_int = 0;
    mp_zero(a);
    i = 0i32;
    loop  {
        if !(*p.offset(i as isize) > 0i32 as libc::c_uint) {
            current_block = 6873731126896040597;
            break ;
        }
        res = mpl_set_bit(a, *p.offset(i as isize), 1i32 as mp_size);
        if 0i32 > res { current_block = 13004805521902200896; break ; }
        i += 1
    }
    match current_block {
        6873731126896040597 => {
            res = mpl_set_bit(a, 0i32 as mp_size, 1i32 as mp_size);
            0i32 > res;
        }
        _ => { }
    }
    return res;
}
/* Compute xor-multiply of two binary polynomials  (a1, a0) x (b1, b0)
 * result is a binary polynomial in 4 mp_digits r[4].
 * The caller MUST ensure that r has the right amount of space allocated.
 */
#[no_mangle]
pub unsafe extern "C" fn s_bmul_2x2(mut r: *mut mp_digit, a1: mp_digit,
                                    a0: mp_digit, b1: mp_digit,
                                    b0: mp_digit) {
    let mut m1: mp_digit = 0;
    let mut m0: mp_digit = 0;
    s_bmul_1x1(r.offset(3isize), r.offset(2isize), a1, b1);
    s_bmul_1x1(r.offset(1isize), r, a0, b0);
    s_bmul_1x1(&mut m1, &mut m0, a0 ^ a1, b0 ^ b1);
    let ref mut fresh22 = *r.offset(2isize);
    *fresh22 ^= m1 ^ *r.offset(1isize) ^ *r.offset(3isize);
    *r.offset(1isize) =
        *r.offset(3isize) ^ *r.offset(2isize) ^ *r.offset(0isize) ^ m1 ^ m0;
}
/* Compute xor-multiply of two binary polynomials  (a2, a1, a0) x (b2, b1, b0)
 * result is a binary polynomial in 6 mp_digits r[6].
 * The caller MUST ensure that r has the right amount of space allocated.
 */
#[no_mangle]
pub unsafe extern "C" fn s_bmul_3x3(mut r: *mut mp_digit, a2: mp_digit,
                                    a1: mp_digit, a0: mp_digit, b2: mp_digit,
                                    b1: mp_digit, b0: mp_digit) {
    let mut zm: [mp_digit; 4] = [0; 4];
    s_bmul_1x1(r.offset(5isize), r.offset(4isize), a2, b2);
    s_bmul_2x2(zm.as_mut_ptr(), a1, a2 ^ a0, b1, b2 ^ b0);
    s_bmul_2x2(r, a1, a0, b1, b0);
    zm[3usize] ^= *r.offset(3isize);
    zm[2usize] ^= *r.offset(2isize);
    zm[1usize] ^= *r.offset(1isize) ^ *r.offset(5isize);
    zm[0usize] ^= *r.offset(0isize) ^ *r.offset(4isize);
    let ref mut fresh23 = *r.offset(5isize);
    *fresh23 ^= zm[3usize];
    let ref mut fresh24 = *r.offset(4isize);
    *fresh24 ^= zm[2usize];
    let ref mut fresh25 = *r.offset(3isize);
    *fresh25 ^= zm[1usize];
    let ref mut fresh26 = *r.offset(2isize);
    *fresh26 ^= zm[0usize];
}
/* Compute xor-multiply of two binary polynomials  (a3, a2, a1, a0) x (b3, b2, b1, b0)
 * result is a binary polynomial in 8 mp_digits r[8].
 * The caller MUST ensure that r has the right amount of space allocated.
 */
#[no_mangle]
pub unsafe extern "C" fn s_bmul_4x4(mut r: *mut mp_digit, a3: mp_digit,
                                    a2: mp_digit, a1: mp_digit, a0: mp_digit,
                                    b3: mp_digit, b2: mp_digit, b1: mp_digit,
                                    b0: mp_digit) {
    let mut zm: [mp_digit; 4] = [0; 4];
    s_bmul_2x2(r.offset(4isize), a3, a2, b3, b2);
    s_bmul_2x2(zm.as_mut_ptr(), a3 ^ a1, a2 ^ a0, b3 ^ b1, b2 ^ b0);
    s_bmul_2x2(r, a1, a0, b1, b0);
    zm[3usize] ^= *r.offset(3isize) ^ *r.offset(7isize);
    zm[2usize] ^= *r.offset(2isize) ^ *r.offset(6isize);
    zm[1usize] ^= *r.offset(1isize) ^ *r.offset(5isize);
    zm[0usize] ^= *r.offset(0isize) ^ *r.offset(4isize);
    let ref mut fresh27 = *r.offset(5isize);
    *fresh27 ^= zm[3usize];
    let ref mut fresh28 = *r.offset(4isize);
    *fresh28 ^= zm[2usize];
    let ref mut fresh29 = *r.offset(3isize);
    *fresh29 ^= zm[1usize];
    let ref mut fresh30 = *r.offset(2isize);
    *fresh30 ^= zm[0usize];
}