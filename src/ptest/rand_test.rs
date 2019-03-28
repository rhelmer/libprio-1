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
    #[no_mangle]
    fn mp_mod_d(a: *const mp_int, d: mp_digit, c: *mut mp_digit) -> mp_err;
    /* MP_MODARITH */
    /* Comparisons             */
    #[no_mangle]
    fn mp_cmp_z(a: *const mp_int) -> libc::c_int;
    #[no_mangle]
    fn mp_cmp_d(a: *const mp_int, d: mp_digit) -> libc::c_int;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    #[no_mangle]
    fn mp_read_radix(mp: *mut mp_int, str: *const libc::c_char,
                     radix: libc::c_int) -> mp_err;
    #[no_mangle]
    fn mp_to_fixlen_octets(mp: *const mp_int, str: *mut libc::c_uchar,
                           len: mp_size) -> mp_err;
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
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    /*
 * Generate a random number x such that
 *    0 <= x < max
 * using the NSS random number generator.
 */
    #[no_mangle]
    fn rand_int(out: *mut mp_int, max: *const mp_int) -> SECStatus;
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
** A status code. Status's are used by procedures that return status
** values. Again the motivation is so that a compiler can generate
** warnings when return values are wrong. Correct testing of status codes:
**
**  SECStatus rv;
**  rv = some_function (some_argument);
**  if (rv != SECSuccess)
**      do_an_error_thing();
**
*/
pub type _SECStatus = libc::c_int;
pub const SECSuccess: _SECStatus = 0;
pub const SECFailure: _SECStatus = -1;
pub const SECWouldBlock: _SECStatus = -2;
pub type SECStatus = _SECStatus;
/*
 * Return a mask that masks out all of the zero bits
 */
unsafe extern "C" fn msb_mask(mut val: libc::c_uchar) -> libc::c_uchar {
    let mut mask: libc::c_uchar = 0;
    mask = 0i32 as libc::c_uchar;
    while val as libc::c_int & mask as libc::c_int != val as libc::c_int {
        mask = (((mask as libc::c_int) << 1i32) + 1i32) as libc::c_uchar
    }
    return mask;
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn mu_test__util_msb_mast() {
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(msb_mask(0x01) == 0x01)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(msb_mask(0x01) == 0x01)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if msb_mask(0x1i32 as libc::c_uchar) as libc::c_int == 0x1i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0x01) == 0x01) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 20i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0x01) == 0x01) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 20i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(msb_mask(0x02) == 0x03)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(msb_mask(0x02) == 0x03)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if msb_mask(0x2i32 as libc::c_uchar) as libc::c_int == 0x3i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0x02) == 0x03) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 21i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0x02) == 0x03) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 21i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(msb_mask(0x0C) == 0x0F)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(msb_mask(0x0C) == 0x0F)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if msb_mask(0xci32 as libc::c_uchar) as libc::c_int == 0xfi32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0x0C) == 0x0F) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 22i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0x0C) == 0x0F) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 22i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(msb_mask(0x1C) == 0x1F)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(msb_mask(0x1C) == 0x1F)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if msb_mask(0x1ci32 as libc::c_uchar) as libc::c_int == 0x1fi32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0x1C) == 0x1F) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 23i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0x1C) == 0x1F) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 23i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(msb_mask(0xFF) == 0xFF)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(msb_mask(0xFF) == 0xFF)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if msb_mask(0xffi32 as libc::c_uchar) as libc::c_int == 0xffi32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0xFF) == 0xFF) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 24i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(msb_mask(0xFF) == 0xFF) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 24i32);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_rand_once(mut limit: libc::c_int) {
    let mut max: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut out: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(mp_init(&max) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(mp_init(&max) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        }
    }
    if mp_init(&mut max) == 0i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(mp_init(&max) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 33i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(mp_init(&max) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 33i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(mp_init(&out) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(mp_init(&out) == 0)...\n\x00" as
                        *const u8 as *const libc::c_char);
        }
    }
    if mp_init(&mut out) == 0i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(mp_init(&out) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 34i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(mp_init(&out) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 34i32);
            }
        }
    }
    mp_set(&mut max, limit as mp_digit);
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(rand_int(&out, &max) == 0)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(rand_int(&out, &max) == 0)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if rand_int(&mut out, &mut max) as libc::c_int == 0i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(rand_int(&out, &max) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 38i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(rand_int(&out, &max) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 38i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if mp_cmp_d(&mut out, limit as mp_digit) == -1i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 39i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 39i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if mp_cmp_z(&mut out) > -1i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 40i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 40i32);
            }
        }
    }
    mp_clear(&mut max);
    mp_clear(&mut out);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_rand__multiple_of_8() {
    test_rand_once(256i32);
    test_rand_once(256i32 * 256i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_rand__near_multiple_of_8() {
    test_rand_once(256i32 + 1i32);
    test_rand_once(256i32 * 256i32 + 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_rand__odd() {
    test_rand_once(39i32);
    test_rand_once(123i32);
    test_rand_once(993123i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_rand__large() {
    test_rand_once(1231239933i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_rand__bit() {
    test_rand_once(1i32);
    let mut i: libc::c_int = 0i32;
    while i < 100i32 { test_rand_once(2i32); i += 1 };
}
#[no_mangle]
pub unsafe extern "C" fn test_rand_distribution(mut limit: libc::c_int) {
    let mut current_block: u64;
    let mut rv: SECStatus = SECSuccess;
    let mut bins: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut max: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut out: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    max.dp = 0 as *mut mp_digit;
    out.dp = 0 as *mut mp_digit;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (bins = calloc(limit, sizeof(int)))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (bins = calloc(limit, sizeof(int)))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    bins =
        calloc(limit as libc::c_ulong,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as
            *mut libc::c_int;
    v = !bins.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check((v = (bins = calloc(limit, sizeof(int))))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 94i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check((v = (bins = calloc(limit, sizeof(int))))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 94i32);
            }
        }
    }
    if !v {
        rv = SECFailure
    } else {
        let mut v_0: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (mp_init(&max))) == 0)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (mp_init(&max))) == 0)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        v_0 = 0 != mp_init(&mut max);
        if v_0 as libc::c_int == 0i32 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/rand_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 96i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/rand_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 96i32);
                }
            }
        }
        if v_0 as libc::c_int != 0i32 {
            rv = SECFailure
        } else {
            let mut v_1: bool = false;
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((v = (mp_init(&out))) == 0)...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (mp_init(&out))) == 0)...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            v_1 = 0 != mp_init(&mut out);
            if v_1 as libc::c_int == 0i32 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/rand_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                97i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/rand_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                97i32);
                    }
                }
            }
            if v_1 as libc::c_int != 0i32 {
                rv = SECFailure
            } else {
                mp_set(&mut max, limit as mp_digit);
                let mut i: libc::c_int = 0i32;
                while i < limit { *bins.offset(i as isize) = 0i32; i += 1 }
                let mut i_0: libc::c_int = 0i32;
                loop  {
                    if !(i_0 < limit * limit) {
                        current_block = 13526015532137226550;
                        break ;
                    }
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(rand_int(&out, &max) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(rand_int(&out, &max) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if rand_int(&mut out, &mut max) as libc::c_int == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/rand_test.c:%d: mu_check(rand_int(&out, &max) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 106i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/rand_test.c:%d: mu_check(rand_int(&out, &max) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 106i32);
                            }
                        }
                    }
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if mp_cmp_d(&mut out, limit as mp_digit) == -1i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 107i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 107i32);
                            }
                        }
                    }
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if mp_cmp_z(&mut out) > -1i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 108i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 108i32);
                            }
                        }
                    }
                    let mut ival: [libc::c_uchar; 2] =
                        [0i32 as libc::c_uchar, 0i32 as libc::c_uchar];
                    let mut v_2: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (mp_to_fixlen_octets(&out, ival, 2))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (mp_to_fixlen_octets(&out, ival, 2))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    v_2 =
                        0 !=
                            mp_to_fixlen_octets(&mut out, ival.as_mut_ptr(),
                                                2i32 as mp_size);
                    if v_2 as libc::c_int == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/rand_test.c:%d: mu_check((v = (mp_to_fixlen_octets(&out, ival, 2))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 111i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/rand_test.c:%d: mu_check((v = (mp_to_fixlen_octets(&out, ival, 2))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 111i32);
                            }
                        }
                    }
                    if v_2 as libc::c_int != 0i32 {
                        rv = SECFailure;
                        current_block = 8662225482666887503;
                        break ;
                    } else {
                        if (ival[1usize] as libc::c_int +
                                256i32 * ival[0usize] as libc::c_int) < limit
                           {
                            *bins.offset((ival[1usize] as libc::c_int +
                                              256i32 *
                                                  ival[0usize] as libc::c_int)
                                             as isize) += 1i32
                        } else {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/rand_test.c:%d: mu_check(0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 115i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/rand_test.c:%d: mu_check(0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 115i32);
                                }
                            }
                        }
                        i_0 += 1
                    }
                }
                match current_block {
                    8662225482666887503 => { }
                    _ => {
                        let mut i_1: libc::c_int = 0i32;
                        while i_1 < limit {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(bins[i] > limit / 2)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(bins[i] > limit / 2)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if *bins.offset(i_1 as isize) > limit / 2i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/rand_test.c:%d: mu_check(bins[i] > limit / 2) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                120i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/rand_test.c:%d: mu_check(bins[i] > limit / 2) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                120i32);
                                    }
                                }
                            }
                            i_1 += 1
                        }
                    }
                }
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(rv == SECSuccess)...\n\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(rv == SECSuccess)...\n\x00" as
                        *const u8 as *const libc::c_char);
        }
    }
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 124i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 124i32);
            }
        }
    }
    if !bins.is_null() { free(bins as *mut libc::c_void); }
    mp_clear(&mut max);
    mp_clear(&mut out);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__rand_distribution123() {
    test_rand_distribution(123i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__rand_distribution257() {
    test_rand_distribution(257i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__rand_distribution259() {
    test_rand_distribution(259i32);
}
#[no_mangle]
pub unsafe extern "C" fn test_rand_distribution_large(mut max: *mut mp_int) {
    let mut current_block: u64;
    let mut rv: SECStatus = SECSuccess;
    let mut limit: libc::c_int = 16i32;
    let mut bins: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut out: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    out.dp = 0 as *mut mp_digit;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (mp_init(&out))) == 0)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (mp_init(&out))) == 0)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    v = 0 != mp_init(&mut out);
    if v as libc::c_int == 0i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 158i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 158i32);
            }
        }
    }
    if v as libc::c_int != 0i32 {
        rv = SECFailure
    } else {
        let mut v_0: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (bins = calloc(limit, sizeof(int)))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (bins = calloc(limit, sizeof(int)))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        bins =
            calloc(limit as libc::c_ulong,
                   ::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as
                *mut libc::c_int;
        v_0 = !bins.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/rand_test.c:%d: mu_check((v = (bins = calloc(limit, sizeof(int))))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 159i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/rand_test.c:%d: mu_check((v = (bins = calloc(limit, sizeof(int))))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 159i32);
                }
            }
        }
        if !v_0 {
            rv = SECFailure
        } else {
            let mut i: libc::c_int = 0i32;
            while i < limit { *bins.offset(i as isize) = 0i32; i += 1 }
            let mut i_0: libc::c_int = 0i32;
            loop  {
                if !(i_0 < 100i32 * limit * limit) {
                    current_block = 15514437232607373049;
                    break ;
                }
                let mut v_1: bool = false;
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((v = (rand_int(&out, max))) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (rand_int(&out, max))) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                v_1 = 0 != rand_int(&mut out, max) as u64;
                if v_1 as libc::c_int == 0i32 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/rand_test.c:%d: mu_check((v = (rand_int(&out, max))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    166i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/rand_test.c:%d: mu_check((v = (rand_int(&out, max))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    166i32);
                        }
                    }
                }
                if v_1 as libc::c_int != 0i32 {
                    rv = SECFailure;
                    current_block = 5194334453749097548;
                    break ;
                } else {
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(mp_cmp(&out, max) == -1)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(mp_cmp(&out, max) == -1)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if mp_cmp(&mut out, max) == -1i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp(&out, max) == -1) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 167i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp(&out, max) == -1) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 167i32);
                            }
                        }
                    }
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if mp_cmp_z(&mut out) > -1i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 168i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/rand_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 168i32);
                            }
                        }
                    }
                    let mut res: libc::c_ulong = 0;
                    let mut v_2: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (mp_mod_d(&out, limit, &res))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (mp_mod_d(&out, limit, &res))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    v_2 =
                        0 != mp_mod_d(&mut out, limit as mp_digit, &mut res);
                    if v_2 as libc::c_int == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/rand_test.c:%d: mu_check((v = (mp_mod_d(&out, limit, &res))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 171i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/rand_test.c:%d: mu_check((v = (mp_mod_d(&out, limit, &res))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 171i32);
                            }
                        }
                    }
                    if v_2 as libc::c_int != 0i32 {
                        rv = SECFailure;
                        current_block = 5194334453749097548;
                        break ;
                    } else { *bins.offset(res as isize) += 1i32; i_0 += 1 }
                }
            }
            match current_block {
                5194334453749097548 => { }
                _ => {
                    let mut i_1: libc::c_int = 0i32;
                    while i_1 < limit {
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check(bins[i] > limit / 2)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check(bins[i] > limit / 2)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        if *bins.offset(i_1 as isize) > limit / 2i32 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/rand_test.c:%d: mu_check(bins[i] > limit / 2) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 176i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/rand_test.c:%d: mu_check(bins[i] > limit / 2) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 176i32);
                                }
                            }
                        }
                        i_1 += 1
                    }
                }
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(rv == SECSuccess)...\n\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(rv == SECSuccess)...\n\x00" as
                        *const u8 as *const libc::c_char);
        }
    }
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 180i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 180i32);
            }
        }
    }
    if !bins.is_null() { free(bins as *mut libc::c_void); }
    mp_clear(&mut out);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__rand_distribution_large() {
    let mut bytes: [libc::c_char; 26] = [0; 26];
    let mut rv: SECStatus = SECSuccess;
    let mut max: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    max.dp = 0 as *mut mp_digit;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (mp_init(&max))) == 0)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (mp_init(&max))) == 0)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    v = 0 != mp_init(&mut max);
    if v as libc::c_int == 0i32 {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 192i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 192i32);
            }
        }
    }
    if v as libc::c_int != 0i32 {
        rv = SECFailure
    } else {
        bytes =
            *::std::mem::transmute::<&[u8; 26],
                                     &mut [libc::c_char; 26]>(b"FF1230985198451798EDC8123\x00");
        let mut v_0: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (mp_read_radix(&max, bytes, 16))) == 0)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (mp_read_radix(&max, bytes, 16))) == 0)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        v_0 = 0 != mp_read_radix(&mut max, bytes.as_mut_ptr(), 16i32);
        if v_0 as libc::c_int == 0i32 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/rand_test.c:%d: mu_check((v = (mp_read_radix(&max, bytes, 16))) == 0) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 195i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/rand_test.c:%d: mu_check((v = (mp_read_radix(&max, bytes, 16))) == 0) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 195i32);
                }
            }
        }
        if v_0 as libc::c_int != 0i32 {
            rv = SECFailure
        } else { test_rand_distribution_large(&mut max); }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(rv == SECSuccess)...\n\x00" as
                        *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(rv == SECSuccess)...\n\x00" as
                        *const u8 as *const libc::c_char);
        }
    }
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/rand_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 199i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/rand_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 199i32);
            }
        }
    }
    mp_clear(&mut max);
}