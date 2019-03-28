#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc)]
extern crate libc;
extern "C" {
    /* Memory management       */
    #[no_mangle]
    fn mp_init(mp: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_init_size(mp: *mut mp_int, prec: mp_size) -> mp_err;
    #[no_mangle]
    fn mp_exch(mp1: *mut mp_int, mp2: *mut mp_int);
    #[no_mangle]
    fn mp_clear(mp: *mut mp_int);
    #[no_mangle]
    fn mp_set(mp: *mut mp_int, d: mp_digit);
    /* Single digit arithmetic */
    #[no_mangle]
    fn mp_add_d(a: *const mp_int, d: mp_digit, b: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_sub_d(a: *const mp_int, d: mp_digit, b: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_mul_2(a: *const mp_int, c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_div_2d(a: *const mp_int, d: mp_digit, q: *mut mp_int,
                 r: *mut mp_int) -> mp_err;
    /* Modular arithmetic      */
    #[no_mangle]
    fn mp_mod(a: *const mp_int, m: *const mp_int, c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_mod_d(a: *const mp_int, d: mp_digit, c: *mut mp_digit) -> mp_err;
    #[no_mangle]
    fn mp_sqrmod(a: *const mp_int, m: *const mp_int, c: *mut mp_int)
     -> mp_err;
    #[no_mangle]
    fn mp_exptmod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                  c: *mut mp_int) -> mp_err;
    /* MP_MODARITH */
    /* Comparisons             */
    #[no_mangle]
    fn mp_cmp_z(a: *const mp_int) -> libc::c_int;
    #[no_mangle]
    fn mp_cmp_d(a: *const mp_int, d: mp_digit) -> libc::c_int;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    /* Miscellaneous */
    #[no_mangle]
    fn mp_trailing_zeros(mp: *const mp_int) -> mp_size;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn s_mp_pad(mp: *mut mp_int, min: mp_size) -> mp_err;
    /* Get & Set the value of a bit */
    #[no_mangle]
    fn mpl_set_bit(a: *mut mp_int, bitNum: mp_size, value: mp_size) -> mp_err;
    #[no_mangle]
    fn mpl_significant_bits(a: *const mp_int) -> mp_size;
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
 *  mpprime.h
 *
 *  Utilities for finding and working with prime and pseudo-prime
 *  integers
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* number of primes available */
#[no_mangle]
pub static mut prime_tab_size: libc::c_int = 6542i32;
#[no_mangle]
pub static mut prime_tab: [mp_digit; 6542] =
    [0x2i32 as mp_digit, 0x3i32 as mp_digit, 0x5i32 as mp_digit,
     0x7i32 as mp_digit, 0xbi32 as mp_digit, 0xdi32 as mp_digit,
     0x11i32 as mp_digit, 0x13i32 as mp_digit, 0x17i32 as mp_digit,
     0x1di32 as mp_digit, 0x1fi32 as mp_digit, 0x25i32 as mp_digit,
     0x29i32 as mp_digit, 0x2bi32 as mp_digit, 0x2fi32 as mp_digit,
     0x35i32 as mp_digit, 0x3bi32 as mp_digit, 0x3di32 as mp_digit,
     0x43i32 as mp_digit, 0x47i32 as mp_digit, 0x49i32 as mp_digit,
     0x4fi32 as mp_digit, 0x53i32 as mp_digit, 0x59i32 as mp_digit,
     0x61i32 as mp_digit, 0x65i32 as mp_digit, 0x67i32 as mp_digit,
     0x6bi32 as mp_digit, 0x6di32 as mp_digit, 0x71i32 as mp_digit,
     0x7fi32 as mp_digit, 0x83i32 as mp_digit, 0x89i32 as mp_digit,
     0x8bi32 as mp_digit, 0x95i32 as mp_digit, 0x97i32 as mp_digit,
     0x9di32 as mp_digit, 0xa3i32 as mp_digit, 0xa7i32 as mp_digit,
     0xadi32 as mp_digit, 0xb3i32 as mp_digit, 0xb5i32 as mp_digit,
     0xbfi32 as mp_digit, 0xc1i32 as mp_digit, 0xc5i32 as mp_digit,
     0xc7i32 as mp_digit, 0xd3i32 as mp_digit, 0xdfi32 as mp_digit,
     0xe3i32 as mp_digit, 0xe5i32 as mp_digit, 0xe9i32 as mp_digit,
     0xefi32 as mp_digit, 0xf1i32 as mp_digit, 0xfbi32 as mp_digit,
     0x101i32 as mp_digit, 0x107i32 as mp_digit, 0x10di32 as mp_digit,
     0x10fi32 as mp_digit, 0x115i32 as mp_digit, 0x119i32 as mp_digit,
     0x11bi32 as mp_digit, 0x125i32 as mp_digit, 0x133i32 as mp_digit,
     0x137i32 as mp_digit, 0x139i32 as mp_digit, 0x13di32 as mp_digit,
     0x14bi32 as mp_digit, 0x151i32 as mp_digit, 0x15bi32 as mp_digit,
     0x15di32 as mp_digit, 0x161i32 as mp_digit, 0x167i32 as mp_digit,
     0x16fi32 as mp_digit, 0x175i32 as mp_digit, 0x17bi32 as mp_digit,
     0x17fi32 as mp_digit, 0x185i32 as mp_digit, 0x18di32 as mp_digit,
     0x191i32 as mp_digit, 0x199i32 as mp_digit, 0x1a3i32 as mp_digit,
     0x1a5i32 as mp_digit, 0x1afi32 as mp_digit, 0x1b1i32 as mp_digit,
     0x1b7i32 as mp_digit, 0x1bbi32 as mp_digit, 0x1c1i32 as mp_digit,
     0x1c9i32 as mp_digit, 0x1cdi32 as mp_digit, 0x1cfi32 as mp_digit,
     0x1d3i32 as mp_digit, 0x1dfi32 as mp_digit, 0x1e7i32 as mp_digit,
     0x1ebi32 as mp_digit, 0x1f3i32 as mp_digit, 0x1f7i32 as mp_digit,
     0x1fdi32 as mp_digit, 0x209i32 as mp_digit, 0x20bi32 as mp_digit,
     0x21di32 as mp_digit, 0x223i32 as mp_digit, 0x22di32 as mp_digit,
     0x233i32 as mp_digit, 0x239i32 as mp_digit, 0x23bi32 as mp_digit,
     0x241i32 as mp_digit, 0x24bi32 as mp_digit, 0x251i32 as mp_digit,
     0x257i32 as mp_digit, 0x259i32 as mp_digit, 0x25fi32 as mp_digit,
     0x265i32 as mp_digit, 0x269i32 as mp_digit, 0x26bi32 as mp_digit,
     0x277i32 as mp_digit, 0x281i32 as mp_digit, 0x283i32 as mp_digit,
     0x287i32 as mp_digit, 0x28di32 as mp_digit, 0x293i32 as mp_digit,
     0x295i32 as mp_digit, 0x2a1i32 as mp_digit, 0x2a5i32 as mp_digit,
     0x2abi32 as mp_digit, 0x2b3i32 as mp_digit, 0x2bdi32 as mp_digit,
     0x2c5i32 as mp_digit, 0x2cfi32 as mp_digit, 0x2d7i32 as mp_digit,
     0x2ddi32 as mp_digit, 0x2e3i32 as mp_digit, 0x2e7i32 as mp_digit,
     0x2efi32 as mp_digit, 0x2f5i32 as mp_digit, 0x2f9i32 as mp_digit,
     0x301i32 as mp_digit, 0x305i32 as mp_digit, 0x313i32 as mp_digit,
     0x31di32 as mp_digit, 0x329i32 as mp_digit, 0x32bi32 as mp_digit,
     0x335i32 as mp_digit, 0x337i32 as mp_digit, 0x33bi32 as mp_digit,
     0x33di32 as mp_digit, 0x347i32 as mp_digit, 0x355i32 as mp_digit,
     0x359i32 as mp_digit, 0x35bi32 as mp_digit, 0x35fi32 as mp_digit,
     0x36di32 as mp_digit, 0x371i32 as mp_digit, 0x373i32 as mp_digit,
     0x377i32 as mp_digit, 0x38bi32 as mp_digit, 0x38fi32 as mp_digit,
     0x397i32 as mp_digit, 0x3a1i32 as mp_digit, 0x3a9i32 as mp_digit,
     0x3adi32 as mp_digit, 0x3b3i32 as mp_digit, 0x3b9i32 as mp_digit,
     0x3c7i32 as mp_digit, 0x3cbi32 as mp_digit, 0x3d1i32 as mp_digit,
     0x3d7i32 as mp_digit, 0x3dfi32 as mp_digit, 0x3e5i32 as mp_digit,
     0x3f1i32 as mp_digit, 0x3f5i32 as mp_digit, 0x3fbi32 as mp_digit,
     0x3fdi32 as mp_digit, 0x407i32 as mp_digit, 0x409i32 as mp_digit,
     0x40fi32 as mp_digit, 0x419i32 as mp_digit, 0x41bi32 as mp_digit,
     0x425i32 as mp_digit, 0x427i32 as mp_digit, 0x42di32 as mp_digit,
     0x43fi32 as mp_digit, 0x443i32 as mp_digit, 0x445i32 as mp_digit,
     0x449i32 as mp_digit, 0x44fi32 as mp_digit, 0x455i32 as mp_digit,
     0x45di32 as mp_digit, 0x463i32 as mp_digit, 0x469i32 as mp_digit,
     0x47fi32 as mp_digit, 0x481i32 as mp_digit, 0x48bi32 as mp_digit,
     0x493i32 as mp_digit, 0x49di32 as mp_digit, 0x4a3i32 as mp_digit,
     0x4a9i32 as mp_digit, 0x4b1i32 as mp_digit, 0x4bdi32 as mp_digit,
     0x4c1i32 as mp_digit, 0x4c7i32 as mp_digit, 0x4cdi32 as mp_digit,
     0x4cfi32 as mp_digit, 0x4d5i32 as mp_digit, 0x4e1i32 as mp_digit,
     0x4ebi32 as mp_digit, 0x4fdi32 as mp_digit, 0x4ffi32 as mp_digit,
     0x503i32 as mp_digit, 0x509i32 as mp_digit, 0x50bi32 as mp_digit,
     0x511i32 as mp_digit, 0x515i32 as mp_digit, 0x517i32 as mp_digit,
     0x51bi32 as mp_digit, 0x527i32 as mp_digit, 0x529i32 as mp_digit,
     0x52fi32 as mp_digit, 0x551i32 as mp_digit, 0x557i32 as mp_digit,
     0x55di32 as mp_digit, 0x565i32 as mp_digit, 0x577i32 as mp_digit,
     0x581i32 as mp_digit, 0x58fi32 as mp_digit, 0x593i32 as mp_digit,
     0x595i32 as mp_digit, 0x599i32 as mp_digit, 0x59fi32 as mp_digit,
     0x5a7i32 as mp_digit, 0x5abi32 as mp_digit, 0x5adi32 as mp_digit,
     0x5b3i32 as mp_digit, 0x5bfi32 as mp_digit, 0x5c9i32 as mp_digit,
     0x5cbi32 as mp_digit, 0x5cfi32 as mp_digit, 0x5d1i32 as mp_digit,
     0x5d5i32 as mp_digit, 0x5dbi32 as mp_digit, 0x5e7i32 as mp_digit,
     0x5f3i32 as mp_digit, 0x5fbi32 as mp_digit, 0x607i32 as mp_digit,
     0x60di32 as mp_digit, 0x611i32 as mp_digit, 0x617i32 as mp_digit,
     0x61fi32 as mp_digit, 0x623i32 as mp_digit, 0x62bi32 as mp_digit,
     0x62fi32 as mp_digit, 0x63di32 as mp_digit, 0x641i32 as mp_digit,
     0x647i32 as mp_digit, 0x649i32 as mp_digit, 0x64di32 as mp_digit,
     0x653i32 as mp_digit, 0x655i32 as mp_digit, 0x65bi32 as mp_digit,
     0x665i32 as mp_digit, 0x679i32 as mp_digit, 0x67fi32 as mp_digit,
     0x683i32 as mp_digit, 0x685i32 as mp_digit, 0x69di32 as mp_digit,
     0x6a1i32 as mp_digit, 0x6a3i32 as mp_digit, 0x6adi32 as mp_digit,
     0x6b9i32 as mp_digit, 0x6bbi32 as mp_digit, 0x6c5i32 as mp_digit,
     0x6cdi32 as mp_digit, 0x6d3i32 as mp_digit, 0x6d9i32 as mp_digit,
     0x6dfi32 as mp_digit, 0x6f1i32 as mp_digit, 0x6f7i32 as mp_digit,
     0x6fbi32 as mp_digit, 0x6fdi32 as mp_digit, 0x709i32 as mp_digit,
     0x713i32 as mp_digit, 0x71fi32 as mp_digit, 0x727i32 as mp_digit,
     0x737i32 as mp_digit, 0x745i32 as mp_digit, 0x74bi32 as mp_digit,
     0x74fi32 as mp_digit, 0x751i32 as mp_digit, 0x755i32 as mp_digit,
     0x757i32 as mp_digit, 0x761i32 as mp_digit, 0x76di32 as mp_digit,
     0x773i32 as mp_digit, 0x779i32 as mp_digit, 0x78bi32 as mp_digit,
     0x78di32 as mp_digit, 0x79di32 as mp_digit, 0x79fi32 as mp_digit,
     0x7b5i32 as mp_digit, 0x7bbi32 as mp_digit, 0x7c3i32 as mp_digit,
     0x7c9i32 as mp_digit, 0x7cdi32 as mp_digit, 0x7cfi32 as mp_digit,
     0x7d3i32 as mp_digit, 0x7dbi32 as mp_digit, 0x7e1i32 as mp_digit,
     0x7ebi32 as mp_digit, 0x7edi32 as mp_digit, 0x7f7i32 as mp_digit,
     0x805i32 as mp_digit, 0x80fi32 as mp_digit, 0x815i32 as mp_digit,
     0x821i32 as mp_digit, 0x823i32 as mp_digit, 0x827i32 as mp_digit,
     0x829i32 as mp_digit, 0x833i32 as mp_digit, 0x83fi32 as mp_digit,
     0x841i32 as mp_digit, 0x851i32 as mp_digit, 0x853i32 as mp_digit,
     0x859i32 as mp_digit, 0x85di32 as mp_digit, 0x85fi32 as mp_digit,
     0x869i32 as mp_digit, 0x871i32 as mp_digit, 0x883i32 as mp_digit,
     0x89bi32 as mp_digit, 0x89fi32 as mp_digit, 0x8a5i32 as mp_digit,
     0x8adi32 as mp_digit, 0x8bdi32 as mp_digit, 0x8bfi32 as mp_digit,
     0x8c3i32 as mp_digit, 0x8cbi32 as mp_digit, 0x8dbi32 as mp_digit,
     0x8ddi32 as mp_digit, 0x8e1i32 as mp_digit, 0x8e9i32 as mp_digit,
     0x8efi32 as mp_digit, 0x8f5i32 as mp_digit, 0x8f9i32 as mp_digit,
     0x905i32 as mp_digit, 0x907i32 as mp_digit, 0x91di32 as mp_digit,
     0x923i32 as mp_digit, 0x925i32 as mp_digit, 0x92bi32 as mp_digit,
     0x92fi32 as mp_digit, 0x935i32 as mp_digit, 0x943i32 as mp_digit,
     0x949i32 as mp_digit, 0x94di32 as mp_digit, 0x94fi32 as mp_digit,
     0x955i32 as mp_digit, 0x959i32 as mp_digit, 0x95fi32 as mp_digit,
     0x96bi32 as mp_digit, 0x971i32 as mp_digit, 0x977i32 as mp_digit,
     0x985i32 as mp_digit, 0x989i32 as mp_digit, 0x98fi32 as mp_digit,
     0x99bi32 as mp_digit, 0x9a3i32 as mp_digit, 0x9a9i32 as mp_digit,
     0x9adi32 as mp_digit, 0x9c7i32 as mp_digit, 0x9d9i32 as mp_digit,
     0x9e3i32 as mp_digit, 0x9ebi32 as mp_digit, 0x9efi32 as mp_digit,
     0x9f5i32 as mp_digit, 0x9f7i32 as mp_digit, 0x9fdi32 as mp_digit,
     0xa13i32 as mp_digit, 0xa1fi32 as mp_digit, 0xa21i32 as mp_digit,
     0xa31i32 as mp_digit, 0xa39i32 as mp_digit, 0xa3di32 as mp_digit,
     0xa49i32 as mp_digit, 0xa57i32 as mp_digit, 0xa61i32 as mp_digit,
     0xa63i32 as mp_digit, 0xa67i32 as mp_digit, 0xa6fi32 as mp_digit,
     0xa75i32 as mp_digit, 0xa7bi32 as mp_digit, 0xa7fi32 as mp_digit,
     0xa81i32 as mp_digit, 0xa85i32 as mp_digit, 0xa8bi32 as mp_digit,
     0xa93i32 as mp_digit, 0xa97i32 as mp_digit, 0xa99i32 as mp_digit,
     0xa9fi32 as mp_digit, 0xaa9i32 as mp_digit, 0xaabi32 as mp_digit,
     0xab5i32 as mp_digit, 0xabdi32 as mp_digit, 0xac1i32 as mp_digit,
     0xacfi32 as mp_digit, 0xad9i32 as mp_digit, 0xae5i32 as mp_digit,
     0xae7i32 as mp_digit, 0xaedi32 as mp_digit, 0xaf1i32 as mp_digit,
     0xaf3i32 as mp_digit, 0xb03i32 as mp_digit, 0xb11i32 as mp_digit,
     0xb15i32 as mp_digit, 0xb1bi32 as mp_digit, 0xb23i32 as mp_digit,
     0xb29i32 as mp_digit, 0xb2di32 as mp_digit, 0xb3fi32 as mp_digit,
     0xb47i32 as mp_digit, 0xb51i32 as mp_digit, 0xb57i32 as mp_digit,
     0xb5di32 as mp_digit, 0xb65i32 as mp_digit, 0xb6fi32 as mp_digit,
     0xb7bi32 as mp_digit, 0xb89i32 as mp_digit, 0xb8di32 as mp_digit,
     0xb93i32 as mp_digit, 0xb99i32 as mp_digit, 0xb9bi32 as mp_digit,
     0xbb7i32 as mp_digit, 0xbb9i32 as mp_digit, 0xbc3i32 as mp_digit,
     0xbcbi32 as mp_digit, 0xbcfi32 as mp_digit, 0xbddi32 as mp_digit,
     0xbe1i32 as mp_digit, 0xbe9i32 as mp_digit, 0xbf5i32 as mp_digit,
     0xbfbi32 as mp_digit, 0xc07i32 as mp_digit, 0xc0bi32 as mp_digit,
     0xc11i32 as mp_digit, 0xc25i32 as mp_digit, 0xc2fi32 as mp_digit,
     0xc31i32 as mp_digit, 0xc41i32 as mp_digit, 0xc5bi32 as mp_digit,
     0xc5fi32 as mp_digit, 0xc61i32 as mp_digit, 0xc6di32 as mp_digit,
     0xc73i32 as mp_digit, 0xc77i32 as mp_digit, 0xc83i32 as mp_digit,
     0xc89i32 as mp_digit, 0xc91i32 as mp_digit, 0xc95i32 as mp_digit,
     0xc9di32 as mp_digit, 0xcb3i32 as mp_digit, 0xcb5i32 as mp_digit,
     0xcb9i32 as mp_digit, 0xcbbi32 as mp_digit, 0xcc7i32 as mp_digit,
     0xce3i32 as mp_digit, 0xce5i32 as mp_digit, 0xcebi32 as mp_digit,
     0xcf1i32 as mp_digit, 0xcf7i32 as mp_digit, 0xcfbi32 as mp_digit,
     0xd01i32 as mp_digit, 0xd03i32 as mp_digit, 0xd0fi32 as mp_digit,
     0xd13i32 as mp_digit, 0xd1fi32 as mp_digit, 0xd21i32 as mp_digit,
     0xd2bi32 as mp_digit, 0xd2di32 as mp_digit, 0xd3di32 as mp_digit,
     0xd3fi32 as mp_digit, 0xd4fi32 as mp_digit, 0xd55i32 as mp_digit,
     0xd69i32 as mp_digit, 0xd79i32 as mp_digit, 0xd81i32 as mp_digit,
     0xd85i32 as mp_digit, 0xd87i32 as mp_digit, 0xd8bi32 as mp_digit,
     0xd8di32 as mp_digit, 0xda3i32 as mp_digit, 0xdabi32 as mp_digit,
     0xdb7i32 as mp_digit, 0xdbdi32 as mp_digit, 0xdc7i32 as mp_digit,
     0xdc9i32 as mp_digit, 0xdcdi32 as mp_digit, 0xdd3i32 as mp_digit,
     0xdd5i32 as mp_digit, 0xddbi32 as mp_digit, 0xde5i32 as mp_digit,
     0xde7i32 as mp_digit, 0xdf3i32 as mp_digit, 0xdfdi32 as mp_digit,
     0xdffi32 as mp_digit, 0xe09i32 as mp_digit, 0xe17i32 as mp_digit,
     0xe1di32 as mp_digit, 0xe21i32 as mp_digit, 0xe27i32 as mp_digit,
     0xe2fi32 as mp_digit, 0xe35i32 as mp_digit, 0xe3bi32 as mp_digit,
     0xe4bi32 as mp_digit, 0xe57i32 as mp_digit, 0xe59i32 as mp_digit,
     0xe5di32 as mp_digit, 0xe6bi32 as mp_digit, 0xe71i32 as mp_digit,
     0xe75i32 as mp_digit, 0xe7di32 as mp_digit, 0xe87i32 as mp_digit,
     0xe8fi32 as mp_digit, 0xe95i32 as mp_digit, 0xe9bi32 as mp_digit,
     0xeb1i32 as mp_digit, 0xeb7i32 as mp_digit, 0xeb9i32 as mp_digit,
     0xec3i32 as mp_digit, 0xed1i32 as mp_digit, 0xed5i32 as mp_digit,
     0xedbi32 as mp_digit, 0xeedi32 as mp_digit, 0xeefi32 as mp_digit,
     0xef9i32 as mp_digit, 0xf07i32 as mp_digit, 0xf0bi32 as mp_digit,
     0xf0di32 as mp_digit, 0xf17i32 as mp_digit, 0xf25i32 as mp_digit,
     0xf29i32 as mp_digit, 0xf31i32 as mp_digit, 0xf43i32 as mp_digit,
     0xf47i32 as mp_digit, 0xf4di32 as mp_digit, 0xf4fi32 as mp_digit,
     0xf53i32 as mp_digit, 0xf59i32 as mp_digit, 0xf5bi32 as mp_digit,
     0xf67i32 as mp_digit, 0xf6bi32 as mp_digit, 0xf7fi32 as mp_digit,
     0xf95i32 as mp_digit, 0xfa1i32 as mp_digit, 0xfa3i32 as mp_digit,
     0xfa7i32 as mp_digit, 0xfadi32 as mp_digit, 0xfb3i32 as mp_digit,
     0xfb5i32 as mp_digit, 0xfbbi32 as mp_digit, 0xfd1i32 as mp_digit,
     0xfd3i32 as mp_digit, 0xfd9i32 as mp_digit, 0xfe9i32 as mp_digit,
     0xfefi32 as mp_digit, 0xffbi32 as mp_digit, 0xffdi32 as mp_digit,
     0x1003i32 as mp_digit, 0x100fi32 as mp_digit, 0x101fi32 as mp_digit,
     0x1021i32 as mp_digit, 0x1025i32 as mp_digit, 0x102bi32 as mp_digit,
     0x1039i32 as mp_digit, 0x103di32 as mp_digit, 0x103fi32 as mp_digit,
     0x1051i32 as mp_digit, 0x1069i32 as mp_digit, 0x1073i32 as mp_digit,
     0x1079i32 as mp_digit, 0x107bi32 as mp_digit, 0x1085i32 as mp_digit,
     0x1087i32 as mp_digit, 0x1091i32 as mp_digit, 0x1093i32 as mp_digit,
     0x109di32 as mp_digit, 0x10a3i32 as mp_digit, 0x10a5i32 as mp_digit,
     0x10afi32 as mp_digit, 0x10b1i32 as mp_digit, 0x10bbi32 as mp_digit,
     0x10c1i32 as mp_digit, 0x10c9i32 as mp_digit, 0x10e7i32 as mp_digit,
     0x10f1i32 as mp_digit, 0x10f3i32 as mp_digit, 0x10fdi32 as mp_digit,
     0x1105i32 as mp_digit, 0x110bi32 as mp_digit, 0x1115i32 as mp_digit,
     0x1127i32 as mp_digit, 0x112di32 as mp_digit, 0x1139i32 as mp_digit,
     0x1145i32 as mp_digit, 0x1147i32 as mp_digit, 0x1159i32 as mp_digit,
     0x115fi32 as mp_digit, 0x1163i32 as mp_digit, 0x1169i32 as mp_digit,
     0x116fi32 as mp_digit, 0x1181i32 as mp_digit, 0x1183i32 as mp_digit,
     0x118di32 as mp_digit, 0x119bi32 as mp_digit, 0x11a1i32 as mp_digit,
     0x11a5i32 as mp_digit, 0x11a7i32 as mp_digit, 0x11abi32 as mp_digit,
     0x11c3i32 as mp_digit, 0x11c5i32 as mp_digit, 0x11d1i32 as mp_digit,
     0x11d7i32 as mp_digit, 0x11e7i32 as mp_digit, 0x11efi32 as mp_digit,
     0x11f5i32 as mp_digit, 0x11fbi32 as mp_digit, 0x120di32 as mp_digit,
     0x121di32 as mp_digit, 0x121fi32 as mp_digit, 0x1223i32 as mp_digit,
     0x1229i32 as mp_digit, 0x122bi32 as mp_digit, 0x1231i32 as mp_digit,
     0x1237i32 as mp_digit, 0x1241i32 as mp_digit, 0x1247i32 as mp_digit,
     0x1253i32 as mp_digit, 0x125fi32 as mp_digit, 0x1271i32 as mp_digit,
     0x1273i32 as mp_digit, 0x1279i32 as mp_digit, 0x127di32 as mp_digit,
     0x128fi32 as mp_digit, 0x1297i32 as mp_digit, 0x12afi32 as mp_digit,
     0x12b3i32 as mp_digit, 0x12b5i32 as mp_digit, 0x12b9i32 as mp_digit,
     0x12bfi32 as mp_digit, 0x12c1i32 as mp_digit, 0x12cdi32 as mp_digit,
     0x12d1i32 as mp_digit, 0x12dfi32 as mp_digit, 0x12fdi32 as mp_digit,
     0x1307i32 as mp_digit, 0x130di32 as mp_digit, 0x1319i32 as mp_digit,
     0x1327i32 as mp_digit, 0x132di32 as mp_digit, 0x1337i32 as mp_digit,
     0x1343i32 as mp_digit, 0x1345i32 as mp_digit, 0x1349i32 as mp_digit,
     0x134fi32 as mp_digit, 0x1357i32 as mp_digit, 0x135di32 as mp_digit,
     0x1367i32 as mp_digit, 0x1369i32 as mp_digit, 0x136di32 as mp_digit,
     0x137bi32 as mp_digit, 0x1381i32 as mp_digit, 0x1387i32 as mp_digit,
     0x138bi32 as mp_digit, 0x1391i32 as mp_digit, 0x1393i32 as mp_digit,
     0x139di32 as mp_digit, 0x139fi32 as mp_digit, 0x13afi32 as mp_digit,
     0x13bbi32 as mp_digit, 0x13c3i32 as mp_digit, 0x13d5i32 as mp_digit,
     0x13d9i32 as mp_digit, 0x13dfi32 as mp_digit, 0x13ebi32 as mp_digit,
     0x13edi32 as mp_digit, 0x13f3i32 as mp_digit, 0x13f9i32 as mp_digit,
     0x13ffi32 as mp_digit, 0x141bi32 as mp_digit, 0x1421i32 as mp_digit,
     0x142fi32 as mp_digit, 0x1433i32 as mp_digit, 0x143bi32 as mp_digit,
     0x1445i32 as mp_digit, 0x144di32 as mp_digit, 0x1459i32 as mp_digit,
     0x146bi32 as mp_digit, 0x146fi32 as mp_digit, 0x1471i32 as mp_digit,
     0x1475i32 as mp_digit, 0x148di32 as mp_digit, 0x1499i32 as mp_digit,
     0x149fi32 as mp_digit, 0x14a1i32 as mp_digit, 0x14b1i32 as mp_digit,
     0x14b7i32 as mp_digit, 0x14bdi32 as mp_digit, 0x14cbi32 as mp_digit,
     0x14d5i32 as mp_digit, 0x14e3i32 as mp_digit, 0x14e7i32 as mp_digit,
     0x1505i32 as mp_digit, 0x150bi32 as mp_digit, 0x1511i32 as mp_digit,
     0x1517i32 as mp_digit, 0x151fi32 as mp_digit, 0x1525i32 as mp_digit,
     0x1529i32 as mp_digit, 0x152bi32 as mp_digit, 0x1537i32 as mp_digit,
     0x153di32 as mp_digit, 0x1541i32 as mp_digit, 0x1543i32 as mp_digit,
     0x1549i32 as mp_digit, 0x155fi32 as mp_digit, 0x1565i32 as mp_digit,
     0x1567i32 as mp_digit, 0x156bi32 as mp_digit, 0x157di32 as mp_digit,
     0x157fi32 as mp_digit, 0x1583i32 as mp_digit, 0x158fi32 as mp_digit,
     0x1591i32 as mp_digit, 0x1597i32 as mp_digit, 0x159bi32 as mp_digit,
     0x15b5i32 as mp_digit, 0x15bbi32 as mp_digit, 0x15c1i32 as mp_digit,
     0x15c5i32 as mp_digit, 0x15cdi32 as mp_digit, 0x15d7i32 as mp_digit,
     0x15f7i32 as mp_digit, 0x1607i32 as mp_digit, 0x1609i32 as mp_digit,
     0x160fi32 as mp_digit, 0x1613i32 as mp_digit, 0x1615i32 as mp_digit,
     0x1619i32 as mp_digit, 0x161bi32 as mp_digit, 0x1625i32 as mp_digit,
     0x1633i32 as mp_digit, 0x1639i32 as mp_digit, 0x163di32 as mp_digit,
     0x1645i32 as mp_digit, 0x164fi32 as mp_digit, 0x1655i32 as mp_digit,
     0x1669i32 as mp_digit, 0x166di32 as mp_digit, 0x166fi32 as mp_digit,
     0x1675i32 as mp_digit, 0x1693i32 as mp_digit, 0x1697i32 as mp_digit,
     0x169fi32 as mp_digit, 0x16a9i32 as mp_digit, 0x16afi32 as mp_digit,
     0x16b5i32 as mp_digit, 0x16bdi32 as mp_digit, 0x16c3i32 as mp_digit,
     0x16cfi32 as mp_digit, 0x16d3i32 as mp_digit, 0x16d9i32 as mp_digit,
     0x16dbi32 as mp_digit, 0x16e1i32 as mp_digit, 0x16e5i32 as mp_digit,
     0x16ebi32 as mp_digit, 0x16edi32 as mp_digit, 0x16f7i32 as mp_digit,
     0x16f9i32 as mp_digit, 0x1709i32 as mp_digit, 0x170fi32 as mp_digit,
     0x1723i32 as mp_digit, 0x1727i32 as mp_digit, 0x1733i32 as mp_digit,
     0x1741i32 as mp_digit, 0x175di32 as mp_digit, 0x1763i32 as mp_digit,
     0x1777i32 as mp_digit, 0x177bi32 as mp_digit, 0x178di32 as mp_digit,
     0x1795i32 as mp_digit, 0x179bi32 as mp_digit, 0x179fi32 as mp_digit,
     0x17a5i32 as mp_digit, 0x17b3i32 as mp_digit, 0x17b9i32 as mp_digit,
     0x17bfi32 as mp_digit, 0x17c9i32 as mp_digit, 0x17cbi32 as mp_digit,
     0x17d5i32 as mp_digit, 0x17e1i32 as mp_digit, 0x17e9i32 as mp_digit,
     0x17f3i32 as mp_digit, 0x17f5i32 as mp_digit, 0x17ffi32 as mp_digit,
     0x1807i32 as mp_digit, 0x1813i32 as mp_digit, 0x181di32 as mp_digit,
     0x1835i32 as mp_digit, 0x1837i32 as mp_digit, 0x183bi32 as mp_digit,
     0x1843i32 as mp_digit, 0x1849i32 as mp_digit, 0x184di32 as mp_digit,
     0x1855i32 as mp_digit, 0x1867i32 as mp_digit, 0x1871i32 as mp_digit,
     0x1877i32 as mp_digit, 0x187di32 as mp_digit, 0x187fi32 as mp_digit,
     0x1885i32 as mp_digit, 0x188fi32 as mp_digit, 0x189bi32 as mp_digit,
     0x189di32 as mp_digit, 0x18a7i32 as mp_digit, 0x18adi32 as mp_digit,
     0x18b3i32 as mp_digit, 0x18b9i32 as mp_digit, 0x18c1i32 as mp_digit,
     0x18c7i32 as mp_digit, 0x18d1i32 as mp_digit, 0x18d7i32 as mp_digit,
     0x18d9i32 as mp_digit, 0x18dfi32 as mp_digit, 0x18e5i32 as mp_digit,
     0x18ebi32 as mp_digit, 0x18f5i32 as mp_digit, 0x18fdi32 as mp_digit,
     0x1915i32 as mp_digit, 0x191bi32 as mp_digit, 0x1931i32 as mp_digit,
     0x1933i32 as mp_digit, 0x1945i32 as mp_digit, 0x1949i32 as mp_digit,
     0x1951i32 as mp_digit, 0x195bi32 as mp_digit, 0x1979i32 as mp_digit,
     0x1981i32 as mp_digit, 0x1993i32 as mp_digit, 0x1997i32 as mp_digit,
     0x1999i32 as mp_digit, 0x19a3i32 as mp_digit, 0x19a9i32 as mp_digit,
     0x19abi32 as mp_digit, 0x19b1i32 as mp_digit, 0x19b5i32 as mp_digit,
     0x19c7i32 as mp_digit, 0x19cfi32 as mp_digit, 0x19dbi32 as mp_digit,
     0x19edi32 as mp_digit, 0x19fdi32 as mp_digit, 0x1a03i32 as mp_digit,
     0x1a05i32 as mp_digit, 0x1a11i32 as mp_digit, 0x1a17i32 as mp_digit,
     0x1a21i32 as mp_digit, 0x1a23i32 as mp_digit, 0x1a2di32 as mp_digit,
     0x1a2fi32 as mp_digit, 0x1a35i32 as mp_digit, 0x1a3fi32 as mp_digit,
     0x1a4di32 as mp_digit, 0x1a51i32 as mp_digit, 0x1a69i32 as mp_digit,
     0x1a6bi32 as mp_digit, 0x1a7bi32 as mp_digit, 0x1a7di32 as mp_digit,
     0x1a87i32 as mp_digit, 0x1a89i32 as mp_digit, 0x1a93i32 as mp_digit,
     0x1aa7i32 as mp_digit, 0x1aabi32 as mp_digit, 0x1aadi32 as mp_digit,
     0x1ab1i32 as mp_digit, 0x1ab9i32 as mp_digit, 0x1ac9i32 as mp_digit,
     0x1acfi32 as mp_digit, 0x1ad5i32 as mp_digit, 0x1ad7i32 as mp_digit,
     0x1ae3i32 as mp_digit, 0x1af3i32 as mp_digit, 0x1afbi32 as mp_digit,
     0x1affi32 as mp_digit, 0x1b05i32 as mp_digit, 0x1b23i32 as mp_digit,
     0x1b25i32 as mp_digit, 0x1b2fi32 as mp_digit, 0x1b31i32 as mp_digit,
     0x1b37i32 as mp_digit, 0x1b3bi32 as mp_digit, 0x1b41i32 as mp_digit,
     0x1b47i32 as mp_digit, 0x1b4fi32 as mp_digit, 0x1b55i32 as mp_digit,
     0x1b59i32 as mp_digit, 0x1b65i32 as mp_digit, 0x1b6bi32 as mp_digit,
     0x1b73i32 as mp_digit, 0x1b7fi32 as mp_digit, 0x1b83i32 as mp_digit,
     0x1b91i32 as mp_digit, 0x1b9di32 as mp_digit, 0x1ba7i32 as mp_digit,
     0x1bbfi32 as mp_digit, 0x1bc5i32 as mp_digit, 0x1bd1i32 as mp_digit,
     0x1bd7i32 as mp_digit, 0x1bd9i32 as mp_digit, 0x1befi32 as mp_digit,
     0x1bf7i32 as mp_digit, 0x1c09i32 as mp_digit, 0x1c13i32 as mp_digit,
     0x1c19i32 as mp_digit, 0x1c27i32 as mp_digit, 0x1c2bi32 as mp_digit,
     0x1c2di32 as mp_digit, 0x1c33i32 as mp_digit, 0x1c3di32 as mp_digit,
     0x1c45i32 as mp_digit, 0x1c4bi32 as mp_digit, 0x1c4fi32 as mp_digit,
     0x1c55i32 as mp_digit, 0x1c73i32 as mp_digit, 0x1c81i32 as mp_digit,
     0x1c8bi32 as mp_digit, 0x1c8di32 as mp_digit, 0x1c99i32 as mp_digit,
     0x1ca3i32 as mp_digit, 0x1ca5i32 as mp_digit, 0x1cb5i32 as mp_digit,
     0x1cb7i32 as mp_digit, 0x1cc9i32 as mp_digit, 0x1ce1i32 as mp_digit,
     0x1cf3i32 as mp_digit, 0x1cf9i32 as mp_digit, 0x1d09i32 as mp_digit,
     0x1d1bi32 as mp_digit, 0x1d21i32 as mp_digit, 0x1d23i32 as mp_digit,
     0x1d35i32 as mp_digit, 0x1d39i32 as mp_digit, 0x1d3fi32 as mp_digit,
     0x1d41i32 as mp_digit, 0x1d4bi32 as mp_digit, 0x1d53i32 as mp_digit,
     0x1d5di32 as mp_digit, 0x1d63i32 as mp_digit, 0x1d69i32 as mp_digit,
     0x1d71i32 as mp_digit, 0x1d75i32 as mp_digit, 0x1d7bi32 as mp_digit,
     0x1d7di32 as mp_digit, 0x1d87i32 as mp_digit, 0x1d89i32 as mp_digit,
     0x1d95i32 as mp_digit, 0x1d99i32 as mp_digit, 0x1d9fi32 as mp_digit,
     0x1da5i32 as mp_digit, 0x1da7i32 as mp_digit, 0x1db3i32 as mp_digit,
     0x1db7i32 as mp_digit, 0x1dc5i32 as mp_digit, 0x1dd7i32 as mp_digit,
     0x1ddbi32 as mp_digit, 0x1de1i32 as mp_digit, 0x1df5i32 as mp_digit,
     0x1df9i32 as mp_digit, 0x1e01i32 as mp_digit, 0x1e07i32 as mp_digit,
     0x1e0bi32 as mp_digit, 0x1e13i32 as mp_digit, 0x1e17i32 as mp_digit,
     0x1e25i32 as mp_digit, 0x1e2bi32 as mp_digit, 0x1e2fi32 as mp_digit,
     0x1e3di32 as mp_digit, 0x1e49i32 as mp_digit, 0x1e4di32 as mp_digit,
     0x1e4fi32 as mp_digit, 0x1e6di32 as mp_digit, 0x1e71i32 as mp_digit,
     0x1e89i32 as mp_digit, 0x1e8fi32 as mp_digit, 0x1e95i32 as mp_digit,
     0x1ea1i32 as mp_digit, 0x1eadi32 as mp_digit, 0x1ebbi32 as mp_digit,
     0x1ec1i32 as mp_digit, 0x1ec5i32 as mp_digit, 0x1ec7i32 as mp_digit,
     0x1ecbi32 as mp_digit, 0x1eddi32 as mp_digit, 0x1ee3i32 as mp_digit,
     0x1eefi32 as mp_digit, 0x1ef7i32 as mp_digit, 0x1efdi32 as mp_digit,
     0x1f01i32 as mp_digit, 0x1f0di32 as mp_digit, 0x1f0fi32 as mp_digit,
     0x1f1bi32 as mp_digit, 0x1f39i32 as mp_digit, 0x1f49i32 as mp_digit,
     0x1f4bi32 as mp_digit, 0x1f51i32 as mp_digit, 0x1f67i32 as mp_digit,
     0x1f75i32 as mp_digit, 0x1f7bi32 as mp_digit, 0x1f85i32 as mp_digit,
     0x1f91i32 as mp_digit, 0x1f97i32 as mp_digit, 0x1f99i32 as mp_digit,
     0x1f9di32 as mp_digit, 0x1fa5i32 as mp_digit, 0x1fafi32 as mp_digit,
     0x1fb5i32 as mp_digit, 0x1fbbi32 as mp_digit, 0x1fd3i32 as mp_digit,
     0x1fe1i32 as mp_digit, 0x1fe7i32 as mp_digit, 0x1febi32 as mp_digit,
     0x1ff3i32 as mp_digit, 0x1fffi32 as mp_digit, 0x2011i32 as mp_digit,
     0x201bi32 as mp_digit, 0x201di32 as mp_digit, 0x2027i32 as mp_digit,
     0x2029i32 as mp_digit, 0x202di32 as mp_digit, 0x2033i32 as mp_digit,
     0x2047i32 as mp_digit, 0x204di32 as mp_digit, 0x2051i32 as mp_digit,
     0x205fi32 as mp_digit, 0x2063i32 as mp_digit, 0x2065i32 as mp_digit,
     0x2069i32 as mp_digit, 0x2077i32 as mp_digit, 0x207di32 as mp_digit,
     0x2089i32 as mp_digit, 0x20a1i32 as mp_digit, 0x20abi32 as mp_digit,
     0x20b1i32 as mp_digit, 0x20b9i32 as mp_digit, 0x20c3i32 as mp_digit,
     0x20c5i32 as mp_digit, 0x20e3i32 as mp_digit, 0x20e7i32 as mp_digit,
     0x20edi32 as mp_digit, 0x20efi32 as mp_digit, 0x20fbi32 as mp_digit,
     0x20ffi32 as mp_digit, 0x210di32 as mp_digit, 0x2113i32 as mp_digit,
     0x2135i32 as mp_digit, 0x2141i32 as mp_digit, 0x2149i32 as mp_digit,
     0x214fi32 as mp_digit, 0x2159i32 as mp_digit, 0x215bi32 as mp_digit,
     0x215fi32 as mp_digit, 0x2173i32 as mp_digit, 0x217di32 as mp_digit,
     0x2185i32 as mp_digit, 0x2195i32 as mp_digit, 0x2197i32 as mp_digit,
     0x21a1i32 as mp_digit, 0x21afi32 as mp_digit, 0x21b3i32 as mp_digit,
     0x21b5i32 as mp_digit, 0x21c1i32 as mp_digit, 0x21c7i32 as mp_digit,
     0x21d7i32 as mp_digit, 0x21ddi32 as mp_digit, 0x21e5i32 as mp_digit,
     0x21e9i32 as mp_digit, 0x21f1i32 as mp_digit, 0x21f5i32 as mp_digit,
     0x21fbi32 as mp_digit, 0x2203i32 as mp_digit, 0x2209i32 as mp_digit,
     0x220fi32 as mp_digit, 0x221bi32 as mp_digit, 0x2221i32 as mp_digit,
     0x2225i32 as mp_digit, 0x222bi32 as mp_digit, 0x2231i32 as mp_digit,
     0x2239i32 as mp_digit, 0x224bi32 as mp_digit, 0x224fi32 as mp_digit,
     0x2263i32 as mp_digit, 0x2267i32 as mp_digit, 0x2273i32 as mp_digit,
     0x2275i32 as mp_digit, 0x227fi32 as mp_digit, 0x2285i32 as mp_digit,
     0x2287i32 as mp_digit, 0x2291i32 as mp_digit, 0x229di32 as mp_digit,
     0x229fi32 as mp_digit, 0x22a3i32 as mp_digit, 0x22b7i32 as mp_digit,
     0x22bdi32 as mp_digit, 0x22dbi32 as mp_digit, 0x22e1i32 as mp_digit,
     0x22e5i32 as mp_digit, 0x22edi32 as mp_digit, 0x22f7i32 as mp_digit,
     0x2303i32 as mp_digit, 0x2309i32 as mp_digit, 0x230bi32 as mp_digit,
     0x2327i32 as mp_digit, 0x2329i32 as mp_digit, 0x232fi32 as mp_digit,
     0x2333i32 as mp_digit, 0x2335i32 as mp_digit, 0x2345i32 as mp_digit,
     0x2351i32 as mp_digit, 0x2353i32 as mp_digit, 0x2359i32 as mp_digit,
     0x2363i32 as mp_digit, 0x236bi32 as mp_digit, 0x2383i32 as mp_digit,
     0x238fi32 as mp_digit, 0x2395i32 as mp_digit, 0x23a7i32 as mp_digit,
     0x23adi32 as mp_digit, 0x23b1i32 as mp_digit, 0x23bfi32 as mp_digit,
     0x23c5i32 as mp_digit, 0x23c9i32 as mp_digit, 0x23d5i32 as mp_digit,
     0x23ddi32 as mp_digit, 0x23e3i32 as mp_digit, 0x23efi32 as mp_digit,
     0x23f3i32 as mp_digit, 0x23f9i32 as mp_digit, 0x2405i32 as mp_digit,
     0x240bi32 as mp_digit, 0x2417i32 as mp_digit, 0x2419i32 as mp_digit,
     0x2429i32 as mp_digit, 0x243di32 as mp_digit, 0x2441i32 as mp_digit,
     0x2443i32 as mp_digit, 0x244di32 as mp_digit, 0x245fi32 as mp_digit,
     0x2467i32 as mp_digit, 0x246bi32 as mp_digit, 0x2479i32 as mp_digit,
     0x247di32 as mp_digit, 0x247fi32 as mp_digit, 0x2485i32 as mp_digit,
     0x249bi32 as mp_digit, 0x24a1i32 as mp_digit, 0x24afi32 as mp_digit,
     0x24b5i32 as mp_digit, 0x24bbi32 as mp_digit, 0x24c5i32 as mp_digit,
     0x24cbi32 as mp_digit, 0x24cdi32 as mp_digit, 0x24d7i32 as mp_digit,
     0x24d9i32 as mp_digit, 0x24ddi32 as mp_digit, 0x24dfi32 as mp_digit,
     0x24f5i32 as mp_digit, 0x24f7i32 as mp_digit, 0x24fbi32 as mp_digit,
     0x2501i32 as mp_digit, 0x2507i32 as mp_digit, 0x2513i32 as mp_digit,
     0x2519i32 as mp_digit, 0x2527i32 as mp_digit, 0x2531i32 as mp_digit,
     0x253di32 as mp_digit, 0x2543i32 as mp_digit, 0x254bi32 as mp_digit,
     0x254fi32 as mp_digit, 0x2573i32 as mp_digit, 0x2581i32 as mp_digit,
     0x258di32 as mp_digit, 0x2593i32 as mp_digit, 0x2597i32 as mp_digit,
     0x259di32 as mp_digit, 0x259fi32 as mp_digit, 0x25abi32 as mp_digit,
     0x25b1i32 as mp_digit, 0x25bdi32 as mp_digit, 0x25cdi32 as mp_digit,
     0x25cfi32 as mp_digit, 0x25d9i32 as mp_digit, 0x25e1i32 as mp_digit,
     0x25f7i32 as mp_digit, 0x25f9i32 as mp_digit, 0x2605i32 as mp_digit,
     0x260bi32 as mp_digit, 0x260fi32 as mp_digit, 0x2615i32 as mp_digit,
     0x2627i32 as mp_digit, 0x2629i32 as mp_digit, 0x2635i32 as mp_digit,
     0x263bi32 as mp_digit, 0x263fi32 as mp_digit, 0x264bi32 as mp_digit,
     0x2653i32 as mp_digit, 0x2659i32 as mp_digit, 0x2665i32 as mp_digit,
     0x2669i32 as mp_digit, 0x266fi32 as mp_digit, 0x267bi32 as mp_digit,
     0x2681i32 as mp_digit, 0x2683i32 as mp_digit, 0x268fi32 as mp_digit,
     0x269bi32 as mp_digit, 0x269fi32 as mp_digit, 0x26adi32 as mp_digit,
     0x26b3i32 as mp_digit, 0x26c3i32 as mp_digit, 0x26c9i32 as mp_digit,
     0x26cbi32 as mp_digit, 0x26d5i32 as mp_digit, 0x26ddi32 as mp_digit,
     0x26efi32 as mp_digit, 0x26f5i32 as mp_digit, 0x2717i32 as mp_digit,
     0x2719i32 as mp_digit, 0x2735i32 as mp_digit, 0x2737i32 as mp_digit,
     0x274di32 as mp_digit, 0x2753i32 as mp_digit, 0x2755i32 as mp_digit,
     0x275fi32 as mp_digit, 0x276bi32 as mp_digit, 0x276di32 as mp_digit,
     0x2773i32 as mp_digit, 0x2777i32 as mp_digit, 0x277fi32 as mp_digit,
     0x2795i32 as mp_digit, 0x279bi32 as mp_digit, 0x279di32 as mp_digit,
     0x27a7i32 as mp_digit, 0x27afi32 as mp_digit, 0x27b3i32 as mp_digit,
     0x27b9i32 as mp_digit, 0x27c1i32 as mp_digit, 0x27c5i32 as mp_digit,
     0x27d1i32 as mp_digit, 0x27e3i32 as mp_digit, 0x27efi32 as mp_digit,
     0x2803i32 as mp_digit, 0x2807i32 as mp_digit, 0x280di32 as mp_digit,
     0x2813i32 as mp_digit, 0x281bi32 as mp_digit, 0x281fi32 as mp_digit,
     0x2821i32 as mp_digit, 0x2831i32 as mp_digit, 0x283di32 as mp_digit,
     0x283fi32 as mp_digit, 0x2849i32 as mp_digit, 0x2851i32 as mp_digit,
     0x285bi32 as mp_digit, 0x285di32 as mp_digit, 0x2861i32 as mp_digit,
     0x2867i32 as mp_digit, 0x2875i32 as mp_digit, 0x2881i32 as mp_digit,
     0x2897i32 as mp_digit, 0x289fi32 as mp_digit, 0x28bbi32 as mp_digit,
     0x28bdi32 as mp_digit, 0x28c1i32 as mp_digit, 0x28d5i32 as mp_digit,
     0x28d9i32 as mp_digit, 0x28dbi32 as mp_digit, 0x28dfi32 as mp_digit,
     0x28edi32 as mp_digit, 0x28f7i32 as mp_digit, 0x2903i32 as mp_digit,
     0x2905i32 as mp_digit, 0x2911i32 as mp_digit, 0x2921i32 as mp_digit,
     0x2923i32 as mp_digit, 0x293fi32 as mp_digit, 0x2947i32 as mp_digit,
     0x295di32 as mp_digit, 0x2965i32 as mp_digit, 0x2969i32 as mp_digit,
     0x296fi32 as mp_digit, 0x2975i32 as mp_digit, 0x2983i32 as mp_digit,
     0x2987i32 as mp_digit, 0x298fi32 as mp_digit, 0x299bi32 as mp_digit,
     0x29a1i32 as mp_digit, 0x29a7i32 as mp_digit, 0x29abi32 as mp_digit,
     0x29bfi32 as mp_digit, 0x29c3i32 as mp_digit, 0x29d5i32 as mp_digit,
     0x29d7i32 as mp_digit, 0x29e3i32 as mp_digit, 0x29e9i32 as mp_digit,
     0x29edi32 as mp_digit, 0x29f3i32 as mp_digit, 0x2a01i32 as mp_digit,
     0x2a13i32 as mp_digit, 0x2a1di32 as mp_digit, 0x2a25i32 as mp_digit,
     0x2a2fi32 as mp_digit, 0x2a4fi32 as mp_digit, 0x2a55i32 as mp_digit,
     0x2a5fi32 as mp_digit, 0x2a65i32 as mp_digit, 0x2a6bi32 as mp_digit,
     0x2a6di32 as mp_digit, 0x2a73i32 as mp_digit, 0x2a83i32 as mp_digit,
     0x2a89i32 as mp_digit, 0x2a8bi32 as mp_digit, 0x2a97i32 as mp_digit,
     0x2a9di32 as mp_digit, 0x2ab9i32 as mp_digit, 0x2abbi32 as mp_digit,
     0x2ac5i32 as mp_digit, 0x2acdi32 as mp_digit, 0x2addi32 as mp_digit,
     0x2ae3i32 as mp_digit, 0x2aebi32 as mp_digit, 0x2af1i32 as mp_digit,
     0x2afbi32 as mp_digit, 0x2b13i32 as mp_digit, 0x2b27i32 as mp_digit,
     0x2b31i32 as mp_digit, 0x2b33i32 as mp_digit, 0x2b3di32 as mp_digit,
     0x2b3fi32 as mp_digit, 0x2b4bi32 as mp_digit, 0x2b4fi32 as mp_digit,
     0x2b55i32 as mp_digit, 0x2b69i32 as mp_digit, 0x2b6di32 as mp_digit,
     0x2b6fi32 as mp_digit, 0x2b7bi32 as mp_digit, 0x2b8di32 as mp_digit,
     0x2b97i32 as mp_digit, 0x2b99i32 as mp_digit, 0x2ba3i32 as mp_digit,
     0x2ba5i32 as mp_digit, 0x2ba9i32 as mp_digit, 0x2bbdi32 as mp_digit,
     0x2bcdi32 as mp_digit, 0x2be7i32 as mp_digit, 0x2bebi32 as mp_digit,
     0x2bf3i32 as mp_digit, 0x2bf9i32 as mp_digit, 0x2bfdi32 as mp_digit,
     0x2c09i32 as mp_digit, 0x2c0fi32 as mp_digit, 0x2c17i32 as mp_digit,
     0x2c23i32 as mp_digit, 0x2c2fi32 as mp_digit, 0x2c35i32 as mp_digit,
     0x2c39i32 as mp_digit, 0x2c41i32 as mp_digit, 0x2c57i32 as mp_digit,
     0x2c59i32 as mp_digit, 0x2c69i32 as mp_digit, 0x2c77i32 as mp_digit,
     0x2c81i32 as mp_digit, 0x2c87i32 as mp_digit, 0x2c93i32 as mp_digit,
     0x2c9fi32 as mp_digit, 0x2cadi32 as mp_digit, 0x2cb3i32 as mp_digit,
     0x2cb7i32 as mp_digit, 0x2ccbi32 as mp_digit, 0x2ccfi32 as mp_digit,
     0x2cdbi32 as mp_digit, 0x2ce1i32 as mp_digit, 0x2ce3i32 as mp_digit,
     0x2ce9i32 as mp_digit, 0x2cefi32 as mp_digit, 0x2cffi32 as mp_digit,
     0x2d07i32 as mp_digit, 0x2d1di32 as mp_digit, 0x2d1fi32 as mp_digit,
     0x2d3bi32 as mp_digit, 0x2d43i32 as mp_digit, 0x2d49i32 as mp_digit,
     0x2d4di32 as mp_digit, 0x2d61i32 as mp_digit, 0x2d65i32 as mp_digit,
     0x2d71i32 as mp_digit, 0x2d89i32 as mp_digit, 0x2d9di32 as mp_digit,
     0x2da1i32 as mp_digit, 0x2da9i32 as mp_digit, 0x2db3i32 as mp_digit,
     0x2db5i32 as mp_digit, 0x2dc5i32 as mp_digit, 0x2dc7i32 as mp_digit,
     0x2dd3i32 as mp_digit, 0x2ddfi32 as mp_digit, 0x2e01i32 as mp_digit,
     0x2e03i32 as mp_digit, 0x2e07i32 as mp_digit, 0x2e0di32 as mp_digit,
     0x2e19i32 as mp_digit, 0x2e1fi32 as mp_digit, 0x2e25i32 as mp_digit,
     0x2e2di32 as mp_digit, 0x2e33i32 as mp_digit, 0x2e37i32 as mp_digit,
     0x2e39i32 as mp_digit, 0x2e3fi32 as mp_digit, 0x2e57i32 as mp_digit,
     0x2e5bi32 as mp_digit, 0x2e6fi32 as mp_digit, 0x2e79i32 as mp_digit,
     0x2e7fi32 as mp_digit, 0x2e85i32 as mp_digit, 0x2e93i32 as mp_digit,
     0x2e97i32 as mp_digit, 0x2e9di32 as mp_digit, 0x2ea3i32 as mp_digit,
     0x2ea5i32 as mp_digit, 0x2eb1i32 as mp_digit, 0x2eb7i32 as mp_digit,
     0x2ec1i32 as mp_digit, 0x2ec3i32 as mp_digit, 0x2ecdi32 as mp_digit,
     0x2ed3i32 as mp_digit, 0x2ee7i32 as mp_digit, 0x2eebi32 as mp_digit,
     0x2f05i32 as mp_digit, 0x2f09i32 as mp_digit, 0x2f0bi32 as mp_digit,
     0x2f11i32 as mp_digit, 0x2f27i32 as mp_digit, 0x2f29i32 as mp_digit,
     0x2f41i32 as mp_digit, 0x2f45i32 as mp_digit, 0x2f4bi32 as mp_digit,
     0x2f4di32 as mp_digit, 0x2f51i32 as mp_digit, 0x2f57i32 as mp_digit,
     0x2f6fi32 as mp_digit, 0x2f75i32 as mp_digit, 0x2f7di32 as mp_digit,
     0x2f81i32 as mp_digit, 0x2f83i32 as mp_digit, 0x2fa5i32 as mp_digit,
     0x2fabi32 as mp_digit, 0x2fb3i32 as mp_digit, 0x2fc3i32 as mp_digit,
     0x2fcfi32 as mp_digit, 0x2fd1i32 as mp_digit, 0x2fdbi32 as mp_digit,
     0x2fddi32 as mp_digit, 0x2fe7i32 as mp_digit, 0x2fedi32 as mp_digit,
     0x2ff5i32 as mp_digit, 0x2ff9i32 as mp_digit, 0x3001i32 as mp_digit,
     0x300di32 as mp_digit, 0x3023i32 as mp_digit, 0x3029i32 as mp_digit,
     0x3037i32 as mp_digit, 0x303bi32 as mp_digit, 0x3055i32 as mp_digit,
     0x3059i32 as mp_digit, 0x305bi32 as mp_digit, 0x3067i32 as mp_digit,
     0x3071i32 as mp_digit, 0x3079i32 as mp_digit, 0x307di32 as mp_digit,
     0x3085i32 as mp_digit, 0x3091i32 as mp_digit, 0x3095i32 as mp_digit,
     0x30a3i32 as mp_digit, 0x30a9i32 as mp_digit, 0x30b9i32 as mp_digit,
     0x30bfi32 as mp_digit, 0x30c7i32 as mp_digit, 0x30cbi32 as mp_digit,
     0x30d1i32 as mp_digit, 0x30d7i32 as mp_digit, 0x30dfi32 as mp_digit,
     0x30e5i32 as mp_digit, 0x30efi32 as mp_digit, 0x30fbi32 as mp_digit,
     0x30fdi32 as mp_digit, 0x3103i32 as mp_digit, 0x3109i32 as mp_digit,
     0x3119i32 as mp_digit, 0x3121i32 as mp_digit, 0x3127i32 as mp_digit,
     0x312di32 as mp_digit, 0x3139i32 as mp_digit, 0x3143i32 as mp_digit,
     0x3145i32 as mp_digit, 0x314bi32 as mp_digit, 0x315di32 as mp_digit,
     0x3161i32 as mp_digit, 0x3167i32 as mp_digit, 0x316di32 as mp_digit,
     0x3173i32 as mp_digit, 0x317fi32 as mp_digit, 0x3191i32 as mp_digit,
     0x3199i32 as mp_digit, 0x319fi32 as mp_digit, 0x31a9i32 as mp_digit,
     0x31b1i32 as mp_digit, 0x31c3i32 as mp_digit, 0x31c7i32 as mp_digit,
     0x31d5i32 as mp_digit, 0x31dbi32 as mp_digit, 0x31edi32 as mp_digit,
     0x31f7i32 as mp_digit, 0x31ffi32 as mp_digit, 0x3209i32 as mp_digit,
     0x3215i32 as mp_digit, 0x3217i32 as mp_digit, 0x321di32 as mp_digit,
     0x3229i32 as mp_digit, 0x3235i32 as mp_digit, 0x3259i32 as mp_digit,
     0x325di32 as mp_digit, 0x3263i32 as mp_digit, 0x326bi32 as mp_digit,
     0x326fi32 as mp_digit, 0x3275i32 as mp_digit, 0x3277i32 as mp_digit,
     0x327bi32 as mp_digit, 0x328di32 as mp_digit, 0x3299i32 as mp_digit,
     0x329fi32 as mp_digit, 0x32a7i32 as mp_digit, 0x32adi32 as mp_digit,
     0x32b3i32 as mp_digit, 0x32b7i32 as mp_digit, 0x32c9i32 as mp_digit,
     0x32cbi32 as mp_digit, 0x32cfi32 as mp_digit, 0x32d1i32 as mp_digit,
     0x32e9i32 as mp_digit, 0x32edi32 as mp_digit, 0x32f3i32 as mp_digit,
     0x32f9i32 as mp_digit, 0x3307i32 as mp_digit, 0x3325i32 as mp_digit,
     0x332bi32 as mp_digit, 0x332fi32 as mp_digit, 0x3335i32 as mp_digit,
     0x3341i32 as mp_digit, 0x3347i32 as mp_digit, 0x335bi32 as mp_digit,
     0x335fi32 as mp_digit, 0x3367i32 as mp_digit, 0x336bi32 as mp_digit,
     0x3373i32 as mp_digit, 0x3379i32 as mp_digit, 0x337fi32 as mp_digit,
     0x3383i32 as mp_digit, 0x33a1i32 as mp_digit, 0x33a3i32 as mp_digit,
     0x33adi32 as mp_digit, 0x33b9i32 as mp_digit, 0x33c1i32 as mp_digit,
     0x33cbi32 as mp_digit, 0x33d3i32 as mp_digit, 0x33ebi32 as mp_digit,
     0x33f1i32 as mp_digit, 0x33fdi32 as mp_digit, 0x3401i32 as mp_digit,
     0x340fi32 as mp_digit, 0x3413i32 as mp_digit, 0x3419i32 as mp_digit,
     0x341bi32 as mp_digit, 0x3437i32 as mp_digit, 0x3445i32 as mp_digit,
     0x3455i32 as mp_digit, 0x3457i32 as mp_digit, 0x3463i32 as mp_digit,
     0x3469i32 as mp_digit, 0x346di32 as mp_digit, 0x3481i32 as mp_digit,
     0x348bi32 as mp_digit, 0x3491i32 as mp_digit, 0x3497i32 as mp_digit,
     0x349di32 as mp_digit, 0x34a5i32 as mp_digit, 0x34afi32 as mp_digit,
     0x34bbi32 as mp_digit, 0x34c9i32 as mp_digit, 0x34d3i32 as mp_digit,
     0x34e1i32 as mp_digit, 0x34f1i32 as mp_digit, 0x34ffi32 as mp_digit,
     0x3509i32 as mp_digit, 0x3517i32 as mp_digit, 0x351di32 as mp_digit,
     0x352di32 as mp_digit, 0x3533i32 as mp_digit, 0x353bi32 as mp_digit,
     0x3541i32 as mp_digit, 0x3551i32 as mp_digit, 0x3565i32 as mp_digit,
     0x356fi32 as mp_digit, 0x3571i32 as mp_digit, 0x3577i32 as mp_digit,
     0x357bi32 as mp_digit, 0x357di32 as mp_digit, 0x3581i32 as mp_digit,
     0x358di32 as mp_digit, 0x358fi32 as mp_digit, 0x3599i32 as mp_digit,
     0x359bi32 as mp_digit, 0x35a1i32 as mp_digit, 0x35b7i32 as mp_digit,
     0x35bdi32 as mp_digit, 0x35bfi32 as mp_digit, 0x35c3i32 as mp_digit,
     0x35d5i32 as mp_digit, 0x35ddi32 as mp_digit, 0x35e7i32 as mp_digit,
     0x35efi32 as mp_digit, 0x3605i32 as mp_digit, 0x3607i32 as mp_digit,
     0x3611i32 as mp_digit, 0x3623i32 as mp_digit, 0x3631i32 as mp_digit,
     0x3635i32 as mp_digit, 0x3637i32 as mp_digit, 0x363bi32 as mp_digit,
     0x364di32 as mp_digit, 0x364fi32 as mp_digit, 0x3653i32 as mp_digit,
     0x3659i32 as mp_digit, 0x3661i32 as mp_digit, 0x366bi32 as mp_digit,
     0x366di32 as mp_digit, 0x368bi32 as mp_digit, 0x368fi32 as mp_digit,
     0x36adi32 as mp_digit, 0x36afi32 as mp_digit, 0x36b9i32 as mp_digit,
     0x36bbi32 as mp_digit, 0x36cdi32 as mp_digit, 0x36d1i32 as mp_digit,
     0x36e3i32 as mp_digit, 0x36e9i32 as mp_digit, 0x36f7i32 as mp_digit,
     0x3701i32 as mp_digit, 0x3703i32 as mp_digit, 0x3707i32 as mp_digit,
     0x371bi32 as mp_digit, 0x373fi32 as mp_digit, 0x3745i32 as mp_digit,
     0x3749i32 as mp_digit, 0x374fi32 as mp_digit, 0x375di32 as mp_digit,
     0x3761i32 as mp_digit, 0x3775i32 as mp_digit, 0x377fi32 as mp_digit,
     0x378di32 as mp_digit, 0x37a3i32 as mp_digit, 0x37a9i32 as mp_digit,
     0x37abi32 as mp_digit, 0x37c9i32 as mp_digit, 0x37d5i32 as mp_digit,
     0x37dfi32 as mp_digit, 0x37f1i32 as mp_digit, 0x37f3i32 as mp_digit,
     0x37f7i32 as mp_digit, 0x3805i32 as mp_digit, 0x380bi32 as mp_digit,
     0x3821i32 as mp_digit, 0x3833i32 as mp_digit, 0x3835i32 as mp_digit,
     0x3841i32 as mp_digit, 0x3847i32 as mp_digit, 0x384bi32 as mp_digit,
     0x3853i32 as mp_digit, 0x3857i32 as mp_digit, 0x385fi32 as mp_digit,
     0x3865i32 as mp_digit, 0x386fi32 as mp_digit, 0x3871i32 as mp_digit,
     0x387di32 as mp_digit, 0x388fi32 as mp_digit, 0x3899i32 as mp_digit,
     0x38a7i32 as mp_digit, 0x38b7i32 as mp_digit, 0x38c5i32 as mp_digit,
     0x38c9i32 as mp_digit, 0x38cfi32 as mp_digit, 0x38d5i32 as mp_digit,
     0x38d7i32 as mp_digit, 0x38ddi32 as mp_digit, 0x38e1i32 as mp_digit,
     0x38e3i32 as mp_digit, 0x38ffi32 as mp_digit, 0x3901i32 as mp_digit,
     0x391di32 as mp_digit, 0x3923i32 as mp_digit, 0x3925i32 as mp_digit,
     0x3929i32 as mp_digit, 0x392fi32 as mp_digit, 0x393di32 as mp_digit,
     0x3941i32 as mp_digit, 0x394di32 as mp_digit, 0x395bi32 as mp_digit,
     0x396bi32 as mp_digit, 0x3979i32 as mp_digit, 0x397di32 as mp_digit,
     0x3983i32 as mp_digit, 0x398bi32 as mp_digit, 0x3991i32 as mp_digit,
     0x3995i32 as mp_digit, 0x399bi32 as mp_digit, 0x39a1i32 as mp_digit,
     0x39a7i32 as mp_digit, 0x39afi32 as mp_digit, 0x39b3i32 as mp_digit,
     0x39bbi32 as mp_digit, 0x39bfi32 as mp_digit, 0x39cdi32 as mp_digit,
     0x39ddi32 as mp_digit, 0x39e5i32 as mp_digit, 0x39ebi32 as mp_digit,
     0x39efi32 as mp_digit, 0x39fbi32 as mp_digit, 0x3a03i32 as mp_digit,
     0x3a13i32 as mp_digit, 0x3a15i32 as mp_digit, 0x3a1fi32 as mp_digit,
     0x3a27i32 as mp_digit, 0x3a2bi32 as mp_digit, 0x3a31i32 as mp_digit,
     0x3a4bi32 as mp_digit, 0x3a51i32 as mp_digit, 0x3a5bi32 as mp_digit,
     0x3a63i32 as mp_digit, 0x3a67i32 as mp_digit, 0x3a6di32 as mp_digit,
     0x3a79i32 as mp_digit, 0x3a87i32 as mp_digit, 0x3aa5i32 as mp_digit,
     0x3aa9i32 as mp_digit, 0x3ab7i32 as mp_digit, 0x3acdi32 as mp_digit,
     0x3ad5i32 as mp_digit, 0x3ae1i32 as mp_digit, 0x3ae5i32 as mp_digit,
     0x3aebi32 as mp_digit, 0x3af3i32 as mp_digit, 0x3afdi32 as mp_digit,
     0x3b03i32 as mp_digit, 0x3b11i32 as mp_digit, 0x3b1bi32 as mp_digit,
     0x3b21i32 as mp_digit, 0x3b23i32 as mp_digit, 0x3b2di32 as mp_digit,
     0x3b39i32 as mp_digit, 0x3b45i32 as mp_digit, 0x3b53i32 as mp_digit,
     0x3b59i32 as mp_digit, 0x3b5fi32 as mp_digit, 0x3b71i32 as mp_digit,
     0x3b7bi32 as mp_digit, 0x3b81i32 as mp_digit, 0x3b89i32 as mp_digit,
     0x3b9bi32 as mp_digit, 0x3b9fi32 as mp_digit, 0x3ba5i32 as mp_digit,
     0x3ba7i32 as mp_digit, 0x3badi32 as mp_digit, 0x3bb7i32 as mp_digit,
     0x3bb9i32 as mp_digit, 0x3bc3i32 as mp_digit, 0x3bcbi32 as mp_digit,
     0x3bd1i32 as mp_digit, 0x3bd7i32 as mp_digit, 0x3be1i32 as mp_digit,
     0x3be3i32 as mp_digit, 0x3bf5i32 as mp_digit, 0x3bffi32 as mp_digit,
     0x3c01i32 as mp_digit, 0x3c0di32 as mp_digit, 0x3c11i32 as mp_digit,
     0x3c17i32 as mp_digit, 0x3c1fi32 as mp_digit, 0x3c29i32 as mp_digit,
     0x3c35i32 as mp_digit, 0x3c43i32 as mp_digit, 0x3c4fi32 as mp_digit,
     0x3c53i32 as mp_digit, 0x3c5bi32 as mp_digit, 0x3c65i32 as mp_digit,
     0x3c6bi32 as mp_digit, 0x3c71i32 as mp_digit, 0x3c85i32 as mp_digit,
     0x3c89i32 as mp_digit, 0x3c97i32 as mp_digit, 0x3ca7i32 as mp_digit,
     0x3cb5i32 as mp_digit, 0x3cbfi32 as mp_digit, 0x3cc7i32 as mp_digit,
     0x3cd1i32 as mp_digit, 0x3cddi32 as mp_digit, 0x3cdfi32 as mp_digit,
     0x3cf1i32 as mp_digit, 0x3cf7i32 as mp_digit, 0x3d03i32 as mp_digit,
     0x3d0di32 as mp_digit, 0x3d19i32 as mp_digit, 0x3d1bi32 as mp_digit,
     0x3d1fi32 as mp_digit, 0x3d21i32 as mp_digit, 0x3d2di32 as mp_digit,
     0x3d33i32 as mp_digit, 0x3d37i32 as mp_digit, 0x3d3fi32 as mp_digit,
     0x3d43i32 as mp_digit, 0x3d6fi32 as mp_digit, 0x3d73i32 as mp_digit,
     0x3d75i32 as mp_digit, 0x3d79i32 as mp_digit, 0x3d7bi32 as mp_digit,
     0x3d85i32 as mp_digit, 0x3d91i32 as mp_digit, 0x3d97i32 as mp_digit,
     0x3d9di32 as mp_digit, 0x3dabi32 as mp_digit, 0x3dafi32 as mp_digit,
     0x3db5i32 as mp_digit, 0x3dbbi32 as mp_digit, 0x3dc1i32 as mp_digit,
     0x3dc9i32 as mp_digit, 0x3dcfi32 as mp_digit, 0x3df3i32 as mp_digit,
     0x3e05i32 as mp_digit, 0x3e09i32 as mp_digit, 0x3e0fi32 as mp_digit,
     0x3e11i32 as mp_digit, 0x3e1di32 as mp_digit, 0x3e23i32 as mp_digit,
     0x3e29i32 as mp_digit, 0x3e2fi32 as mp_digit, 0x3e33i32 as mp_digit,
     0x3e41i32 as mp_digit, 0x3e57i32 as mp_digit, 0x3e63i32 as mp_digit,
     0x3e65i32 as mp_digit, 0x3e77i32 as mp_digit, 0x3e81i32 as mp_digit,
     0x3e87i32 as mp_digit, 0x3ea1i32 as mp_digit, 0x3eb9i32 as mp_digit,
     0x3ebdi32 as mp_digit, 0x3ebfi32 as mp_digit, 0x3ec3i32 as mp_digit,
     0x3ec5i32 as mp_digit, 0x3ec9i32 as mp_digit, 0x3ed7i32 as mp_digit,
     0x3edbi32 as mp_digit, 0x3ee1i32 as mp_digit, 0x3ee7i32 as mp_digit,
     0x3eefi32 as mp_digit, 0x3effi32 as mp_digit, 0x3f0bi32 as mp_digit,
     0x3f0di32 as mp_digit, 0x3f37i32 as mp_digit, 0x3f3bi32 as mp_digit,
     0x3f3di32 as mp_digit, 0x3f41i32 as mp_digit, 0x3f59i32 as mp_digit,
     0x3f5fi32 as mp_digit, 0x3f65i32 as mp_digit, 0x3f67i32 as mp_digit,
     0x3f79i32 as mp_digit, 0x3f7di32 as mp_digit, 0x3f8bi32 as mp_digit,
     0x3f91i32 as mp_digit, 0x3fadi32 as mp_digit, 0x3fbfi32 as mp_digit,
     0x3fcdi32 as mp_digit, 0x3fd3i32 as mp_digit, 0x3fddi32 as mp_digit,
     0x3fe9i32 as mp_digit, 0x3febi32 as mp_digit, 0x3ff1i32 as mp_digit,
     0x3ffdi32 as mp_digit, 0x401bi32 as mp_digit, 0x4021i32 as mp_digit,
     0x4025i32 as mp_digit, 0x402bi32 as mp_digit, 0x4031i32 as mp_digit,
     0x403fi32 as mp_digit, 0x4043i32 as mp_digit, 0x4045i32 as mp_digit,
     0x405di32 as mp_digit, 0x4061i32 as mp_digit, 0x4067i32 as mp_digit,
     0x406di32 as mp_digit, 0x4087i32 as mp_digit, 0x4091i32 as mp_digit,
     0x40a3i32 as mp_digit, 0x40a9i32 as mp_digit, 0x40b1i32 as mp_digit,
     0x40b7i32 as mp_digit, 0x40bdi32 as mp_digit, 0x40dbi32 as mp_digit,
     0x40dfi32 as mp_digit, 0x40ebi32 as mp_digit, 0x40f7i32 as mp_digit,
     0x40f9i32 as mp_digit, 0x4109i32 as mp_digit, 0x410bi32 as mp_digit,
     0x4111i32 as mp_digit, 0x4115i32 as mp_digit, 0x4121i32 as mp_digit,
     0x4133i32 as mp_digit, 0x4135i32 as mp_digit, 0x413bi32 as mp_digit,
     0x413fi32 as mp_digit, 0x4159i32 as mp_digit, 0x4165i32 as mp_digit,
     0x416bi32 as mp_digit, 0x4177i32 as mp_digit, 0x417bi32 as mp_digit,
     0x4193i32 as mp_digit, 0x41abi32 as mp_digit, 0x41b7i32 as mp_digit,
     0x41bdi32 as mp_digit, 0x41bfi32 as mp_digit, 0x41cbi32 as mp_digit,
     0x41e7i32 as mp_digit, 0x41efi32 as mp_digit, 0x41f3i32 as mp_digit,
     0x41f9i32 as mp_digit, 0x4205i32 as mp_digit, 0x4207i32 as mp_digit,
     0x4219i32 as mp_digit, 0x421fi32 as mp_digit, 0x4223i32 as mp_digit,
     0x4229i32 as mp_digit, 0x422fi32 as mp_digit, 0x4243i32 as mp_digit,
     0x4253i32 as mp_digit, 0x4255i32 as mp_digit, 0x425bi32 as mp_digit,
     0x4261i32 as mp_digit, 0x4273i32 as mp_digit, 0x427di32 as mp_digit,
     0x4283i32 as mp_digit, 0x4285i32 as mp_digit, 0x4289i32 as mp_digit,
     0x4291i32 as mp_digit, 0x4297i32 as mp_digit, 0x429di32 as mp_digit,
     0x42b5i32 as mp_digit, 0x42c5i32 as mp_digit, 0x42cbi32 as mp_digit,
     0x42d3i32 as mp_digit, 0x42ddi32 as mp_digit, 0x42e3i32 as mp_digit,
     0x42f1i32 as mp_digit, 0x4307i32 as mp_digit, 0x430fi32 as mp_digit,
     0x431fi32 as mp_digit, 0x4325i32 as mp_digit, 0x4327i32 as mp_digit,
     0x4333i32 as mp_digit, 0x4337i32 as mp_digit, 0x4339i32 as mp_digit,
     0x434fi32 as mp_digit, 0x4357i32 as mp_digit, 0x4369i32 as mp_digit,
     0x438bi32 as mp_digit, 0x438di32 as mp_digit, 0x4393i32 as mp_digit,
     0x43a5i32 as mp_digit, 0x43a9i32 as mp_digit, 0x43afi32 as mp_digit,
     0x43b5i32 as mp_digit, 0x43bdi32 as mp_digit, 0x43c7i32 as mp_digit,
     0x43cfi32 as mp_digit, 0x43e1i32 as mp_digit, 0x43e7i32 as mp_digit,
     0x43ebi32 as mp_digit, 0x43edi32 as mp_digit, 0x43f1i32 as mp_digit,
     0x43f9i32 as mp_digit, 0x4409i32 as mp_digit, 0x440bi32 as mp_digit,
     0x4417i32 as mp_digit, 0x4423i32 as mp_digit, 0x4429i32 as mp_digit,
     0x443bi32 as mp_digit, 0x443fi32 as mp_digit, 0x4445i32 as mp_digit,
     0x444bi32 as mp_digit, 0x4451i32 as mp_digit, 0x4453i32 as mp_digit,
     0x4459i32 as mp_digit, 0x4465i32 as mp_digit, 0x446fi32 as mp_digit,
     0x4483i32 as mp_digit, 0x448fi32 as mp_digit, 0x44a1i32 as mp_digit,
     0x44a5i32 as mp_digit, 0x44abi32 as mp_digit, 0x44adi32 as mp_digit,
     0x44bdi32 as mp_digit, 0x44bfi32 as mp_digit, 0x44c9i32 as mp_digit,
     0x44d7i32 as mp_digit, 0x44dbi32 as mp_digit, 0x44f9i32 as mp_digit,
     0x44fbi32 as mp_digit, 0x4505i32 as mp_digit, 0x4511i32 as mp_digit,
     0x4513i32 as mp_digit, 0x452bi32 as mp_digit, 0x4531i32 as mp_digit,
     0x4541i32 as mp_digit, 0x4549i32 as mp_digit, 0x4553i32 as mp_digit,
     0x4555i32 as mp_digit, 0x4561i32 as mp_digit, 0x4577i32 as mp_digit,
     0x457di32 as mp_digit, 0x457fi32 as mp_digit, 0x458fi32 as mp_digit,
     0x45a3i32 as mp_digit, 0x45adi32 as mp_digit, 0x45afi32 as mp_digit,
     0x45bbi32 as mp_digit, 0x45c7i32 as mp_digit, 0x45d9i32 as mp_digit,
     0x45e3i32 as mp_digit, 0x45efi32 as mp_digit, 0x45f5i32 as mp_digit,
     0x45f7i32 as mp_digit, 0x4601i32 as mp_digit, 0x4603i32 as mp_digit,
     0x4609i32 as mp_digit, 0x4613i32 as mp_digit, 0x4625i32 as mp_digit,
     0x4627i32 as mp_digit, 0x4633i32 as mp_digit, 0x4639i32 as mp_digit,
     0x463di32 as mp_digit, 0x4643i32 as mp_digit, 0x4645i32 as mp_digit,
     0x465di32 as mp_digit, 0x4679i32 as mp_digit, 0x467bi32 as mp_digit,
     0x467fi32 as mp_digit, 0x4681i32 as mp_digit, 0x468bi32 as mp_digit,
     0x468di32 as mp_digit, 0x469di32 as mp_digit, 0x46a9i32 as mp_digit,
     0x46b1i32 as mp_digit, 0x46c7i32 as mp_digit, 0x46c9i32 as mp_digit,
     0x46cfi32 as mp_digit, 0x46d3i32 as mp_digit, 0x46d5i32 as mp_digit,
     0x46dfi32 as mp_digit, 0x46e5i32 as mp_digit, 0x46f9i32 as mp_digit,
     0x4705i32 as mp_digit, 0x470fi32 as mp_digit, 0x4717i32 as mp_digit,
     0x4723i32 as mp_digit, 0x4729i32 as mp_digit, 0x472fi32 as mp_digit,
     0x4735i32 as mp_digit, 0x4739i32 as mp_digit, 0x474bi32 as mp_digit,
     0x474di32 as mp_digit, 0x4751i32 as mp_digit, 0x475di32 as mp_digit,
     0x476fi32 as mp_digit, 0x4771i32 as mp_digit, 0x477di32 as mp_digit,
     0x4783i32 as mp_digit, 0x4787i32 as mp_digit, 0x4789i32 as mp_digit,
     0x4799i32 as mp_digit, 0x47a5i32 as mp_digit, 0x47b1i32 as mp_digit,
     0x47bfi32 as mp_digit, 0x47c3i32 as mp_digit, 0x47cbi32 as mp_digit,
     0x47ddi32 as mp_digit, 0x47e1i32 as mp_digit, 0x47edi32 as mp_digit,
     0x47fbi32 as mp_digit, 0x4801i32 as mp_digit, 0x4807i32 as mp_digit,
     0x480bi32 as mp_digit, 0x4813i32 as mp_digit, 0x4819i32 as mp_digit,
     0x481di32 as mp_digit, 0x4831i32 as mp_digit, 0x483di32 as mp_digit,
     0x4847i32 as mp_digit, 0x4855i32 as mp_digit, 0x4859i32 as mp_digit,
     0x485bi32 as mp_digit, 0x486bi32 as mp_digit, 0x486di32 as mp_digit,
     0x4879i32 as mp_digit, 0x4897i32 as mp_digit, 0x489bi32 as mp_digit,
     0x48a1i32 as mp_digit, 0x48b9i32 as mp_digit, 0x48cdi32 as mp_digit,
     0x48e5i32 as mp_digit, 0x48efi32 as mp_digit, 0x48f7i32 as mp_digit,
     0x4903i32 as mp_digit, 0x490di32 as mp_digit, 0x4919i32 as mp_digit,
     0x491fi32 as mp_digit, 0x492bi32 as mp_digit, 0x4937i32 as mp_digit,
     0x493di32 as mp_digit, 0x4945i32 as mp_digit, 0x4955i32 as mp_digit,
     0x4963i32 as mp_digit, 0x4969i32 as mp_digit, 0x496di32 as mp_digit,
     0x4973i32 as mp_digit, 0x4997i32 as mp_digit, 0x49abi32 as mp_digit,
     0x49b5i32 as mp_digit, 0x49d3i32 as mp_digit, 0x49dfi32 as mp_digit,
     0x49e1i32 as mp_digit, 0x49e5i32 as mp_digit, 0x49e7i32 as mp_digit,
     0x4a03i32 as mp_digit, 0x4a0fi32 as mp_digit, 0x4a1di32 as mp_digit,
     0x4a23i32 as mp_digit, 0x4a39i32 as mp_digit, 0x4a41i32 as mp_digit,
     0x4a45i32 as mp_digit, 0x4a57i32 as mp_digit, 0x4a5di32 as mp_digit,
     0x4a6bi32 as mp_digit, 0x4a7di32 as mp_digit, 0x4a81i32 as mp_digit,
     0x4a87i32 as mp_digit, 0x4a89i32 as mp_digit, 0x4a8fi32 as mp_digit,
     0x4ab1i32 as mp_digit, 0x4ac3i32 as mp_digit, 0x4ac5i32 as mp_digit,
     0x4ad5i32 as mp_digit, 0x4adbi32 as mp_digit, 0x4aedi32 as mp_digit,
     0x4aefi32 as mp_digit, 0x4b07i32 as mp_digit, 0x4b0bi32 as mp_digit,
     0x4b0di32 as mp_digit, 0x4b13i32 as mp_digit, 0x4b1fi32 as mp_digit,
     0x4b25i32 as mp_digit, 0x4b31i32 as mp_digit, 0x4b3bi32 as mp_digit,
     0x4b43i32 as mp_digit, 0x4b49i32 as mp_digit, 0x4b59i32 as mp_digit,
     0x4b65i32 as mp_digit, 0x4b6di32 as mp_digit, 0x4b77i32 as mp_digit,
     0x4b85i32 as mp_digit, 0x4badi32 as mp_digit, 0x4bb3i32 as mp_digit,
     0x4bb5i32 as mp_digit, 0x4bbbi32 as mp_digit, 0x4bbfi32 as mp_digit,
     0x4bcbi32 as mp_digit, 0x4bd9i32 as mp_digit, 0x4bddi32 as mp_digit,
     0x4bdfi32 as mp_digit, 0x4be3i32 as mp_digit, 0x4be5i32 as mp_digit,
     0x4be9i32 as mp_digit, 0x4bf1i32 as mp_digit, 0x4bf7i32 as mp_digit,
     0x4c01i32 as mp_digit, 0x4c07i32 as mp_digit, 0x4c0di32 as mp_digit,
     0x4c0fi32 as mp_digit, 0x4c15i32 as mp_digit, 0x4c1bi32 as mp_digit,
     0x4c21i32 as mp_digit, 0x4c2di32 as mp_digit, 0x4c33i32 as mp_digit,
     0x4c4bi32 as mp_digit, 0x4c55i32 as mp_digit, 0x4c57i32 as mp_digit,
     0x4c61i32 as mp_digit, 0x4c67i32 as mp_digit, 0x4c73i32 as mp_digit,
     0x4c79i32 as mp_digit, 0x4c7fi32 as mp_digit, 0x4c8di32 as mp_digit,
     0x4c93i32 as mp_digit, 0x4c99i32 as mp_digit, 0x4ccdi32 as mp_digit,
     0x4ce1i32 as mp_digit, 0x4ce7i32 as mp_digit, 0x4cf1i32 as mp_digit,
     0x4cf3i32 as mp_digit, 0x4cfdi32 as mp_digit, 0x4d05i32 as mp_digit,
     0x4d0fi32 as mp_digit, 0x4d1bi32 as mp_digit, 0x4d27i32 as mp_digit,
     0x4d29i32 as mp_digit, 0x4d2fi32 as mp_digit, 0x4d33i32 as mp_digit,
     0x4d41i32 as mp_digit, 0x4d51i32 as mp_digit, 0x4d59i32 as mp_digit,
     0x4d65i32 as mp_digit, 0x4d6bi32 as mp_digit, 0x4d81i32 as mp_digit,
     0x4d83i32 as mp_digit, 0x4d8di32 as mp_digit, 0x4d95i32 as mp_digit,
     0x4d9bi32 as mp_digit, 0x4db1i32 as mp_digit, 0x4db3i32 as mp_digit,
     0x4dc9i32 as mp_digit, 0x4dcfi32 as mp_digit, 0x4dd7i32 as mp_digit,
     0x4de1i32 as mp_digit, 0x4dedi32 as mp_digit, 0x4df9i32 as mp_digit,
     0x4dfbi32 as mp_digit, 0x4e05i32 as mp_digit, 0x4e0bi32 as mp_digit,
     0x4e17i32 as mp_digit, 0x4e19i32 as mp_digit, 0x4e1di32 as mp_digit,
     0x4e2bi32 as mp_digit, 0x4e35i32 as mp_digit, 0x4e37i32 as mp_digit,
     0x4e3di32 as mp_digit, 0x4e4fi32 as mp_digit, 0x4e53i32 as mp_digit,
     0x4e5fi32 as mp_digit, 0x4e67i32 as mp_digit, 0x4e79i32 as mp_digit,
     0x4e85i32 as mp_digit, 0x4e8bi32 as mp_digit, 0x4e91i32 as mp_digit,
     0x4e95i32 as mp_digit, 0x4e9bi32 as mp_digit, 0x4ea1i32 as mp_digit,
     0x4eafi32 as mp_digit, 0x4eb3i32 as mp_digit, 0x4eb5i32 as mp_digit,
     0x4ec1i32 as mp_digit, 0x4ecdi32 as mp_digit, 0x4ed1i32 as mp_digit,
     0x4ed7i32 as mp_digit, 0x4ee9i32 as mp_digit, 0x4efbi32 as mp_digit,
     0x4f07i32 as mp_digit, 0x4f09i32 as mp_digit, 0x4f19i32 as mp_digit,
     0x4f25i32 as mp_digit, 0x4f2di32 as mp_digit, 0x4f3fi32 as mp_digit,
     0x4f49i32 as mp_digit, 0x4f63i32 as mp_digit, 0x4f67i32 as mp_digit,
     0x4f6di32 as mp_digit, 0x4f75i32 as mp_digit, 0x4f7bi32 as mp_digit,
     0x4f81i32 as mp_digit, 0x4f85i32 as mp_digit, 0x4f87i32 as mp_digit,
     0x4f91i32 as mp_digit, 0x4fa5i32 as mp_digit, 0x4fa9i32 as mp_digit,
     0x4fafi32 as mp_digit, 0x4fb7i32 as mp_digit, 0x4fbbi32 as mp_digit,
     0x4fcfi32 as mp_digit, 0x4fd9i32 as mp_digit, 0x4fdbi32 as mp_digit,
     0x4ffdi32 as mp_digit, 0x4fffi32 as mp_digit, 0x5003i32 as mp_digit,
     0x501bi32 as mp_digit, 0x501di32 as mp_digit, 0x5029i32 as mp_digit,
     0x5035i32 as mp_digit, 0x503fi32 as mp_digit, 0x5045i32 as mp_digit,
     0x5047i32 as mp_digit, 0x5053i32 as mp_digit, 0x5071i32 as mp_digit,
     0x5077i32 as mp_digit, 0x5083i32 as mp_digit, 0x5093i32 as mp_digit,
     0x509fi32 as mp_digit, 0x50a1i32 as mp_digit, 0x50b7i32 as mp_digit,
     0x50c9i32 as mp_digit, 0x50d5i32 as mp_digit, 0x50e3i32 as mp_digit,
     0x50edi32 as mp_digit, 0x50efi32 as mp_digit, 0x50fbi32 as mp_digit,
     0x5107i32 as mp_digit, 0x510bi32 as mp_digit, 0x510di32 as mp_digit,
     0x5111i32 as mp_digit, 0x5117i32 as mp_digit, 0x5123i32 as mp_digit,
     0x5125i32 as mp_digit, 0x5135i32 as mp_digit, 0x5147i32 as mp_digit,
     0x5149i32 as mp_digit, 0x5171i32 as mp_digit, 0x5179i32 as mp_digit,
     0x5189i32 as mp_digit, 0x518fi32 as mp_digit, 0x5197i32 as mp_digit,
     0x51a1i32 as mp_digit, 0x51a3i32 as mp_digit, 0x51a7i32 as mp_digit,
     0x51b9i32 as mp_digit, 0x51c1i32 as mp_digit, 0x51cbi32 as mp_digit,
     0x51d3i32 as mp_digit, 0x51dfi32 as mp_digit, 0x51e3i32 as mp_digit,
     0x51f5i32 as mp_digit, 0x51f7i32 as mp_digit, 0x5209i32 as mp_digit,
     0x5213i32 as mp_digit, 0x5215i32 as mp_digit, 0x5219i32 as mp_digit,
     0x521bi32 as mp_digit, 0x521fi32 as mp_digit, 0x5227i32 as mp_digit,
     0x5243i32 as mp_digit, 0x5245i32 as mp_digit, 0x524bi32 as mp_digit,
     0x5261i32 as mp_digit, 0x526di32 as mp_digit, 0x5273i32 as mp_digit,
     0x5281i32 as mp_digit, 0x5293i32 as mp_digit, 0x5297i32 as mp_digit,
     0x529di32 as mp_digit, 0x52a5i32 as mp_digit, 0x52abi32 as mp_digit,
     0x52b1i32 as mp_digit, 0x52bbi32 as mp_digit, 0x52c3i32 as mp_digit,
     0x52c7i32 as mp_digit, 0x52c9i32 as mp_digit, 0x52dbi32 as mp_digit,
     0x52e5i32 as mp_digit, 0x52ebi32 as mp_digit, 0x52ffi32 as mp_digit,
     0x5315i32 as mp_digit, 0x531di32 as mp_digit, 0x5323i32 as mp_digit,
     0x5341i32 as mp_digit, 0x5345i32 as mp_digit, 0x5347i32 as mp_digit,
     0x534bi32 as mp_digit, 0x535di32 as mp_digit, 0x5363i32 as mp_digit,
     0x5381i32 as mp_digit, 0x5383i32 as mp_digit, 0x5387i32 as mp_digit,
     0x538fi32 as mp_digit, 0x5395i32 as mp_digit, 0x5399i32 as mp_digit,
     0x539fi32 as mp_digit, 0x53abi32 as mp_digit, 0x53b9i32 as mp_digit,
     0x53dbi32 as mp_digit, 0x53e9i32 as mp_digit, 0x53efi32 as mp_digit,
     0x53f3i32 as mp_digit, 0x53f5i32 as mp_digit, 0x53fbi32 as mp_digit,
     0x53ffi32 as mp_digit, 0x540di32 as mp_digit, 0x5411i32 as mp_digit,
     0x5413i32 as mp_digit, 0x5419i32 as mp_digit, 0x5435i32 as mp_digit,
     0x5437i32 as mp_digit, 0x543bi32 as mp_digit, 0x5441i32 as mp_digit,
     0x5449i32 as mp_digit, 0x5453i32 as mp_digit, 0x5455i32 as mp_digit,
     0x545fi32 as mp_digit, 0x5461i32 as mp_digit, 0x546bi32 as mp_digit,
     0x546di32 as mp_digit, 0x5471i32 as mp_digit, 0x548fi32 as mp_digit,
     0x5491i32 as mp_digit, 0x549di32 as mp_digit, 0x54a9i32 as mp_digit,
     0x54b3i32 as mp_digit, 0x54c5i32 as mp_digit, 0x54d1i32 as mp_digit,
     0x54dfi32 as mp_digit, 0x54e9i32 as mp_digit, 0x54ebi32 as mp_digit,
     0x54f7i32 as mp_digit, 0x54fdi32 as mp_digit, 0x5507i32 as mp_digit,
     0x550di32 as mp_digit, 0x551bi32 as mp_digit, 0x5527i32 as mp_digit,
     0x552bi32 as mp_digit, 0x5539i32 as mp_digit, 0x553di32 as mp_digit,
     0x554fi32 as mp_digit, 0x5551i32 as mp_digit, 0x555bi32 as mp_digit,
     0x5563i32 as mp_digit, 0x5567i32 as mp_digit, 0x556fi32 as mp_digit,
     0x5579i32 as mp_digit, 0x5585i32 as mp_digit, 0x5597i32 as mp_digit,
     0x55a9i32 as mp_digit, 0x55b1i32 as mp_digit, 0x55b7i32 as mp_digit,
     0x55c9i32 as mp_digit, 0x55d9i32 as mp_digit, 0x55e7i32 as mp_digit,
     0x55edi32 as mp_digit, 0x55f3i32 as mp_digit, 0x55fdi32 as mp_digit,
     0x560bi32 as mp_digit, 0x560fi32 as mp_digit, 0x5615i32 as mp_digit,
     0x5617i32 as mp_digit, 0x5623i32 as mp_digit, 0x562fi32 as mp_digit,
     0x5633i32 as mp_digit, 0x5639i32 as mp_digit, 0x563fi32 as mp_digit,
     0x564bi32 as mp_digit, 0x564di32 as mp_digit, 0x565di32 as mp_digit,
     0x565fi32 as mp_digit, 0x566bi32 as mp_digit, 0x5671i32 as mp_digit,
     0x5675i32 as mp_digit, 0x5683i32 as mp_digit, 0x5689i32 as mp_digit,
     0x568di32 as mp_digit, 0x568fi32 as mp_digit, 0x569bi32 as mp_digit,
     0x56adi32 as mp_digit, 0x56b1i32 as mp_digit, 0x56d5i32 as mp_digit,
     0x56e7i32 as mp_digit, 0x56f3i32 as mp_digit, 0x56ffi32 as mp_digit,
     0x5701i32 as mp_digit, 0x5705i32 as mp_digit, 0x5707i32 as mp_digit,
     0x570bi32 as mp_digit, 0x5713i32 as mp_digit, 0x571fi32 as mp_digit,
     0x5723i32 as mp_digit, 0x5747i32 as mp_digit, 0x574di32 as mp_digit,
     0x575fi32 as mp_digit, 0x5761i32 as mp_digit, 0x576di32 as mp_digit,
     0x5777i32 as mp_digit, 0x577di32 as mp_digit, 0x5789i32 as mp_digit,
     0x57a1i32 as mp_digit, 0x57a9i32 as mp_digit, 0x57afi32 as mp_digit,
     0x57b5i32 as mp_digit, 0x57c5i32 as mp_digit, 0x57d1i32 as mp_digit,
     0x57d3i32 as mp_digit, 0x57e5i32 as mp_digit, 0x57efi32 as mp_digit,
     0x5803i32 as mp_digit, 0x580di32 as mp_digit, 0x580fi32 as mp_digit,
     0x5815i32 as mp_digit, 0x5827i32 as mp_digit, 0x582bi32 as mp_digit,
     0x582di32 as mp_digit, 0x5855i32 as mp_digit, 0x585bi32 as mp_digit,
     0x585di32 as mp_digit, 0x586di32 as mp_digit, 0x586fi32 as mp_digit,
     0x5873i32 as mp_digit, 0x587bi32 as mp_digit, 0x588di32 as mp_digit,
     0x5897i32 as mp_digit, 0x58a3i32 as mp_digit, 0x58a9i32 as mp_digit,
     0x58abi32 as mp_digit, 0x58b5i32 as mp_digit, 0x58bdi32 as mp_digit,
     0x58c1i32 as mp_digit, 0x58c7i32 as mp_digit, 0x58d3i32 as mp_digit,
     0x58d5i32 as mp_digit, 0x58dfi32 as mp_digit, 0x58f1i32 as mp_digit,
     0x58f9i32 as mp_digit, 0x58ffi32 as mp_digit, 0x5903i32 as mp_digit,
     0x5917i32 as mp_digit, 0x591bi32 as mp_digit, 0x5921i32 as mp_digit,
     0x5945i32 as mp_digit, 0x594bi32 as mp_digit, 0x594di32 as mp_digit,
     0x5957i32 as mp_digit, 0x595di32 as mp_digit, 0x5975i32 as mp_digit,
     0x597bi32 as mp_digit, 0x5989i32 as mp_digit, 0x5999i32 as mp_digit,
     0x599fi32 as mp_digit, 0x59b1i32 as mp_digit, 0x59b3i32 as mp_digit,
     0x59bdi32 as mp_digit, 0x59d1i32 as mp_digit, 0x59dbi32 as mp_digit,
     0x59e3i32 as mp_digit, 0x59e9i32 as mp_digit, 0x59edi32 as mp_digit,
     0x59f3i32 as mp_digit, 0x59f5i32 as mp_digit, 0x59ffi32 as mp_digit,
     0x5a01i32 as mp_digit, 0x5a0di32 as mp_digit, 0x5a11i32 as mp_digit,
     0x5a13i32 as mp_digit, 0x5a17i32 as mp_digit, 0x5a1fi32 as mp_digit,
     0x5a29i32 as mp_digit, 0x5a2fi32 as mp_digit, 0x5a3bi32 as mp_digit,
     0x5a4di32 as mp_digit, 0x5a5bi32 as mp_digit, 0x5a67i32 as mp_digit,
     0x5a77i32 as mp_digit, 0x5a7fi32 as mp_digit, 0x5a85i32 as mp_digit,
     0x5a95i32 as mp_digit, 0x5a9di32 as mp_digit, 0x5aa1i32 as mp_digit,
     0x5aa3i32 as mp_digit, 0x5aa9i32 as mp_digit, 0x5abbi32 as mp_digit,
     0x5ad3i32 as mp_digit, 0x5ae5i32 as mp_digit, 0x5aefi32 as mp_digit,
     0x5afbi32 as mp_digit, 0x5afdi32 as mp_digit, 0x5b01i32 as mp_digit,
     0x5b0fi32 as mp_digit, 0x5b19i32 as mp_digit, 0x5b1fi32 as mp_digit,
     0x5b25i32 as mp_digit, 0x5b2bi32 as mp_digit, 0x5b3di32 as mp_digit,
     0x5b49i32 as mp_digit, 0x5b4bi32 as mp_digit, 0x5b67i32 as mp_digit,
     0x5b79i32 as mp_digit, 0x5b87i32 as mp_digit, 0x5b97i32 as mp_digit,
     0x5ba3i32 as mp_digit, 0x5bb1i32 as mp_digit, 0x5bc9i32 as mp_digit,
     0x5bd5i32 as mp_digit, 0x5bebi32 as mp_digit, 0x5bf1i32 as mp_digit,
     0x5bf3i32 as mp_digit, 0x5bfdi32 as mp_digit, 0x5c05i32 as mp_digit,
     0x5c09i32 as mp_digit, 0x5c0bi32 as mp_digit, 0x5c0fi32 as mp_digit,
     0x5c1di32 as mp_digit, 0x5c29i32 as mp_digit, 0x5c2fi32 as mp_digit,
     0x5c33i32 as mp_digit, 0x5c39i32 as mp_digit, 0x5c47i32 as mp_digit,
     0x5c4bi32 as mp_digit, 0x5c4di32 as mp_digit, 0x5c51i32 as mp_digit,
     0x5c6fi32 as mp_digit, 0x5c75i32 as mp_digit, 0x5c77i32 as mp_digit,
     0x5c7di32 as mp_digit, 0x5c87i32 as mp_digit, 0x5c89i32 as mp_digit,
     0x5ca7i32 as mp_digit, 0x5cbdi32 as mp_digit, 0x5cbfi32 as mp_digit,
     0x5cc3i32 as mp_digit, 0x5cc9i32 as mp_digit, 0x5cd1i32 as mp_digit,
     0x5cd7i32 as mp_digit, 0x5cddi32 as mp_digit, 0x5cedi32 as mp_digit,
     0x5cf9i32 as mp_digit, 0x5d05i32 as mp_digit, 0x5d0bi32 as mp_digit,
     0x5d13i32 as mp_digit, 0x5d17i32 as mp_digit, 0x5d19i32 as mp_digit,
     0x5d31i32 as mp_digit, 0x5d3di32 as mp_digit, 0x5d41i32 as mp_digit,
     0x5d47i32 as mp_digit, 0x5d4fi32 as mp_digit, 0x5d55i32 as mp_digit,
     0x5d5bi32 as mp_digit, 0x5d65i32 as mp_digit, 0x5d67i32 as mp_digit,
     0x5d6di32 as mp_digit, 0x5d79i32 as mp_digit, 0x5d95i32 as mp_digit,
     0x5da3i32 as mp_digit, 0x5da9i32 as mp_digit, 0x5dadi32 as mp_digit,
     0x5db9i32 as mp_digit, 0x5dc1i32 as mp_digit, 0x5dc7i32 as mp_digit,
     0x5dd3i32 as mp_digit, 0x5dd7i32 as mp_digit, 0x5dddi32 as mp_digit,
     0x5debi32 as mp_digit, 0x5df1i32 as mp_digit, 0x5dfdi32 as mp_digit,
     0x5e07i32 as mp_digit, 0x5e0di32 as mp_digit, 0x5e13i32 as mp_digit,
     0x5e1bi32 as mp_digit, 0x5e21i32 as mp_digit, 0x5e27i32 as mp_digit,
     0x5e2bi32 as mp_digit, 0x5e2di32 as mp_digit, 0x5e31i32 as mp_digit,
     0x5e39i32 as mp_digit, 0x5e45i32 as mp_digit, 0x5e49i32 as mp_digit,
     0x5e57i32 as mp_digit, 0x5e69i32 as mp_digit, 0x5e73i32 as mp_digit,
     0x5e75i32 as mp_digit, 0x5e85i32 as mp_digit, 0x5e8bi32 as mp_digit,
     0x5e9fi32 as mp_digit, 0x5ea5i32 as mp_digit, 0x5eafi32 as mp_digit,
     0x5eb7i32 as mp_digit, 0x5ebbi32 as mp_digit, 0x5ed9i32 as mp_digit,
     0x5efdi32 as mp_digit, 0x5f09i32 as mp_digit, 0x5f11i32 as mp_digit,
     0x5f27i32 as mp_digit, 0x5f33i32 as mp_digit, 0x5f35i32 as mp_digit,
     0x5f3bi32 as mp_digit, 0x5f47i32 as mp_digit, 0x5f57i32 as mp_digit,
     0x5f5di32 as mp_digit, 0x5f63i32 as mp_digit, 0x5f65i32 as mp_digit,
     0x5f77i32 as mp_digit, 0x5f7bi32 as mp_digit, 0x5f95i32 as mp_digit,
     0x5f99i32 as mp_digit, 0x5fa1i32 as mp_digit, 0x5fb3i32 as mp_digit,
     0x5fbdi32 as mp_digit, 0x5fc5i32 as mp_digit, 0x5fcfi32 as mp_digit,
     0x5fd5i32 as mp_digit, 0x5fe3i32 as mp_digit, 0x5fe7i32 as mp_digit,
     0x5ffbi32 as mp_digit, 0x6011i32 as mp_digit, 0x6023i32 as mp_digit,
     0x602fi32 as mp_digit, 0x6037i32 as mp_digit, 0x6053i32 as mp_digit,
     0x605fi32 as mp_digit, 0x6065i32 as mp_digit, 0x606bi32 as mp_digit,
     0x6073i32 as mp_digit, 0x6079i32 as mp_digit, 0x6085i32 as mp_digit,
     0x609di32 as mp_digit, 0x60adi32 as mp_digit, 0x60bbi32 as mp_digit,
     0x60bfi32 as mp_digit, 0x60cdi32 as mp_digit, 0x60d9i32 as mp_digit,
     0x60dfi32 as mp_digit, 0x60e9i32 as mp_digit, 0x60f5i32 as mp_digit,
     0x6109i32 as mp_digit, 0x610fi32 as mp_digit, 0x6113i32 as mp_digit,
     0x611bi32 as mp_digit, 0x612di32 as mp_digit, 0x6139i32 as mp_digit,
     0x614bi32 as mp_digit, 0x6155i32 as mp_digit, 0x6157i32 as mp_digit,
     0x615bi32 as mp_digit, 0x616fi32 as mp_digit, 0x6179i32 as mp_digit,
     0x6187i32 as mp_digit, 0x618bi32 as mp_digit, 0x6191i32 as mp_digit,
     0x6193i32 as mp_digit, 0x619di32 as mp_digit, 0x61b5i32 as mp_digit,
     0x61c7i32 as mp_digit, 0x61c9i32 as mp_digit, 0x61cdi32 as mp_digit,
     0x61e1i32 as mp_digit, 0x61f1i32 as mp_digit, 0x61ffi32 as mp_digit,
     0x6209i32 as mp_digit, 0x6217i32 as mp_digit, 0x621di32 as mp_digit,
     0x6221i32 as mp_digit, 0x6227i32 as mp_digit, 0x623bi32 as mp_digit,
     0x6241i32 as mp_digit, 0x624bi32 as mp_digit, 0x6251i32 as mp_digit,
     0x6253i32 as mp_digit, 0x625fi32 as mp_digit, 0x6265i32 as mp_digit,
     0x6283i32 as mp_digit, 0x628di32 as mp_digit, 0x6295i32 as mp_digit,
     0x629bi32 as mp_digit, 0x629fi32 as mp_digit, 0x62a5i32 as mp_digit,
     0x62adi32 as mp_digit, 0x62d5i32 as mp_digit, 0x62d7i32 as mp_digit,
     0x62dbi32 as mp_digit, 0x62ddi32 as mp_digit, 0x62e9i32 as mp_digit,
     0x62fbi32 as mp_digit, 0x62ffi32 as mp_digit, 0x6305i32 as mp_digit,
     0x630di32 as mp_digit, 0x6317i32 as mp_digit, 0x631di32 as mp_digit,
     0x632fi32 as mp_digit, 0x6341i32 as mp_digit, 0x6343i32 as mp_digit,
     0x634fi32 as mp_digit, 0x635fi32 as mp_digit, 0x6367i32 as mp_digit,
     0x636di32 as mp_digit, 0x6371i32 as mp_digit, 0x6377i32 as mp_digit,
     0x637di32 as mp_digit, 0x637fi32 as mp_digit, 0x63b3i32 as mp_digit,
     0x63c1i32 as mp_digit, 0x63c5i32 as mp_digit, 0x63d9i32 as mp_digit,
     0x63e9i32 as mp_digit, 0x63ebi32 as mp_digit, 0x63efi32 as mp_digit,
     0x63f5i32 as mp_digit, 0x6401i32 as mp_digit, 0x6403i32 as mp_digit,
     0x6409i32 as mp_digit, 0x6415i32 as mp_digit, 0x6421i32 as mp_digit,
     0x6427i32 as mp_digit, 0x642bi32 as mp_digit, 0x6439i32 as mp_digit,
     0x6443i32 as mp_digit, 0x6449i32 as mp_digit, 0x644fi32 as mp_digit,
     0x645di32 as mp_digit, 0x6467i32 as mp_digit, 0x6475i32 as mp_digit,
     0x6485i32 as mp_digit, 0x648di32 as mp_digit, 0x6493i32 as mp_digit,
     0x649fi32 as mp_digit, 0x64a3i32 as mp_digit, 0x64abi32 as mp_digit,
     0x64c1i32 as mp_digit, 0x64c7i32 as mp_digit, 0x64c9i32 as mp_digit,
     0x64dbi32 as mp_digit, 0x64f1i32 as mp_digit, 0x64f7i32 as mp_digit,
     0x64f9i32 as mp_digit, 0x650bi32 as mp_digit, 0x6511i32 as mp_digit,
     0x6521i32 as mp_digit, 0x652fi32 as mp_digit, 0x6539i32 as mp_digit,
     0x653fi32 as mp_digit, 0x654bi32 as mp_digit, 0x654di32 as mp_digit,
     0x6553i32 as mp_digit, 0x6557i32 as mp_digit, 0x655fi32 as mp_digit,
     0x6571i32 as mp_digit, 0x657di32 as mp_digit, 0x658di32 as mp_digit,
     0x658fi32 as mp_digit, 0x6593i32 as mp_digit, 0x65a1i32 as mp_digit,
     0x65a5i32 as mp_digit, 0x65adi32 as mp_digit, 0x65b9i32 as mp_digit,
     0x65c5i32 as mp_digit, 0x65e3i32 as mp_digit, 0x65f3i32 as mp_digit,
     0x65fbi32 as mp_digit, 0x65ffi32 as mp_digit, 0x6601i32 as mp_digit,
     0x6607i32 as mp_digit, 0x661di32 as mp_digit, 0x6629i32 as mp_digit,
     0x6631i32 as mp_digit, 0x663bi32 as mp_digit, 0x6641i32 as mp_digit,
     0x6647i32 as mp_digit, 0x664di32 as mp_digit, 0x665bi32 as mp_digit,
     0x6661i32 as mp_digit, 0x6673i32 as mp_digit, 0x667di32 as mp_digit,
     0x6689i32 as mp_digit, 0x668bi32 as mp_digit, 0x6695i32 as mp_digit,
     0x6697i32 as mp_digit, 0x669bi32 as mp_digit, 0x66b5i32 as mp_digit,
     0x66b9i32 as mp_digit, 0x66c5i32 as mp_digit, 0x66cdi32 as mp_digit,
     0x66d1i32 as mp_digit, 0x66e3i32 as mp_digit, 0x66ebi32 as mp_digit,
     0x66f5i32 as mp_digit, 0x6703i32 as mp_digit, 0x6713i32 as mp_digit,
     0x6719i32 as mp_digit, 0x671fi32 as mp_digit, 0x6727i32 as mp_digit,
     0x6731i32 as mp_digit, 0x6737i32 as mp_digit, 0x673fi32 as mp_digit,
     0x6745i32 as mp_digit, 0x6751i32 as mp_digit, 0x675bi32 as mp_digit,
     0x676fi32 as mp_digit, 0x6779i32 as mp_digit, 0x6781i32 as mp_digit,
     0x6785i32 as mp_digit, 0x6791i32 as mp_digit, 0x67abi32 as mp_digit,
     0x67bdi32 as mp_digit, 0x67c1i32 as mp_digit, 0x67cdi32 as mp_digit,
     0x67dfi32 as mp_digit, 0x67e5i32 as mp_digit, 0x6803i32 as mp_digit,
     0x6809i32 as mp_digit, 0x6811i32 as mp_digit, 0x6817i32 as mp_digit,
     0x682di32 as mp_digit, 0x6839i32 as mp_digit, 0x683bi32 as mp_digit,
     0x683fi32 as mp_digit, 0x6845i32 as mp_digit, 0x684bi32 as mp_digit,
     0x684di32 as mp_digit, 0x6857i32 as mp_digit, 0x6859i32 as mp_digit,
     0x685di32 as mp_digit, 0x6863i32 as mp_digit, 0x6869i32 as mp_digit,
     0x686bi32 as mp_digit, 0x6871i32 as mp_digit, 0x6887i32 as mp_digit,
     0x6899i32 as mp_digit, 0x689fi32 as mp_digit, 0x68b1i32 as mp_digit,
     0x68bdi32 as mp_digit, 0x68c5i32 as mp_digit, 0x68d1i32 as mp_digit,
     0x68d7i32 as mp_digit, 0x68e1i32 as mp_digit, 0x68edi32 as mp_digit,
     0x68efi32 as mp_digit, 0x68ffi32 as mp_digit, 0x6901i32 as mp_digit,
     0x690bi32 as mp_digit, 0x690di32 as mp_digit, 0x6917i32 as mp_digit,
     0x6929i32 as mp_digit, 0x692fi32 as mp_digit, 0x6943i32 as mp_digit,
     0x6947i32 as mp_digit, 0x6949i32 as mp_digit, 0x694fi32 as mp_digit,
     0x6965i32 as mp_digit, 0x696bi32 as mp_digit, 0x6971i32 as mp_digit,
     0x6983i32 as mp_digit, 0x6989i32 as mp_digit, 0x6997i32 as mp_digit,
     0x69a3i32 as mp_digit, 0x69b3i32 as mp_digit, 0x69b5i32 as mp_digit,
     0x69bbi32 as mp_digit, 0x69c1i32 as mp_digit, 0x69c5i32 as mp_digit,
     0x69d3i32 as mp_digit, 0x69dfi32 as mp_digit, 0x69e3i32 as mp_digit,
     0x69e5i32 as mp_digit, 0x69f7i32 as mp_digit, 0x6a07i32 as mp_digit,
     0x6a2bi32 as mp_digit, 0x6a37i32 as mp_digit, 0x6a3di32 as mp_digit,
     0x6a4bi32 as mp_digit, 0x6a67i32 as mp_digit, 0x6a69i32 as mp_digit,
     0x6a75i32 as mp_digit, 0x6a7bi32 as mp_digit, 0x6a87i32 as mp_digit,
     0x6a8di32 as mp_digit, 0x6a91i32 as mp_digit, 0x6a93i32 as mp_digit,
     0x6aa3i32 as mp_digit, 0x6ac1i32 as mp_digit, 0x6ac9i32 as mp_digit,
     0x6ae1i32 as mp_digit, 0x6ae7i32 as mp_digit, 0x6b05i32 as mp_digit,
     0x6b0fi32 as mp_digit, 0x6b11i32 as mp_digit, 0x6b23i32 as mp_digit,
     0x6b27i32 as mp_digit, 0x6b2di32 as mp_digit, 0x6b39i32 as mp_digit,
     0x6b41i32 as mp_digit, 0x6b57i32 as mp_digit, 0x6b59i32 as mp_digit,
     0x6b5fi32 as mp_digit, 0x6b75i32 as mp_digit, 0x6b87i32 as mp_digit,
     0x6b89i32 as mp_digit, 0x6b93i32 as mp_digit, 0x6b95i32 as mp_digit,
     0x6b9fi32 as mp_digit, 0x6bbdi32 as mp_digit, 0x6bbfi32 as mp_digit,
     0x6bdbi32 as mp_digit, 0x6be1i32 as mp_digit, 0x6befi32 as mp_digit,
     0x6bffi32 as mp_digit, 0x6c05i32 as mp_digit, 0x6c19i32 as mp_digit,
     0x6c29i32 as mp_digit, 0x6c2bi32 as mp_digit, 0x6c31i32 as mp_digit,
     0x6c35i32 as mp_digit, 0x6c55i32 as mp_digit, 0x6c59i32 as mp_digit,
     0x6c5bi32 as mp_digit, 0x6c5fi32 as mp_digit, 0x6c65i32 as mp_digit,
     0x6c67i32 as mp_digit, 0x6c73i32 as mp_digit, 0x6c77i32 as mp_digit,
     0x6c7di32 as mp_digit, 0x6c83i32 as mp_digit, 0x6c8fi32 as mp_digit,
     0x6c91i32 as mp_digit, 0x6c97i32 as mp_digit, 0x6c9bi32 as mp_digit,
     0x6ca1i32 as mp_digit, 0x6ca9i32 as mp_digit, 0x6cafi32 as mp_digit,
     0x6cb3i32 as mp_digit, 0x6cc7i32 as mp_digit, 0x6ccbi32 as mp_digit,
     0x6cebi32 as mp_digit, 0x6cf5i32 as mp_digit, 0x6cfdi32 as mp_digit,
     0x6d0di32 as mp_digit, 0x6d0fi32 as mp_digit, 0x6d25i32 as mp_digit,
     0x6d27i32 as mp_digit, 0x6d2bi32 as mp_digit, 0x6d31i32 as mp_digit,
     0x6d39i32 as mp_digit, 0x6d3fi32 as mp_digit, 0x6d4fi32 as mp_digit,
     0x6d5di32 as mp_digit, 0x6d61i32 as mp_digit, 0x6d73i32 as mp_digit,
     0x6d7bi32 as mp_digit, 0x6d7fi32 as mp_digit, 0x6d93i32 as mp_digit,
     0x6d99i32 as mp_digit, 0x6da5i32 as mp_digit, 0x6db1i32 as mp_digit,
     0x6db7i32 as mp_digit, 0x6dc1i32 as mp_digit, 0x6dc3i32 as mp_digit,
     0x6dcdi32 as mp_digit, 0x6dcfi32 as mp_digit, 0x6ddbi32 as mp_digit,
     0x6df7i32 as mp_digit, 0x6e03i32 as mp_digit, 0x6e15i32 as mp_digit,
     0x6e17i32 as mp_digit, 0x6e29i32 as mp_digit, 0x6e33i32 as mp_digit,
     0x6e3bi32 as mp_digit, 0x6e45i32 as mp_digit, 0x6e75i32 as mp_digit,
     0x6e77i32 as mp_digit, 0x6e7bi32 as mp_digit, 0x6e81i32 as mp_digit,
     0x6e89i32 as mp_digit, 0x6e93i32 as mp_digit, 0x6e95i32 as mp_digit,
     0x6e9fi32 as mp_digit, 0x6ebdi32 as mp_digit, 0x6ebfi32 as mp_digit,
     0x6ee3i32 as mp_digit, 0x6ee9i32 as mp_digit, 0x6ef3i32 as mp_digit,
     0x6ef9i32 as mp_digit, 0x6efbi32 as mp_digit, 0x6f0di32 as mp_digit,
     0x6f11i32 as mp_digit, 0x6f17i32 as mp_digit, 0x6f1fi32 as mp_digit,
     0x6f2fi32 as mp_digit, 0x6f3di32 as mp_digit, 0x6f4di32 as mp_digit,
     0x6f53i32 as mp_digit, 0x6f61i32 as mp_digit, 0x6f65i32 as mp_digit,
     0x6f79i32 as mp_digit, 0x6f7di32 as mp_digit, 0x6f83i32 as mp_digit,
     0x6f85i32 as mp_digit, 0x6f8fi32 as mp_digit, 0x6f9bi32 as mp_digit,
     0x6f9di32 as mp_digit, 0x6fa3i32 as mp_digit, 0x6fafi32 as mp_digit,
     0x6fb5i32 as mp_digit, 0x6fbbi32 as mp_digit, 0x6fbfi32 as mp_digit,
     0x6fcbi32 as mp_digit, 0x6fcdi32 as mp_digit, 0x6fd3i32 as mp_digit,
     0x6fd7i32 as mp_digit, 0x6fe3i32 as mp_digit, 0x6fe9i32 as mp_digit,
     0x6ff1i32 as mp_digit, 0x6ff5i32 as mp_digit, 0x6ff7i32 as mp_digit,
     0x6ffdi32 as mp_digit, 0x700fi32 as mp_digit, 0x7019i32 as mp_digit,
     0x701fi32 as mp_digit, 0x7027i32 as mp_digit, 0x7033i32 as mp_digit,
     0x7039i32 as mp_digit, 0x704fi32 as mp_digit, 0x7051i32 as mp_digit,
     0x7057i32 as mp_digit, 0x7063i32 as mp_digit, 0x7075i32 as mp_digit,
     0x7079i32 as mp_digit, 0x7087i32 as mp_digit, 0x708di32 as mp_digit,
     0x7091i32 as mp_digit, 0x70a5i32 as mp_digit, 0x70abi32 as mp_digit,
     0x70bbi32 as mp_digit, 0x70c3i32 as mp_digit, 0x70c7i32 as mp_digit,
     0x70cfi32 as mp_digit, 0x70e5i32 as mp_digit, 0x70edi32 as mp_digit,
     0x70f9i32 as mp_digit, 0x70ffi32 as mp_digit, 0x7105i32 as mp_digit,
     0x7115i32 as mp_digit, 0x7121i32 as mp_digit, 0x7133i32 as mp_digit,
     0x7151i32 as mp_digit, 0x7159i32 as mp_digit, 0x715di32 as mp_digit,
     0x715fi32 as mp_digit, 0x7163i32 as mp_digit, 0x7169i32 as mp_digit,
     0x7183i32 as mp_digit, 0x7187i32 as mp_digit, 0x7195i32 as mp_digit,
     0x71adi32 as mp_digit, 0x71c3i32 as mp_digit, 0x71c9i32 as mp_digit,
     0x71cbi32 as mp_digit, 0x71d1i32 as mp_digit, 0x71dbi32 as mp_digit,
     0x71e1i32 as mp_digit, 0x71efi32 as mp_digit, 0x71f5i32 as mp_digit,
     0x71fbi32 as mp_digit, 0x7207i32 as mp_digit, 0x7211i32 as mp_digit,
     0x7217i32 as mp_digit, 0x7219i32 as mp_digit, 0x7225i32 as mp_digit,
     0x722fi32 as mp_digit, 0x723bi32 as mp_digit, 0x7243i32 as mp_digit,
     0x7255i32 as mp_digit, 0x7267i32 as mp_digit, 0x7271i32 as mp_digit,
     0x7277i32 as mp_digit, 0x727fi32 as mp_digit, 0x728fi32 as mp_digit,
     0x7295i32 as mp_digit, 0x729bi32 as mp_digit, 0x72a3i32 as mp_digit,
     0x72b3i32 as mp_digit, 0x72c7i32 as mp_digit, 0x72cbi32 as mp_digit,
     0x72cdi32 as mp_digit, 0x72d7i32 as mp_digit, 0x72d9i32 as mp_digit,
     0x72e3i32 as mp_digit, 0x72efi32 as mp_digit, 0x72f5i32 as mp_digit,
     0x72fdi32 as mp_digit, 0x7303i32 as mp_digit, 0x730di32 as mp_digit,
     0x7321i32 as mp_digit, 0x732bi32 as mp_digit, 0x733di32 as mp_digit,
     0x7357i32 as mp_digit, 0x735bi32 as mp_digit, 0x7361i32 as mp_digit,
     0x737fi32 as mp_digit, 0x7381i32 as mp_digit, 0x7385i32 as mp_digit,
     0x738di32 as mp_digit, 0x7393i32 as mp_digit, 0x739fi32 as mp_digit,
     0x73abi32 as mp_digit, 0x73bdi32 as mp_digit, 0x73c1i32 as mp_digit,
     0x73c9i32 as mp_digit, 0x73dfi32 as mp_digit, 0x73e5i32 as mp_digit,
     0x73e7i32 as mp_digit, 0x73f3i32 as mp_digit, 0x7415i32 as mp_digit,
     0x741bi32 as mp_digit, 0x742di32 as mp_digit, 0x7439i32 as mp_digit,
     0x743fi32 as mp_digit, 0x7441i32 as mp_digit, 0x745di32 as mp_digit,
     0x746bi32 as mp_digit, 0x747bi32 as mp_digit, 0x7489i32 as mp_digit,
     0x748di32 as mp_digit, 0x749bi32 as mp_digit, 0x74a7i32 as mp_digit,
     0x74abi32 as mp_digit, 0x74b1i32 as mp_digit, 0x74b7i32 as mp_digit,
     0x74b9i32 as mp_digit, 0x74ddi32 as mp_digit, 0x74e1i32 as mp_digit,
     0x74e7i32 as mp_digit, 0x74fbi32 as mp_digit, 0x7507i32 as mp_digit,
     0x751fi32 as mp_digit, 0x7525i32 as mp_digit, 0x753bi32 as mp_digit,
     0x753di32 as mp_digit, 0x754di32 as mp_digit, 0x755fi32 as mp_digit,
     0x756bi32 as mp_digit, 0x7577i32 as mp_digit, 0x7589i32 as mp_digit,
     0x758bi32 as mp_digit, 0x7591i32 as mp_digit, 0x7597i32 as mp_digit,
     0x759di32 as mp_digit, 0x75a1i32 as mp_digit, 0x75a7i32 as mp_digit,
     0x75b5i32 as mp_digit, 0x75b9i32 as mp_digit, 0x75bbi32 as mp_digit,
     0x75d1i32 as mp_digit, 0x75d9i32 as mp_digit, 0x75e5i32 as mp_digit,
     0x75ebi32 as mp_digit, 0x75f5i32 as mp_digit, 0x75fbi32 as mp_digit,
     0x7603i32 as mp_digit, 0x760fi32 as mp_digit, 0x7621i32 as mp_digit,
     0x762di32 as mp_digit, 0x7633i32 as mp_digit, 0x763di32 as mp_digit,
     0x763fi32 as mp_digit, 0x7655i32 as mp_digit, 0x7663i32 as mp_digit,
     0x7669i32 as mp_digit, 0x766fi32 as mp_digit, 0x7673i32 as mp_digit,
     0x7685i32 as mp_digit, 0x768bi32 as mp_digit, 0x769fi32 as mp_digit,
     0x76b5i32 as mp_digit, 0x76b7i32 as mp_digit, 0x76c3i32 as mp_digit,
     0x76dbi32 as mp_digit, 0x76dfi32 as mp_digit, 0x76f1i32 as mp_digit,
     0x7703i32 as mp_digit, 0x7705i32 as mp_digit, 0x771bi32 as mp_digit,
     0x771di32 as mp_digit, 0x7721i32 as mp_digit, 0x772di32 as mp_digit,
     0x7735i32 as mp_digit, 0x7741i32 as mp_digit, 0x774bi32 as mp_digit,
     0x7759i32 as mp_digit, 0x775di32 as mp_digit, 0x775fi32 as mp_digit,
     0x7771i32 as mp_digit, 0x7781i32 as mp_digit, 0x77a7i32 as mp_digit,
     0x77adi32 as mp_digit, 0x77b3i32 as mp_digit, 0x77b9i32 as mp_digit,
     0x77c5i32 as mp_digit, 0x77cfi32 as mp_digit, 0x77d5i32 as mp_digit,
     0x77e1i32 as mp_digit, 0x77e9i32 as mp_digit, 0x77efi32 as mp_digit,
     0x77f3i32 as mp_digit, 0x77f9i32 as mp_digit, 0x7807i32 as mp_digit,
     0x7825i32 as mp_digit, 0x782bi32 as mp_digit, 0x7835i32 as mp_digit,
     0x783di32 as mp_digit, 0x7853i32 as mp_digit, 0x7859i32 as mp_digit,
     0x7861i32 as mp_digit, 0x786di32 as mp_digit, 0x7877i32 as mp_digit,
     0x7879i32 as mp_digit, 0x7883i32 as mp_digit, 0x7885i32 as mp_digit,
     0x788bi32 as mp_digit, 0x7895i32 as mp_digit, 0x7897i32 as mp_digit,
     0x78a1i32 as mp_digit, 0x78adi32 as mp_digit, 0x78bfi32 as mp_digit,
     0x78d3i32 as mp_digit, 0x78d9i32 as mp_digit, 0x78ddi32 as mp_digit,
     0x78e5i32 as mp_digit, 0x78fbi32 as mp_digit, 0x7901i32 as mp_digit,
     0x7907i32 as mp_digit, 0x7925i32 as mp_digit, 0x792bi32 as mp_digit,
     0x7939i32 as mp_digit, 0x793fi32 as mp_digit, 0x794bi32 as mp_digit,
     0x7957i32 as mp_digit, 0x795di32 as mp_digit, 0x7967i32 as mp_digit,
     0x7969i32 as mp_digit, 0x7973i32 as mp_digit, 0x7991i32 as mp_digit,
     0x7993i32 as mp_digit, 0x79a3i32 as mp_digit, 0x79abi32 as mp_digit,
     0x79afi32 as mp_digit, 0x79b1i32 as mp_digit, 0x79b7i32 as mp_digit,
     0x79c9i32 as mp_digit, 0x79cdi32 as mp_digit, 0x79cfi32 as mp_digit,
     0x79d5i32 as mp_digit, 0x79d9i32 as mp_digit, 0x79f3i32 as mp_digit,
     0x79f7i32 as mp_digit, 0x79ffi32 as mp_digit, 0x7a05i32 as mp_digit,
     0x7a0fi32 as mp_digit, 0x7a11i32 as mp_digit, 0x7a15i32 as mp_digit,
     0x7a1bi32 as mp_digit, 0x7a23i32 as mp_digit, 0x7a27i32 as mp_digit,
     0x7a2di32 as mp_digit, 0x7a4bi32 as mp_digit, 0x7a57i32 as mp_digit,
     0x7a59i32 as mp_digit, 0x7a5fi32 as mp_digit, 0x7a65i32 as mp_digit,
     0x7a69i32 as mp_digit, 0x7a7di32 as mp_digit, 0x7a93i32 as mp_digit,
     0x7a9bi32 as mp_digit, 0x7a9fi32 as mp_digit, 0x7aa1i32 as mp_digit,
     0x7aa5i32 as mp_digit, 0x7aedi32 as mp_digit, 0x7af5i32 as mp_digit,
     0x7af9i32 as mp_digit, 0x7b01i32 as mp_digit, 0x7b17i32 as mp_digit,
     0x7b19i32 as mp_digit, 0x7b1di32 as mp_digit, 0x7b2bi32 as mp_digit,
     0x7b35i32 as mp_digit, 0x7b37i32 as mp_digit, 0x7b3bi32 as mp_digit,
     0x7b4fi32 as mp_digit, 0x7b55i32 as mp_digit, 0x7b5fi32 as mp_digit,
     0x7b71i32 as mp_digit, 0x7b77i32 as mp_digit, 0x7b8bi32 as mp_digit,
     0x7b9bi32 as mp_digit, 0x7ba1i32 as mp_digit, 0x7ba9i32 as mp_digit,
     0x7bafi32 as mp_digit, 0x7bb3i32 as mp_digit, 0x7bc7i32 as mp_digit,
     0x7bd3i32 as mp_digit, 0x7be9i32 as mp_digit, 0x7bebi32 as mp_digit,
     0x7befi32 as mp_digit, 0x7bf1i32 as mp_digit, 0x7bfdi32 as mp_digit,
     0x7c07i32 as mp_digit, 0x7c19i32 as mp_digit, 0x7c1bi32 as mp_digit,
     0x7c31i32 as mp_digit, 0x7c37i32 as mp_digit, 0x7c49i32 as mp_digit,
     0x7c67i32 as mp_digit, 0x7c69i32 as mp_digit, 0x7c73i32 as mp_digit,
     0x7c81i32 as mp_digit, 0x7c8bi32 as mp_digit, 0x7c93i32 as mp_digit,
     0x7ca3i32 as mp_digit, 0x7cd5i32 as mp_digit, 0x7cdbi32 as mp_digit,
     0x7ce5i32 as mp_digit, 0x7cedi32 as mp_digit, 0x7cf7i32 as mp_digit,
     0x7d03i32 as mp_digit, 0x7d09i32 as mp_digit, 0x7d1bi32 as mp_digit,
     0x7d1di32 as mp_digit, 0x7d33i32 as mp_digit, 0x7d39i32 as mp_digit,
     0x7d3bi32 as mp_digit, 0x7d3fi32 as mp_digit, 0x7d45i32 as mp_digit,
     0x7d4di32 as mp_digit, 0x7d53i32 as mp_digit, 0x7d59i32 as mp_digit,
     0x7d63i32 as mp_digit, 0x7d75i32 as mp_digit, 0x7d77i32 as mp_digit,
     0x7d8di32 as mp_digit, 0x7d8fi32 as mp_digit, 0x7d9fi32 as mp_digit,
     0x7dadi32 as mp_digit, 0x7db7i32 as mp_digit, 0x7dbdi32 as mp_digit,
     0x7dbfi32 as mp_digit, 0x7dcbi32 as mp_digit, 0x7dd5i32 as mp_digit,
     0x7de9i32 as mp_digit, 0x7dedi32 as mp_digit, 0x7dfbi32 as mp_digit,
     0x7e01i32 as mp_digit, 0x7e05i32 as mp_digit, 0x7e29i32 as mp_digit,
     0x7e2bi32 as mp_digit, 0x7e2fi32 as mp_digit, 0x7e35i32 as mp_digit,
     0x7e41i32 as mp_digit, 0x7e43i32 as mp_digit, 0x7e47i32 as mp_digit,
     0x7e55i32 as mp_digit, 0x7e61i32 as mp_digit, 0x7e67i32 as mp_digit,
     0x7e6bi32 as mp_digit, 0x7e71i32 as mp_digit, 0x7e73i32 as mp_digit,
     0x7e79i32 as mp_digit, 0x7e7di32 as mp_digit, 0x7e91i32 as mp_digit,
     0x7e9bi32 as mp_digit, 0x7e9di32 as mp_digit, 0x7ea7i32 as mp_digit,
     0x7eadi32 as mp_digit, 0x7eb9i32 as mp_digit, 0x7ebbi32 as mp_digit,
     0x7ed3i32 as mp_digit, 0x7edfi32 as mp_digit, 0x7eebi32 as mp_digit,
     0x7ef1i32 as mp_digit, 0x7ef7i32 as mp_digit, 0x7efbi32 as mp_digit,
     0x7f13i32 as mp_digit, 0x7f15i32 as mp_digit, 0x7f19i32 as mp_digit,
     0x7f31i32 as mp_digit, 0x7f33i32 as mp_digit, 0x7f39i32 as mp_digit,
     0x7f3di32 as mp_digit, 0x7f43i32 as mp_digit, 0x7f4bi32 as mp_digit,
     0x7f5bi32 as mp_digit, 0x7f61i32 as mp_digit, 0x7f63i32 as mp_digit,
     0x7f6di32 as mp_digit, 0x7f79i32 as mp_digit, 0x7f87i32 as mp_digit,
     0x7f8di32 as mp_digit, 0x7fafi32 as mp_digit, 0x7fb5i32 as mp_digit,
     0x7fc3i32 as mp_digit, 0x7fc9i32 as mp_digit, 0x7fcdi32 as mp_digit,
     0x7fcfi32 as mp_digit, 0x7fedi32 as mp_digit, 0x8003i32 as mp_digit,
     0x800bi32 as mp_digit, 0x800fi32 as mp_digit, 0x8015i32 as mp_digit,
     0x801di32 as mp_digit, 0x8021i32 as mp_digit, 0x8023i32 as mp_digit,
     0x803fi32 as mp_digit, 0x8041i32 as mp_digit, 0x8047i32 as mp_digit,
     0x804bi32 as mp_digit, 0x8065i32 as mp_digit, 0x8077i32 as mp_digit,
     0x808di32 as mp_digit, 0x808fi32 as mp_digit, 0x8095i32 as mp_digit,
     0x80a5i32 as mp_digit, 0x80abi32 as mp_digit, 0x80adi32 as mp_digit,
     0x80bdi32 as mp_digit, 0x80c9i32 as mp_digit, 0x80cbi32 as mp_digit,
     0x80d7i32 as mp_digit, 0x80dbi32 as mp_digit, 0x80e1i32 as mp_digit,
     0x80e7i32 as mp_digit, 0x80f5i32 as mp_digit, 0x80ffi32 as mp_digit,
     0x8105i32 as mp_digit, 0x810di32 as mp_digit, 0x8119i32 as mp_digit,
     0x811di32 as mp_digit, 0x812fi32 as mp_digit, 0x8131i32 as mp_digit,
     0x813bi32 as mp_digit, 0x8143i32 as mp_digit, 0x8153i32 as mp_digit,
     0x8159i32 as mp_digit, 0x815fi32 as mp_digit, 0x817di32 as mp_digit,
     0x817fi32 as mp_digit, 0x8189i32 as mp_digit, 0x819bi32 as mp_digit,
     0x819di32 as mp_digit, 0x81a7i32 as mp_digit, 0x81afi32 as mp_digit,
     0x81b3i32 as mp_digit, 0x81bbi32 as mp_digit, 0x81c7i32 as mp_digit,
     0x81dfi32 as mp_digit, 0x8207i32 as mp_digit, 0x8209i32 as mp_digit,
     0x8215i32 as mp_digit, 0x821fi32 as mp_digit, 0x8225i32 as mp_digit,
     0x8231i32 as mp_digit, 0x8233i32 as mp_digit, 0x823fi32 as mp_digit,
     0x8243i32 as mp_digit, 0x8245i32 as mp_digit, 0x8249i32 as mp_digit,
     0x824fi32 as mp_digit, 0x8261i32 as mp_digit, 0x826fi32 as mp_digit,
     0x827bi32 as mp_digit, 0x8281i32 as mp_digit, 0x8285i32 as mp_digit,
     0x8293i32 as mp_digit, 0x82b1i32 as mp_digit, 0x82b5i32 as mp_digit,
     0x82bdi32 as mp_digit, 0x82c7i32 as mp_digit, 0x82cfi32 as mp_digit,
     0x82d5i32 as mp_digit, 0x82dfi32 as mp_digit, 0x82f1i32 as mp_digit,
     0x82f9i32 as mp_digit, 0x82fdi32 as mp_digit, 0x830bi32 as mp_digit,
     0x831bi32 as mp_digit, 0x8321i32 as mp_digit, 0x8329i32 as mp_digit,
     0x832di32 as mp_digit, 0x8333i32 as mp_digit, 0x8335i32 as mp_digit,
     0x833fi32 as mp_digit, 0x8341i32 as mp_digit, 0x834di32 as mp_digit,
     0x8351i32 as mp_digit, 0x8353i32 as mp_digit, 0x8357i32 as mp_digit,
     0x835di32 as mp_digit, 0x8365i32 as mp_digit, 0x8369i32 as mp_digit,
     0x836fi32 as mp_digit, 0x838fi32 as mp_digit, 0x83a7i32 as mp_digit,
     0x83b1i32 as mp_digit, 0x83b9i32 as mp_digit, 0x83cbi32 as mp_digit,
     0x83d5i32 as mp_digit, 0x83d7i32 as mp_digit, 0x83ddi32 as mp_digit,
     0x83e7i32 as mp_digit, 0x83e9i32 as mp_digit, 0x83edi32 as mp_digit,
     0x83ffi32 as mp_digit, 0x8405i32 as mp_digit, 0x8411i32 as mp_digit,
     0x8413i32 as mp_digit, 0x8423i32 as mp_digit, 0x8425i32 as mp_digit,
     0x843bi32 as mp_digit, 0x8441i32 as mp_digit, 0x8447i32 as mp_digit,
     0x844fi32 as mp_digit, 0x8461i32 as mp_digit, 0x8465i32 as mp_digit,
     0x8477i32 as mp_digit, 0x8483i32 as mp_digit, 0x848bi32 as mp_digit,
     0x8491i32 as mp_digit, 0x8495i32 as mp_digit, 0x84a9i32 as mp_digit,
     0x84afi32 as mp_digit, 0x84cdi32 as mp_digit, 0x84e3i32 as mp_digit,
     0x84efi32 as mp_digit, 0x84f1i32 as mp_digit, 0x84f7i32 as mp_digit,
     0x8509i32 as mp_digit, 0x850di32 as mp_digit, 0x854bi32 as mp_digit,
     0x854fi32 as mp_digit, 0x8551i32 as mp_digit, 0x855di32 as mp_digit,
     0x8563i32 as mp_digit, 0x856di32 as mp_digit, 0x856fi32 as mp_digit,
     0x857bi32 as mp_digit, 0x8587i32 as mp_digit, 0x85a3i32 as mp_digit,
     0x85a5i32 as mp_digit, 0x85a9i32 as mp_digit, 0x85b7i32 as mp_digit,
     0x85cdi32 as mp_digit, 0x85d3i32 as mp_digit, 0x85d5i32 as mp_digit,
     0x85dbi32 as mp_digit, 0x85e1i32 as mp_digit, 0x85ebi32 as mp_digit,
     0x85f9i32 as mp_digit, 0x85fdi32 as mp_digit, 0x85ffi32 as mp_digit,
     0x8609i32 as mp_digit, 0x860fi32 as mp_digit, 0x8617i32 as mp_digit,
     0x8621i32 as mp_digit, 0x862fi32 as mp_digit, 0x8639i32 as mp_digit,
     0x863fi32 as mp_digit, 0x8641i32 as mp_digit, 0x864di32 as mp_digit,
     0x8663i32 as mp_digit, 0x8675i32 as mp_digit, 0x867di32 as mp_digit,
     0x8687i32 as mp_digit, 0x8699i32 as mp_digit, 0x86a5i32 as mp_digit,
     0x86a7i32 as mp_digit, 0x86b3i32 as mp_digit, 0x86b7i32 as mp_digit,
     0x86c3i32 as mp_digit, 0x86c5i32 as mp_digit, 0x86cfi32 as mp_digit,
     0x86d1i32 as mp_digit, 0x86d7i32 as mp_digit, 0x86e9i32 as mp_digit,
     0x86efi32 as mp_digit, 0x86f5i32 as mp_digit, 0x8717i32 as mp_digit,
     0x871di32 as mp_digit, 0x871fi32 as mp_digit, 0x872bi32 as mp_digit,
     0x872fi32 as mp_digit, 0x8735i32 as mp_digit, 0x8747i32 as mp_digit,
     0x8759i32 as mp_digit, 0x875bi32 as mp_digit, 0x876bi32 as mp_digit,
     0x8771i32 as mp_digit, 0x8777i32 as mp_digit, 0x877fi32 as mp_digit,
     0x8785i32 as mp_digit, 0x878fi32 as mp_digit, 0x87a1i32 as mp_digit,
     0x87a9i32 as mp_digit, 0x87b3i32 as mp_digit, 0x87bbi32 as mp_digit,
     0x87c5i32 as mp_digit, 0x87c7i32 as mp_digit, 0x87cbi32 as mp_digit,
     0x87ddi32 as mp_digit, 0x87f7i32 as mp_digit, 0x8803i32 as mp_digit,
     0x8819i32 as mp_digit, 0x881bi32 as mp_digit, 0x881fi32 as mp_digit,
     0x8821i32 as mp_digit, 0x8837i32 as mp_digit, 0x883di32 as mp_digit,
     0x8843i32 as mp_digit, 0x8851i32 as mp_digit, 0x8861i32 as mp_digit,
     0x8867i32 as mp_digit, 0x887bi32 as mp_digit, 0x8885i32 as mp_digit,
     0x8891i32 as mp_digit, 0x8893i32 as mp_digit, 0x88a5i32 as mp_digit,
     0x88cfi32 as mp_digit, 0x88d3i32 as mp_digit, 0x88ebi32 as mp_digit,
     0x88edi32 as mp_digit, 0x88f3i32 as mp_digit, 0x88fdi32 as mp_digit,
     0x8909i32 as mp_digit, 0x890bi32 as mp_digit, 0x8911i32 as mp_digit,
     0x891bi32 as mp_digit, 0x8923i32 as mp_digit, 0x8927i32 as mp_digit,
     0x892di32 as mp_digit, 0x8939i32 as mp_digit, 0x8945i32 as mp_digit,
     0x894di32 as mp_digit, 0x8951i32 as mp_digit, 0x8957i32 as mp_digit,
     0x8963i32 as mp_digit, 0x8981i32 as mp_digit, 0x8995i32 as mp_digit,
     0x899bi32 as mp_digit, 0x89b3i32 as mp_digit, 0x89b9i32 as mp_digit,
     0x89c3i32 as mp_digit, 0x89cfi32 as mp_digit, 0x89d1i32 as mp_digit,
     0x89dbi32 as mp_digit, 0x89efi32 as mp_digit, 0x89f5i32 as mp_digit,
     0x89fbi32 as mp_digit, 0x89ffi32 as mp_digit, 0x8a0bi32 as mp_digit,
     0x8a19i32 as mp_digit, 0x8a23i32 as mp_digit, 0x8a35i32 as mp_digit,
     0x8a41i32 as mp_digit, 0x8a49i32 as mp_digit, 0x8a4fi32 as mp_digit,
     0x8a5bi32 as mp_digit, 0x8a5fi32 as mp_digit, 0x8a6di32 as mp_digit,
     0x8a77i32 as mp_digit, 0x8a79i32 as mp_digit, 0x8a85i32 as mp_digit,
     0x8aa3i32 as mp_digit, 0x8ab3i32 as mp_digit, 0x8ab5i32 as mp_digit,
     0x8ac1i32 as mp_digit, 0x8ac7i32 as mp_digit, 0x8acbi32 as mp_digit,
     0x8acdi32 as mp_digit, 0x8ad1i32 as mp_digit, 0x8ad7i32 as mp_digit,
     0x8af1i32 as mp_digit, 0x8af5i32 as mp_digit, 0x8b07i32 as mp_digit,
     0x8b09i32 as mp_digit, 0x8b0di32 as mp_digit, 0x8b13i32 as mp_digit,
     0x8b21i32 as mp_digit, 0x8b57i32 as mp_digit, 0x8b5di32 as mp_digit,
     0x8b91i32 as mp_digit, 0x8b93i32 as mp_digit, 0x8ba3i32 as mp_digit,
     0x8ba9i32 as mp_digit, 0x8bafi32 as mp_digit, 0x8bbbi32 as mp_digit,
     0x8bd5i32 as mp_digit, 0x8bd9i32 as mp_digit, 0x8bdbi32 as mp_digit,
     0x8be1i32 as mp_digit, 0x8bf7i32 as mp_digit, 0x8bfdi32 as mp_digit,
     0x8bffi32 as mp_digit, 0x8c0bi32 as mp_digit, 0x8c17i32 as mp_digit,
     0x8c1di32 as mp_digit, 0x8c27i32 as mp_digit, 0x8c39i32 as mp_digit,
     0x8c3bi32 as mp_digit, 0x8c47i32 as mp_digit, 0x8c53i32 as mp_digit,
     0x8c5di32 as mp_digit, 0x8c6fi32 as mp_digit, 0x8c7bi32 as mp_digit,
     0x8c81i32 as mp_digit, 0x8c89i32 as mp_digit, 0x8c8fi32 as mp_digit,
     0x8c99i32 as mp_digit, 0x8c9fi32 as mp_digit, 0x8ca7i32 as mp_digit,
     0x8cabi32 as mp_digit, 0x8cadi32 as mp_digit, 0x8cb1i32 as mp_digit,
     0x8cc5i32 as mp_digit, 0x8cddi32 as mp_digit, 0x8ce3i32 as mp_digit,
     0x8ce9i32 as mp_digit, 0x8cf3i32 as mp_digit, 0x8d01i32 as mp_digit,
     0x8d0bi32 as mp_digit, 0x8d0di32 as mp_digit, 0x8d23i32 as mp_digit,
     0x8d29i32 as mp_digit, 0x8d37i32 as mp_digit, 0x8d41i32 as mp_digit,
     0x8d5bi32 as mp_digit, 0x8d5fi32 as mp_digit, 0x8d71i32 as mp_digit,
     0x8d79i32 as mp_digit, 0x8d85i32 as mp_digit, 0x8d91i32 as mp_digit,
     0x8d9bi32 as mp_digit, 0x8da7i32 as mp_digit, 0x8dadi32 as mp_digit,
     0x8db5i32 as mp_digit, 0x8dc5i32 as mp_digit, 0x8dcbi32 as mp_digit,
     0x8dd3i32 as mp_digit, 0x8dd9i32 as mp_digit, 0x8ddfi32 as mp_digit,
     0x8df5i32 as mp_digit, 0x8df7i32 as mp_digit, 0x8e01i32 as mp_digit,
     0x8e15i32 as mp_digit, 0x8e1fi32 as mp_digit, 0x8e25i32 as mp_digit,
     0x8e51i32 as mp_digit, 0x8e63i32 as mp_digit, 0x8e69i32 as mp_digit,
     0x8e73i32 as mp_digit, 0x8e75i32 as mp_digit, 0x8e79i32 as mp_digit,
     0x8e7fi32 as mp_digit, 0x8e8di32 as mp_digit, 0x8e91i32 as mp_digit,
     0x8eabi32 as mp_digit, 0x8eafi32 as mp_digit, 0x8eb1i32 as mp_digit,
     0x8ebdi32 as mp_digit, 0x8ec7i32 as mp_digit, 0x8ecfi32 as mp_digit,
     0x8ed3i32 as mp_digit, 0x8edbi32 as mp_digit, 0x8ee7i32 as mp_digit,
     0x8eebi32 as mp_digit, 0x8ef7i32 as mp_digit, 0x8effi32 as mp_digit,
     0x8f15i32 as mp_digit, 0x8f1di32 as mp_digit, 0x8f23i32 as mp_digit,
     0x8f2di32 as mp_digit, 0x8f3fi32 as mp_digit, 0x8f45i32 as mp_digit,
     0x8f4bi32 as mp_digit, 0x8f53i32 as mp_digit, 0x8f59i32 as mp_digit,
     0x8f65i32 as mp_digit, 0x8f69i32 as mp_digit, 0x8f71i32 as mp_digit,
     0x8f83i32 as mp_digit, 0x8f8di32 as mp_digit, 0x8f99i32 as mp_digit,
     0x8f9fi32 as mp_digit, 0x8fabi32 as mp_digit, 0x8fadi32 as mp_digit,
     0x8fb3i32 as mp_digit, 0x8fb7i32 as mp_digit, 0x8fb9i32 as mp_digit,
     0x8fc9i32 as mp_digit, 0x8fd5i32 as mp_digit, 0x8fe1i32 as mp_digit,
     0x8fefi32 as mp_digit, 0x8ff9i32 as mp_digit, 0x9007i32 as mp_digit,
     0x900di32 as mp_digit, 0x9017i32 as mp_digit, 0x9023i32 as mp_digit,
     0x9025i32 as mp_digit, 0x9031i32 as mp_digit, 0x9037i32 as mp_digit,
     0x903bi32 as mp_digit, 0x9041i32 as mp_digit, 0x9043i32 as mp_digit,
     0x904fi32 as mp_digit, 0x9053i32 as mp_digit, 0x906di32 as mp_digit,
     0x9073i32 as mp_digit, 0x9085i32 as mp_digit, 0x908bi32 as mp_digit,
     0x9095i32 as mp_digit, 0x909bi32 as mp_digit, 0x909di32 as mp_digit,
     0x90afi32 as mp_digit, 0x90b9i32 as mp_digit, 0x90c1i32 as mp_digit,
     0x90c5i32 as mp_digit, 0x90dfi32 as mp_digit, 0x90e9i32 as mp_digit,
     0x90fdi32 as mp_digit, 0x9103i32 as mp_digit, 0x9113i32 as mp_digit,
     0x9127i32 as mp_digit, 0x9133i32 as mp_digit, 0x913di32 as mp_digit,
     0x9145i32 as mp_digit, 0x914fi32 as mp_digit, 0x9151i32 as mp_digit,
     0x9161i32 as mp_digit, 0x9167i32 as mp_digit, 0x917bi32 as mp_digit,
     0x9185i32 as mp_digit, 0x9199i32 as mp_digit, 0x919di32 as mp_digit,
     0x91bbi32 as mp_digit, 0x91bdi32 as mp_digit, 0x91c1i32 as mp_digit,
     0x91c9i32 as mp_digit, 0x91d9i32 as mp_digit, 0x91dbi32 as mp_digit,
     0x91edi32 as mp_digit, 0x91f1i32 as mp_digit, 0x91f3i32 as mp_digit,
     0x91f9i32 as mp_digit, 0x9203i32 as mp_digit, 0x9215i32 as mp_digit,
     0x9221i32 as mp_digit, 0x922fi32 as mp_digit, 0x9241i32 as mp_digit,
     0x9247i32 as mp_digit, 0x9257i32 as mp_digit, 0x926bi32 as mp_digit,
     0x9271i32 as mp_digit, 0x9275i32 as mp_digit, 0x927di32 as mp_digit,
     0x9283i32 as mp_digit, 0x9287i32 as mp_digit, 0x928di32 as mp_digit,
     0x9299i32 as mp_digit, 0x92a1i32 as mp_digit, 0x92abi32 as mp_digit,
     0x92adi32 as mp_digit, 0x92b9i32 as mp_digit, 0x92bfi32 as mp_digit,
     0x92c3i32 as mp_digit, 0x92c5i32 as mp_digit, 0x92cbi32 as mp_digit,
     0x92d5i32 as mp_digit, 0x92d7i32 as mp_digit, 0x92e7i32 as mp_digit,
     0x92f3i32 as mp_digit, 0x9301i32 as mp_digit, 0x930bi32 as mp_digit,
     0x9311i32 as mp_digit, 0x9319i32 as mp_digit, 0x931fi32 as mp_digit,
     0x933bi32 as mp_digit, 0x933di32 as mp_digit, 0x9343i32 as mp_digit,
     0x9355i32 as mp_digit, 0x9373i32 as mp_digit, 0x9395i32 as mp_digit,
     0x9397i32 as mp_digit, 0x93a7i32 as mp_digit, 0x93b3i32 as mp_digit,
     0x93b5i32 as mp_digit, 0x93c7i32 as mp_digit, 0x93d7i32 as mp_digit,
     0x93ddi32 as mp_digit, 0x93e5i32 as mp_digit, 0x93efi32 as mp_digit,
     0x93f7i32 as mp_digit, 0x9401i32 as mp_digit, 0x9409i32 as mp_digit,
     0x9413i32 as mp_digit, 0x943fi32 as mp_digit, 0x9445i32 as mp_digit,
     0x944bi32 as mp_digit, 0x944fi32 as mp_digit, 0x9463i32 as mp_digit,
     0x9467i32 as mp_digit, 0x9469i32 as mp_digit, 0x946di32 as mp_digit,
     0x947bi32 as mp_digit, 0x9497i32 as mp_digit, 0x949fi32 as mp_digit,
     0x94a5i32 as mp_digit, 0x94b5i32 as mp_digit, 0x94c3i32 as mp_digit,
     0x94e1i32 as mp_digit, 0x94e7i32 as mp_digit, 0x9505i32 as mp_digit,
     0x9509i32 as mp_digit, 0x9517i32 as mp_digit, 0x9521i32 as mp_digit,
     0x9527i32 as mp_digit, 0x952di32 as mp_digit, 0x9535i32 as mp_digit,
     0x9539i32 as mp_digit, 0x954bi32 as mp_digit, 0x9557i32 as mp_digit,
     0x955di32 as mp_digit, 0x955fi32 as mp_digit, 0x9575i32 as mp_digit,
     0x9581i32 as mp_digit, 0x9589i32 as mp_digit, 0x958fi32 as mp_digit,
     0x959bi32 as mp_digit, 0x959fi32 as mp_digit, 0x95adi32 as mp_digit,
     0x95b1i32 as mp_digit, 0x95b7i32 as mp_digit, 0x95b9i32 as mp_digit,
     0x95bdi32 as mp_digit, 0x95cfi32 as mp_digit, 0x95e3i32 as mp_digit,
     0x95e9i32 as mp_digit, 0x95f9i32 as mp_digit, 0x961fi32 as mp_digit,
     0x962fi32 as mp_digit, 0x9631i32 as mp_digit, 0x9635i32 as mp_digit,
     0x963bi32 as mp_digit, 0x963di32 as mp_digit, 0x9665i32 as mp_digit,
     0x968fi32 as mp_digit, 0x969di32 as mp_digit, 0x96a1i32 as mp_digit,
     0x96a7i32 as mp_digit, 0x96a9i32 as mp_digit, 0x96c1i32 as mp_digit,
     0x96cbi32 as mp_digit, 0x96d1i32 as mp_digit, 0x96d3i32 as mp_digit,
     0x96e5i32 as mp_digit, 0x96efi32 as mp_digit, 0x96fbi32 as mp_digit,
     0x96fdi32 as mp_digit, 0x970di32 as mp_digit, 0x970fi32 as mp_digit,
     0x9715i32 as mp_digit, 0x9725i32 as mp_digit, 0x972bi32 as mp_digit,
     0x9733i32 as mp_digit, 0x9737i32 as mp_digit, 0x9739i32 as mp_digit,
     0x9743i32 as mp_digit, 0x9749i32 as mp_digit, 0x9751i32 as mp_digit,
     0x975bi32 as mp_digit, 0x975di32 as mp_digit, 0x976fi32 as mp_digit,
     0x977fi32 as mp_digit, 0x9787i32 as mp_digit, 0x9793i32 as mp_digit,
     0x97a5i32 as mp_digit, 0x97b1i32 as mp_digit, 0x97b7i32 as mp_digit,
     0x97c3i32 as mp_digit, 0x97cdi32 as mp_digit, 0x97d3i32 as mp_digit,
     0x97d9i32 as mp_digit, 0x97ebi32 as mp_digit, 0x97f7i32 as mp_digit,
     0x9805i32 as mp_digit, 0x9809i32 as mp_digit, 0x980bi32 as mp_digit,
     0x9815i32 as mp_digit, 0x9829i32 as mp_digit, 0x982fi32 as mp_digit,
     0x983bi32 as mp_digit, 0x9841i32 as mp_digit, 0x9851i32 as mp_digit,
     0x986bi32 as mp_digit, 0x986fi32 as mp_digit, 0x9881i32 as mp_digit,
     0x9883i32 as mp_digit, 0x9887i32 as mp_digit, 0x98a7i32 as mp_digit,
     0x98b1i32 as mp_digit, 0x98b9i32 as mp_digit, 0x98bfi32 as mp_digit,
     0x98c3i32 as mp_digit, 0x98c9i32 as mp_digit, 0x98cfi32 as mp_digit,
     0x98ddi32 as mp_digit, 0x98e3i32 as mp_digit, 0x98f5i32 as mp_digit,
     0x98f9i32 as mp_digit, 0x98fbi32 as mp_digit, 0x990di32 as mp_digit,
     0x9917i32 as mp_digit, 0x991fi32 as mp_digit, 0x9929i32 as mp_digit,
     0x9931i32 as mp_digit, 0x993bi32 as mp_digit, 0x993di32 as mp_digit,
     0x9941i32 as mp_digit, 0x9947i32 as mp_digit, 0x9949i32 as mp_digit,
     0x9953i32 as mp_digit, 0x997di32 as mp_digit, 0x9985i32 as mp_digit,
     0x9991i32 as mp_digit, 0x9995i32 as mp_digit, 0x999bi32 as mp_digit,
     0x99adi32 as mp_digit, 0x99afi32 as mp_digit, 0x99bfi32 as mp_digit,
     0x99c7i32 as mp_digit, 0x99cbi32 as mp_digit, 0x99cdi32 as mp_digit,
     0x99d7i32 as mp_digit, 0x99e5i32 as mp_digit, 0x99f1i32 as mp_digit,
     0x99fbi32 as mp_digit, 0x9a0fi32 as mp_digit, 0x9a13i32 as mp_digit,
     0x9a1bi32 as mp_digit, 0x9a25i32 as mp_digit, 0x9a4bi32 as mp_digit,
     0x9a4fi32 as mp_digit, 0x9a55i32 as mp_digit, 0x9a57i32 as mp_digit,
     0x9a61i32 as mp_digit, 0x9a75i32 as mp_digit, 0x9a7fi32 as mp_digit,
     0x9a8bi32 as mp_digit, 0x9a91i32 as mp_digit, 0x9a9di32 as mp_digit,
     0x9ab7i32 as mp_digit, 0x9ac3i32 as mp_digit, 0x9ac7i32 as mp_digit,
     0x9acfi32 as mp_digit, 0x9aebi32 as mp_digit, 0x9af3i32 as mp_digit,
     0x9af7i32 as mp_digit, 0x9affi32 as mp_digit, 0x9b17i32 as mp_digit,
     0x9b1di32 as mp_digit, 0x9b27i32 as mp_digit, 0x9b2fi32 as mp_digit,
     0x9b35i32 as mp_digit, 0x9b45i32 as mp_digit, 0x9b51i32 as mp_digit,
     0x9b59i32 as mp_digit, 0x9b63i32 as mp_digit, 0x9b6fi32 as mp_digit,
     0x9b77i32 as mp_digit, 0x9b8di32 as mp_digit, 0x9b93i32 as mp_digit,
     0x9b95i32 as mp_digit, 0x9b9fi32 as mp_digit, 0x9ba1i32 as mp_digit,
     0x9ba7i32 as mp_digit, 0x9bb1i32 as mp_digit, 0x9bb7i32 as mp_digit,
     0x9bbdi32 as mp_digit, 0x9bc5i32 as mp_digit, 0x9bcbi32 as mp_digit,
     0x9bcfi32 as mp_digit, 0x9bddi32 as mp_digit, 0x9bf9i32 as mp_digit,
     0x9c01i32 as mp_digit, 0x9c11i32 as mp_digit, 0x9c23i32 as mp_digit,
     0x9c2bi32 as mp_digit, 0x9c2fi32 as mp_digit, 0x9c35i32 as mp_digit,
     0x9c49i32 as mp_digit, 0x9c4di32 as mp_digit, 0x9c5fi32 as mp_digit,
     0x9c65i32 as mp_digit, 0x9c67i32 as mp_digit, 0x9c7fi32 as mp_digit,
     0x9c97i32 as mp_digit, 0x9c9di32 as mp_digit, 0x9ca3i32 as mp_digit,
     0x9cafi32 as mp_digit, 0x9cbbi32 as mp_digit, 0x9cbfi32 as mp_digit,
     0x9cc1i32 as mp_digit, 0x9cd7i32 as mp_digit, 0x9cd9i32 as mp_digit,
     0x9ce3i32 as mp_digit, 0x9ce9i32 as mp_digit, 0x9cf1i32 as mp_digit,
     0x9cfdi32 as mp_digit, 0x9d01i32 as mp_digit, 0x9d15i32 as mp_digit,
     0x9d27i32 as mp_digit, 0x9d2di32 as mp_digit, 0x9d31i32 as mp_digit,
     0x9d3di32 as mp_digit, 0x9d55i32 as mp_digit, 0x9d5bi32 as mp_digit,
     0x9d61i32 as mp_digit, 0x9d97i32 as mp_digit, 0x9d9fi32 as mp_digit,
     0x9da5i32 as mp_digit, 0x9da9i32 as mp_digit, 0x9dc3i32 as mp_digit,
     0x9de7i32 as mp_digit, 0x9debi32 as mp_digit, 0x9dedi32 as mp_digit,
     0x9df1i32 as mp_digit, 0x9e0bi32 as mp_digit, 0x9e17i32 as mp_digit,
     0x9e23i32 as mp_digit, 0x9e27i32 as mp_digit, 0x9e2di32 as mp_digit,
     0x9e33i32 as mp_digit, 0x9e3bi32 as mp_digit, 0x9e47i32 as mp_digit,
     0x9e51i32 as mp_digit, 0x9e53i32 as mp_digit, 0x9e5fi32 as mp_digit,
     0x9e6fi32 as mp_digit, 0x9e81i32 as mp_digit, 0x9e87i32 as mp_digit,
     0x9e8fi32 as mp_digit, 0x9e95i32 as mp_digit, 0x9ea1i32 as mp_digit,
     0x9eb3i32 as mp_digit, 0x9ebdi32 as mp_digit, 0x9ebfi32 as mp_digit,
     0x9ef5i32 as mp_digit, 0x9ef9i32 as mp_digit, 0x9efbi32 as mp_digit,
     0x9f05i32 as mp_digit, 0x9f23i32 as mp_digit, 0x9f2fi32 as mp_digit,
     0x9f37i32 as mp_digit, 0x9f3bi32 as mp_digit, 0x9f43i32 as mp_digit,
     0x9f53i32 as mp_digit, 0x9f61i32 as mp_digit, 0x9f6di32 as mp_digit,
     0x9f73i32 as mp_digit, 0x9f77i32 as mp_digit, 0x9f7di32 as mp_digit,
     0x9f89i32 as mp_digit, 0x9f8fi32 as mp_digit, 0x9f91i32 as mp_digit,
     0x9f95i32 as mp_digit, 0x9fa3i32 as mp_digit, 0x9fafi32 as mp_digit,
     0x9fb3i32 as mp_digit, 0x9fc1i32 as mp_digit, 0x9fc7i32 as mp_digit,
     0x9fdfi32 as mp_digit, 0x9fe5i32 as mp_digit, 0x9febi32 as mp_digit,
     0x9ff5i32 as mp_digit, 0xa001i32 as mp_digit, 0xa00di32 as mp_digit,
     0xa021i32 as mp_digit, 0xa033i32 as mp_digit, 0xa039i32 as mp_digit,
     0xa03fi32 as mp_digit, 0xa04fi32 as mp_digit, 0xa057i32 as mp_digit,
     0xa05bi32 as mp_digit, 0xa061i32 as mp_digit, 0xa075i32 as mp_digit,
     0xa079i32 as mp_digit, 0xa099i32 as mp_digit, 0xa09di32 as mp_digit,
     0xa0abi32 as mp_digit, 0xa0b5i32 as mp_digit, 0xa0b7i32 as mp_digit,
     0xa0bdi32 as mp_digit, 0xa0c9i32 as mp_digit, 0xa0d9i32 as mp_digit,
     0xa0dbi32 as mp_digit, 0xa0dfi32 as mp_digit, 0xa0e5i32 as mp_digit,
     0xa0f1i32 as mp_digit, 0xa0f3i32 as mp_digit, 0xa0fdi32 as mp_digit,
     0xa105i32 as mp_digit, 0xa10bi32 as mp_digit, 0xa10fi32 as mp_digit,
     0xa111i32 as mp_digit, 0xa11bi32 as mp_digit, 0xa129i32 as mp_digit,
     0xa12fi32 as mp_digit, 0xa135i32 as mp_digit, 0xa141i32 as mp_digit,
     0xa153i32 as mp_digit, 0xa175i32 as mp_digit, 0xa17di32 as mp_digit,
     0xa187i32 as mp_digit, 0xa18di32 as mp_digit, 0xa1a5i32 as mp_digit,
     0xa1abi32 as mp_digit, 0xa1adi32 as mp_digit, 0xa1b7i32 as mp_digit,
     0xa1c3i32 as mp_digit, 0xa1c5i32 as mp_digit, 0xa1e3i32 as mp_digit,
     0xa1edi32 as mp_digit, 0xa1fbi32 as mp_digit, 0xa207i32 as mp_digit,
     0xa213i32 as mp_digit, 0xa223i32 as mp_digit, 0xa229i32 as mp_digit,
     0xa22fi32 as mp_digit, 0xa231i32 as mp_digit, 0xa243i32 as mp_digit,
     0xa247i32 as mp_digit, 0xa24di32 as mp_digit, 0xa26bi32 as mp_digit,
     0xa279i32 as mp_digit, 0xa27di32 as mp_digit, 0xa283i32 as mp_digit,
     0xa289i32 as mp_digit, 0xa28bi32 as mp_digit, 0xa291i32 as mp_digit,
     0xa295i32 as mp_digit, 0xa29bi32 as mp_digit, 0xa2a9i32 as mp_digit,
     0xa2afi32 as mp_digit, 0xa2b3i32 as mp_digit, 0xa2bbi32 as mp_digit,
     0xa2c5i32 as mp_digit, 0xa2d1i32 as mp_digit, 0xa2d7i32 as mp_digit,
     0xa2f7i32 as mp_digit, 0xa301i32 as mp_digit, 0xa309i32 as mp_digit,
     0xa31fi32 as mp_digit, 0xa321i32 as mp_digit, 0xa32bi32 as mp_digit,
     0xa331i32 as mp_digit, 0xa349i32 as mp_digit, 0xa351i32 as mp_digit,
     0xa355i32 as mp_digit, 0xa373i32 as mp_digit, 0xa379i32 as mp_digit,
     0xa37bi32 as mp_digit, 0xa387i32 as mp_digit, 0xa397i32 as mp_digit,
     0xa39fi32 as mp_digit, 0xa3a5i32 as mp_digit, 0xa3a9i32 as mp_digit,
     0xa3afi32 as mp_digit, 0xa3b7i32 as mp_digit, 0xa3c7i32 as mp_digit,
     0xa3d5i32 as mp_digit, 0xa3dbi32 as mp_digit, 0xa3e1i32 as mp_digit,
     0xa3e5i32 as mp_digit, 0xa3e7i32 as mp_digit, 0xa3f1i32 as mp_digit,
     0xa3fdi32 as mp_digit, 0xa3ffi32 as mp_digit, 0xa40fi32 as mp_digit,
     0xa41di32 as mp_digit, 0xa421i32 as mp_digit, 0xa423i32 as mp_digit,
     0xa427i32 as mp_digit, 0xa43bi32 as mp_digit, 0xa44di32 as mp_digit,
     0xa457i32 as mp_digit, 0xa459i32 as mp_digit, 0xa463i32 as mp_digit,
     0xa469i32 as mp_digit, 0xa475i32 as mp_digit, 0xa493i32 as mp_digit,
     0xa49bi32 as mp_digit, 0xa4adi32 as mp_digit, 0xa4b9i32 as mp_digit,
     0xa4c3i32 as mp_digit, 0xa4c5i32 as mp_digit, 0xa4cbi32 as mp_digit,
     0xa4d1i32 as mp_digit, 0xa4d5i32 as mp_digit, 0xa4e1i32 as mp_digit,
     0xa4edi32 as mp_digit, 0xa4efi32 as mp_digit, 0xa4f3i32 as mp_digit,
     0xa4ffi32 as mp_digit, 0xa511i32 as mp_digit, 0xa529i32 as mp_digit,
     0xa52bi32 as mp_digit, 0xa535i32 as mp_digit, 0xa53bi32 as mp_digit,
     0xa543i32 as mp_digit, 0xa553i32 as mp_digit, 0xa55bi32 as mp_digit,
     0xa561i32 as mp_digit, 0xa56di32 as mp_digit, 0xa577i32 as mp_digit,
     0xa585i32 as mp_digit, 0xa58bi32 as mp_digit, 0xa597i32 as mp_digit,
     0xa59di32 as mp_digit, 0xa5a3i32 as mp_digit, 0xa5a7i32 as mp_digit,
     0xa5a9i32 as mp_digit, 0xa5c1i32 as mp_digit, 0xa5c5i32 as mp_digit,
     0xa5cbi32 as mp_digit, 0xa5d3i32 as mp_digit, 0xa5d9i32 as mp_digit,
     0xa5ddi32 as mp_digit, 0xa5dfi32 as mp_digit, 0xa5e3i32 as mp_digit,
     0xa5e9i32 as mp_digit, 0xa5f7i32 as mp_digit, 0xa5fbi32 as mp_digit,
     0xa603i32 as mp_digit, 0xa60di32 as mp_digit, 0xa625i32 as mp_digit,
     0xa63di32 as mp_digit, 0xa649i32 as mp_digit, 0xa64bi32 as mp_digit,
     0xa651i32 as mp_digit, 0xa65di32 as mp_digit, 0xa673i32 as mp_digit,
     0xa691i32 as mp_digit, 0xa693i32 as mp_digit, 0xa699i32 as mp_digit,
     0xa6abi32 as mp_digit, 0xa6b5i32 as mp_digit, 0xa6bbi32 as mp_digit,
     0xa6c1i32 as mp_digit, 0xa6c9i32 as mp_digit, 0xa6cdi32 as mp_digit,
     0xa6cfi32 as mp_digit, 0xa6d5i32 as mp_digit, 0xa6dfi32 as mp_digit,
     0xa6e7i32 as mp_digit, 0xa6f1i32 as mp_digit, 0xa6f7i32 as mp_digit,
     0xa6ffi32 as mp_digit, 0xa70fi32 as mp_digit, 0xa715i32 as mp_digit,
     0xa723i32 as mp_digit, 0xa729i32 as mp_digit, 0xa72di32 as mp_digit,
     0xa745i32 as mp_digit, 0xa74di32 as mp_digit, 0xa757i32 as mp_digit,
     0xa759i32 as mp_digit, 0xa765i32 as mp_digit, 0xa76bi32 as mp_digit,
     0xa76fi32 as mp_digit, 0xa793i32 as mp_digit, 0xa795i32 as mp_digit,
     0xa7abi32 as mp_digit, 0xa7b1i32 as mp_digit, 0xa7b9i32 as mp_digit,
     0xa7bfi32 as mp_digit, 0xa7c9i32 as mp_digit, 0xa7d1i32 as mp_digit,
     0xa7d7i32 as mp_digit, 0xa7e3i32 as mp_digit, 0xa7edi32 as mp_digit,
     0xa7fbi32 as mp_digit, 0xa805i32 as mp_digit, 0xa80bi32 as mp_digit,
     0xa81di32 as mp_digit, 0xa829i32 as mp_digit, 0xa82bi32 as mp_digit,
     0xa837i32 as mp_digit, 0xa83bi32 as mp_digit, 0xa855i32 as mp_digit,
     0xa85fi32 as mp_digit, 0xa86di32 as mp_digit, 0xa87di32 as mp_digit,
     0xa88fi32 as mp_digit, 0xa897i32 as mp_digit, 0xa8a9i32 as mp_digit,
     0xa8b5i32 as mp_digit, 0xa8c1i32 as mp_digit, 0xa8c7i32 as mp_digit,
     0xa8d7i32 as mp_digit, 0xa8e5i32 as mp_digit, 0xa8fdi32 as mp_digit,
     0xa907i32 as mp_digit, 0xa913i32 as mp_digit, 0xa91bi32 as mp_digit,
     0xa931i32 as mp_digit, 0xa937i32 as mp_digit, 0xa939i32 as mp_digit,
     0xa943i32 as mp_digit, 0xa97fi32 as mp_digit, 0xa985i32 as mp_digit,
     0xa987i32 as mp_digit, 0xa98bi32 as mp_digit, 0xa993i32 as mp_digit,
     0xa9a3i32 as mp_digit, 0xa9b1i32 as mp_digit, 0xa9bbi32 as mp_digit,
     0xa9c1i32 as mp_digit, 0xa9d9i32 as mp_digit, 0xa9dfi32 as mp_digit,
     0xa9ebi32 as mp_digit, 0xa9fdi32 as mp_digit, 0xaa15i32 as mp_digit,
     0xaa17i32 as mp_digit, 0xaa35i32 as mp_digit, 0xaa39i32 as mp_digit,
     0xaa3bi32 as mp_digit, 0xaa47i32 as mp_digit, 0xaa4di32 as mp_digit,
     0xaa57i32 as mp_digit, 0xaa59i32 as mp_digit, 0xaa5di32 as mp_digit,
     0xaa6bi32 as mp_digit, 0xaa71i32 as mp_digit, 0xaa81i32 as mp_digit,
     0xaa83i32 as mp_digit, 0xaa8di32 as mp_digit, 0xaa95i32 as mp_digit,
     0xaaabi32 as mp_digit, 0xaabfi32 as mp_digit, 0xaac5i32 as mp_digit,
     0xaac9i32 as mp_digit, 0xaae9i32 as mp_digit, 0xaaefi32 as mp_digit,
     0xab01i32 as mp_digit, 0xab05i32 as mp_digit, 0xab07i32 as mp_digit,
     0xab0bi32 as mp_digit, 0xab0di32 as mp_digit, 0xab11i32 as mp_digit,
     0xab19i32 as mp_digit, 0xab4di32 as mp_digit, 0xab5bi32 as mp_digit,
     0xab71i32 as mp_digit, 0xab73i32 as mp_digit, 0xab89i32 as mp_digit,
     0xab9di32 as mp_digit, 0xaba7i32 as mp_digit, 0xabafi32 as mp_digit,
     0xabb9i32 as mp_digit, 0xabbbi32 as mp_digit, 0xabc1i32 as mp_digit,
     0xabc5i32 as mp_digit, 0xabd3i32 as mp_digit, 0xabd7i32 as mp_digit,
     0xabddi32 as mp_digit, 0xabf1i32 as mp_digit, 0xabf5i32 as mp_digit,
     0xabfbi32 as mp_digit, 0xabfdi32 as mp_digit, 0xac09i32 as mp_digit,
     0xac15i32 as mp_digit, 0xac1bi32 as mp_digit, 0xac27i32 as mp_digit,
     0xac37i32 as mp_digit, 0xac39i32 as mp_digit, 0xac45i32 as mp_digit,
     0xac4fi32 as mp_digit, 0xac57i32 as mp_digit, 0xac5bi32 as mp_digit,
     0xac61i32 as mp_digit, 0xac63i32 as mp_digit, 0xac7fi32 as mp_digit,
     0xac8bi32 as mp_digit, 0xac93i32 as mp_digit, 0xac9di32 as mp_digit,
     0xaca9i32 as mp_digit, 0xacabi32 as mp_digit, 0xacafi32 as mp_digit,
     0xacbdi32 as mp_digit, 0xacd9i32 as mp_digit, 0xace1i32 as mp_digit,
     0xace7i32 as mp_digit, 0xacebi32 as mp_digit, 0xacedi32 as mp_digit,
     0xacf1i32 as mp_digit, 0xacf7i32 as mp_digit, 0xacf9i32 as mp_digit,
     0xad05i32 as mp_digit, 0xad3fi32 as mp_digit, 0xad45i32 as mp_digit,
     0xad53i32 as mp_digit, 0xad5di32 as mp_digit, 0xad5fi32 as mp_digit,
     0xad65i32 as mp_digit, 0xad81i32 as mp_digit, 0xada1i32 as mp_digit,
     0xada5i32 as mp_digit, 0xadc3i32 as mp_digit, 0xadcbi32 as mp_digit,
     0xadd1i32 as mp_digit, 0xadd5i32 as mp_digit, 0xaddbi32 as mp_digit,
     0xade7i32 as mp_digit, 0xadf3i32 as mp_digit, 0xadf5i32 as mp_digit,
     0xadf9i32 as mp_digit, 0xadffi32 as mp_digit, 0xae05i32 as mp_digit,
     0xae13i32 as mp_digit, 0xae23i32 as mp_digit, 0xae2bi32 as mp_digit,
     0xae49i32 as mp_digit, 0xae4di32 as mp_digit, 0xae4fi32 as mp_digit,
     0xae59i32 as mp_digit, 0xae61i32 as mp_digit, 0xae67i32 as mp_digit,
     0xae6bi32 as mp_digit, 0xae71i32 as mp_digit, 0xae8bi32 as mp_digit,
     0xae8fi32 as mp_digit, 0xae9bi32 as mp_digit, 0xae9di32 as mp_digit,
     0xaea7i32 as mp_digit, 0xaeb9i32 as mp_digit, 0xaec5i32 as mp_digit,
     0xaed1i32 as mp_digit, 0xaee3i32 as mp_digit, 0xaee5i32 as mp_digit,
     0xaee9i32 as mp_digit, 0xaef5i32 as mp_digit, 0xaefdi32 as mp_digit,
     0xaf09i32 as mp_digit, 0xaf13i32 as mp_digit, 0xaf27i32 as mp_digit,
     0xaf2bi32 as mp_digit, 0xaf33i32 as mp_digit, 0xaf43i32 as mp_digit,
     0xaf4fi32 as mp_digit, 0xaf57i32 as mp_digit, 0xaf5di32 as mp_digit,
     0xaf6di32 as mp_digit, 0xaf75i32 as mp_digit, 0xaf7fi32 as mp_digit,
     0xaf8bi32 as mp_digit, 0xaf99i32 as mp_digit, 0xaf9fi32 as mp_digit,
     0xafa3i32 as mp_digit, 0xafabi32 as mp_digit, 0xafb7i32 as mp_digit,
     0xafbbi32 as mp_digit, 0xafcfi32 as mp_digit, 0xafd5i32 as mp_digit,
     0xaffdi32 as mp_digit, 0xb005i32 as mp_digit, 0xb015i32 as mp_digit,
     0xb01bi32 as mp_digit, 0xb03fi32 as mp_digit, 0xb041i32 as mp_digit,
     0xb047i32 as mp_digit, 0xb04bi32 as mp_digit, 0xb051i32 as mp_digit,
     0xb053i32 as mp_digit, 0xb069i32 as mp_digit, 0xb07bi32 as mp_digit,
     0xb07di32 as mp_digit, 0xb087i32 as mp_digit, 0xb08di32 as mp_digit,
     0xb0b1i32 as mp_digit, 0xb0bfi32 as mp_digit, 0xb0cbi32 as mp_digit,
     0xb0cfi32 as mp_digit, 0xb0e1i32 as mp_digit, 0xb0e9i32 as mp_digit,
     0xb0edi32 as mp_digit, 0xb0fbi32 as mp_digit, 0xb105i32 as mp_digit,
     0xb107i32 as mp_digit, 0xb111i32 as mp_digit, 0xb119i32 as mp_digit,
     0xb11di32 as mp_digit, 0xb11fi32 as mp_digit, 0xb131i32 as mp_digit,
     0xb141i32 as mp_digit, 0xb14di32 as mp_digit, 0xb15bi32 as mp_digit,
     0xb165i32 as mp_digit, 0xb173i32 as mp_digit, 0xb179i32 as mp_digit,
     0xb17fi32 as mp_digit, 0xb1a9i32 as mp_digit, 0xb1b3i32 as mp_digit,
     0xb1b9i32 as mp_digit, 0xb1bfi32 as mp_digit, 0xb1d3i32 as mp_digit,
     0xb1ddi32 as mp_digit, 0xb1e5i32 as mp_digit, 0xb1f1i32 as mp_digit,
     0xb1f5i32 as mp_digit, 0xb201i32 as mp_digit, 0xb213i32 as mp_digit,
     0xb215i32 as mp_digit, 0xb21fi32 as mp_digit, 0xb22di32 as mp_digit,
     0xb23fi32 as mp_digit, 0xb249i32 as mp_digit, 0xb25bi32 as mp_digit,
     0xb263i32 as mp_digit, 0xb269i32 as mp_digit, 0xb26di32 as mp_digit,
     0xb27bi32 as mp_digit, 0xb281i32 as mp_digit, 0xb28bi32 as mp_digit,
     0xb2a9i32 as mp_digit, 0xb2b7i32 as mp_digit, 0xb2bdi32 as mp_digit,
     0xb2c3i32 as mp_digit, 0xb2c7i32 as mp_digit, 0xb2d3i32 as mp_digit,
     0xb2f9i32 as mp_digit, 0xb2fdi32 as mp_digit, 0xb2ffi32 as mp_digit,
     0xb303i32 as mp_digit, 0xb309i32 as mp_digit, 0xb311i32 as mp_digit,
     0xb31di32 as mp_digit, 0xb327i32 as mp_digit, 0xb32di32 as mp_digit,
     0xb33fi32 as mp_digit, 0xb345i32 as mp_digit, 0xb377i32 as mp_digit,
     0xb37di32 as mp_digit, 0xb381i32 as mp_digit, 0xb387i32 as mp_digit,
     0xb393i32 as mp_digit, 0xb39bi32 as mp_digit, 0xb3a5i32 as mp_digit,
     0xb3c5i32 as mp_digit, 0xb3cbi32 as mp_digit, 0xb3e1i32 as mp_digit,
     0xb3e3i32 as mp_digit, 0xb3edi32 as mp_digit, 0xb3f9i32 as mp_digit,
     0xb40bi32 as mp_digit, 0xb40di32 as mp_digit, 0xb413i32 as mp_digit,
     0xb417i32 as mp_digit, 0xb435i32 as mp_digit, 0xb43di32 as mp_digit,
     0xb443i32 as mp_digit, 0xb449i32 as mp_digit, 0xb45bi32 as mp_digit,
     0xb465i32 as mp_digit, 0xb467i32 as mp_digit, 0xb46bi32 as mp_digit,
     0xb477i32 as mp_digit, 0xb48bi32 as mp_digit, 0xb495i32 as mp_digit,
     0xb49di32 as mp_digit, 0xb4b5i32 as mp_digit, 0xb4bfi32 as mp_digit,
     0xb4c1i32 as mp_digit, 0xb4c7i32 as mp_digit, 0xb4ddi32 as mp_digit,
     0xb4e3i32 as mp_digit, 0xb4e5i32 as mp_digit, 0xb4f7i32 as mp_digit,
     0xb501i32 as mp_digit, 0xb50di32 as mp_digit, 0xb50fi32 as mp_digit,
     0xb52di32 as mp_digit, 0xb53fi32 as mp_digit, 0xb54bi32 as mp_digit,
     0xb567i32 as mp_digit, 0xb569i32 as mp_digit, 0xb56fi32 as mp_digit,
     0xb573i32 as mp_digit, 0xb579i32 as mp_digit, 0xb587i32 as mp_digit,
     0xb58di32 as mp_digit, 0xb599i32 as mp_digit, 0xb5a3i32 as mp_digit,
     0xb5abi32 as mp_digit, 0xb5afi32 as mp_digit, 0xb5bbi32 as mp_digit,
     0xb5d5i32 as mp_digit, 0xb5dfi32 as mp_digit, 0xb5e7i32 as mp_digit,
     0xb5edi32 as mp_digit, 0xb5fdi32 as mp_digit, 0xb5ffi32 as mp_digit,
     0xb609i32 as mp_digit, 0xb61bi32 as mp_digit, 0xb629i32 as mp_digit,
     0xb62fi32 as mp_digit, 0xb633i32 as mp_digit, 0xb639i32 as mp_digit,
     0xb647i32 as mp_digit, 0xb657i32 as mp_digit, 0xb659i32 as mp_digit,
     0xb65fi32 as mp_digit, 0xb663i32 as mp_digit, 0xb66fi32 as mp_digit,
     0xb683i32 as mp_digit, 0xb687i32 as mp_digit, 0xb69bi32 as mp_digit,
     0xb69fi32 as mp_digit, 0xb6a5i32 as mp_digit, 0xb6b1i32 as mp_digit,
     0xb6b3i32 as mp_digit, 0xb6d7i32 as mp_digit, 0xb6dbi32 as mp_digit,
     0xb6e1i32 as mp_digit, 0xb6e3i32 as mp_digit, 0xb6edi32 as mp_digit,
     0xb6efi32 as mp_digit, 0xb705i32 as mp_digit, 0xb70di32 as mp_digit,
     0xb713i32 as mp_digit, 0xb71di32 as mp_digit, 0xb729i32 as mp_digit,
     0xb735i32 as mp_digit, 0xb747i32 as mp_digit, 0xb755i32 as mp_digit,
     0xb76di32 as mp_digit, 0xb791i32 as mp_digit, 0xb795i32 as mp_digit,
     0xb7a9i32 as mp_digit, 0xb7c1i32 as mp_digit, 0xb7cbi32 as mp_digit,
     0xb7d1i32 as mp_digit, 0xb7d3i32 as mp_digit, 0xb7efi32 as mp_digit,
     0xb7f5i32 as mp_digit, 0xb807i32 as mp_digit, 0xb80fi32 as mp_digit,
     0xb813i32 as mp_digit, 0xb819i32 as mp_digit, 0xb821i32 as mp_digit,
     0xb827i32 as mp_digit, 0xb82bi32 as mp_digit, 0xb82di32 as mp_digit,
     0xb839i32 as mp_digit, 0xb855i32 as mp_digit, 0xb867i32 as mp_digit,
     0xb875i32 as mp_digit, 0xb885i32 as mp_digit, 0xb893i32 as mp_digit,
     0xb8a5i32 as mp_digit, 0xb8afi32 as mp_digit, 0xb8b7i32 as mp_digit,
     0xb8bdi32 as mp_digit, 0xb8c1i32 as mp_digit, 0xb8c7i32 as mp_digit,
     0xb8cdi32 as mp_digit, 0xb8d5i32 as mp_digit, 0xb8ebi32 as mp_digit,
     0xb8f7i32 as mp_digit, 0xb8f9i32 as mp_digit, 0xb903i32 as mp_digit,
     0xb915i32 as mp_digit, 0xb91bi32 as mp_digit, 0xb91di32 as mp_digit,
     0xb92fi32 as mp_digit, 0xb939i32 as mp_digit, 0xb93bi32 as mp_digit,
     0xb947i32 as mp_digit, 0xb951i32 as mp_digit, 0xb963i32 as mp_digit,
     0xb983i32 as mp_digit, 0xb989i32 as mp_digit, 0xb98di32 as mp_digit,
     0xb993i32 as mp_digit, 0xb999i32 as mp_digit, 0xb9a1i32 as mp_digit,
     0xb9a7i32 as mp_digit, 0xb9adi32 as mp_digit, 0xb9b7i32 as mp_digit,
     0xb9cbi32 as mp_digit, 0xb9d1i32 as mp_digit, 0xb9ddi32 as mp_digit,
     0xb9e7i32 as mp_digit, 0xb9efi32 as mp_digit, 0xb9f9i32 as mp_digit,
     0xba07i32 as mp_digit, 0xba0di32 as mp_digit, 0xba17i32 as mp_digit,
     0xba25i32 as mp_digit, 0xba29i32 as mp_digit, 0xba2bi32 as mp_digit,
     0xba41i32 as mp_digit, 0xba53i32 as mp_digit, 0xba55i32 as mp_digit,
     0xba5fi32 as mp_digit, 0xba61i32 as mp_digit, 0xba65i32 as mp_digit,
     0xba79i32 as mp_digit, 0xba7di32 as mp_digit, 0xba7fi32 as mp_digit,
     0xbaa1i32 as mp_digit, 0xbaa3i32 as mp_digit, 0xbaafi32 as mp_digit,
     0xbab5i32 as mp_digit, 0xbabfi32 as mp_digit, 0xbac1i32 as mp_digit,
     0xbacbi32 as mp_digit, 0xbaddi32 as mp_digit, 0xbae3i32 as mp_digit,
     0xbaf1i32 as mp_digit, 0xbafdi32 as mp_digit, 0xbb09i32 as mp_digit,
     0xbb1fi32 as mp_digit, 0xbb27i32 as mp_digit, 0xbb2di32 as mp_digit,
     0xbb3di32 as mp_digit, 0xbb43i32 as mp_digit, 0xbb4bi32 as mp_digit,
     0xbb4fi32 as mp_digit, 0xbb5bi32 as mp_digit, 0xbb61i32 as mp_digit,
     0xbb69i32 as mp_digit, 0xbb6di32 as mp_digit, 0xbb91i32 as mp_digit,
     0xbb97i32 as mp_digit, 0xbb9di32 as mp_digit, 0xbbb1i32 as mp_digit,
     0xbbc9i32 as mp_digit, 0xbbcfi32 as mp_digit, 0xbbdbi32 as mp_digit,
     0xbbedi32 as mp_digit, 0xbbf7i32 as mp_digit, 0xbbf9i32 as mp_digit,
     0xbc03i32 as mp_digit, 0xbc1di32 as mp_digit, 0xbc23i32 as mp_digit,
     0xbc33i32 as mp_digit, 0xbc3bi32 as mp_digit, 0xbc41i32 as mp_digit,
     0xbc45i32 as mp_digit, 0xbc5di32 as mp_digit, 0xbc6fi32 as mp_digit,
     0xbc77i32 as mp_digit, 0xbc83i32 as mp_digit, 0xbc8fi32 as mp_digit,
     0xbc99i32 as mp_digit, 0xbcabi32 as mp_digit, 0xbcb7i32 as mp_digit,
     0xbcb9i32 as mp_digit, 0xbcd1i32 as mp_digit, 0xbcd5i32 as mp_digit,
     0xbce1i32 as mp_digit, 0xbcf3i32 as mp_digit, 0xbcffi32 as mp_digit,
     0xbd0di32 as mp_digit, 0xbd17i32 as mp_digit, 0xbd19i32 as mp_digit,
     0xbd1di32 as mp_digit, 0xbd35i32 as mp_digit, 0xbd41i32 as mp_digit,
     0xbd4fi32 as mp_digit, 0xbd59i32 as mp_digit, 0xbd5fi32 as mp_digit,
     0xbd61i32 as mp_digit, 0xbd67i32 as mp_digit, 0xbd6bi32 as mp_digit,
     0xbd71i32 as mp_digit, 0xbd8bi32 as mp_digit, 0xbd8fi32 as mp_digit,
     0xbd95i32 as mp_digit, 0xbd9bi32 as mp_digit, 0xbd9di32 as mp_digit,
     0xbdb3i32 as mp_digit, 0xbdbbi32 as mp_digit, 0xbdcdi32 as mp_digit,
     0xbdd1i32 as mp_digit, 0xbde3i32 as mp_digit, 0xbdebi32 as mp_digit,
     0xbdefi32 as mp_digit, 0xbe07i32 as mp_digit, 0xbe09i32 as mp_digit,
     0xbe15i32 as mp_digit, 0xbe21i32 as mp_digit, 0xbe25i32 as mp_digit,
     0xbe27i32 as mp_digit, 0xbe5bi32 as mp_digit, 0xbe5di32 as mp_digit,
     0xbe6fi32 as mp_digit, 0xbe75i32 as mp_digit, 0xbe79i32 as mp_digit,
     0xbe7fi32 as mp_digit, 0xbe8bi32 as mp_digit, 0xbe8di32 as mp_digit,
     0xbe93i32 as mp_digit, 0xbe9fi32 as mp_digit, 0xbea9i32 as mp_digit,
     0xbeb1i32 as mp_digit, 0xbeb5i32 as mp_digit, 0xbeb7i32 as mp_digit,
     0xbecfi32 as mp_digit, 0xbed9i32 as mp_digit, 0xbedbi32 as mp_digit,
     0xbee5i32 as mp_digit, 0xbee7i32 as mp_digit, 0xbef3i32 as mp_digit,
     0xbef9i32 as mp_digit, 0xbf0bi32 as mp_digit, 0xbf33i32 as mp_digit,
     0xbf39i32 as mp_digit, 0xbf4di32 as mp_digit, 0xbf5di32 as mp_digit,
     0xbf5fi32 as mp_digit, 0xbf6bi32 as mp_digit, 0xbf71i32 as mp_digit,
     0xbf7bi32 as mp_digit, 0xbf87i32 as mp_digit, 0xbf89i32 as mp_digit,
     0xbf8di32 as mp_digit, 0xbf93i32 as mp_digit, 0xbfa1i32 as mp_digit,
     0xbfadi32 as mp_digit, 0xbfb9i32 as mp_digit, 0xbfcfi32 as mp_digit,
     0xbfd5i32 as mp_digit, 0xbfddi32 as mp_digit, 0xbfe1i32 as mp_digit,
     0xbfe3i32 as mp_digit, 0xbff3i32 as mp_digit, 0xc005i32 as mp_digit,
     0xc011i32 as mp_digit, 0xc013i32 as mp_digit, 0xc019i32 as mp_digit,
     0xc029i32 as mp_digit, 0xc02fi32 as mp_digit, 0xc031i32 as mp_digit,
     0xc037i32 as mp_digit, 0xc03bi32 as mp_digit, 0xc047i32 as mp_digit,
     0xc065i32 as mp_digit, 0xc06di32 as mp_digit, 0xc07di32 as mp_digit,
     0xc07fi32 as mp_digit, 0xc091i32 as mp_digit, 0xc09bi32 as mp_digit,
     0xc0b3i32 as mp_digit, 0xc0b5i32 as mp_digit, 0xc0bbi32 as mp_digit,
     0xc0d3i32 as mp_digit, 0xc0d7i32 as mp_digit, 0xc0d9i32 as mp_digit,
     0xc0efi32 as mp_digit, 0xc0f1i32 as mp_digit, 0xc101i32 as mp_digit,
     0xc103i32 as mp_digit, 0xc109i32 as mp_digit, 0xc115i32 as mp_digit,
     0xc119i32 as mp_digit, 0xc12bi32 as mp_digit, 0xc133i32 as mp_digit,
     0xc137i32 as mp_digit, 0xc145i32 as mp_digit, 0xc149i32 as mp_digit,
     0xc15bi32 as mp_digit, 0xc173i32 as mp_digit, 0xc179i32 as mp_digit,
     0xc17bi32 as mp_digit, 0xc181i32 as mp_digit, 0xc18bi32 as mp_digit,
     0xc18di32 as mp_digit, 0xc197i32 as mp_digit, 0xc1bdi32 as mp_digit,
     0xc1c3i32 as mp_digit, 0xc1cdi32 as mp_digit, 0xc1dbi32 as mp_digit,
     0xc1e1i32 as mp_digit, 0xc1e7i32 as mp_digit, 0xc1ffi32 as mp_digit,
     0xc203i32 as mp_digit, 0xc205i32 as mp_digit, 0xc211i32 as mp_digit,
     0xc221i32 as mp_digit, 0xc22fi32 as mp_digit, 0xc23fi32 as mp_digit,
     0xc24bi32 as mp_digit, 0xc24di32 as mp_digit, 0xc253i32 as mp_digit,
     0xc25di32 as mp_digit, 0xc277i32 as mp_digit, 0xc27bi32 as mp_digit,
     0xc27di32 as mp_digit, 0xc289i32 as mp_digit, 0xc28fi32 as mp_digit,
     0xc293i32 as mp_digit, 0xc29fi32 as mp_digit, 0xc2a7i32 as mp_digit,
     0xc2b3i32 as mp_digit, 0xc2bdi32 as mp_digit, 0xc2cfi32 as mp_digit,
     0xc2d5i32 as mp_digit, 0xc2e3i32 as mp_digit, 0xc2ffi32 as mp_digit,
     0xc301i32 as mp_digit, 0xc307i32 as mp_digit, 0xc311i32 as mp_digit,
     0xc313i32 as mp_digit, 0xc317i32 as mp_digit, 0xc325i32 as mp_digit,
     0xc347i32 as mp_digit, 0xc349i32 as mp_digit, 0xc34fi32 as mp_digit,
     0xc365i32 as mp_digit, 0xc367i32 as mp_digit, 0xc371i32 as mp_digit,
     0xc37fi32 as mp_digit, 0xc383i32 as mp_digit, 0xc385i32 as mp_digit,
     0xc395i32 as mp_digit, 0xc39di32 as mp_digit, 0xc3a7i32 as mp_digit,
     0xc3adi32 as mp_digit, 0xc3b5i32 as mp_digit, 0xc3bfi32 as mp_digit,
     0xc3c7i32 as mp_digit, 0xc3cbi32 as mp_digit, 0xc3d1i32 as mp_digit,
     0xc3d3i32 as mp_digit, 0xc3e3i32 as mp_digit, 0xc3e9i32 as mp_digit,
     0xc3efi32 as mp_digit, 0xc401i32 as mp_digit, 0xc41fi32 as mp_digit,
     0xc42di32 as mp_digit, 0xc433i32 as mp_digit, 0xc437i32 as mp_digit,
     0xc455i32 as mp_digit, 0xc457i32 as mp_digit, 0xc461i32 as mp_digit,
     0xc46fi32 as mp_digit, 0xc473i32 as mp_digit, 0xc487i32 as mp_digit,
     0xc491i32 as mp_digit, 0xc499i32 as mp_digit, 0xc49di32 as mp_digit,
     0xc4a5i32 as mp_digit, 0xc4b7i32 as mp_digit, 0xc4bbi32 as mp_digit,
     0xc4c9i32 as mp_digit, 0xc4cfi32 as mp_digit, 0xc4d3i32 as mp_digit,
     0xc4ebi32 as mp_digit, 0xc4f1i32 as mp_digit, 0xc4f7i32 as mp_digit,
     0xc509i32 as mp_digit, 0xc51bi32 as mp_digit, 0xc51di32 as mp_digit,
     0xc541i32 as mp_digit, 0xc547i32 as mp_digit, 0xc551i32 as mp_digit,
     0xc55fi32 as mp_digit, 0xc56bi32 as mp_digit, 0xc56fi32 as mp_digit,
     0xc575i32 as mp_digit, 0xc577i32 as mp_digit, 0xc595i32 as mp_digit,
     0xc59bi32 as mp_digit, 0xc59fi32 as mp_digit, 0xc5a1i32 as mp_digit,
     0xc5a7i32 as mp_digit, 0xc5c3i32 as mp_digit, 0xc5d7i32 as mp_digit,
     0xc5dbi32 as mp_digit, 0xc5efi32 as mp_digit, 0xc5fbi32 as mp_digit,
     0xc613i32 as mp_digit, 0xc623i32 as mp_digit, 0xc635i32 as mp_digit,
     0xc641i32 as mp_digit, 0xc64fi32 as mp_digit, 0xc655i32 as mp_digit,
     0xc659i32 as mp_digit, 0xc665i32 as mp_digit, 0xc685i32 as mp_digit,
     0xc691i32 as mp_digit, 0xc697i32 as mp_digit, 0xc6a1i32 as mp_digit,
     0xc6a9i32 as mp_digit, 0xc6b3i32 as mp_digit, 0xc6b9i32 as mp_digit,
     0xc6cbi32 as mp_digit, 0xc6cdi32 as mp_digit, 0xc6ddi32 as mp_digit,
     0xc6ebi32 as mp_digit, 0xc6f1i32 as mp_digit, 0xc707i32 as mp_digit,
     0xc70di32 as mp_digit, 0xc719i32 as mp_digit, 0xc71bi32 as mp_digit,
     0xc72di32 as mp_digit, 0xc731i32 as mp_digit, 0xc739i32 as mp_digit,
     0xc757i32 as mp_digit, 0xc763i32 as mp_digit, 0xc767i32 as mp_digit,
     0xc773i32 as mp_digit, 0xc775i32 as mp_digit, 0xc77fi32 as mp_digit,
     0xc7a5i32 as mp_digit, 0xc7bbi32 as mp_digit, 0xc7bdi32 as mp_digit,
     0xc7c1i32 as mp_digit, 0xc7cfi32 as mp_digit, 0xc7d5i32 as mp_digit,
     0xc7e1i32 as mp_digit, 0xc7f9i32 as mp_digit, 0xc7fdi32 as mp_digit,
     0xc7ffi32 as mp_digit, 0xc803i32 as mp_digit, 0xc811i32 as mp_digit,
     0xc81di32 as mp_digit, 0xc827i32 as mp_digit, 0xc829i32 as mp_digit,
     0xc839i32 as mp_digit, 0xc83fi32 as mp_digit, 0xc853i32 as mp_digit,
     0xc857i32 as mp_digit, 0xc86bi32 as mp_digit, 0xc881i32 as mp_digit,
     0xc88di32 as mp_digit, 0xc88fi32 as mp_digit, 0xc893i32 as mp_digit,
     0xc895i32 as mp_digit, 0xc8a1i32 as mp_digit, 0xc8b7i32 as mp_digit,
     0xc8cfi32 as mp_digit, 0xc8d5i32 as mp_digit, 0xc8dbi32 as mp_digit,
     0xc8ddi32 as mp_digit, 0xc8e3i32 as mp_digit, 0xc8e7i32 as mp_digit,
     0xc8edi32 as mp_digit, 0xc8efi32 as mp_digit, 0xc8f9i32 as mp_digit,
     0xc905i32 as mp_digit, 0xc911i32 as mp_digit, 0xc917i32 as mp_digit,
     0xc919i32 as mp_digit, 0xc91fi32 as mp_digit, 0xc92fi32 as mp_digit,
     0xc937i32 as mp_digit, 0xc93di32 as mp_digit, 0xc941i32 as mp_digit,
     0xc953i32 as mp_digit, 0xc95fi32 as mp_digit, 0xc96bi32 as mp_digit,
     0xc979i32 as mp_digit, 0xc97di32 as mp_digit, 0xc989i32 as mp_digit,
     0xc98fi32 as mp_digit, 0xc997i32 as mp_digit, 0xc99di32 as mp_digit,
     0xc9afi32 as mp_digit, 0xc9b5i32 as mp_digit, 0xc9bfi32 as mp_digit,
     0xc9cbi32 as mp_digit, 0xc9d9i32 as mp_digit, 0xc9dfi32 as mp_digit,
     0xc9e3i32 as mp_digit, 0xc9ebi32 as mp_digit, 0xca01i32 as mp_digit,
     0xca07i32 as mp_digit, 0xca09i32 as mp_digit, 0xca25i32 as mp_digit,
     0xca37i32 as mp_digit, 0xca39i32 as mp_digit, 0xca4bi32 as mp_digit,
     0xca55i32 as mp_digit, 0xca5bi32 as mp_digit, 0xca69i32 as mp_digit,
     0xca73i32 as mp_digit, 0xca75i32 as mp_digit, 0xca7fi32 as mp_digit,
     0xca8di32 as mp_digit, 0xca93i32 as mp_digit, 0xca9di32 as mp_digit,
     0xca9fi32 as mp_digit, 0xcab5i32 as mp_digit, 0xcabbi32 as mp_digit,
     0xcac3i32 as mp_digit, 0xcac9i32 as mp_digit, 0xcad9i32 as mp_digit,
     0xcae5i32 as mp_digit, 0xcaedi32 as mp_digit, 0xcb03i32 as mp_digit,
     0xcb05i32 as mp_digit, 0xcb09i32 as mp_digit, 0xcb17i32 as mp_digit,
     0xcb29i32 as mp_digit, 0xcb35i32 as mp_digit, 0xcb3bi32 as mp_digit,
     0xcb53i32 as mp_digit, 0xcb59i32 as mp_digit, 0xcb63i32 as mp_digit,
     0xcb65i32 as mp_digit, 0xcb71i32 as mp_digit, 0xcb87i32 as mp_digit,
     0xcb99i32 as mp_digit, 0xcb9fi32 as mp_digit, 0xcbb3i32 as mp_digit,
     0xcbb9i32 as mp_digit, 0xcbc3i32 as mp_digit, 0xcbd1i32 as mp_digit,
     0xcbd5i32 as mp_digit, 0xcbd7i32 as mp_digit, 0xcbddi32 as mp_digit,
     0xcbe9i32 as mp_digit, 0xcbffi32 as mp_digit, 0xcc0di32 as mp_digit,
     0xcc19i32 as mp_digit, 0xcc1di32 as mp_digit, 0xcc23i32 as mp_digit,
     0xcc2bi32 as mp_digit, 0xcc41i32 as mp_digit, 0xcc43i32 as mp_digit,
     0xcc4di32 as mp_digit, 0xcc59i32 as mp_digit, 0xcc61i32 as mp_digit,
     0xcc89i32 as mp_digit, 0xcc8bi32 as mp_digit, 0xcc91i32 as mp_digit,
     0xcc9bi32 as mp_digit, 0xcca3i32 as mp_digit, 0xcca7i32 as mp_digit,
     0xccd1i32 as mp_digit, 0xcce5i32 as mp_digit, 0xcce9i32 as mp_digit,
     0xcd09i32 as mp_digit, 0xcd15i32 as mp_digit, 0xcd1fi32 as mp_digit,
     0xcd25i32 as mp_digit, 0xcd31i32 as mp_digit, 0xcd3di32 as mp_digit,
     0xcd3fi32 as mp_digit, 0xcd49i32 as mp_digit, 0xcd51i32 as mp_digit,
     0xcd57i32 as mp_digit, 0xcd5bi32 as mp_digit, 0xcd63i32 as mp_digit,
     0xcd67i32 as mp_digit, 0xcd81i32 as mp_digit, 0xcd93i32 as mp_digit,
     0xcd97i32 as mp_digit, 0xcd9fi32 as mp_digit, 0xcdbbi32 as mp_digit,
     0xcdc1i32 as mp_digit, 0xcdd3i32 as mp_digit, 0xcdd9i32 as mp_digit,
     0xcde5i32 as mp_digit, 0xcde7i32 as mp_digit, 0xcdf1i32 as mp_digit,
     0xcdf7i32 as mp_digit, 0xcdfdi32 as mp_digit, 0xce0bi32 as mp_digit,
     0xce15i32 as mp_digit, 0xce21i32 as mp_digit, 0xce2fi32 as mp_digit,
     0xce47i32 as mp_digit, 0xce4di32 as mp_digit, 0xce51i32 as mp_digit,
     0xce65i32 as mp_digit, 0xce7bi32 as mp_digit, 0xce7di32 as mp_digit,
     0xce8fi32 as mp_digit, 0xce93i32 as mp_digit, 0xce99i32 as mp_digit,
     0xcea5i32 as mp_digit, 0xcea7i32 as mp_digit, 0xceb7i32 as mp_digit,
     0xcec9i32 as mp_digit, 0xced7i32 as mp_digit, 0xceddi32 as mp_digit,
     0xcee3i32 as mp_digit, 0xcee7i32 as mp_digit, 0xceedi32 as mp_digit,
     0xcef5i32 as mp_digit, 0xcf07i32 as mp_digit, 0xcf0bi32 as mp_digit,
     0xcf19i32 as mp_digit, 0xcf37i32 as mp_digit, 0xcf3bi32 as mp_digit,
     0xcf4di32 as mp_digit, 0xcf55i32 as mp_digit, 0xcf5fi32 as mp_digit,
     0xcf61i32 as mp_digit, 0xcf65i32 as mp_digit, 0xcf6di32 as mp_digit,
     0xcf79i32 as mp_digit, 0xcf7di32 as mp_digit, 0xcf89i32 as mp_digit,
     0xcf9bi32 as mp_digit, 0xcf9di32 as mp_digit, 0xcfa9i32 as mp_digit,
     0xcfb3i32 as mp_digit, 0xcfb5i32 as mp_digit, 0xcfc5i32 as mp_digit,
     0xcfcdi32 as mp_digit, 0xcfd1i32 as mp_digit, 0xcfefi32 as mp_digit,
     0xcff1i32 as mp_digit, 0xcff7i32 as mp_digit, 0xd013i32 as mp_digit,
     0xd015i32 as mp_digit, 0xd01fi32 as mp_digit, 0xd021i32 as mp_digit,
     0xd033i32 as mp_digit, 0xd03di32 as mp_digit, 0xd04bi32 as mp_digit,
     0xd04fi32 as mp_digit, 0xd069i32 as mp_digit, 0xd06fi32 as mp_digit,
     0xd081i32 as mp_digit, 0xd085i32 as mp_digit, 0xd099i32 as mp_digit,
     0xd09fi32 as mp_digit, 0xd0a3i32 as mp_digit, 0xd0abi32 as mp_digit,
     0xd0bdi32 as mp_digit, 0xd0c1i32 as mp_digit, 0xd0cdi32 as mp_digit,
     0xd0e7i32 as mp_digit, 0xd0ffi32 as mp_digit, 0xd103i32 as mp_digit,
     0xd117i32 as mp_digit, 0xd12di32 as mp_digit, 0xd12fi32 as mp_digit,
     0xd141i32 as mp_digit, 0xd157i32 as mp_digit, 0xd159i32 as mp_digit,
     0xd15di32 as mp_digit, 0xd169i32 as mp_digit, 0xd16bi32 as mp_digit,
     0xd171i32 as mp_digit, 0xd177i32 as mp_digit, 0xd17di32 as mp_digit,
     0xd181i32 as mp_digit, 0xd187i32 as mp_digit, 0xd195i32 as mp_digit,
     0xd199i32 as mp_digit, 0xd1b1i32 as mp_digit, 0xd1bdi32 as mp_digit,
     0xd1c3i32 as mp_digit, 0xd1d5i32 as mp_digit, 0xd1d7i32 as mp_digit,
     0xd1e3i32 as mp_digit, 0xd1ffi32 as mp_digit, 0xd20di32 as mp_digit,
     0xd211i32 as mp_digit, 0xd217i32 as mp_digit, 0xd21fi32 as mp_digit,
     0xd235i32 as mp_digit, 0xd23bi32 as mp_digit, 0xd247i32 as mp_digit,
     0xd259i32 as mp_digit, 0xd261i32 as mp_digit, 0xd265i32 as mp_digit,
     0xd279i32 as mp_digit, 0xd27fi32 as mp_digit, 0xd283i32 as mp_digit,
     0xd289i32 as mp_digit, 0xd28bi32 as mp_digit, 0xd29di32 as mp_digit,
     0xd2a3i32 as mp_digit, 0xd2a7i32 as mp_digit, 0xd2b3i32 as mp_digit,
     0xd2bfi32 as mp_digit, 0xd2c7i32 as mp_digit, 0xd2e3i32 as mp_digit,
     0xd2e9i32 as mp_digit, 0xd2f1i32 as mp_digit, 0xd2fbi32 as mp_digit,
     0xd2fdi32 as mp_digit, 0xd315i32 as mp_digit, 0xd321i32 as mp_digit,
     0xd32bi32 as mp_digit, 0xd343i32 as mp_digit, 0xd34bi32 as mp_digit,
     0xd355i32 as mp_digit, 0xd369i32 as mp_digit, 0xd375i32 as mp_digit,
     0xd37bi32 as mp_digit, 0xd387i32 as mp_digit, 0xd393i32 as mp_digit,
     0xd397i32 as mp_digit, 0xd3a5i32 as mp_digit, 0xd3b1i32 as mp_digit,
     0xd3c9i32 as mp_digit, 0xd3ebi32 as mp_digit, 0xd3fdi32 as mp_digit,
     0xd405i32 as mp_digit, 0xd40fi32 as mp_digit, 0xd415i32 as mp_digit,
     0xd427i32 as mp_digit, 0xd42fi32 as mp_digit, 0xd433i32 as mp_digit,
     0xd43bi32 as mp_digit, 0xd44bi32 as mp_digit, 0xd459i32 as mp_digit,
     0xd45fi32 as mp_digit, 0xd463i32 as mp_digit, 0xd469i32 as mp_digit,
     0xd481i32 as mp_digit, 0xd483i32 as mp_digit, 0xd489i32 as mp_digit,
     0xd48di32 as mp_digit, 0xd493i32 as mp_digit, 0xd495i32 as mp_digit,
     0xd4a5i32 as mp_digit, 0xd4abi32 as mp_digit, 0xd4b1i32 as mp_digit,
     0xd4c5i32 as mp_digit, 0xd4ddi32 as mp_digit, 0xd4e1i32 as mp_digit,
     0xd4e3i32 as mp_digit, 0xd4e7i32 as mp_digit, 0xd4f5i32 as mp_digit,
     0xd4f9i32 as mp_digit, 0xd50bi32 as mp_digit, 0xd50di32 as mp_digit,
     0xd513i32 as mp_digit, 0xd51fi32 as mp_digit, 0xd523i32 as mp_digit,
     0xd531i32 as mp_digit, 0xd535i32 as mp_digit, 0xd537i32 as mp_digit,
     0xd549i32 as mp_digit, 0xd559i32 as mp_digit, 0xd55fi32 as mp_digit,
     0xd565i32 as mp_digit, 0xd567i32 as mp_digit, 0xd577i32 as mp_digit,
     0xd58bi32 as mp_digit, 0xd591i32 as mp_digit, 0xd597i32 as mp_digit,
     0xd5b5i32 as mp_digit, 0xd5b9i32 as mp_digit, 0xd5c1i32 as mp_digit,
     0xd5c7i32 as mp_digit, 0xd5dfi32 as mp_digit, 0xd5efi32 as mp_digit,
     0xd5f5i32 as mp_digit, 0xd5fbi32 as mp_digit, 0xd603i32 as mp_digit,
     0xd60fi32 as mp_digit, 0xd62di32 as mp_digit, 0xd631i32 as mp_digit,
     0xd643i32 as mp_digit, 0xd655i32 as mp_digit, 0xd65di32 as mp_digit,
     0xd661i32 as mp_digit, 0xd67bi32 as mp_digit, 0xd685i32 as mp_digit,
     0xd687i32 as mp_digit, 0xd69di32 as mp_digit, 0xd6a5i32 as mp_digit,
     0xd6afi32 as mp_digit, 0xd6bdi32 as mp_digit, 0xd6c3i32 as mp_digit,
     0xd6c7i32 as mp_digit, 0xd6d9i32 as mp_digit, 0xd6e1i32 as mp_digit,
     0xd6edi32 as mp_digit, 0xd709i32 as mp_digit, 0xd70bi32 as mp_digit,
     0xd711i32 as mp_digit, 0xd715i32 as mp_digit, 0xd721i32 as mp_digit,
     0xd727i32 as mp_digit, 0xd73fi32 as mp_digit, 0xd745i32 as mp_digit,
     0xd74di32 as mp_digit, 0xd757i32 as mp_digit, 0xd76bi32 as mp_digit,
     0xd77bi32 as mp_digit, 0xd783i32 as mp_digit, 0xd7a1i32 as mp_digit,
     0xd7a7i32 as mp_digit, 0xd7adi32 as mp_digit, 0xd7b1i32 as mp_digit,
     0xd7b3i32 as mp_digit, 0xd7bdi32 as mp_digit, 0xd7cbi32 as mp_digit,
     0xd7d1i32 as mp_digit, 0xd7dbi32 as mp_digit, 0xd7fbi32 as mp_digit,
     0xd811i32 as mp_digit, 0xd823i32 as mp_digit, 0xd825i32 as mp_digit,
     0xd829i32 as mp_digit, 0xd82bi32 as mp_digit, 0xd82fi32 as mp_digit,
     0xd837i32 as mp_digit, 0xd84di32 as mp_digit, 0xd855i32 as mp_digit,
     0xd867i32 as mp_digit, 0xd873i32 as mp_digit, 0xd88fi32 as mp_digit,
     0xd891i32 as mp_digit, 0xd8a1i32 as mp_digit, 0xd8adi32 as mp_digit,
     0xd8bfi32 as mp_digit, 0xd8cdi32 as mp_digit, 0xd8d7i32 as mp_digit,
     0xd8e9i32 as mp_digit, 0xd8f5i32 as mp_digit, 0xd8fbi32 as mp_digit,
     0xd91bi32 as mp_digit, 0xd925i32 as mp_digit, 0xd933i32 as mp_digit,
     0xd939i32 as mp_digit, 0xd943i32 as mp_digit, 0xd945i32 as mp_digit,
     0xd94fi32 as mp_digit, 0xd951i32 as mp_digit, 0xd957i32 as mp_digit,
     0xd96di32 as mp_digit, 0xd96fi32 as mp_digit, 0xd973i32 as mp_digit,
     0xd979i32 as mp_digit, 0xd981i32 as mp_digit, 0xd98bi32 as mp_digit,
     0xd991i32 as mp_digit, 0xd99fi32 as mp_digit, 0xd9a5i32 as mp_digit,
     0xd9a9i32 as mp_digit, 0xd9b5i32 as mp_digit, 0xd9d3i32 as mp_digit,
     0xd9ebi32 as mp_digit, 0xd9f1i32 as mp_digit, 0xd9f7i32 as mp_digit,
     0xd9ffi32 as mp_digit, 0xda05i32 as mp_digit, 0xda09i32 as mp_digit,
     0xda0bi32 as mp_digit, 0xda0fi32 as mp_digit, 0xda15i32 as mp_digit,
     0xda1di32 as mp_digit, 0xda23i32 as mp_digit, 0xda29i32 as mp_digit,
     0xda3fi32 as mp_digit, 0xda51i32 as mp_digit, 0xda59i32 as mp_digit,
     0xda5di32 as mp_digit, 0xda5fi32 as mp_digit, 0xda71i32 as mp_digit,
     0xda77i32 as mp_digit, 0xda7bi32 as mp_digit, 0xda7di32 as mp_digit,
     0xda8di32 as mp_digit, 0xda9fi32 as mp_digit, 0xdab3i32 as mp_digit,
     0xdabdi32 as mp_digit, 0xdac3i32 as mp_digit, 0xdac9i32 as mp_digit,
     0xdae7i32 as mp_digit, 0xdae9i32 as mp_digit, 0xdaf5i32 as mp_digit,
     0xdb11i32 as mp_digit, 0xdb17i32 as mp_digit, 0xdb1di32 as mp_digit,
     0xdb23i32 as mp_digit, 0xdb25i32 as mp_digit, 0xdb31i32 as mp_digit,
     0xdb3bi32 as mp_digit, 0xdb43i32 as mp_digit, 0xdb55i32 as mp_digit,
     0xdb67i32 as mp_digit, 0xdb6bi32 as mp_digit, 0xdb73i32 as mp_digit,
     0xdb85i32 as mp_digit, 0xdb8fi32 as mp_digit, 0xdb91i32 as mp_digit,
     0xdbadi32 as mp_digit, 0xdbafi32 as mp_digit, 0xdbb9i32 as mp_digit,
     0xdbc7i32 as mp_digit, 0xdbcbi32 as mp_digit, 0xdbcdi32 as mp_digit,
     0xdbebi32 as mp_digit, 0xdbf7i32 as mp_digit, 0xdc0di32 as mp_digit,
     0xdc27i32 as mp_digit, 0xdc31i32 as mp_digit, 0xdc39i32 as mp_digit,
     0xdc3fi32 as mp_digit, 0xdc49i32 as mp_digit, 0xdc51i32 as mp_digit,
     0xdc61i32 as mp_digit, 0xdc6fi32 as mp_digit, 0xdc75i32 as mp_digit,
     0xdc7bi32 as mp_digit, 0xdc85i32 as mp_digit, 0xdc93i32 as mp_digit,
     0xdc99i32 as mp_digit, 0xdc9di32 as mp_digit, 0xdc9fi32 as mp_digit,
     0xdca9i32 as mp_digit, 0xdcb5i32 as mp_digit, 0xdcb7i32 as mp_digit,
     0xdcbdi32 as mp_digit, 0xdcc7i32 as mp_digit, 0xdccfi32 as mp_digit,
     0xdcd3i32 as mp_digit, 0xdcd5i32 as mp_digit, 0xdcdfi32 as mp_digit,
     0xdcf9i32 as mp_digit, 0xdd0fi32 as mp_digit, 0xdd15i32 as mp_digit,
     0xdd17i32 as mp_digit, 0xdd23i32 as mp_digit, 0xdd35i32 as mp_digit,
     0xdd39i32 as mp_digit, 0xdd53i32 as mp_digit, 0xdd57i32 as mp_digit,
     0xdd5fi32 as mp_digit, 0xdd69i32 as mp_digit, 0xdd6fi32 as mp_digit,
     0xdd7di32 as mp_digit, 0xdd87i32 as mp_digit, 0xdd89i32 as mp_digit,
     0xdd9bi32 as mp_digit, 0xdda1i32 as mp_digit, 0xddabi32 as mp_digit,
     0xddbfi32 as mp_digit, 0xddc5i32 as mp_digit, 0xddcbi32 as mp_digit,
     0xddcfi32 as mp_digit, 0xdde7i32 as mp_digit, 0xdde9i32 as mp_digit,
     0xddedi32 as mp_digit, 0xddf5i32 as mp_digit, 0xddfbi32 as mp_digit,
     0xde0bi32 as mp_digit, 0xde19i32 as mp_digit, 0xde29i32 as mp_digit,
     0xde3bi32 as mp_digit, 0xde3di32 as mp_digit, 0xde41i32 as mp_digit,
     0xde4di32 as mp_digit, 0xde4fi32 as mp_digit, 0xde59i32 as mp_digit,
     0xde5bi32 as mp_digit, 0xde61i32 as mp_digit, 0xde6di32 as mp_digit,
     0xde77i32 as mp_digit, 0xde7di32 as mp_digit, 0xde83i32 as mp_digit,
     0xde97i32 as mp_digit, 0xde9di32 as mp_digit, 0xdea1i32 as mp_digit,
     0xdea7i32 as mp_digit, 0xdecdi32 as mp_digit, 0xded1i32 as mp_digit,
     0xded7i32 as mp_digit, 0xdee3i32 as mp_digit, 0xdef1i32 as mp_digit,
     0xdef5i32 as mp_digit, 0xdf01i32 as mp_digit, 0xdf09i32 as mp_digit,
     0xdf13i32 as mp_digit, 0xdf1fi32 as mp_digit, 0xdf2bi32 as mp_digit,
     0xdf33i32 as mp_digit, 0xdf37i32 as mp_digit, 0xdf3di32 as mp_digit,
     0xdf4bi32 as mp_digit, 0xdf55i32 as mp_digit, 0xdf5bi32 as mp_digit,
     0xdf67i32 as mp_digit, 0xdf69i32 as mp_digit, 0xdf73i32 as mp_digit,
     0xdf85i32 as mp_digit, 0xdf87i32 as mp_digit, 0xdf99i32 as mp_digit,
     0xdfa3i32 as mp_digit, 0xdfabi32 as mp_digit, 0xdfb5i32 as mp_digit,
     0xdfb7i32 as mp_digit, 0xdfc3i32 as mp_digit, 0xdfc7i32 as mp_digit,
     0xdfd5i32 as mp_digit, 0xdff1i32 as mp_digit, 0xdff3i32 as mp_digit,
     0xe003i32 as mp_digit, 0xe005i32 as mp_digit, 0xe017i32 as mp_digit,
     0xe01di32 as mp_digit, 0xe027i32 as mp_digit, 0xe02di32 as mp_digit,
     0xe035i32 as mp_digit, 0xe045i32 as mp_digit, 0xe053i32 as mp_digit,
     0xe071i32 as mp_digit, 0xe07bi32 as mp_digit, 0xe08fi32 as mp_digit,
     0xe095i32 as mp_digit, 0xe09fi32 as mp_digit, 0xe0b7i32 as mp_digit,
     0xe0b9i32 as mp_digit, 0xe0d5i32 as mp_digit, 0xe0d7i32 as mp_digit,
     0xe0e3i32 as mp_digit, 0xe0f3i32 as mp_digit, 0xe0f9i32 as mp_digit,
     0xe101i32 as mp_digit, 0xe125i32 as mp_digit, 0xe129i32 as mp_digit,
     0xe131i32 as mp_digit, 0xe135i32 as mp_digit, 0xe143i32 as mp_digit,
     0xe14fi32 as mp_digit, 0xe159i32 as mp_digit, 0xe161i32 as mp_digit,
     0xe16di32 as mp_digit, 0xe171i32 as mp_digit, 0xe177i32 as mp_digit,
     0xe17fi32 as mp_digit, 0xe183i32 as mp_digit, 0xe189i32 as mp_digit,
     0xe197i32 as mp_digit, 0xe1adi32 as mp_digit, 0xe1b5i32 as mp_digit,
     0xe1bbi32 as mp_digit, 0xe1bfi32 as mp_digit, 0xe1c1i32 as mp_digit,
     0xe1cbi32 as mp_digit, 0xe1d1i32 as mp_digit, 0xe1e5i32 as mp_digit,
     0xe1efi32 as mp_digit, 0xe1f7i32 as mp_digit, 0xe1fdi32 as mp_digit,
     0xe203i32 as mp_digit, 0xe219i32 as mp_digit, 0xe22bi32 as mp_digit,
     0xe22di32 as mp_digit, 0xe23di32 as mp_digit, 0xe243i32 as mp_digit,
     0xe257i32 as mp_digit, 0xe25bi32 as mp_digit, 0xe275i32 as mp_digit,
     0xe279i32 as mp_digit, 0xe287i32 as mp_digit, 0xe29di32 as mp_digit,
     0xe2abi32 as mp_digit, 0xe2afi32 as mp_digit, 0xe2bbi32 as mp_digit,
     0xe2c1i32 as mp_digit, 0xe2c9i32 as mp_digit, 0xe2cdi32 as mp_digit,
     0xe2d3i32 as mp_digit, 0xe2d9i32 as mp_digit, 0xe2f3i32 as mp_digit,
     0xe2fdi32 as mp_digit, 0xe2ffi32 as mp_digit, 0xe311i32 as mp_digit,
     0xe323i32 as mp_digit, 0xe327i32 as mp_digit, 0xe329i32 as mp_digit,
     0xe339i32 as mp_digit, 0xe33bi32 as mp_digit, 0xe34di32 as mp_digit,
     0xe351i32 as mp_digit, 0xe357i32 as mp_digit, 0xe35fi32 as mp_digit,
     0xe363i32 as mp_digit, 0xe369i32 as mp_digit, 0xe375i32 as mp_digit,
     0xe377i32 as mp_digit, 0xe37di32 as mp_digit, 0xe383i32 as mp_digit,
     0xe39fi32 as mp_digit, 0xe3c5i32 as mp_digit, 0xe3c9i32 as mp_digit,
     0xe3d1i32 as mp_digit, 0xe3e1i32 as mp_digit, 0xe3fbi32 as mp_digit,
     0xe3ffi32 as mp_digit, 0xe401i32 as mp_digit, 0xe40bi32 as mp_digit,
     0xe417i32 as mp_digit, 0xe419i32 as mp_digit, 0xe423i32 as mp_digit,
     0xe42bi32 as mp_digit, 0xe431i32 as mp_digit, 0xe43bi32 as mp_digit,
     0xe447i32 as mp_digit, 0xe449i32 as mp_digit, 0xe453i32 as mp_digit,
     0xe455i32 as mp_digit, 0xe46di32 as mp_digit, 0xe471i32 as mp_digit,
     0xe48fi32 as mp_digit, 0xe4a9i32 as mp_digit, 0xe4afi32 as mp_digit,
     0xe4b5i32 as mp_digit, 0xe4c7i32 as mp_digit, 0xe4cdi32 as mp_digit,
     0xe4d3i32 as mp_digit, 0xe4e9i32 as mp_digit, 0xe4ebi32 as mp_digit,
     0xe4f5i32 as mp_digit, 0xe507i32 as mp_digit, 0xe521i32 as mp_digit,
     0xe525i32 as mp_digit, 0xe537i32 as mp_digit, 0xe53fi32 as mp_digit,
     0xe545i32 as mp_digit, 0xe54bi32 as mp_digit, 0xe557i32 as mp_digit,
     0xe567i32 as mp_digit, 0xe56di32 as mp_digit, 0xe575i32 as mp_digit,
     0xe585i32 as mp_digit, 0xe58bi32 as mp_digit, 0xe593i32 as mp_digit,
     0xe5a3i32 as mp_digit, 0xe5a5i32 as mp_digit, 0xe5cfi32 as mp_digit,
     0xe609i32 as mp_digit, 0xe611i32 as mp_digit, 0xe615i32 as mp_digit,
     0xe61bi32 as mp_digit, 0xe61di32 as mp_digit, 0xe621i32 as mp_digit,
     0xe629i32 as mp_digit, 0xe639i32 as mp_digit, 0xe63fi32 as mp_digit,
     0xe653i32 as mp_digit, 0xe657i32 as mp_digit, 0xe663i32 as mp_digit,
     0xe66fi32 as mp_digit, 0xe675i32 as mp_digit, 0xe681i32 as mp_digit,
     0xe683i32 as mp_digit, 0xe68di32 as mp_digit, 0xe68fi32 as mp_digit,
     0xe695i32 as mp_digit, 0xe6abi32 as mp_digit, 0xe6adi32 as mp_digit,
     0xe6b7i32 as mp_digit, 0xe6bdi32 as mp_digit, 0xe6c5i32 as mp_digit,
     0xe6cbi32 as mp_digit, 0xe6d5i32 as mp_digit, 0xe6e3i32 as mp_digit,
     0xe6e9i32 as mp_digit, 0xe6efi32 as mp_digit, 0xe6f3i32 as mp_digit,
     0xe705i32 as mp_digit, 0xe70di32 as mp_digit, 0xe717i32 as mp_digit,
     0xe71fi32 as mp_digit, 0xe72fi32 as mp_digit, 0xe73di32 as mp_digit,
     0xe747i32 as mp_digit, 0xe749i32 as mp_digit, 0xe753i32 as mp_digit,
     0xe755i32 as mp_digit, 0xe761i32 as mp_digit, 0xe767i32 as mp_digit,
     0xe76bi32 as mp_digit, 0xe77fi32 as mp_digit, 0xe789i32 as mp_digit,
     0xe791i32 as mp_digit, 0xe7c5i32 as mp_digit, 0xe7cdi32 as mp_digit,
     0xe7d7i32 as mp_digit, 0xe7ddi32 as mp_digit, 0xe7dfi32 as mp_digit,
     0xe7e9i32 as mp_digit, 0xe7f1i32 as mp_digit, 0xe7fbi32 as mp_digit,
     0xe801i32 as mp_digit, 0xe807i32 as mp_digit, 0xe80fi32 as mp_digit,
     0xe819i32 as mp_digit, 0xe81bi32 as mp_digit, 0xe831i32 as mp_digit,
     0xe833i32 as mp_digit, 0xe837i32 as mp_digit, 0xe83di32 as mp_digit,
     0xe84bi32 as mp_digit, 0xe84fi32 as mp_digit, 0xe851i32 as mp_digit,
     0xe869i32 as mp_digit, 0xe875i32 as mp_digit, 0xe879i32 as mp_digit,
     0xe893i32 as mp_digit, 0xe8a5i32 as mp_digit, 0xe8a9i32 as mp_digit,
     0xe8afi32 as mp_digit, 0xe8bdi32 as mp_digit, 0xe8dbi32 as mp_digit,
     0xe8e1i32 as mp_digit, 0xe8e5i32 as mp_digit, 0xe8ebi32 as mp_digit,
     0xe8edi32 as mp_digit, 0xe903i32 as mp_digit, 0xe90bi32 as mp_digit,
     0xe90fi32 as mp_digit, 0xe915i32 as mp_digit, 0xe917i32 as mp_digit,
     0xe92di32 as mp_digit, 0xe933i32 as mp_digit, 0xe93bi32 as mp_digit,
     0xe94bi32 as mp_digit, 0xe951i32 as mp_digit, 0xe95fi32 as mp_digit,
     0xe963i32 as mp_digit, 0xe969i32 as mp_digit, 0xe97bi32 as mp_digit,
     0xe983i32 as mp_digit, 0xe98fi32 as mp_digit, 0xe995i32 as mp_digit,
     0xe9a1i32 as mp_digit, 0xe9b9i32 as mp_digit, 0xe9d7i32 as mp_digit,
     0xe9e7i32 as mp_digit, 0xe9efi32 as mp_digit, 0xea11i32 as mp_digit,
     0xea19i32 as mp_digit, 0xea2fi32 as mp_digit, 0xea35i32 as mp_digit,
     0xea43i32 as mp_digit, 0xea4di32 as mp_digit, 0xea5fi32 as mp_digit,
     0xea6di32 as mp_digit, 0xea71i32 as mp_digit, 0xea7di32 as mp_digit,
     0xea85i32 as mp_digit, 0xea89i32 as mp_digit, 0xeaadi32 as mp_digit,
     0xeab3i32 as mp_digit, 0xeab9i32 as mp_digit, 0xeabbi32 as mp_digit,
     0xeac5i32 as mp_digit, 0xeac7i32 as mp_digit, 0xeacbi32 as mp_digit,
     0xeadfi32 as mp_digit, 0xeae5i32 as mp_digit, 0xeaebi32 as mp_digit,
     0xeaf5i32 as mp_digit, 0xeb01i32 as mp_digit, 0xeb07i32 as mp_digit,
     0xeb09i32 as mp_digit, 0xeb31i32 as mp_digit, 0xeb39i32 as mp_digit,
     0xeb3fi32 as mp_digit, 0xeb5bi32 as mp_digit, 0xeb61i32 as mp_digit,
     0xeb63i32 as mp_digit, 0xeb6fi32 as mp_digit, 0xeb81i32 as mp_digit,
     0xeb85i32 as mp_digit, 0xeb9di32 as mp_digit, 0xebabi32 as mp_digit,
     0xebb1i32 as mp_digit, 0xebb7i32 as mp_digit, 0xebc1i32 as mp_digit,
     0xebd5i32 as mp_digit, 0xebdfi32 as mp_digit, 0xebedi32 as mp_digit,
     0xebfdi32 as mp_digit, 0xec0bi32 as mp_digit, 0xec1bi32 as mp_digit,
     0xec21i32 as mp_digit, 0xec29i32 as mp_digit, 0xec4di32 as mp_digit,
     0xec51i32 as mp_digit, 0xec5di32 as mp_digit, 0xec69i32 as mp_digit,
     0xec6fi32 as mp_digit, 0xec7bi32 as mp_digit, 0xecadi32 as mp_digit,
     0xecb9i32 as mp_digit, 0xecbfi32 as mp_digit, 0xecc3i32 as mp_digit,
     0xecc9i32 as mp_digit, 0xeccfi32 as mp_digit, 0xecd7i32 as mp_digit,
     0xecddi32 as mp_digit, 0xece7i32 as mp_digit, 0xece9i32 as mp_digit,
     0xecf3i32 as mp_digit, 0xecf5i32 as mp_digit, 0xed07i32 as mp_digit,
     0xed11i32 as mp_digit, 0xed1fi32 as mp_digit, 0xed2fi32 as mp_digit,
     0xed37i32 as mp_digit, 0xed3di32 as mp_digit, 0xed41i32 as mp_digit,
     0xed55i32 as mp_digit, 0xed59i32 as mp_digit, 0xed5bi32 as mp_digit,
     0xed65i32 as mp_digit, 0xed6bi32 as mp_digit, 0xed79i32 as mp_digit,
     0xed8bi32 as mp_digit, 0xed95i32 as mp_digit, 0xedbbi32 as mp_digit,
     0xedc5i32 as mp_digit, 0xedd7i32 as mp_digit, 0xedd9i32 as mp_digit,
     0xede3i32 as mp_digit, 0xede5i32 as mp_digit, 0xedf1i32 as mp_digit,
     0xedf5i32 as mp_digit, 0xedf7i32 as mp_digit, 0xedfbi32 as mp_digit,
     0xee09i32 as mp_digit, 0xee0fi32 as mp_digit, 0xee19i32 as mp_digit,
     0xee21i32 as mp_digit, 0xee49i32 as mp_digit, 0xee4fi32 as mp_digit,
     0xee63i32 as mp_digit, 0xee67i32 as mp_digit, 0xee73i32 as mp_digit,
     0xee7bi32 as mp_digit, 0xee81i32 as mp_digit, 0xeea3i32 as mp_digit,
     0xeeabi32 as mp_digit, 0xeec1i32 as mp_digit, 0xeec9i32 as mp_digit,
     0xeed5i32 as mp_digit, 0xeedfi32 as mp_digit, 0xeee1i32 as mp_digit,
     0xeef1i32 as mp_digit, 0xef1bi32 as mp_digit, 0xef27i32 as mp_digit,
     0xef2fi32 as mp_digit, 0xef45i32 as mp_digit, 0xef4di32 as mp_digit,
     0xef63i32 as mp_digit, 0xef6bi32 as mp_digit, 0xef71i32 as mp_digit,
     0xef93i32 as mp_digit, 0xef95i32 as mp_digit, 0xef9bi32 as mp_digit,
     0xef9fi32 as mp_digit, 0xefadi32 as mp_digit, 0xefb3i32 as mp_digit,
     0xefc3i32 as mp_digit, 0xefc5i32 as mp_digit, 0xefdbi32 as mp_digit,
     0xefe1i32 as mp_digit, 0xefe9i32 as mp_digit, 0xf001i32 as mp_digit,
     0xf017i32 as mp_digit, 0xf01di32 as mp_digit, 0xf01fi32 as mp_digit,
     0xf02bi32 as mp_digit, 0xf02fi32 as mp_digit, 0xf035i32 as mp_digit,
     0xf043i32 as mp_digit, 0xf047i32 as mp_digit, 0xf04fi32 as mp_digit,
     0xf067i32 as mp_digit, 0xf06bi32 as mp_digit, 0xf071i32 as mp_digit,
     0xf077i32 as mp_digit, 0xf079i32 as mp_digit, 0xf08fi32 as mp_digit,
     0xf0a3i32 as mp_digit, 0xf0a9i32 as mp_digit, 0xf0adi32 as mp_digit,
     0xf0bbi32 as mp_digit, 0xf0bfi32 as mp_digit, 0xf0c5i32 as mp_digit,
     0xf0cbi32 as mp_digit, 0xf0d3i32 as mp_digit, 0xf0d9i32 as mp_digit,
     0xf0e3i32 as mp_digit, 0xf0e9i32 as mp_digit, 0xf0f1i32 as mp_digit,
     0xf0f7i32 as mp_digit, 0xf107i32 as mp_digit, 0xf115i32 as mp_digit,
     0xf11bi32 as mp_digit, 0xf121i32 as mp_digit, 0xf137i32 as mp_digit,
     0xf13di32 as mp_digit, 0xf155i32 as mp_digit, 0xf175i32 as mp_digit,
     0xf17bi32 as mp_digit, 0xf18di32 as mp_digit, 0xf193i32 as mp_digit,
     0xf1a5i32 as mp_digit, 0xf1afi32 as mp_digit, 0xf1b7i32 as mp_digit,
     0xf1d5i32 as mp_digit, 0xf1e7i32 as mp_digit, 0xf1edi32 as mp_digit,
     0xf1fdi32 as mp_digit, 0xf209i32 as mp_digit, 0xf20fi32 as mp_digit,
     0xf21bi32 as mp_digit, 0xf21di32 as mp_digit, 0xf223i32 as mp_digit,
     0xf227i32 as mp_digit, 0xf233i32 as mp_digit, 0xf23bi32 as mp_digit,
     0xf241i32 as mp_digit, 0xf257i32 as mp_digit, 0xf25fi32 as mp_digit,
     0xf265i32 as mp_digit, 0xf269i32 as mp_digit, 0xf277i32 as mp_digit,
     0xf281i32 as mp_digit, 0xf293i32 as mp_digit, 0xf2a7i32 as mp_digit,
     0xf2b1i32 as mp_digit, 0xf2b3i32 as mp_digit, 0xf2b9i32 as mp_digit,
     0xf2bdi32 as mp_digit, 0xf2bfi32 as mp_digit, 0xf2dbi32 as mp_digit,
     0xf2edi32 as mp_digit, 0xf2efi32 as mp_digit, 0xf2f9i32 as mp_digit,
     0xf2ffi32 as mp_digit, 0xf305i32 as mp_digit, 0xf30bi32 as mp_digit,
     0xf319i32 as mp_digit, 0xf341i32 as mp_digit, 0xf359i32 as mp_digit,
     0xf35bi32 as mp_digit, 0xf35fi32 as mp_digit, 0xf367i32 as mp_digit,
     0xf373i32 as mp_digit, 0xf377i32 as mp_digit, 0xf38bi32 as mp_digit,
     0xf38fi32 as mp_digit, 0xf3afi32 as mp_digit, 0xf3c1i32 as mp_digit,
     0xf3d1i32 as mp_digit, 0xf3d7i32 as mp_digit, 0xf3fbi32 as mp_digit,
     0xf403i32 as mp_digit, 0xf409i32 as mp_digit, 0xf40di32 as mp_digit,
     0xf413i32 as mp_digit, 0xf421i32 as mp_digit, 0xf425i32 as mp_digit,
     0xf42bi32 as mp_digit, 0xf445i32 as mp_digit, 0xf44bi32 as mp_digit,
     0xf455i32 as mp_digit, 0xf463i32 as mp_digit, 0xf475i32 as mp_digit,
     0xf47fi32 as mp_digit, 0xf485i32 as mp_digit, 0xf48bi32 as mp_digit,
     0xf499i32 as mp_digit, 0xf4a3i32 as mp_digit, 0xf4a9i32 as mp_digit,
     0xf4afi32 as mp_digit, 0xf4bdi32 as mp_digit, 0xf4c3i32 as mp_digit,
     0xf4dbi32 as mp_digit, 0xf4dfi32 as mp_digit, 0xf4edi32 as mp_digit,
     0xf503i32 as mp_digit, 0xf50bi32 as mp_digit, 0xf517i32 as mp_digit,
     0xf521i32 as mp_digit, 0xf529i32 as mp_digit, 0xf535i32 as mp_digit,
     0xf547i32 as mp_digit, 0xf551i32 as mp_digit, 0xf563i32 as mp_digit,
     0xf56bi32 as mp_digit, 0xf583i32 as mp_digit, 0xf58di32 as mp_digit,
     0xf595i32 as mp_digit, 0xf599i32 as mp_digit, 0xf5b1i32 as mp_digit,
     0xf5b7i32 as mp_digit, 0xf5c9i32 as mp_digit, 0xf5cfi32 as mp_digit,
     0xf5d1i32 as mp_digit, 0xf5dbi32 as mp_digit, 0xf5f9i32 as mp_digit,
     0xf5fbi32 as mp_digit, 0xf605i32 as mp_digit, 0xf607i32 as mp_digit,
     0xf60bi32 as mp_digit, 0xf60di32 as mp_digit, 0xf635i32 as mp_digit,
     0xf637i32 as mp_digit, 0xf653i32 as mp_digit, 0xf65bi32 as mp_digit,
     0xf661i32 as mp_digit, 0xf667i32 as mp_digit, 0xf679i32 as mp_digit,
     0xf67fi32 as mp_digit, 0xf689i32 as mp_digit, 0xf697i32 as mp_digit,
     0xf69bi32 as mp_digit, 0xf6adi32 as mp_digit, 0xf6cbi32 as mp_digit,
     0xf6ddi32 as mp_digit, 0xf6dfi32 as mp_digit, 0xf6ebi32 as mp_digit,
     0xf709i32 as mp_digit, 0xf70fi32 as mp_digit, 0xf72di32 as mp_digit,
     0xf731i32 as mp_digit, 0xf743i32 as mp_digit, 0xf74fi32 as mp_digit,
     0xf751i32 as mp_digit, 0xf755i32 as mp_digit, 0xf763i32 as mp_digit,
     0xf769i32 as mp_digit, 0xf773i32 as mp_digit, 0xf779i32 as mp_digit,
     0xf781i32 as mp_digit, 0xf787i32 as mp_digit, 0xf791i32 as mp_digit,
     0xf79di32 as mp_digit, 0xf79fi32 as mp_digit, 0xf7a5i32 as mp_digit,
     0xf7b1i32 as mp_digit, 0xf7bbi32 as mp_digit, 0xf7bdi32 as mp_digit,
     0xf7cfi32 as mp_digit, 0xf7d3i32 as mp_digit, 0xf7e7i32 as mp_digit,
     0xf7ebi32 as mp_digit, 0xf7f1i32 as mp_digit, 0xf7ffi32 as mp_digit,
     0xf805i32 as mp_digit, 0xf80bi32 as mp_digit, 0xf821i32 as mp_digit,
     0xf827i32 as mp_digit, 0xf82di32 as mp_digit, 0xf835i32 as mp_digit,
     0xf847i32 as mp_digit, 0xf859i32 as mp_digit, 0xf863i32 as mp_digit,
     0xf865i32 as mp_digit, 0xf86fi32 as mp_digit, 0xf871i32 as mp_digit,
     0xf877i32 as mp_digit, 0xf87bi32 as mp_digit, 0xf881i32 as mp_digit,
     0xf88di32 as mp_digit, 0xf89fi32 as mp_digit, 0xf8a1i32 as mp_digit,
     0xf8abi32 as mp_digit, 0xf8b3i32 as mp_digit, 0xf8b7i32 as mp_digit,
     0xf8c9i32 as mp_digit, 0xf8cbi32 as mp_digit, 0xf8d1i32 as mp_digit,
     0xf8d7i32 as mp_digit, 0xf8ddi32 as mp_digit, 0xf8e7i32 as mp_digit,
     0xf8efi32 as mp_digit, 0xf8f9i32 as mp_digit, 0xf8ffi32 as mp_digit,
     0xf911i32 as mp_digit, 0xf91di32 as mp_digit, 0xf925i32 as mp_digit,
     0xf931i32 as mp_digit, 0xf937i32 as mp_digit, 0xf93bi32 as mp_digit,
     0xf941i32 as mp_digit, 0xf94fi32 as mp_digit, 0xf95fi32 as mp_digit,
     0xf961i32 as mp_digit, 0xf96di32 as mp_digit, 0xf971i32 as mp_digit,
     0xf977i32 as mp_digit, 0xf99di32 as mp_digit, 0xf9a3i32 as mp_digit,
     0xf9a9i32 as mp_digit, 0xf9b9i32 as mp_digit, 0xf9cdi32 as mp_digit,
     0xf9e9i32 as mp_digit, 0xf9fdi32 as mp_digit, 0xfa07i32 as mp_digit,
     0xfa0di32 as mp_digit, 0xfa13i32 as mp_digit, 0xfa21i32 as mp_digit,
     0xfa25i32 as mp_digit, 0xfa3fi32 as mp_digit, 0xfa43i32 as mp_digit,
     0xfa51i32 as mp_digit, 0xfa5bi32 as mp_digit, 0xfa6di32 as mp_digit,
     0xfa7bi32 as mp_digit, 0xfa97i32 as mp_digit, 0xfa99i32 as mp_digit,
     0xfa9di32 as mp_digit, 0xfaabi32 as mp_digit, 0xfabbi32 as mp_digit,
     0xfabdi32 as mp_digit, 0xfad9i32 as mp_digit, 0xfadfi32 as mp_digit,
     0xfae7i32 as mp_digit, 0xfaedi32 as mp_digit, 0xfb0fi32 as mp_digit,
     0xfb17i32 as mp_digit, 0xfb1bi32 as mp_digit, 0xfb2di32 as mp_digit,
     0xfb2fi32 as mp_digit, 0xfb3fi32 as mp_digit, 0xfb47i32 as mp_digit,
     0xfb4di32 as mp_digit, 0xfb75i32 as mp_digit, 0xfb7di32 as mp_digit,
     0xfb8fi32 as mp_digit, 0xfb93i32 as mp_digit, 0xfbb1i32 as mp_digit,
     0xfbb7i32 as mp_digit, 0xfbc3i32 as mp_digit, 0xfbc5i32 as mp_digit,
     0xfbe3i32 as mp_digit, 0xfbe9i32 as mp_digit, 0xfbf3i32 as mp_digit,
     0xfc01i32 as mp_digit, 0xfc29i32 as mp_digit, 0xfc37i32 as mp_digit,
     0xfc41i32 as mp_digit, 0xfc43i32 as mp_digit, 0xfc4fi32 as mp_digit,
     0xfc59i32 as mp_digit, 0xfc61i32 as mp_digit, 0xfc65i32 as mp_digit,
     0xfc6di32 as mp_digit, 0xfc73i32 as mp_digit, 0xfc79i32 as mp_digit,
     0xfc95i32 as mp_digit, 0xfc97i32 as mp_digit, 0xfc9bi32 as mp_digit,
     0xfca7i32 as mp_digit, 0xfcb5i32 as mp_digit, 0xfcc5i32 as mp_digit,
     0xfccdi32 as mp_digit, 0xfcebi32 as mp_digit, 0xfcfbi32 as mp_digit,
     0xfd0di32 as mp_digit, 0xfd0fi32 as mp_digit, 0xfd19i32 as mp_digit,
     0xfd2bi32 as mp_digit, 0xfd31i32 as mp_digit, 0xfd51i32 as mp_digit,
     0xfd55i32 as mp_digit, 0xfd67i32 as mp_digit, 0xfd6di32 as mp_digit,
     0xfd6fi32 as mp_digit, 0xfd7bi32 as mp_digit, 0xfd85i32 as mp_digit,
     0xfd97i32 as mp_digit, 0xfd99i32 as mp_digit, 0xfd9fi32 as mp_digit,
     0xfda9i32 as mp_digit, 0xfdb7i32 as mp_digit, 0xfdc9i32 as mp_digit,
     0xfde5i32 as mp_digit, 0xfdebi32 as mp_digit, 0xfdf3i32 as mp_digit,
     0xfe03i32 as mp_digit, 0xfe05i32 as mp_digit, 0xfe09i32 as mp_digit,
     0xfe1di32 as mp_digit, 0xfe27i32 as mp_digit, 0xfe2fi32 as mp_digit,
     0xfe41i32 as mp_digit, 0xfe4bi32 as mp_digit, 0xfe4di32 as mp_digit,
     0xfe57i32 as mp_digit, 0xfe5fi32 as mp_digit, 0xfe63i32 as mp_digit,
     0xfe69i32 as mp_digit, 0xfe75i32 as mp_digit, 0xfe7bi32 as mp_digit,
     0xfe8fi32 as mp_digit, 0xfe93i32 as mp_digit, 0xfe95i32 as mp_digit,
     0xfe9bi32 as mp_digit, 0xfe9fi32 as mp_digit, 0xfeb3i32 as mp_digit,
     0xfebdi32 as mp_digit, 0xfed7i32 as mp_digit, 0xfee9i32 as mp_digit,
     0xfef3i32 as mp_digit, 0xfef5i32 as mp_digit, 0xff07i32 as mp_digit,
     0xff0di32 as mp_digit, 0xff1di32 as mp_digit, 0xff2bi32 as mp_digit,
     0xff2fi32 as mp_digit, 0xff49i32 as mp_digit, 0xff4di32 as mp_digit,
     0xff5bi32 as mp_digit, 0xff65i32 as mp_digit, 0xff71i32 as mp_digit,
     0xff7fi32 as mp_digit, 0xff85i32 as mp_digit, 0xff8bi32 as mp_digit,
     0xff8fi32 as mp_digit, 0xff9di32 as mp_digit, 0xffa7i32 as mp_digit,
     0xffa9i32 as mp_digit, 0xffc7i32 as mp_digit, 0xffd9i32 as mp_digit,
     0xffefi32 as mp_digit, 0xfff1i32 as mp_digit];
/* Tests for divisibility    */
#[no_mangle]
pub unsafe extern "C" fn mpp_divis(mut a: *mut mp_int, mut b: *mut mp_int)
 -> mp_err {
    let mut res: mp_err = 0;
    let mut rem: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    res = mp_init(&mut rem);
    if res != 0i32 { return res }
    res = mp_mod(a, b, &mut rem);
    if !(res != 0i32) {
        if mp_cmp_z(&mut rem) == 0i32 { res = 0i32 } else { res = -1i32 }
    }
    mp_clear(&mut rem);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mpp_divis_d(mut a: *mut mp_int, mut d: mp_digit)
 -> mp_err {
    let mut res: mp_err = 0;
    let mut rem: mp_digit = 0;
    if a.is_null() { return -4i32 }
    if d == 0i32 as libc::c_ulong { return -1i32 }
    res = mp_mod_d(a, d, &mut rem);
    if res != 0i32 { return res }
    if rem == 0i32 as libc::c_ulong { return 0i32 } else { return -1i32 };
}
/* Random selection          */
#[no_mangle]
pub unsafe extern "C" fn mpp_random(mut a: *mut mp_int) -> mp_err {
    let mut next: mp_digit = 0i32 as mp_digit;
    let mut ix: libc::c_uint = 0;
    let mut jx: libc::c_uint = 0;
    if a.is_null() { return -4i32 }
    ix = 0i32 as libc::c_uint;
    while ix < (*a).used {
        jx = 0i32 as libc::c_uint;
        while (jx as libc::c_ulong) <
                  ::std::mem::size_of::<mp_digit>() as libc::c_ulong {
            next =
                next << 8i32 |
                    (rand() & 127i32 * 2i32 + 1i32) as libc::c_ulong;
            jx = jx.wrapping_add(1)
        }
        *(*a).dp.offset(ix as isize) = next;
        ix = ix.wrapping_add(1)
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpp_random_size(mut a: *mut mp_int,
                                         mut prec: mp_size) -> mp_err {
    let mut res: mp_err = 0;
    if !(!a.is_null() && prec > 0i32 as libc::c_uint) { return -4i32 }
    res = s_mp_pad(a, prec);
    if res != 0i32 { return res }
    return mpp_random(a);
}
/* Pseudo-primality testing  */
#[no_mangle]
pub unsafe extern "C" fn mpp_divis_vector(mut a: *mut mp_int,
                                          mut vec: *const mp_digit,
                                          mut size: libc::c_int,
                                          mut which: *mut libc::c_int)
 -> mp_err {
    if !(!a.is_null() && !vec.is_null() && size > 0i32) { return -4i32 }
    return s_mpp_divp(a, vec, size, which);
}
/*
 *  mpprime.c
 *
 *  Utilities for finding and working with prime and pseudo-prime
 *  integers
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* determines size of hard-wired prime table */
/*
   Test if any of a given vector of digits divides a.  If not, MP_NO
   is returned; otherwise, MP_YES is returned and 'which' is set to
   the index of the integer in the vector which divided a.
 */
#[no_mangle]
pub unsafe extern "C" fn s_mpp_divp(mut a: *mut mp_int,
                                    mut vec: *const mp_digit,
                                    mut size: libc::c_int,
                                    mut which: *mut libc::c_int) -> mp_err {
    let mut res: mp_err = 0;
    let mut rem: mp_digit = 0;
    let mut ix: libc::c_int = 0;
    ix = 0i32;
    while ix < size {
        res = mp_mod_d(a, *vec.offset(ix as isize), &mut rem);
        if res != 0i32 { return res }
        if rem == 0i32 as libc::c_ulong {
            if !which.is_null() { *which = ix }
            return 0i32
        }
        ix += 1
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpp_divis_primes(mut a: *mut mp_int,
                                          mut np: *mut mp_digit) -> mp_err {
    let mut size: libc::c_int = 0;
    let mut which: libc::c_int = 0;
    let mut res: mp_err = 0;
    if !(!a.is_null() && !np.is_null()) { return -4i32 }
    size = *np as libc::c_int;
    if size > prime_tab_size { size = prime_tab_size }
    res = mpp_divis_vector(a, prime_tab.as_ptr(), size, &mut which);
    if res == 0i32 { *np = prime_tab[which as usize] }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mpp_fermat(mut a: *mut mp_int, mut w: mp_digit)
 -> mp_err {
    let mut base: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut test: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut res: mp_err = 0;
    res = mp_init(&mut base);
    if res != 0i32 { return res }
    mp_set(&mut base, w);
    res = mp_init(&mut test);
    if !(res != 0i32) {
        /* Compute test = base^a (mod a) */
        res = mp_exptmod(&mut base, a, a, &mut test);
        if !(res != 0i32) {
            if mp_cmp(&mut base, &mut test) == 0i32 {
                res = 0i32
            } else { res = -1i32 }
        }
        mp_clear(&mut test);
    }
    mp_clear(&mut base);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mpp_fermat_list(mut a: *mut mp_int,
                                         mut primes: *const mp_digit,
                                         mut nPrimes: mp_size) -> mp_err {
    let mut rv: mp_err = 0i32;
    loop  {
        let fresh0 = nPrimes;
        nPrimes = nPrimes.wrapping_sub(1);
        if !(fresh0 > 0i32 as libc::c_uint && rv == 0i32) { break ; }
        let fresh1 = primes;
        primes = primes.offset(1);
        rv = mpp_fermat(a, *fresh1)
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn mpp_pprime(mut a: *mut mp_int, mut nt: libc::c_int)
 -> mp_err {
    let mut current_block: u64;
    let mut res: mp_err = 0;
    /* "amo" = "a minus one" */
    let mut x: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut amo: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut m: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut z: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut iter: libc::c_int = 0;
    let mut jx: libc::c_uint = 0;
    let mut b: mp_size = 0;
    if a.is_null() { return -4i32 }
    x.dp = 0 as *mut mp_digit;
    amo.dp = 0 as *mut mp_digit;
    m.dp = 0 as *mut mp_digit;
    z.dp = 0 as *mut mp_digit;
    /* Initialize temporaries... */
    res = mp_init(&mut amo);
    if !(0i32 > res) {
        /* Compute amo = a - 1 for what follows...    */
        res = mp_sub_d(a, 1i32 as mp_digit, &mut amo);
        if !(0i32 > res) {
            b = mp_trailing_zeros(&mut amo);
            if 0 == b {
                /* a was even ? */
                res = -1i32
            } else {
                res = mp_init_size(&mut x, (*a).used);
                if !(0i32 > res) {
                    res = mp_init(&mut z);
                    if !(0i32 > res) {
                        res = mp_init(&mut m);
                        if !(0i32 > res) {
                            res =
                                mp_div_2d(&mut amo, b as mp_digit, &mut m,
                                          0 as *mut mp_int);
                            if !(0i32 > res) {
                                /* Do the test nt times... */
                                iter = 0i32;
                                's_108:
                                    loop  {
                                        if !(iter < nt) {
                                            current_block =
                                                1356832168064818221;
                                            break ;
                                        }
                                        /* Choose a random value for 1 < x < a      */
                                        res = s_mp_pad(&mut x, (*a).used);
                                        if 0i32 > res {
                                            current_block =
                                                9331382782940327004;
                                            break ;
                                        }
                                        mpp_random(&mut x);
                                        res = mp_mod(&mut x, a, &mut x);
                                        if 0i32 > res {
                                            current_block =
                                                9331382782940327004;
                                            break ;
                                        }
                                        if mp_cmp_d(&mut x, 1i32 as mp_digit)
                                               <= 0i32 {
                                            iter -= 1
                                        } else {
                                            /* choose a new x */
                                            /* Compute z = (x ** m) mod a               */
                                            res =
                                                mp_exptmod(&mut x, &mut m, a,
                                                           &mut z);
                                            if 0i32 > res {
                                                current_block =
                                                    9331382782940327004;
                                                break ;
                                            }
                                            if mp_cmp_d(&mut z,
                                                        1i32 as mp_digit) ==
                                                   0i32 ||
                                                   mp_cmp(&mut z, &mut amo) ==
                                                       0i32 {
                                                res = 0i32
                                            } else {
                                                res = -1i32;
                                                jx = 1i32 as libc::c_uint;
                                                while jx < b {
                                                    /* z = z^2 (mod a) */
                                                    res =
                                                        mp_sqrmod(&mut z, a,
                                                                  &mut z);
                                                    if 0i32 > res {
                                                        current_block =
                                                            9331382782940327004;
                                                        break 's_108 ;
                                                    }
                                                    res = -1i32;
                                                    if mp_cmp_d(&mut z,
                                                                1i32 as
                                                                    mp_digit)
                                                           == 0i32 {
                                                        break ;
                                                    }
                                                    if mp_cmp(&mut z,
                                                              &mut amo) ==
                                                           0i32 {
                                                        res = 0i32;
                                                        break ;
                                                    } else {
                                                        jx =
                                                            jx.wrapping_add(1)
                                                    }
                                                }
                                                /* end testing loop */
                                                /* If the test passes, we will continue iterating, but a failed
           test means the candidate is definitely NOT prime, so we will
           immediately break out of this loop
         */
                                                if res == -1i32 {
                                                    current_block =
                                                        1356832168064818221;
                                                    break ;
                                                }
                                            }
                                        }
                                        iter += 1
                                    }
                                match current_block {
                                    9331382782940327004 => { }
                                    _ => { }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    /* end iterations loop */
    mp_clear(&mut m);
    mp_clear(&mut z);
    mp_clear(&mut x);
    mp_clear(&mut amo);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mpp_sieve(mut trial: *mut mp_int,
                                   mut primes: *const mp_digit,
                                   mut nPrimes: mp_size,
                                   mut sieve: *mut libc::c_uchar,
                                   mut nSieve: mp_size) -> mp_err {
    let mut res: mp_err = 0;
    let mut rem: mp_digit = 0;
    let mut ix: mp_size = 0;
    let mut offset: libc::c_ulong = 0;
    memset(sieve as *mut libc::c_void, 0i32, nSieve as libc::c_ulong);
    ix = 0i32 as mp_size;
    while ix < nPrimes {
        let mut prime: mp_digit = *primes.offset(ix as isize);
        let mut i: mp_size = 0;
        res = mp_mod_d(trial, prime, &mut rem);
        if res != 0i32 { return res }
        if rem == 0i32 as libc::c_ulong {
            offset = 0i32 as libc::c_ulong
        } else { offset = prime.wrapping_sub(rem) }
        i = offset as mp_size;
        while i < nSieve.wrapping_mul(2i32 as libc::c_uint) {
            if i.wrapping_rem(2i32 as libc::c_uint) == 0i32 as libc::c_uint {
                *sieve.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) =
                    1i32 as libc::c_uchar
            }
            i = (i as libc::c_ulong).wrapping_add(prime) as mp_size as mp_size
        }
        ix = ix.wrapping_add(1)
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpp_make_prime(mut start: *mut mp_int,
                                        mut nBits: mp_size,
                                        mut strong: mp_size) -> mp_err {
    let mut current_block: u64;
    let mut np: mp_digit = 0;
    let mut res: mp_err = 0;
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    let mut trial: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut q: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut num_tests: mp_size = 0;
    let mut sieve: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if start.is_null() { return -4i32 }
    if !(nBits > 16i32 as libc::c_uint) { return -3i32 }
    sieve = malloc((32i32 * 1024i32) as libc::c_ulong) as *mut libc::c_uchar;
    if sieve.is_null() { return -2i32 }
    trial.dp = 0 as *mut mp_digit;
    q.dp = 0 as *mut mp_digit;
    res = mp_init(&mut trial);
    if !(0i32 > res) {
        res = mp_init(&mut q);
        if !(0i32 > res) {
            if nBits >= 2000i32 as libc::c_uint {
                num_tests = 3i32 as mp_size
            } else if nBits >= 1536i32 as libc::c_uint {
                num_tests = 4i32 as mp_size
            } else if nBits >= 1024i32 as libc::c_uint {
                num_tests = 5i32 as mp_size
            } else if nBits >= 550i32 as libc::c_uint {
                num_tests = 6i32 as mp_size
            } else if nBits >= 450i32 as libc::c_uint {
                num_tests = 7i32 as mp_size
            } else if nBits >= 400i32 as libc::c_uint {
                num_tests = 8i32 as mp_size
            } else if nBits >= 350i32 as libc::c_uint {
                num_tests = 9i32 as mp_size
            } else if nBits >= 300i32 as libc::c_uint {
                num_tests = 10i32 as mp_size
            } else if nBits >= 250i32 as libc::c_uint {
                num_tests = 20i32 as mp_size
            } else if nBits >= 200i32 as libc::c_uint {
                num_tests = 41i32 as mp_size
            } else if nBits >= 100i32 as libc::c_uint {
                num_tests = 38i32 as mp_size
            } else { num_tests = 50i32 as mp_size }
            if 0 != strong { nBits = nBits.wrapping_sub(1) }
            res =
                mpl_set_bit(start, nBits.wrapping_sub(1i32 as libc::c_uint),
                            1i32 as mp_size);
            if !(0i32 > res) {
                res = mpl_set_bit(start, 0i32 as mp_size, 1i32 as mp_size);
                if !(0i32 > res) {
                    i =
                        mpl_significant_bits(start).wrapping_sub(1i32 as
                                                                     libc::c_uint);
                    loop  {
                        if !(i >= nBits) {
                            current_block = 3546145585875536353;
                            break ;
                        }
                        res = mpl_set_bit(start, i, 0i32 as mp_size);
                        if 0i32 > res {
                            current_block = 7187023563388198622;
                            break ;
                        }
                        i = i.wrapping_sub(1)
                    }
                    match current_block {
                        7187023563388198622 => { }
                        _ => {
                            /* start sieveing with prime value of 3. */
                            res =
                                mpp_sieve(start,
                                          prime_tab.as_ptr().offset(1isize),
                                          (prime_tab_size - 1i32) as mp_size,
                                          sieve,
                                          (32i32 * 1024i32) as mp_size);
                            if !(0i32 > res) {
                                res = -1i32;
                                i = 0i32 as libc::c_uint;
                                loop  {
                                    if !(i <
                                             (32i32 * 1024i32) as
                                                 libc::c_uint) {
                                        current_block = 13349765058737954042;
                                        break ;
                                    }
                                    /* this number is composite */
                                    if !(0 != *sieve.offset(i as isize)) {
                                        res =
                                            mp_add_d(start,
                                                     (2i32 as
                                                          libc::c_uint).wrapping_mul(i)
                                                         as mp_digit,
                                                     &mut trial);
                                        if 0i32 > res {
                                            current_block =
                                                7187023563388198622;
                                            break ;
                                        }
                                        res =
                                            mpp_fermat(&mut trial,
                                                       2i32 as mp_digit);
                                        if res != 0i32 {
                                            if !(res == -1i32) {
                                                current_block =
                                                    7187023563388198622;
                                                break ;
                                            }
                                        } else {
                                            /* was composite */
                                            res =
                                                mpp_pprime(&mut trial,
                                                           num_tests as
                                                               libc::c_int);
                                            if res != 0i32 {
                                                if !(res == -1i32) {
                                                    current_block =
                                                        7187023563388198622;
                                                    break ;
                                                }
                                            } else if 0 == strong {
                                                /* success !! */
                                                current_block =
                                                    13349765058737954042;
                                                break ;
                                            } else {
                                                /* At this point, we have strong evidence that our candidate
           is itself prime.  If we want a strong prime, we need now
           to test q = 2p + 1 for primality...
        */
                                                res =
                                                    mp_mul_2(&mut trial,
                                                             &mut q);
                                                if 0i32 > res {
                                                    current_block =
                                                        7187023563388198622;
                                                    break ;
                                                }
                                                res =
                                                    mp_add_d(&mut q,
                                                             1i32 as mp_digit,
                                                             &mut q);
                                                if 0i32 > res {
                                                    current_block =
                                                        7187023563388198622;
                                                    break ;
                                                }
                                                np =
                                                    prime_tab_size as
                                                        mp_digit;
                                                res =
                                                    mpp_divis_primes(&mut q,
                                                                     &mut np);
                                                if res == 0i32 {
                                                    /* is composite */
                                                    mp_clear(&mut q);
                                                } else {
                                                    if res != -1i32 {
                                                        current_block =
                                                            7187023563388198622;
                                                        break ;
                                                    }
                                                    res =
                                                        mpp_fermat(&mut q,
                                                                   2i32 as
                                                                       mp_digit);
                                                    if res != 0i32 {
                                                        mp_clear(&mut q);
                                                        if !(res == -1i32) {
                                                            current_block =
                                                                7187023563388198622;
                                                            break ;
                                                        }
                                                    } else {
                                                        /* was composite */
                                                        res =
                                                            mpp_pprime(&mut q,
                                                                       num_tests
                                                                           as
                                                                           libc::c_int);
                                                        if res != 0i32 {
                                                            mp_clear(&mut q);
                                                            if !(res == -1i32)
                                                               {
                                                                current_block
                                                                    =
                                                                    7187023563388198622;
                                                                break ;
                                                            }
                                                        } else {
                                                            /* was composite */
                                                            mp_exch(&mut q,
                                                                    &mut trial);
                                                            mp_clear(&mut q);
                                                            current_block =
                                                                13349765058737954042;
                                                            break ;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    i = i.wrapping_add(1)
                                }
                                match current_block {
                                    7187023563388198622 => { }
                                    _ => {
                                        if res == 0i32 {
                                            mp_exch(&mut trial, start);
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
    mp_clear(&mut trial);
    mp_clear(&mut q);
    if !sieve.is_null() {
        memset(sieve as *mut libc::c_void, 0i32,
               (32i32 * 1024i32) as libc::c_ulong);
        free(sieve as *mut libc::c_void);
    }
    return res;
}