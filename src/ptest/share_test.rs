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
 * Expands or shrinks the MPArray to the desired size. If shrinking,
 * will clear the values on the end of array.
 */
    #[no_mangle]
    fn MPArray_resize(arr: MPArray, newlen: libc::c_int) -> SECStatus;
    /*
 * Copies array from src to dst. Arrays must have the same length.
 */
    #[no_mangle]
    fn MPArray_copy(dst: MPArray, src: const_MPArray) -> SECStatus;
    /*
 * Prio uses Beaver triples to implement one step of the
 * client data validation routine. A Beaver triple is just
 * a sharing of random values a, b, c such that
 *    a * b = c
 */
    #[no_mangle]
    fn BeaverTriple_new() -> BeaverTriple;
    #[no_mangle]
    fn BeaverTriple_clear(t: BeaverTriple);
    #[no_mangle]
    fn BeaverTriple_set_rand(cfg: const_PrioConfig, triple_a: BeaverTriple,
                             triple_b: BeaverTriple) -> SECStatus;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct beaver_triple {
    pub a: mp_int,
    pub b: mp_int,
    pub c: mp_int,
}
pub type BeaverTriple = *mut beaver_triple;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn mu_test_share() {
    let mut rv: SECStatus = SECSuccess;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut a: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut b: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut c: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut t1: BeaverTriple = 0 as BeaverTriple;
    let mut t2: BeaverTriple = 0 as BeaverTriple;
    a.dp = 0 as *mut mp_digit;
    b.dp = 0 as *mut mp_digit;
    c.dp = 0 as *mut mp_digit;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(93))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(93))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(93i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/share_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(93)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 32i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/share_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(93)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 32i32);
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
                        b"\t\t* Checking mu_check((v = (t1 = BeaverTriple_new())))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (t1 = BeaverTriple_new())))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        t1 = BeaverTriple_new();
        v_0 = !t1.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/share_test.c:%d: mu_check((v = (t1 = BeaverTriple_new()))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 33i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/share_test.c:%d: mu_check((v = (t1 = BeaverTriple_new()))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 33i32);
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
                            b"\t\t* Checking mu_check((v = (t2 = BeaverTriple_new())))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (t2 = BeaverTriple_new())))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            t2 = BeaverTriple_new();
            v_1 = !t2.is_null();
            if v_1 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/share_test.c:%d: mu_check((v = (t2 = BeaverTriple_new()))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                34i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/share_test.c:%d: mu_check((v = (t2 = BeaverTriple_new()))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                34i32);
                    }
                }
            }
            if !v_1 {
                rv = SECFailure
            } else {
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check(BeaverTriple_set_rand(cfg, t1, t2) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check(BeaverTriple_set_rand(cfg, t1, t2) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                if BeaverTriple_set_rand(cfg as const_PrioConfig, t1, t2) as
                       libc::c_int == SECSuccess as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/share_test.c:%d: mu_check(BeaverTriple_set_rand(cfg, t1, t2) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    36i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/share_test.c:%d: mu_check(BeaverTriple_set_rand(cfg, t1, t2) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    36i32);
                        }
                    }
                }
                let mut v_2: bool = false;
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((v = (mp_init(&a))) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (mp_init(&a))) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                v_2 = 0 != mp_init(&mut a);
                if v_2 as libc::c_int == 0i32 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/share_test.c:%d: mu_check((v = (mp_init(&a))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    38i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/share_test.c:%d: mu_check((v = (mp_init(&a))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    38i32);
                        }
                    }
                }
                if v_2 as libc::c_int != 0i32 {
                    rv = SECFailure
                } else {
                    let mut v_3: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (mp_init(&b))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (mp_init(&b))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    v_3 = 0 != mp_init(&mut b);
                    if v_3 as libc::c_int == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/share_test.c:%d: mu_check((v = (mp_init(&b))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 39i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/share_test.c:%d: mu_check((v = (mp_init(&b))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 39i32);
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
                                        b"\t\t* Checking mu_check((v = (mp_init(&c))) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (mp_init(&c))) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        v_4 = 0 != mp_init(&mut c);
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
                                            b"build/ptest/share_test.c:%d: mu_check((v = (mp_init(&c))) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 40i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/share_test.c:%d: mu_check((v = (mp_init(&c))) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 40i32);
                                }
                            }
                        }
                        if v_4 as libc::c_int != 0i32 {
                            rv = SECFailure
                        } else {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_addmod(&t1->a, &t2->a, &cfg->modulus, &a) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_addmod(&t1->a, &t2->a, &cfg->modulus, &a) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_addmod(&mut (*t1).a, &mut (*t2).a,
                                         &mut (*cfg).modulus, &mut a) == 0i32
                               {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_addmod(&t1->a, &t2->a, &cfg->modulus, &a) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                42i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_addmod(&t1->a, &t2->a, &cfg->modulus, &a) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                42i32);
                                    }
                                }
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_addmod(&t1->b, &t2->b, &cfg->modulus, &b) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_addmod(&t1->b, &t2->b, &cfg->modulus, &b) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_addmod(&mut (*t1).b, &mut (*t2).b,
                                         &mut (*cfg).modulus, &mut b) == 0i32
                               {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_addmod(&t1->b, &t2->b, &cfg->modulus, &b) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                43i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_addmod(&t1->b, &t2->b, &cfg->modulus, &b) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                43i32);
                                    }
                                }
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_addmod(&t1->c, &t2->c, &cfg->modulus, &c) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_addmod(&t1->c, &t2->c, &cfg->modulus, &c) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_addmod(&mut (*t1).c, &mut (*t2).c,
                                         &mut (*cfg).modulus, &mut c) == 0i32
                               {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_addmod(&t1->c, &t2->c, &cfg->modulus, &c) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                44i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_addmod(&t1->c, &t2->c, &cfg->modulus, &c) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                44i32);
                                    }
                                }
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_mulmod(&a, &b, &cfg->modulus, &a) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_mulmod(&a, &b, &cfg->modulus, &a) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_mulmod(&mut a, &mut b, &mut (*cfg).modulus,
                                         &mut a) == 0i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_mulmod(&a, &b, &cfg->modulus, &a) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                45i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_mulmod(&a, &b, &cfg->modulus, &a) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                45i32);
                                    }
                                }
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp(&a, &c) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp(&a, &c) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp(&mut a, &mut c) == 0i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_cmp(&a, &c) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                46i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_cmp(&a, &c) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                46i32);
                                    }
                                }
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
                        b"build/ptest/share_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 49i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/share_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 49i32);
            }
        }
    }
    mp_clear(&mut a);
    mp_clear(&mut b);
    mp_clear(&mut c);
    PrioConfig_clear(cfg);
    BeaverTriple_clear(t1);
    BeaverTriple_clear(t2);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_arr() {
    let mut rv: SECStatus = SECSuccess;
    let mut arr: MPArray = 0 as MPArray;
    let mut arr2: MPArray = 0 as MPArray;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (arr = MPArray_new(10))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (arr = MPArray_new(10))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    arr = MPArray_new(10i32);
    v = !arr.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/share_test.c:%d: mu_check((v = (arr = MPArray_new(10)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 65i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/share_test.c:%d: mu_check((v = (arr = MPArray_new(10)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 65i32);
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
                        b"\t\t* Checking mu_check((v = (arr2 = MPArray_new(7))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (arr2 = MPArray_new(7))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        arr2 = MPArray_new(7i32);
        v_0 = !arr2.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/share_test.c:%d: mu_check((v = (arr2 = MPArray_new(7)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 66i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/share_test.c:%d: mu_check((v = (arr2 = MPArray_new(7)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 66i32);
                }
            }
        }
        if !v_0 {
            rv = SECFailure
        } else {
            let mut i: libc::c_int = 0i32;
            while i < 10i32 {
                mp_set(&mut *(*arr).data.offset(i as isize), i as mp_digit);
                i += 1
            }
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((rv = (MPArray_resize(arr, 15))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((rv = (MPArray_resize(arr, 15))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            rv = MPArray_resize(arr, 15i32);
            if rv as libc::c_int == SECSuccess as libc::c_int {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/share_test.c:%d: mu_check((rv = (MPArray_resize(arr, 15))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                72i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/share_test.c:%d: mu_check((rv = (MPArray_resize(arr, 15))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                72i32);
                    }
                }
            }
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                let mut i_0: libc::c_int = 10i32;
                while i_0 < 15i32 {
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(mp_cmp_d(&arr->data[i], 0) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(mp_cmp_d(&arr->data[i], 0) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if mp_cmp_d(&mut *(*arr).data.offset(i_0 as isize),
                                0i32 as mp_digit) == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/share_test.c:%d: mu_check(mp_cmp_d(&arr->data[i], 0) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 74i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/share_test.c:%d: mu_check(mp_cmp_d(&arr->data[i], 0) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 74i32);
                            }
                        }
                    }
                    mp_set(&mut *(*arr).data.offset(i_0 as isize),
                           i_0 as mp_digit);
                    i_0 += 1
                }
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((rv = (MPArray_resize(arr, 7))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((rv = (MPArray_resize(arr, 7))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                rv = MPArray_resize(arr, 7i32);
                if rv as libc::c_int == SECSuccess as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/share_test.c:%d: mu_check((rv = (MPArray_resize(arr, 7))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    78i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/share_test.c:%d: mu_check((rv = (MPArray_resize(arr, 7))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    78i32);
                        }
                    }
                }
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    let mut i_1: libc::c_int = 10i32;
                    while i_1 < 7i32 {
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check(mp_cmp_d(&arr->data[i], i) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check(mp_cmp_d(&arr->data[i], i) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        if mp_cmp_d(&mut *(*arr).data.offset(i_1 as isize),
                                    i_1 as mp_digit) == 0i32 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/share_test.c:%d: mu_check(mp_cmp_d(&arr->data[i], i) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 80i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/share_test.c:%d: mu_check(mp_cmp_d(&arr->data[i], i) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 80i32);
                                }
                            }
                        }
                        i_1 += 1
                    }
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((rv = (MPArray_copy(arr2, arr))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((rv = (MPArray_copy(arr2, arr))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    rv = MPArray_copy(arr2, arr as const_MPArray);
                    if rv as libc::c_int == SECSuccess as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/share_test.c:%d: mu_check((rv = (MPArray_copy(arr2, arr))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 83i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/share_test.c:%d: mu_check((rv = (MPArray_copy(arr2, arr))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 83i32);
                            }
                        }
                    }
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        let mut i_2: libc::c_int = 10i32;
                        while i_2 < 7i32 {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp(&arr->data[i], &arr2->data[i]) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp(&arr->data[i], &arr2->data[i]) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp(&mut *(*arr).data.offset(i_2 as isize),
                                      &mut *(*arr2).data.offset(i_2 as isize))
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
                                                b"build/ptest/share_test.c:%d: mu_check(mp_cmp(&arr->data[i], &arr2->data[i]) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                85i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/share_test.c:%d: mu_check(mp_cmp(&arr->data[i], &arr2->data[i]) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                85i32);
                                    }
                                }
                            }
                            i_2 += 1
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
                        b"build/ptest/share_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 89i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/share_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 89i32);
            }
        }
    }
    MPArray_clear(arr);
    MPArray_clear(arr2);
}