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
    pub type PK11SlotInfoStr;
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
    fn mp_addmod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                 c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_mulmod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                 c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_exptmod_d(a: *const mp_int, d: mp_digit, m: *const mp_int,
                    c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_cmp_d(a: *const mp_int, d: mp_digit) -> libc::c_int;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    #[no_mangle]
    fn PrioConfig_clear(cfg: PrioConfig);
    /*
 * Create a PrioConfig object with no encryption keys.  This routine is
 * useful for testing, but PrioClient_encode() will always fail when used with
 * this config.
 */
    #[no_mangle]
    fn PrioConfig_newTest(nFields: libc::c_int) -> PrioConfig;
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
    /*
 * Initialize an array of `mp_int`s of the given length.
 */
    #[no_mangle]
    fn MPArray_new(len: libc::c_int) -> MPArray;
    #[no_mangle]
    fn MPArray_clear(arr: MPArray);
    /*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
    /*
 * Compute the FFT or inverse FFT of the array in `points_in`.
 * The length of the input and output arrays must be a multiple
 * of two and must be no longer than the number of precomputed
 * roots in the PrioConfig object passed in.
 */
    #[no_mangle]
    fn poly_fft(points_out: MPArray, points_in: const_MPArray,
                cfg: const_PrioConfig, invert: bool) -> SECStatus;
    /*
 * Get an array
 *    (r^0, r^1, r^2, ... )
 * where r is an n-th root of unity, for n a power of two
 * less than cfg->n_roots.
 *
 * Do NOT mp_clear() the mp_ints stored in roots_out.
 * These are owned by the PrioConfig object.
 */
    #[no_mangle]
    fn poly_fft_get_roots(roots_out: MPArray, n_points: libc::c_int,
                          cfg: const_PrioConfig, invert: bool) -> SECStatus;
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
/* ***********************************************************************
 * MACROS:      PR_INT16_MAX
 *              PR_INT16_MIN
 *              PR_UINT16_MAX
 * DESCRIPTION:
 *  The maximum and minimum values of a PRInt16 or PRUint16.
************************************************************************/
/* ***********************************************************************
** TYPES:       PRUint32
**              PRInt32
** DESCRIPTION:
**  The int32 types are known to be 32 bits each.
************************************************************************/
pub type PRUint32 = libc::c_uint;
pub type PRUword = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct PLArena {
    pub next: *mut PLArena,
    pub base: PRUword,
    pub limit: PRUword,
    pub avail: PRUword,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct PLArenaPool {
    pub first: PLArena,
    pub current: *mut PLArena,
    pub arenasize: PRUint32,
    pub mask: PRUword,
}
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
 * seccomon.h - common data structures for security libraries
 *
 * This file should have lowest-common-denominator datastructures
 * for security libraries.  It should not be dependent on any other
 * headers, and should not require linking with any libraries.
 */
pub type SECItemType = libc::c_uint;
pub const siBMPString: SECItemType = 15;
pub const siUTF8String: SECItemType = 14;
pub const siVisibleString: SECItemType = 13;
pub const siGeneralizedTime: SECItemType = 12;
pub const siUTCTime: SECItemType = 11;
pub const siUnsignedInteger: SECItemType = 10;
pub const siDEROID: SECItemType = 9;
pub const siAsciiString: SECItemType = 8;
pub const siAsciiNameString: SECItemType = 7;
pub const siEncodedNameBuffer: SECItemType = 6;
pub const siDERNameBuffer: SECItemType = 5;
pub const siEncodedCertBuffer: SECItemType = 4;
pub const siDERCertBuffer: SECItemType = 3;
pub const siCipherDataBuffer: SECItemType = 2;
pub const siClearDataBuffer: SECItemType = 1;
pub const siBuffer: SECItemType = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECItemStr {
    pub type_0: SECItemType,
    pub data: *mut libc::c_uchar,
    pub len: libc::c_uint,
}
pub type SECItem = SECItemStr;
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
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* deprecated */
pub type ECPointEncoding = libc::c_uint;
pub const ECPoint_Undefined: ECPointEncoding = 2;
pub const ECPoint_XOnly: ECPointEncoding = 1;
pub const ECPoint_Uncompressed: ECPointEncoding = 0;
/* an unsigned value, at least 32 bits long */
pub type CK_ULONG = libc::c_ulong;
/* CK_OBJECT_HANDLE is a token-specific identifier for an
 * object  */
pub type CK_OBJECT_HANDLE = CK_ULONG;
/* defined in secmodti.h */
pub type PK11SlotInfo = PK11SlotInfoStr;
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
** RFC 4055 Section 1.2 specifies three different RSA key types.
**
** rsaKey maps to keys with SEC_OID_PKCS1_RSA_ENCRYPTION and can be used for
** both encryption and signatures with old (PKCS #1 v1.5) and new (PKCS #1
** v2.1) padding schemes.
**
** rsaPssKey maps to keys with SEC_OID_PKCS1_RSA_PSS_SIGNATURE and may only
** be used for signatures with PSS padding (PKCS #1 v2.1).
**
** rsaOaepKey maps to keys with SEC_OID_PKCS1_RSA_OAEP_ENCRYPTION and may only
** be used for encryption with OAEP padding (PKCS #1 v2.1).
*/
pub type KeyType = libc::c_uint;
pub const rsaOaepKey: KeyType = 8;
pub const rsaPssKey: KeyType = 7;
pub const ecKey: KeyType = 6;
/* deprecated */
pub const keaKey: KeyType = 5;
pub const dhKey: KeyType = 4;
/* deprecated */
pub const fortezzaKey: KeyType = 3;
pub const dsaKey: KeyType = 2;
pub const rsaKey: KeyType = 1;
pub const nullKey: KeyType = 0;
/*
** RSA Public Key structures
** member names from PKCS#1, section 7.1
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYRSAPublicKeyStr {
    pub arena: *mut PLArenaPool,
    pub modulus: SECItem,
    pub publicExponent: SECItem,
}
pub type SECKEYRSAPublicKey = SECKEYRSAPublicKeyStr;
/*
** DSA Public Key and related structures
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYPQGParamsStr {
    pub arena: *mut PLArenaPool,
    pub prime: SECItem,
    pub subPrime: SECItem,
    pub base: SECItem,
}
/* XXX chrisk: this needs to be expanded to hold j and validationParms (RFC2459 7.3.2) */
pub type SECKEYPQGParams = SECKEYPQGParamsStr;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYDSAPublicKeyStr {
    pub params: SECKEYPQGParams,
    pub publicValue: SECItem,
}
pub type SECKEYDSAPublicKey = SECKEYDSAPublicKeyStr;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYDHPublicKeyStr {
    pub arena: *mut PLArenaPool,
    pub prime: SECItem,
    pub base: SECItem,
    pub publicValue: SECItem,
}
pub type SECKEYDHPublicKey = SECKEYDHPublicKeyStr;
/*
** Elliptic curve Public Key structure
** The PKCS#11 layer needs DER encoding of ANSI X9.62
** parameters value
*/
pub type SECKEYECParams = SECItem;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYECPublicKeyStr {
    pub DEREncodedParams: SECKEYECParams,
    pub size: libc::c_int,
    pub publicValue: SECItem,
    pub encoding: ECPointEncoding,
}
pub type SECKEYECPublicKey = SECKEYECPublicKeyStr;
/*
** FORTEZZA Public Key structures
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYFortezzaPublicKeyStr {
    pub KEAversion: libc::c_int,
    pub DSSversion: libc::c_int,
    pub KMID: [libc::c_uchar; 8],
    pub clearance: SECItem,
    pub KEApriviledge: SECItem,
    pub DSSpriviledge: SECItem,
    pub KEAKey: SECItem,
    pub DSSKey: SECItem,
    pub params: SECKEYPQGParams,
    pub keaParams: SECKEYPQGParams,
}
pub type SECKEYFortezzaPublicKey = SECKEYFortezzaPublicKeyStr;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYKEAParamsStr {
    pub arena: *mut PLArenaPool,
    pub hash: SECItem,
}
pub type SECKEYKEAParams = SECKEYKEAParamsStr;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYKEAPublicKeyStr {
    pub params: SECKEYKEAParams,
    pub publicValue: SECItem,
}
pub type SECKEYKEAPublicKey = SECKEYKEAPublicKeyStr;
/*
** A Generic  public key object.
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYPublicKeyStr {
    pub arena: *mut PLArenaPool,
    pub keyType: KeyType,
    pub pkcs11Slot: *mut PK11SlotInfo,
    pub pkcs11ID: CK_OBJECT_HANDLE,
    pub u: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    pub rsa: SECKEYRSAPublicKey,
    pub dsa: SECKEYDSAPublicKey,
    pub dh: SECKEYDHPublicKey,
    pub kea: SECKEYKEAPublicKey,
    pub fortezza: SECKEYFortezzaPublicKey,
    pub ec: SECKEYECPublicKey,
}
pub type SECKEYPublicKey = SECKEYPublicKeyStr;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct prio_config {
    pub num_data_fields: libc::c_int,
    pub batch_id: *mut libc::c_uchar,
    pub batch_id_len: libc::c_uint,
    pub server_a_pub: PublicKey,
    pub server_b_pub: PublicKey,
    pub modulus: mp_int,
    pub inv2: mp_int,
    pub n_roots: libc::c_int,
    pub generator: mp_int,
}
pub type PublicKey = *mut SECKEYPublicKey;
/*
 * Opaque types
 */
pub type PrioConfig = *mut prio_config;
pub type const_PrioConfig = *const prio_config;
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
pub type unnamed_0 = libc::c_uint;
/* shows the current running check */
pub const MU_CHECK: unnamed_0 = 5;
/* shows test cases progress */
pub const MU_CASE: unnamed_0 = 4;
/* shows test suites progress */
pub const MU_SUITE: unnamed_0 = 3;
/* shows a summary */
pub const MU_SUMMARY: unnamed_0 = 2;
/* shows errors only */
pub const MU_ERROR: unnamed_0 = 1;
/* be completely quiet */
pub const MU_QUIET: unnamed_0 = 0;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mparray {
    pub len: libc::c_int,
    pub data: *mut mp_int,
}
pub type MPArray = *mut mparray;
pub type const_MPArray = *const mparray;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn mu_test__fft_one() {
    let mut rv: SECStatus = SECSuccess;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut points_in: MPArray = 0 as MPArray;
    let mut points_out: MPArray = 0 as MPArray;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(123))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(123))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(123i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/fft_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(123)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 28i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/fft_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(123)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 28i32);
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
                        b"\t\t* Checking mu_check((v = (points_in = MPArray_new(1))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (points_in = MPArray_new(1))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        points_in = MPArray_new(1i32);
        v_0 = !points_in.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/fft_test.c:%d: mu_check((v = (points_in = MPArray_new(1)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 29i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/fft_test.c:%d: mu_check((v = (points_in = MPArray_new(1)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 29i32);
                }
            }
        }
        if !v_0 {
            rv = SECFailure
        } else {
            let mut v_1: bool = false;
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((v = (points_out = MPArray_new(1))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (points_out = MPArray_new(1))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            points_out = MPArray_new(1i32);
            v_1 = !points_out.is_null();
            if v_1 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/fft_test.c:%d: mu_check((v = (points_out = MPArray_new(1)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                30i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/fft_test.c:%d: mu_check((v = (points_out = MPArray_new(1)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                30i32);
                    }
                }
            }
            if !v_1 {
                rv = SECFailure
            } else {
                mp_set(&mut *(*points_in).data.offset(0isize),
                       3i32 as mp_digit);
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                if poly_fft(points_out, points_in as const_MPArray,
                            cfg as const_PrioConfig, 0 != 0i32) as libc::c_int
                       == SECSuccess as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/fft_test.c:%d: mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    33i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/fft_test.c:%d: mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    33i32);
                        }
                    }
                }
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check(mp_cmp_d(&points_in->data[0], 3) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check(mp_cmp_d(&points_in->data[0], 3) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                if mp_cmp_d(&mut *(*points_in).data.offset(0isize),
                            3i32 as mp_digit) == 0i32 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/fft_test.c:%d: mu_check(mp_cmp_d(&points_in->data[0], 3) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    35i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/fft_test.c:%d: mu_check(mp_cmp_d(&points_in->data[0], 3) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    35i32);
                        }
                    }
                }
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check(mp_cmp_d(&points_out->data[0], 3) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check(mp_cmp_d(&points_out->data[0], 3) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                if mp_cmp_d(&mut *(*points_out).data.offset(0isize),
                            3i32 as mp_digit) == 0i32 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/fft_test.c:%d: mu_check(mp_cmp_d(&points_out->data[0], 3) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    36i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/fft_test.c:%d: mu_check(mp_cmp_d(&points_out->data[0], 3) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    36i32);
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
                        b"build/ptest/fft_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 39i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/fft_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 39i32);
            }
        }
    }
    MPArray_clear(points_in);
    MPArray_clear(points_out);
    PrioConfig_clear(cfg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__fft_roots() {
    let mut rv: SECStatus = SECSuccess;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut roots: MPArray = 0 as MPArray;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    tmp.dp = 0 as *mut mp_digit;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(90))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(90))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(90i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/fft_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(90)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 55i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/fft_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(90)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 55i32);
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
                        b"\t\t* Checking mu_check((v = (roots = MPArray_new(4))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (roots = MPArray_new(4))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        roots = MPArray_new(4i32);
        v_0 = !roots.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/fft_test.c:%d: mu_check((v = (roots = MPArray_new(4)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 56i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/fft_test.c:%d: mu_check((v = (roots = MPArray_new(4)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 56i32);
                }
            }
        }
        if !v_0 {
            rv = SECFailure
        } else {
            let mut v_1: bool = false;
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((v = (mp_init(&tmp))) == 0)...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (mp_init(&tmp))) == 0)...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            v_1 = 0 != mp_init(&mut tmp);
            if v_1 as libc::c_int == 0i32 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/fft_test.c:%d: mu_check((v = (mp_init(&tmp))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                57i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/fft_test.c:%d: mu_check((v = (mp_init(&tmp))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                57i32);
                    }
                }
            }
            if v_1 as libc::c_int != 0i32 {
                rv = SECFailure
            } else {
                poly_fft_get_roots(roots, 4i32, cfg as const_PrioConfig,
                                   0 != 0i32);
                let mut i: libc::c_int = 0i32;
                while i < 4i32 {
                    mp_exptmod_d(&mut *(*roots).data.offset(i as isize),
                                 4i32 as mp_digit, &mut (*cfg).modulus,
                                 &mut tmp);
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(mp_cmp_d(&tmp, 1) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(mp_cmp_d(&tmp, 1) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if mp_cmp_d(&mut tmp, 1i32 as mp_digit) == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/fft_test.c:%d: mu_check(mp_cmp_d(&tmp, 1) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 63i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/fft_test.c:%d: mu_check(mp_cmp_d(&tmp, 1) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 63i32);
                            }
                        }
                    }
                    i += 1
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
                        b"build/ptest/fft_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 67i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/fft_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 67i32);
            }
        }
    }
    mp_clear(&mut tmp);
    MPArray_clear(roots);
    PrioConfig_clear(cfg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__fft_simple() {
    let mut rv: SECStatus = SECSuccess;
    let nPoints: libc::c_int = 4i32;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut points_in: MPArray = 0 as MPArray;
    let mut points_out: MPArray = 0 as MPArray;
    let mut roots: MPArray = 0 as MPArray;
    let mut should_be: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    should_be.dp = 0 as *mut mp_digit;
    tmp.dp = 0 as *mut mp_digit;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(140))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(140))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(140i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/fft_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(140)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 88i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/fft_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(140)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 88i32);
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
                        b"\t\t* Checking mu_check((v = (points_in = MPArray_new(nPoints))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (points_in = MPArray_new(nPoints))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        points_in = MPArray_new(nPoints);
        v_0 = !points_in.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/fft_test.c:%d: mu_check((v = (points_in = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 89i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/fft_test.c:%d: mu_check((v = (points_in = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 89i32);
                }
            }
        }
        if !v_0 {
            rv = SECFailure
        } else {
            let mut v_1: bool = false;
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((v = (points_out = MPArray_new(nPoints))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (points_out = MPArray_new(nPoints))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            points_out = MPArray_new(nPoints);
            v_1 = !points_out.is_null();
            if v_1 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/fft_test.c:%d: mu_check((v = (points_out = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                90i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/fft_test.c:%d: mu_check((v = (points_out = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                90i32);
                    }
                }
            }
            if !v_1 {
                rv = SECFailure
            } else {
                let mut v_2: bool = false;
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((v = (roots = MPArray_new(nPoints))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (roots = MPArray_new(nPoints))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                roots = MPArray_new(nPoints);
                v_2 = !roots.is_null();
                if v_2 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/fft_test.c:%d: mu_check((v = (roots = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    91i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/fft_test.c:%d: mu_check((v = (roots = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    91i32);
                        }
                    }
                }
                if !v_2 {
                    rv = SECFailure
                } else {
                    let mut v_3: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (mp_init(&should_be))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (mp_init(&should_be))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    v_3 = 0 != mp_init(&mut should_be);
                    if v_3 as libc::c_int == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/fft_test.c:%d: mu_check((v = (mp_init(&should_be))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 93i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/fft_test.c:%d: mu_check((v = (mp_init(&should_be))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 93i32);
                            }
                        }
                    }
                    if v_3 as libc::c_int != 0i32 {
                        rv = SECFailure
                    } else {
                        let mut v_4: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = (mp_init(&tmp))) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (mp_init(&tmp))) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        v_4 = 0 != mp_init(&mut tmp);
                        if v_4 as libc::c_int == 0i32 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/fft_test.c:%d: mu_check((v = (mp_init(&tmp))) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 94i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/fft_test.c:%d: mu_check((v = (mp_init(&tmp))) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 94i32);
                                }
                            }
                        }
                        if v_4 as libc::c_int != 0i32 {
                            rv = SECFailure
                        } else {
                            poly_fft_get_roots(roots, nPoints,
                                               cfg as const_PrioConfig,
                                               0 != 0i32);
                            mp_set(&mut *(*points_in).data.offset(0isize),
                                   3i32 as mp_digit);
                            mp_set(&mut *(*points_in).data.offset(1isize),
                                   8i32 as mp_digit);
                            mp_set(&mut *(*points_in).data.offset(2isize),
                                   7i32 as mp_digit);
                            mp_set(&mut *(*points_in).data.offset(3isize),
                                   9i32 as mp_digit);
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if poly_fft(points_out,
                                        points_in as const_MPArray,
                                        cfg as const_PrioConfig, 0 != 0i32) as
                                   libc::c_int == SECSuccess as libc::c_int {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/fft_test.c:%d: mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                102i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/fft_test.c:%d: mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                102i32);
                                    }
                                }
                            }
                            let mut i: libc::c_int = 0i32;
                            while i < nPoints {
                                mp_set(&mut should_be, 0i32 as mp_digit);
                                let mut j: libc::c_int = 0i32;
                                while j < nPoints {
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check(mp_exptmod_d(&roots->data[i], j, &cfg->modulus, &tmp) == 0)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(mp_exptmod_d(&roots->data[i], j, &cfg->modulus, &tmp) == 0)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if mp_exptmod_d(&mut *(*roots).data.offset(i
                                                                                   as
                                                                                   isize),
                                                    j as mp_digit,
                                                    &mut (*cfg).modulus,
                                                    &mut tmp) == 0i32 {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/fft_test.c:%d: mu_check(mp_exptmod_d(&roots->data[i], j, &cfg->modulus, &tmp) == 0) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        108i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/fft_test.c:%d: mu_check(mp_exptmod_d(&roots->data[i], j, &cfg->modulus, &tmp) == 0) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        108i32);
                                            }
                                        }
                                    }
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check(mp_mulmod(&tmp, &points_in->data[j], &cfg->modulus, &tmp) == 0)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(mp_mulmod(&tmp, &points_in->data[j], &cfg->modulus, &tmp) == 0)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if mp_mulmod(&mut tmp,
                                                 &mut *(*points_in).data.offset(j
                                                                                    as
                                                                                    isize),
                                                 &mut (*cfg).modulus,
                                                 &mut tmp) == 0i32 {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/fft_test.c:%d: mu_check(mp_mulmod(&tmp, &points_in->data[j], &cfg->modulus, &tmp) == 0) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        110i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/fft_test.c:%d: mu_check(mp_mulmod(&tmp, &points_in->data[j], &cfg->modulus, &tmp) == 0) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        110i32);
                                            }
                                        }
                                    }
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check(mp_addmod(&should_be, &tmp, &cfg->modulus, &should_be) == 0)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(mp_addmod(&should_be, &tmp, &cfg->modulus, &should_be) == 0)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if mp_addmod(&mut should_be, &mut tmp,
                                                 &mut (*cfg).modulus,
                                                 &mut should_be) == 0i32 {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/fft_test.c:%d: mu_check(mp_addmod(&should_be, &tmp, &cfg->modulus, &should_be) == 0) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        112i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/fft_test.c:%d: mu_check(mp_addmod(&should_be, &tmp, &cfg->modulus, &should_be) == 0) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        112i32);
                                            }
                                        }
                                    }
                                    j += 1
                                }
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(mp_cmp(&should_be, &points_out->data[i]) == 0)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(mp_cmp(&should_be, &points_out->data[i]) == 0)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if mp_cmp(&mut should_be,
                                          &mut *(*points_out).data.offset(i as
                                                                              isize))
                                       == 0i32 {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/fft_test.c:%d: mu_check(mp_cmp(&should_be, &points_out->data[i]) == 0) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    122i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/fft_test.c:%d: mu_check(mp_cmp(&should_be, &points_out->data[i]) == 0) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    122i32);
                                        }
                                    }
                                }
                                i += 1
                            }
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
                        b"build/ptest/fft_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 126i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/fft_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 126i32);
            }
        }
    }
    MPArray_clear(roots);
    mp_clear(&mut tmp);
    mp_clear(&mut should_be);
    MPArray_clear(points_in);
    MPArray_clear(points_out);
    PrioConfig_clear(cfg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__fft_invert() {
    let mut rv: SECStatus = SECSuccess;
    let nPoints: libc::c_int = 8i32;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut points_in: MPArray = 0 as MPArray;
    let mut points_out: MPArray = 0 as MPArray;
    let mut points_out2: MPArray = 0 as MPArray;
    let mut roots: MPArray = 0 as MPArray;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(91))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(91))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(91i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/fft_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(91)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 147i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/fft_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(91)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 147i32);
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
                        b"\t\t* Checking mu_check((v = (points_in = MPArray_new(nPoints))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (points_in = MPArray_new(nPoints))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        points_in = MPArray_new(nPoints);
        v_0 = !points_in.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/fft_test.c:%d: mu_check((v = (points_in = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 148i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/fft_test.c:%d: mu_check((v = (points_in = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 148i32);
                }
            }
        }
        if !v_0 {
            rv = SECFailure
        } else {
            let mut v_1: bool = false;
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((v = (points_out = MPArray_new(nPoints))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (points_out = MPArray_new(nPoints))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            points_out = MPArray_new(nPoints);
            v_1 = !points_out.is_null();
            if v_1 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/fft_test.c:%d: mu_check((v = (points_out = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                149i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/fft_test.c:%d: mu_check((v = (points_out = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                149i32);
                    }
                }
            }
            if !v_1 {
                rv = SECFailure
            } else {
                let mut v_2: bool = false;
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((v = (points_out2 = MPArray_new(nPoints))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (points_out2 = MPArray_new(nPoints))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                points_out2 = MPArray_new(nPoints);
                v_2 = !points_out2.is_null();
                if v_2 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/fft_test.c:%d: mu_check((v = (points_out2 = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    150i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/fft_test.c:%d: mu_check((v = (points_out2 = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    150i32);
                        }
                    }
                }
                if !v_2 {
                    rv = SECFailure
                } else {
                    let mut v_3: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (roots = MPArray_new(nPoints))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (roots = MPArray_new(nPoints))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    roots = MPArray_new(nPoints);
                    v_3 = !roots.is_null();
                    if v_3 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/fft_test.c:%d: mu_check((v = (roots = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 151i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/fft_test.c:%d: mu_check((v = (roots = MPArray_new(nPoints)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 151i32);
                            }
                        }
                    }
                    if !v_3 {
                        rv = SECFailure
                    } else {
                        poly_fft_get_roots(roots, nPoints,
                                           cfg as const_PrioConfig,
                                           0 != 0i32);
                        mp_set(&mut *(*points_in).data.offset(0isize),
                               3i32 as mp_digit);
                        mp_set(&mut *(*points_in).data.offset(1isize),
                               8i32 as mp_digit);
                        mp_set(&mut *(*points_in).data.offset(2isize),
                               7i32 as mp_digit);
                        mp_set(&mut *(*points_in).data.offset(3isize),
                               9i32 as mp_digit);
                        mp_set(&mut *(*points_in).data.offset(4isize),
                               123i32 as mp_digit);
                        mp_set(&mut *(*points_in).data.offset(5isize),
                               123123987i32 as mp_digit);
                        mp_set(&mut *(*points_in).data.offset(6isize),
                               2i32 as mp_digit);
                        mp_set(&mut *(*points_in).data.offset(7isize),
                               0i32 as mp_digit);
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        if poly_fft(points_out, points_in as const_MPArray,
                                    cfg as const_PrioConfig, 0 != 0i32) as
                               libc::c_int == SECSuccess as libc::c_int {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/fft_test.c:%d: mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 163i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/fft_test.c:%d: mu_check(poly_fft(points_out, points_in, cfg, 0) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 163i32);
                                }
                            }
                        }
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check(poly_fft(points_out2, points_out, cfg, 1) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check(poly_fft(points_out2, points_out, cfg, 1) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        if poly_fft(points_out2, points_out as const_MPArray,
                                    cfg as const_PrioConfig, 0 != 1i32) as
                               libc::c_int == SECSuccess as libc::c_int {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/fft_test.c:%d: mu_check(poly_fft(points_out2, points_out, cfg, 1) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 164i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/fft_test.c:%d: mu_check(poly_fft(points_out2, points_out, cfg, 1) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 164i32);
                                }
                            }
                        }
                        let mut i: libc::c_int = 0i32;
                        while i < nPoints {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp(&points_out2->data[i], &points_in->data[i]) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp(&points_out2->data[i], &points_in->data[i]) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp(&mut *(*points_out2).data.offset(i as
                                                                           isize),
                                      &mut *(*points_in).data.offset(i as
                                                                         isize))
                                   == 0i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/fft_test.c:%d: mu_check(mp_cmp(&points_out2->data[i], &points_in->data[i]) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                167i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/fft_test.c:%d: mu_check(mp_cmp(&points_out2->data[i], &points_in->data[i]) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                167i32);
                                    }
                                }
                            }
                            i += 1
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
                        b"build/ptest/fft_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 171i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/fft_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 171i32);
            }
        }
    }
    MPArray_clear(roots);
    MPArray_clear(points_in);
    MPArray_clear(points_out);
    MPArray_clear(points_out2);
    PrioConfig_clear(cfg);
}