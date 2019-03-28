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
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    /* Memory management       */
    #[no_mangle]
    fn mp_init(mp: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_clear(mp: *mut mp_int);
    #[no_mangle]
    fn mp_set(mp: *mut mp_int, d: mp_digit);
    /* Full arithmetic         */
    #[no_mangle]
    fn mp_add(a: *const mp_int, b: *const mp_int, c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    #[no_mangle]
    static mut mutest_case_failed: libc::c_int;
    /* checks */
    #[no_mangle]
    static mut mutest_failed_checks: libc::c_int;
    #[no_mangle]
    static mut mutest_passed_checks: libc::c_int;
    /* verbosity */
    #[no_mangle]
    static mut mutest_verbose_level: libc::c_int;
}
pub type __int64_t = libc::c_longlong;
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
 * This file is part of mutest, a simple micro unit testing framework for C.
 *
 * mutest was written by Leandro Lucarella <llucax@gmail.com> and is released
 * under the BOLA license, please see the LICENSE file or visit:
 * http://blitiri.com.ar/p/bola/
 *
 * This header file should be included in the source files that will make up
 * a test suite. It's used for both C and Python implementation, but when
 * using the Python implementation you should define the MUTEST_PY macro.
 * If you implement your mu_run_suites() function yourself, you probably will
 * need to include this header too (see mkmutest).
 *
 * Please, read the README file for more details.
 */
/* fprintf() */
/* verbosity level (each level shows all the previous levels too) */
pub type unnamed = libc::c_uint;
/* shows the current running check */
pub const MU_CHECK: unnamed = 5;
/* shows test cases progress */
pub const MU_CASE: unnamed = 4;
/* shows test suites progress */
pub const MU_SUITE: unnamed = 3;
/* shows a summary */
pub const MU_SUMMARY: unnamed = 2;
/* shows errors only */
pub const MU_ERROR: unnamed = 1;
/* be completely quiet */
pub const MU_QUIET: unnamed = 0;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn mu_test_mpi__add() {
    let mut a: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut b: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut c: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(mp_init(&a) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(mp_init(&a) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        }
    }
    if mp_init(&mut a) == 0i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/mpi_test.c:%d: mu_check(mp_init(&a) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 20i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/mpi_test.c:%d: mu_check(mp_init(&a) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 20i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(mp_init(&b) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(mp_init(&b) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        }
    }
    if mp_init(&mut b) == 0i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/mpi_test.c:%d: mu_check(mp_init(&b) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 21i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/mpi_test.c:%d: mu_check(mp_init(&b) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 21i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(mp_init(&c) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(mp_init(&c) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        }
    }
    if mp_init(&mut c) == 0i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/mpi_test.c:%d: mu_check(mp_init(&c) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 22i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/mpi_test.c:%d: mu_check(mp_init(&c) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 22i32);
            }
        }
    }
    mp_set(&mut a, 10i32 as mp_digit);
    mp_set(&mut b, 7i32 as mp_digit);
    mp_add(&mut a, &mut b, &mut c);
    mp_set(&mut a, 17i32 as mp_digit);
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(mp_cmp(&a, &c) == 0)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(mp_cmp(&a, &c) == 0)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if mp_cmp(&mut a, &mut c) == 0i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/mpi_test.c:%d: mu_check(mp_cmp(&a, &c) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 29i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/mpi_test.c:%d: mu_check(mp_cmp(&a, &c) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 29i32);
            }
        }
    }
    mp_clear(&mut a);
    mp_clear(&mut b);
    mp_clear(&mut c);
}