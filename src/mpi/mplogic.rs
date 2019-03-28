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
    fn mp_copy(from: *const mp_int, to: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn s_mp_pad(mp: *mut mp_int, min: mp_size) -> mp_err;
    #[no_mangle]
    fn s_mp_clamp(mp: *mut mp_int);
    #[no_mangle]
    fn s_mp_mul_2d(mp: *mut mp_int, d: mp_digit) -> mp_err;
    #[no_mangle]
    fn s_mp_div_2d(mp: *mut mp_int, d: mp_digit);
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
/*
 *  mplogic.h
 *
 *  Bitwise logical operations on MPI values
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
  The logical operations treat an mp_int as if it were a bit vector,
  without regard to its sign (an mp_int is represented in a signed
  magnitude format).  Values are treated as if they had an infinite
  string of zeros left of the most-significant bit.
 */
/* Parity results                    */
/* Bitwise functions                 */
/* one's complement  */
#[no_mangle]
pub unsafe extern "C" fn mpl_not(mut a: *mut mp_int, mut b: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    let mut ix: libc::c_uint = 0;
    if !(!a.is_null() && !b.is_null()) { return -4i32 }
    res = mp_copy(a, b);
    if res != 0i32 { return res }
    ix = 0i32 as libc::c_uint;
    while ix < (*b).used {
        *(*b).dp.offset(ix as isize) = !*(*b).dp.offset(ix as isize);
        ix = ix.wrapping_add(1)
    }
    s_mp_clamp(b);
    return 0i32;
}
/* bitwise AND       */
#[no_mangle]
pub unsafe extern "C" fn mpl_and(mut a: *mut mp_int, mut b: *mut mp_int,
                                 mut c: *mut mp_int) -> mp_err {
    let mut which: *mut mp_int = 0 as *mut mp_int;
    let mut other: *mut mp_int = 0 as *mut mp_int;
    let mut res: mp_err = 0;
    let mut ix: libc::c_uint = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if (*a).used <= (*b).used {
        which = a;
        other = b
    } else { which = b; other = a }
    res = mp_copy(which, c);
    if res != 0i32 { return res }
    ix = 0i32 as libc::c_uint;
    while ix < (*which).used {
        let ref mut fresh0 = *(*c).dp.offset(ix as isize);
        *fresh0 &= *(*other).dp.offset(ix as isize);
        ix = ix.wrapping_add(1)
    }
    s_mp_clamp(c);
    return 0i32;
}
/* bitwise OR        */
#[no_mangle]
pub unsafe extern "C" fn mpl_or(mut a: *mut mp_int, mut b: *mut mp_int,
                                mut c: *mut mp_int) -> mp_err {
    let mut which: *mut mp_int = 0 as *mut mp_int;
    let mut other: *mut mp_int = 0 as *mut mp_int;
    let mut res: mp_err = 0;
    let mut ix: libc::c_uint = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if (*a).used >= (*b).used {
        which = a;
        other = b
    } else { which = b; other = a }
    res = mp_copy(which, c);
    if res != 0i32 { return res }
    ix = 0i32 as libc::c_uint;
    while ix < (*which).used {
        let ref mut fresh1 = *(*c).dp.offset(ix as isize);
        *fresh1 |= *(*other).dp.offset(ix as isize);
        ix = ix.wrapping_add(1)
    }
    return 0i32;
}
/* bitwise XOR       */
#[no_mangle]
pub unsafe extern "C" fn mpl_xor(mut a: *mut mp_int, mut b: *mut mp_int,
                                 mut c: *mut mp_int) -> mp_err {
    let mut which: *mut mp_int = 0 as *mut mp_int;
    let mut other: *mut mp_int = 0 as *mut mp_int;
    let mut res: mp_err = 0;
    let mut ix: libc::c_uint = 0;
    if !(!a.is_null() && !b.is_null() && !c.is_null()) { return -4i32 }
    if (*a).used >= (*b).used {
        which = a;
        other = b
    } else { which = b; other = a }
    res = mp_copy(which, c);
    if res != 0i32 { return res }
    ix = 0i32 as libc::c_uint;
    while ix < (*which).used {
        let ref mut fresh2 = *(*c).dp.offset(ix as isize);
        *fresh2 ^= *(*other).dp.offset(ix as isize);
        ix = ix.wrapping_add(1)
    }
    s_mp_clamp(c);
    return 0i32;
}
/* Shift functions                   */
/* right shift    */
#[no_mangle]
pub unsafe extern "C" fn mpl_rsh(mut a: *const mp_int, mut b: *mut mp_int,
                                 mut d: mp_digit) -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null()) { return -4i32 }
    res = mp_copy(a, b);
    if res != 0i32 { return res }
    s_mp_div_2d(b, d);
    return 0i32;
}
/* left shift     */
#[no_mangle]
pub unsafe extern "C" fn mpl_lsh(mut a: *const mp_int, mut b: *mut mp_int,
                                 mut d: mp_digit) -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && !b.is_null()) { return -4i32 }
    res = mp_copy(a, b);
    if res != 0i32 { return res }
    return s_mp_mul_2d(b, d);
}
/* Bit count and parity              */
/* count set bits    */
#[no_mangle]
pub unsafe extern "C" fn mpl_num_set(mut a: *mut mp_int,
                                     mut num: *mut libc::c_int) -> mp_err {
    let mut ix: libc::c_uint = 0;
    let mut db: libc::c_int = 0;
    let mut nset: libc::c_int = 0i32;
    let mut cur: mp_digit = 0;
    let mut reg: libc::c_uchar = 0;
    if a.is_null() { return -4i32 }
    ix = 0i32 as libc::c_uint;
    while ix < (*a).used {
        cur = *(*a).dp.offset(ix as isize);
        db = 0i32;
        while (db as libc::c_ulong) <
                  ::std::mem::size_of::<mp_digit>() as libc::c_ulong {
            reg = (cur >> 8i32 * db) as libc::c_uchar;
            nset += bitc[reg as usize] as libc::c_int;
            db += 1
        }
        ix = ix.wrapping_add(1)
    }
    if !num.is_null() { *num = nset }
    return 0i32;
}
/*
 *  mplogic.c
 *
 *  Bitwise logical operations on MPI values
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* {{{ Lookup table for population count */
static mut bitc: [libc::c_uchar; 256] =
    [0i32 as libc::c_uchar, 1i32 as libc::c_uchar, 1i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 1i32 as libc::c_uchar, 2i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 1i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 2i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 1i32 as libc::c_uchar, 2i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 2i32 as libc::c_uchar,
     3i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 1i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 2i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 2i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     3i32 as libc::c_uchar, 4i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 1i32 as libc::c_uchar, 2i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 2i32 as libc::c_uchar,
     3i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 2i32 as libc::c_uchar,
     3i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     3i32 as libc::c_uchar, 4i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     5i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     5i32 as libc::c_uchar, 5i32 as libc::c_uchar, 6i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     5i32 as libc::c_uchar, 5i32 as libc::c_uchar, 6i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 5i32 as libc::c_uchar, 6i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 7i32 as libc::c_uchar, 1i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 2i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 2i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     3i32 as libc::c_uchar, 4i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 2i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     3i32 as libc::c_uchar, 4i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     3i32 as libc::c_uchar, 4i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     5i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     5i32 as libc::c_uchar, 6i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     5i32 as libc::c_uchar, 6i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 6i32 as libc::c_uchar, 7i32 as libc::c_uchar,
     2i32 as libc::c_uchar, 3i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 3i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 4i32 as libc::c_uchar,
     5i32 as libc::c_uchar, 5i32 as libc::c_uchar, 6i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 5i32 as libc::c_uchar, 6i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 7i32 as libc::c_uchar, 3i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 4i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     5i32 as libc::c_uchar, 6i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 6i32 as libc::c_uchar, 7i32 as libc::c_uchar,
     4i32 as libc::c_uchar, 5i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 5i32 as libc::c_uchar, 6i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 7i32 as libc::c_uchar, 5i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 6i32 as libc::c_uchar, 7i32 as libc::c_uchar,
     6i32 as libc::c_uchar, 7i32 as libc::c_uchar, 7i32 as libc::c_uchar,
     8i32 as libc::c_uchar];
/* count clear bits  */
#[no_mangle]
pub unsafe extern "C" fn mpl_num_clear(mut a: *mut mp_int,
                                       mut num: *mut libc::c_int) -> mp_err {
    let mut ix: libc::c_uint = 0;
    let mut db: libc::c_int = 0;
    let mut nset: libc::c_int = 0i32;
    let mut cur: mp_digit = 0;
    let mut reg: libc::c_uchar = 0;
    if a.is_null() { return -4i32 }
    ix = 0i32 as libc::c_uint;
    while ix < (*a).used {
        cur = *(*a).dp.offset(ix as isize);
        db = 0i32;
        while (db as libc::c_ulong) <
                  ::std::mem::size_of::<mp_digit>() as libc::c_ulong {
            reg = (cur >> 8i32 * db) as libc::c_uchar;
            nset +=
                bitc[(127i32 * 2i32 + 1i32 - reg as libc::c_int) as usize] as
                    libc::c_int;
            db += 1
        }
        ix = ix.wrapping_add(1)
    }
    if !num.is_null() { *num = nset }
    return 0i32;
}
/* determine parity  */
#[no_mangle]
pub unsafe extern "C" fn mpl_parity(mut a: *mut mp_int) -> mp_err {
    let mut ix: libc::c_uint = 0;
    let mut par: libc::c_int = 0i32;
    let mut cur: mp_digit = 0;
    if a.is_null() { return -4i32 }
    ix = 0i32 as libc::c_uint;
    while ix < (*a).used {
        let mut shft: libc::c_int =
            (::std::mem::size_of::<mp_digit>() as
                 libc::c_ulong).wrapping_mul(8i32 as
                                                 libc::c_ulong).wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_ulong)
                as libc::c_int;
        cur = *(*a).dp.offset(ix as isize);
        while shft != 0i32 { cur ^= cur >> shft; shft >>= 1i32 }
        cur &= 1i32 as libc::c_ulong;
        par = (par as libc::c_ulong ^ cur) as libc::c_int;
        ix = ix.wrapping_add(1)
    }
    if 0 != par { return -1i32 } else { return 0i32 };
}
/* Get & Set the value of a bit */
#[no_mangle]
pub unsafe extern "C" fn mpl_set_bit(mut a: *mut mp_int, mut bitNum: mp_size,
                                     mut value: mp_size) -> mp_err {
    let mut ix: mp_size = 0;
    let mut rv: mp_err = 0;
    let mut mask: mp_digit = 0;
    if a.is_null() { return -4i32 }
    ix =
        (bitNum as
             libc::c_ulong).wrapping_div((8i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                              as
                                                                              libc::c_ulong))
            as mp_size;
    if ix.wrapping_add(1i32 as libc::c_uint) > (*a).used {
        rv = s_mp_pad(a, ix.wrapping_add(1i32 as libc::c_uint));
        if rv != 0i32 { return rv }
    }
    bitNum =
        (bitNum as
             libc::c_ulong).wrapping_rem((8i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                              as
                                                                              libc::c_ulong))
            as mp_size;
    mask = (1i32 as mp_digit) << bitNum;
    if 0 != value {
        let ref mut fresh3 = *(*a).dp.offset(ix as isize);
        *fresh3 |= mask
    } else {
        let ref mut fresh4 = *(*a).dp.offset(ix as isize);
        *fresh4 &= !mask
    }
    s_mp_clamp(a);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpl_get_bit(mut a: *const mp_int,
                                     mut bitNum: mp_size) -> mp_err {
    let mut bit: mp_size = 0;
    let mut ix: mp_size = 0;
    let mut rv: mp_err = 0;
    if a.is_null() { return -4i32 }
    ix =
        (bitNum as
             libc::c_ulong).wrapping_div((8i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                              as
                                                                              libc::c_ulong))
            as mp_size;
    if !(ix <= (*a).used.wrapping_sub(1i32 as libc::c_uint)) { return -3i32 }
    bit =
        (bitNum as
             libc::c_ulong).wrapping_rem((8i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                              as
                                                                              libc::c_ulong))
            as mp_size;
    rv = (*(*a).dp.offset(ix as isize) >> bit) as mp_err & 1i32;
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn mpl_get_bits(mut a: *const mp_int,
                                      mut lsbNum: mp_size,
                                      mut numBits: mp_size) -> mp_err {
    let mut rshift: mp_size =
        (lsbNum as
             libc::c_ulong).wrapping_rem((8i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                              as
                                                                              libc::c_ulong))
            as mp_size;
    let mut lsWndx: mp_size =
        (lsbNum as
             libc::c_ulong).wrapping_div((8i32 as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                              as
                                                                              libc::c_ulong))
            as mp_size;
    let mut digit: *mut mp_digit = (*a).dp.offset(lsWndx as isize);
    let mut mask: mp_digit = ((1i32 << numBits) - 1i32) as mp_digit;
    if !((numBits as libc::c_ulong) <
             (8i32 as
                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                  as libc::c_ulong)) {
        return -4i32
    }
    if !((lsbNum as
              libc::c_ulong).wrapping_add((8i32 as
                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                               as
                                                                               libc::c_ulong)).wrapping_sub(1i32
                                                                                                                as
                                                                                                                libc::c_ulong).wrapping_div((8i32
                                                                                                                                                 as
                                                                                                                                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_ulong))
             <= (*a).used as libc::c_ulong) {
        return -3i32
    }
    if (numBits as
            libc::c_ulong).wrapping_add((lsbNum as
                                             libc::c_ulong).wrapping_rem((8i32
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                              as
                                                                                                              libc::c_ulong)))
           <=
           (8i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                as libc::c_ulong) ||
           lsWndx.wrapping_add(1i32 as libc::c_uint) >= (*a).used {
        mask &= *digit.offset(0isize) >> rshift
    } else {
        mask &=
            *digit.offset(0isize) >> rshift |
                *digit.offset(1isize) <<
                    (8i32 as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(rshift
                                                                                         as
                                                                                         libc::c_ulong)
    }
    return mask as mp_err;
}
#[no_mangle]
pub unsafe extern "C" fn mpl_significant_bits(mut a: *const mp_int)
 -> mp_size {
    let mut bits: mp_size = 0i32 as mp_size;
    let mut ix: libc::c_int = 0;
    if a.is_null() { return -4i32 as mp_size }
    ix = (*a).used as libc::c_int;
    while ix > 0i32 {
        let mut d: mp_digit = 0;
        ix -= 1;
        d = *(*a).dp.offset(ix as isize);
        if !(0 != d) { continue ; }
        while 0 != d { bits = bits.wrapping_add(1); d >>= 1i32 }
        break ;
    }
    bits =
        (bits as
             libc::c_ulong).wrapping_add((ix as
                                              libc::c_ulong).wrapping_mul((8i32
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<mp_digit>()
                                                                                                               as
                                                                                                               libc::c_ulong)))
            as mp_size as mp_size;
    if 0 == bits { bits = 1i32 as mp_size }
    return bits;
}