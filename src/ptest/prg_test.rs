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
    /*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
    pub type prg;
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
    #[no_mangle]
    fn mp_addmod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                 c: *mut mp_int) -> mp_err;
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
    #[no_mangle]
    fn PrioConfig_clear(cfg: PrioConfig);
    /*
 * Create a PrioConfig object with no encryption keys.  This routine is
 * useful for testing, but PrioClient_encode() will always fail when used with
 * this config.
 */
    #[no_mangle]
    fn PrioConfig_newTest(nFields: libc::c_int) -> PrioConfig;
    /*
 * Generate a new PRG seed using the NSS global randomness source.
 * Use this routine to initialize the secret that the two Prio servers
 * share.
 */
    #[no_mangle]
    fn PrioPRGSeed_randomize(seed: *mut PrioPRGSeed) -> SECStatus;
    /*
 * Initialize an array of `mp_int`s of the given length.
 */
    #[no_mangle]
    fn MPArray_new(len: libc::c_int) -> MPArray;
    #[no_mangle]
    fn MPArray_clear(arr: MPArray);
    /*
 * Initialize or destroy a pseudo-random generator.
 */
    #[no_mangle]
    fn PRG_new(key: *const libc::c_uchar) -> PRG;
    #[no_mangle]
    fn PRG_clear(prg: PRG);
    /*
 * Produce the next bytes of output from the PRG.
 */
    #[no_mangle]
    fn PRG_get_bytes(prg: PRG, bytes: *mut libc::c_uchar, len: size_t)
     -> SECStatus;
    /*
 * Use the PRG output to sample a big integer x in the range
 *    0 <= x < max.
 */
    #[no_mangle]
    fn PRG_get_int(prg: PRG, out: *mut mp_int, max: *const mp_int)
     -> SECStatus;
    /*
 * Use the PRG output to sample a big integer x in the range
 *    lower <= x < max.
 */
    #[no_mangle]
    fn PRG_get_int_range(prg: PRG, out: *mut mp_int, lower: *const mp_int,
                         max: *const mp_int) -> SECStatus;
    /*
 * Set each item in the array to a pseudorandom value in the range
 * [0, mod), where the values are generated using the PRG.
 */
    #[no_mangle]
    fn PRG_get_array(prg: PRG, arr: MPArray, mod_0: *const mp_int)
     -> SECStatus;
    /*
 * Secret shares the array in `src` into `arrA` using randomness
 * provided by `prgB`. The arrays `src` and `arrA` must be the same
 * length.
 */
    #[no_mangle]
    fn PRG_share_array(prgB: PRG, arrA: MPArray, src: const_MPArray,
                       cfg: const_PrioConfig) -> SECStatus;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
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
    pub u: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
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
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Seed for a pseudo-random generator (PRG). */
pub type PrioPRGSeed = [libc::c_uchar; 16];
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
pub type PRG = *mut prg;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn mu_test__prg_simple() {
    let mut rv: SECStatus = SECSuccess;
    let mut key: PrioPRGSeed = [0; 16];
    let mut prg: PRG = 0 as PRG;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = PrioPRGSeed_randomize(&mut key);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 23i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 23i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        let mut v: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        prg = PRG_new(key.as_mut_ptr() as *const libc::c_uchar);
        v = !prg.is_null();
        if v {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 24i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 24i32);
                }
            }
        }
        if !v { rv = SECFailure }
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
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 27i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 27i32);
            }
        }
    }
    PRG_clear(prg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__prg_repeat() {
    let mut all_zero: bool = false;
    let mut rv: SECStatus = SECSuccess;
    let buflen: libc::c_int = 10000i32;
    let mut buf1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut key: PrioPRGSeed = [0; 16];
    let mut prg1: PRG = 0 as PRG;
    let mut prg2: PRG = 0 as PRG;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = PrioPRGSeed_randomize(&mut key);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 43i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 43i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        let mut v: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (prg1 = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (prg1 = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        prg1 = PRG_new(key.as_mut_ptr() as *const libc::c_uchar);
        v = !prg1.is_null();
        if v {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg1 = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 44i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg1 = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 44i32);
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
                            b"\t\t* Checking mu_check((v = (prg2 = PRG_new(key))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (prg2 = PRG_new(key))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            prg2 = PRG_new(key.as_mut_ptr() as *const libc::c_uchar);
            v_0 = !prg2.is_null();
            if v_0 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (prg2 = PRG_new(key)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                45i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (prg2 = PRG_new(key)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                45i32);
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
                                b"\t\t* Checking mu_check((v = (buf1 = calloc(buflen, sizeof(unsigned char)))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (buf1 = calloc(buflen, sizeof(unsigned char)))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                buf1 =
                    calloc(buflen as libc::c_ulong,
                           ::std::mem::size_of::<libc::c_uchar>() as
                               libc::c_ulong) as *mut libc::c_uchar;
                v_1 = !buf1.is_null();
                if v_1 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (buf1 = calloc(buflen, sizeof(unsigned char))))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    46i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (buf1 = calloc(buflen, sizeof(unsigned char))))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    46i32);
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
                                    b"\t\t* Checking mu_check((v = (buf2 = calloc(buflen, sizeof(unsigned char)))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (buf2 = calloc(buflen, sizeof(unsigned char)))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    buf2 =
                        calloc(buflen as libc::c_ulong,
                               ::std::mem::size_of::<libc::c_uchar>() as
                                   libc::c_ulong) as *mut libc::c_uchar;
                    v_2 = !buf2.is_null();
                    if v_2 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (buf2 = calloc(buflen, sizeof(unsigned char))))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 47i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (buf2 = calloc(buflen, sizeof(unsigned char))))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 47i32);
                            }
                        }
                    }
                    if !v_2 {
                        rv = SECFailure
                    } else {
                        *buf1.offset(3isize) = 'a' as i32 as libc::c_uchar;
                        *buf2.offset(3isize) = 'b' as i32 as libc::c_uchar;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((rv = (PRG_get_bytes(prg1, buf1, buflen))) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((rv = (PRG_get_bytes(prg1, buf1, buflen))) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        rv = PRG_get_bytes(prg1, buf1, buflen as size_t);
                        if rv as libc::c_int == SECSuccess as libc::c_int {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_bytes(prg1, buf1, buflen))) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 52i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_bytes(prg1, buf1, buflen))) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 52i32);
                                }
                            }
                        }
                        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((rv = (PRG_get_bytes(prg2, buf2, buflen))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((rv = (PRG_get_bytes(prg2, buf2, buflen))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            rv = PRG_get_bytes(prg2, buf2, buflen as size_t);
                            if rv as libc::c_int == SECSuccess as libc::c_int
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
                                                b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_bytes(prg2, buf2, buflen))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                53i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_bytes(prg2, buf2, buflen))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                53i32);
                                    }
                                }
                            }
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                all_zero = 0 != 1i32;
                                let mut i: libc::c_int = 0i32;
                                while i < buflen {
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check(buf1[i] == buf2[i])...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(buf1[i] == buf2[i])...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if *buf1.offset(i as isize) as libc::c_int
                                           ==
                                           *buf2.offset(i as isize) as
                                               libc::c_int {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/prg_test.c:%d: mu_check(buf1[i] == buf2[i]) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        57i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/prg_test.c:%d: mu_check(buf1[i] == buf2[i]) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        57i32);
                                            }
                                        }
                                    }
                                    if 0 != *buf1.offset(i as isize) {
                                        all_zero = 0 != 0i32
                                    }
                                    i += 1
                                }
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(!all_zero)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(!all_zero)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if !all_zero {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/prg_test.c:%d: mu_check(!all_zero) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    61i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/prg_test.c:%d: mu_check(!all_zero) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    61i32);
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
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 64i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 64i32);
            }
        }
    }
    if !buf1.is_null() { free(buf1 as *mut libc::c_void); }
    if !buf2.is_null() { free(buf2 as *mut libc::c_void); }
    PRG_clear(prg1);
    PRG_clear(prg2);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__prg_repeat_int() {
    let mut rv: SECStatus = SECSuccess;
    let tries: libc::c_int = 10000i32;
    let mut max: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut out1: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut out2: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    max.dp = 0 as *mut mp_digit;
    out1.dp = 0 as *mut mp_digit;
    out2.dp = 0 as *mut mp_digit;
    let mut key: PrioPRGSeed = [0; 16];
    let mut prg1: PRG = 0 as PRG;
    let mut prg2: PRG = 0 as PRG;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = PrioPRGSeed_randomize(&mut key);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 89i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 89i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        let mut v: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (prg1 = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (prg1 = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        prg1 = PRG_new(key.as_mut_ptr() as *const libc::c_uchar);
        v = !prg1.is_null();
        if v {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg1 = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 90i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg1 = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 90i32);
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
                            b"\t\t* Checking mu_check((v = (prg2 = PRG_new(key))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (prg2 = PRG_new(key))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            prg2 = PRG_new(key.as_mut_ptr() as *const libc::c_uchar);
            v_0 = !prg2.is_null();
            if v_0 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (prg2 = PRG_new(key)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                91i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (prg2 = PRG_new(key)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                91i32);
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
                                b"\t\t* Checking mu_check((v = (mp_init(&max))) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (mp_init(&max))) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                v_1 = 0 != mp_init(&mut max);
                if v_1 as libc::c_int == 0i32 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    93i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    93i32);
                        }
                    }
                }
                if v_1 as libc::c_int != 0i32 {
                    rv = SECFailure
                } else {
                    let mut v_2: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (mp_init(&out1))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (mp_init(&out1))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    v_2 = 0 != mp_init(&mut out1);
                    if v_2 as libc::c_int == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out1))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 94i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out1))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 94i32);
                            }
                        }
                    }
                    if v_2 as libc::c_int != 0i32 {
                        rv = SECFailure
                    } else {
                        let mut v_3: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = (mp_init(&out2))) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (mp_init(&out2))) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        v_3 = 0 != mp_init(&mut out2);
                        if v_3 as libc::c_int == 0i32 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out2))) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 95i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out2))) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 95i32);
                                }
                            }
                        }
                        if v_3 as libc::c_int != 0i32 {
                            rv = SECFailure
                        } else {
                            let mut i: libc::c_int = 0i32;
                            while i < tries {
                                mp_set(&mut max, (i + 1i32) as mp_digit);
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check((rv = (PRG_get_int(prg1, &out1, &max))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check((rv = (PRG_get_int(prg1, &out1, &max))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                rv = PRG_get_int(prg1, &mut out1, &mut max);
                                if rv as libc::c_int ==
                                       SECSuccess as libc::c_int {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int(prg1, &out1, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    99i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int(prg1, &out1, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    99i32);
                                        }
                                    }
                                }
                                if rv as libc::c_int !=
                                       SECSuccess as libc::c_int {
                                    break ;
                                }
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check((rv = (PRG_get_int(prg2, &out2, &max))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check((rv = (PRG_get_int(prg2, &out2, &max))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                rv = PRG_get_int(prg2, &mut out2, &mut max);
                                if rv as libc::c_int ==
                                       SECSuccess as libc::c_int {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int(prg2, &out2, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    100i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int(prg2, &out2, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    100i32);
                                        }
                                    }
                                }
                                if rv as libc::c_int !=
                                       SECSuccess as libc::c_int {
                                    break ;
                                }
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(mp_cmp(&out1, &out2) == 0)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(mp_cmp(&out1, &out2) == 0)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if mp_cmp(&mut out1, &mut out2) == 0i32 {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/prg_test.c:%d: mu_check(mp_cmp(&out1, &out2) == 0) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    101i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/prg_test.c:%d: mu_check(mp_cmp(&out1, &out2) == 0) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    101i32);
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
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 105i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 105i32);
            }
        }
    }
    PRG_clear(prg1);
    PRG_clear(prg2);
    mp_clear(&mut max);
    mp_clear(&mut out1);
    mp_clear(&mut out2);
}
#[no_mangle]
pub unsafe extern "C" fn test_prg_once(mut limit: libc::c_int) {
    let mut rv: SECStatus = SECSuccess;
    let mut key: PrioPRGSeed = [0; 16];
    let mut max: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut out: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut prg: PRG = 0 as PRG;
    max.dp = 0 as *mut mp_digit;
    out.dp = 0 as *mut mp_digit;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = PrioPRGSeed_randomize(&mut key);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 125i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 125i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        let mut v: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        prg = PRG_new(key.as_mut_ptr() as *const libc::c_uchar);
        v = !prg.is_null();
        if v {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 126i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 126i32);
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
                                b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                128i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                128i32);
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
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    129i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    129i32);
                        }
                    }
                }
                if v_1 as libc::c_int != 0i32 {
                    rv = SECFailure
                } else {
                    mp_set(&mut max, limit as mp_digit);
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((rv = (PRG_get_int(prg, &out, &max))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((rv = (PRG_get_int(prg, &out, &max))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    rv = PRG_get_int(prg, &mut out, &mut max);
                    if rv as libc::c_int == SECSuccess as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int(prg, &out, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 133i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int(prg, &out, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 133i32);
                            }
                        }
                    }
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        if mp_cmp_d(&mut out, limit as mp_digit) == -1i32 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 134i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 134i32);
                                }
                            }
                        }
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        if mp_cmp_z(&mut out) > -1i32 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 135i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 135i32);
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
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 138i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 138i32);
            }
        }
    }
    mp_clear(&mut max);
    mp_clear(&mut out);
    PRG_clear(prg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_prg__multiple_of_8() {
    test_prg_once(256i32);
    test_prg_once(256i32 * 256i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_prg__near_multiple_of_8() {
    test_prg_once(256i32 + 1i32);
    test_prg_once(256i32 * 256i32 + 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_prg__odd() {
    test_prg_once(39i32);
    test_prg_once(123i32);
    test_prg_once(993123i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_prg__large() {
    test_prg_once(1231239933i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_prg__bit() {
    test_prg_once(1i32);
    let mut i: libc::c_int = 0i32;
    while i < 100i32 { test_prg_once(2i32); i += 1 };
}
#[no_mangle]
pub unsafe extern "C" fn test_prg_distribution(mut limit: libc::c_int) {
    let mut current_block: u64;
    let mut bins: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rv: SECStatus = SECSuccess;
    let mut key: PrioPRGSeed = [0; 16];
    let mut max: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut out: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut prg: PRG = 0 as PRG;
    max.dp = 0 as *mut mp_digit;
    out.dp = 0 as *mut mp_digit;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = PrioPRGSeed_randomize(&mut key);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 193i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 193i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        let mut v: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        prg = PRG_new(key.as_mut_ptr() as *const libc::c_uchar);
        v = !prg.is_null();
        if v {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 194i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 194i32);
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
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as *mut libc::c_int;
            v_0 = !bins.is_null();
            if v_0 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (bins = calloc(limit, sizeof(int))))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                195i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (bins = calloc(limit, sizeof(int))))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                195i32);
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
                                b"\t\t* Checking mu_check((v = (mp_init(&max))) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (mp_init(&max))) == 0)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                v_1 = 0 != mp_init(&mut max);
                if v_1 as libc::c_int == 0i32 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    197i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    197i32);
                        }
                    }
                }
                if v_1 as libc::c_int != 0i32 {
                    rv = SECFailure
                } else {
                    let mut v_2: bool = false;
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
                    v_2 = 0 != mp_init(&mut out);
                    if v_2 as libc::c_int == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 198i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 198i32);
                            }
                        }
                    }
                    if v_2 as libc::c_int != 0i32 {
                        rv = SECFailure
                    } else {
                        mp_set(&mut max, limit as mp_digit);
                        let mut i: libc::c_int = 0i32;
                        while i < limit {
                            *bins.offset(i as isize) = 0i32;
                            i += 1
                        }
                        let mut i_0: libc::c_int = 0i32;
                        loop  {
                            if !(i_0 < limit * limit) {
                                current_block = 964580764365397959;
                                break ;
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((rv = (PRG_get_int(prg, &out, &max))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((rv = (PRG_get_int(prg, &out, &max))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            rv = PRG_get_int(prg, &mut out, &mut max);
                            if rv as libc::c_int == SECSuccess as libc::c_int
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
                                                b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int(prg, &out, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                207i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int(prg, &out, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                207i32);
                                    }
                                }
                            }
                            if rv as libc::c_int != SECSuccess as libc::c_int
                               {
                                current_block = 10262860442921813504;
                                break ;
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp_d(&mut out, limit as mp_digit) == -1i32
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
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                208i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                208i32);
                                    }
                                }
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp_z(&mut out) > -1i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                209i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                209i32);
                                    }
                                }
                            }
                            let mut ival: [libc::c_uchar; 2] =
                                [0i32 as libc::c_uchar,
                                 0i32 as libc::c_uchar];
                            let mut v_3: bool = false;
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((v = (mp_to_fixlen_octets(&out, ival, 2))) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((v = (mp_to_fixlen_octets(&out, ival, 2))) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            v_3 =
                                0 !=
                                    mp_to_fixlen_octets(&mut out,
                                                        ival.as_mut_ptr(),
                                                        2i32 as mp_size);
                            if v_3 as libc::c_int == 0i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/prg_test.c:%d: mu_check((v = (mp_to_fixlen_octets(&out, ival, 2))) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                212i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check((v = (mp_to_fixlen_octets(&out, ival, 2))) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                212i32);
                                    }
                                }
                            }
                            if v_3 as libc::c_int != 0i32 {
                                rv = SECFailure;
                                current_block = 10262860442921813504;
                                break ;
                            } else {
                                if (ival[1usize] as libc::c_int +
                                        256i32 * ival[0usize] as libc::c_int)
                                       < limit {
                                    *bins.offset((ival[1usize] as libc::c_int
                                                      +
                                                      256i32 *
                                                          ival[0usize] as
                                                              libc::c_int) as
                                                     isize) += 1i32
                                } else {
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
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
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/prg_test.c:%d: mu_check(0) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    216i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/prg_test.c:%d: mu_check(0) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    216i32);
                                        }
                                    }
                                }
                                i_0 += 1
                            }
                        }
                        match current_block {
                            10262860442921813504 => { }
                            _ => {
                                let mut i_1: libc::c_int = 0i32;
                                while i_1 < limit {
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
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
                                    if *bins.offset(i_1 as isize) >
                                           limit / 2i32 {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/prg_test.c:%d: mu_check(bins[i] > limit / 2) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        221i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/prg_test.c:%d: mu_check(bins[i] > limit / 2) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        221i32);
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
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 225i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 225i32);
            }
        }
    }
    if !bins.is_null() { free(bins as *mut libc::c_void); }
    mp_clear(&mut max);
    mp_clear(&mut out);
    PRG_clear(prg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__prg_distribution123() {
    test_prg_distribution(123i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__prg_distribution257() {
    test_prg_distribution(257i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__prg_distribution259() {
    test_prg_distribution(259i32);
}
#[no_mangle]
pub unsafe extern "C" fn test_prg_distribution_large(mut max: *mut mp_int) {
    let mut current_block: u64;
    let limit: libc::c_int = 16i32;
    let mut bins: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rv: SECStatus = SECSuccess;
    let mut key: PrioPRGSeed = [0; 16];
    let mut out: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut prg: PRG = 0 as PRG;
    out.dp = 0 as *mut mp_digit;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = PrioPRGSeed_randomize(&mut key);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 263i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 263i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        let mut v: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        prg = PRG_new(key.as_mut_ptr() as *const libc::c_uchar);
        v = !prg.is_null();
        if v {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 264i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 264i32);
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
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as *mut libc::c_int;
            v_0 = !bins.is_null();
            if v_0 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (bins = calloc(limit, sizeof(int))))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                265i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (bins = calloc(limit, sizeof(int))))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                265i32);
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
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    267i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    267i32);
                        }
                    }
                }
                if v_1 as libc::c_int != 0i32 {
                    rv = SECFailure
                } else {
                    let mut i: libc::c_int = 0i32;
                    while i < limit {
                        *bins.offset(i as isize) = 0i32;
                        i += 1
                    }
                    let mut i_0: libc::c_int = 0i32;
                    loop  {
                        if !(i_0 < 100i32 * limit * limit) {
                            current_block = 14303212209785889906;
                            break ;
                        }
                        let mut v_2: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = (PRG_get_int(prg, &out, max))) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (PRG_get_int(prg, &out, max))) == 0)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        v_2 = 0 != PRG_get_int(prg, &mut out, max) as u64;
                        if v_2 as libc::c_int == 0i32 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/prg_test.c:%d: mu_check((v = (PRG_get_int(prg, &out, max))) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 274i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/prg_test.c:%d: mu_check((v = (PRG_get_int(prg, &out, max))) == 0) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 274i32);
                                }
                            }
                        }
                        if v_2 as libc::c_int != 0i32 {
                            rv = SECFailure;
                            current_block = 9340782638838653304;
                            break ;
                        } else {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp(&out, max) == -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp(&out, max) == -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp(&mut out, max) == -1i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp(&out, max) == -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                275i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp(&out, max) == -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                275i32);
                                    }
                                }
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp_z(&mut out) > -1i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                276i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                276i32);
                                    }
                                }
                            }
                            let mut res: libc::c_ulong = 0;
                            let mut v_3: bool = false;
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((v = (mp_mod_d(&out, limit, &res))) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((v = (mp_mod_d(&out, limit, &res))) == 0)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            v_3 =
                                0 !=
                                    mp_mod_d(&mut out, limit as mp_digit,
                                             &mut res);
                            if v_3 as libc::c_int == 0i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/prg_test.c:%d: mu_check((v = (mp_mod_d(&out, limit, &res))) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                279i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check((v = (mp_mod_d(&out, limit, &res))) == 0) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                279i32);
                                    }
                                }
                            }
                            if v_3 as libc::c_int != 0i32 {
                                rv = SECFailure;
                                current_block = 9340782638838653304;
                                break ;
                            } else {
                                *bins.offset(res as isize) += 1i32;
                                i_0 += 1
                            }
                        }
                    }
                    match current_block {
                        9340782638838653304 => { }
                        _ => {
                            let mut i_1: libc::c_int = 0i32;
                            while i_1 < limit {
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
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
                                                    b"build/ptest/prg_test.c:%d: mu_check(bins[i] > limit / 2) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    284i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/prg_test.c:%d: mu_check(bins[i] > limit / 2) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    284i32);
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
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 288i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 288i32);
            }
        }
    }
    if !bins.is_null() { free(bins as *mut libc::c_void); }
    mp_clear(&mut out);
    PRG_clear(prg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__prg_distribution_large() {
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
                        b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 301i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 301i32);
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
                            b"build/ptest/prg_test.c:%d: mu_check((v = (mp_read_radix(&max, bytes, 16))) == 0) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 304i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (mp_read_radix(&max, bytes, 16))) == 0) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 304i32);
                }
            }
        }
        if v_0 as libc::c_int != 0i32 {
            rv = SECFailure
        } else { test_prg_distribution_large(&mut max); }
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
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 308i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 308i32);
            }
        }
    }
    mp_clear(&mut max);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__prg_share_arr() {
    let mut rv: SECStatus = SECSuccess;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut arr: MPArray = 0 as MPArray;
    let mut arr_share: MPArray = 0 as MPArray;
    let mut prg: PRG = 0 as PRG;
    let mut seed: PrioPRGSeed = [0; 16];
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(72))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(72))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(72i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/prg_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(72)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 322i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(72)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 322i32);
            }
        }
    }
    if !v {
        rv = SECFailure
    } else {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        rv = PrioPRGSeed_randomize(&mut seed);
        if rv as libc::c_int == SECSuccess as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 323i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 323i32);
                }
            }
        }
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            let mut v_0: bool = false;
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
            v_0 = !arr.is_null();
            if v_0 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (arr = MPArray_new(10)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                324i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (arr = MPArray_new(10)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                324i32);
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
                                b"\t\t* Checking mu_check((v = (arr_share = MPArray_new(10))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (arr_share = MPArray_new(10))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                arr_share = MPArray_new(10i32);
                v_1 = !arr_share.is_null();
                if v_1 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (arr_share = MPArray_new(10)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    325i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (arr_share = MPArray_new(10)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    325i32);
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
                                    b"\t\t* Checking mu_check((v = (prg = PRG_new(seed))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (prg = PRG_new(seed))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    prg = PRG_new(seed.as_mut_ptr() as *const libc::c_uchar);
                    v_2 = !prg.is_null();
                    if v_2 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(seed)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 326i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(seed)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 326i32);
                            }
                        }
                    }
                    if !v_2 {
                        rv = SECFailure
                    } else {
                        let mut i: libc::c_int = 0i32;
                        while i < 10i32 {
                            mp_set(&mut *(*arr).data.offset(i as isize),
                                   i as mp_digit);
                            i += 1
                        }
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((rv = (PRG_share_array(prg, arr_share, arr, cfg))) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((rv = (PRG_share_array(prg, arr_share, arr, cfg))) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        rv =
                            PRG_share_array(prg, arr_share,
                                            arr as const_MPArray,
                                            cfg as const_PrioConfig);
                        if rv as libc::c_int == SECSuccess as libc::c_int {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_share_array(prg, arr_share, arr, cfg))) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 332i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_share_array(prg, arr_share, arr, cfg))) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 332i32);
                                }
                            }
                        }
                        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                            PRG_clear(prg);
                            let mut v_3: bool = false;
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((v = (prg = PRG_new(seed))))...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((v = (prg = PRG_new(seed))))...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            prg =
                                PRG_new(seed.as_mut_ptr() as
                                            *const libc::c_uchar);
                            v_3 = !prg.is_null();
                            if v_3 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(seed)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                336i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(seed)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                336i32);
                                    }
                                }
                            }
                            if !v_3 {
                                rv = SECFailure
                            } else {
                                // Read pseudorandom values into arr
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check((rv = (PRG_get_array(prg, arr, &cfg->modulus))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check((rv = (PRG_get_array(prg, arr, &cfg->modulus))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                rv =
                                    PRG_get_array(prg, arr,
                                                  &mut (*cfg).modulus);
                                if rv as libc::c_int ==
                                       SECSuccess as libc::c_int {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_array(prg, arr, &cfg->modulus))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    339i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_array(prg, arr, &cfg->modulus))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    339i32);
                                        }
                                    }
                                }
                                if !(rv as libc::c_int !=
                                         SECSuccess as libc::c_int) {
                                    let mut i_0: libc::c_int = 0i32;
                                    while i_0 < 10i32 {
                                        let mut v_4: bool = false;
                                        if mutest_verbose_level >=
                                               MU_CHECK as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"\t\t* Checking mu_check((v = (mp_addmod(&arr->data[i], &arr_share->data[i], &cfg->modulus, &arr->data[i]))) == 0)...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"\t\t* Checking mu_check((v = (mp_addmod(&arr->data[i], &arr_share->data[i], &cfg->modulus, &arr->data[i]))) == 0)...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            }
                                        }
                                        v_4 =
                                            0 !=
                                                mp_addmod(&mut *(*arr).data.offset(i_0
                                                                                       as
                                                                                       isize),
                                                          &mut *(*arr_share).data.offset(i_0
                                                                                             as
                                                                                             isize),
                                                          &mut (*cfg).modulus,
                                                          &mut *(*arr).data.offset(i_0
                                                                                       as
                                                                                       isize));
                                        if v_4 as libc::c_int == 0i32 {
                                            mutest_passed_checks += 1
                                        } else {
                                            mutest_failed_checks += 1;
                                            mutest_case_failed = 1i32;
                                            if mutest_verbose_level >=
                                                   MU_ERROR as libc::c_int {
                                                if mutest_verbose_level ==
                                                       MU_ERROR as libc::c_int
                                                   {
                                                    fprintf(__stderrp,
                                                            b"build/ptest/prg_test.c:%d: mu_check((v = (mp_addmod(&arr->data[i], &arr_share->data[i], &cfg->modulus, &arr->data[i]))) == 0) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            343i32);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"build/ptest/prg_test.c:%d: mu_check((v = (mp_addmod(&arr->data[i], &arr_share->data[i], &cfg->modulus, &arr->data[i]))) == 0) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            343i32);
                                                }
                                            }
                                        }
                                        if v_4 as libc::c_int != 0i32 {
                                            rv = SECFailure;
                                            break ;
                                        } else {
                                            if mutest_verbose_level >=
                                                   MU_CHECK as libc::c_int {
                                                if mutest_verbose_level ==
                                                       MU_ERROR as libc::c_int
                                                   {
                                                    fprintf(__stderrp,
                                                            b"\t\t* Checking mu_check(mp_cmp_d(&arr->data[i], i) == 0)...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"\t\t* Checking mu_check(mp_cmp_d(&arr->data[i], i) == 0)...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                }
                                            }
                                            if mp_cmp_d(&mut *(*arr).data.offset(i_0
                                                                                     as
                                                                                     isize),
                                                        i_0 as mp_digit) ==
                                                   0i32 {
                                                mutest_passed_checks += 1
                                            } else {
                                                mutest_failed_checks += 1;
                                                mutest_case_failed = 1i32;
                                                if mutest_verbose_level >=
                                                       MU_ERROR as libc::c_int
                                                   {
                                                    if mutest_verbose_level ==
                                                           MU_ERROR as
                                                               libc::c_int {
                                                        fprintf(__stderrp,
                                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&arr->data[i], i) == 0) failed, resuming test case\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                344i32);
                                                    } else {
                                                        fprintf(__stdoutp,
                                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&arr->data[i], i) == 0) failed, resuming test case\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                344i32);
                                                    }
                                                }
                                            }
                                            i_0 += 1
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
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 348i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 348i32);
            }
        }
    }
    PRG_clear(prg);
    MPArray_clear(arr);
    MPArray_clear(arr_share);
    PrioConfig_clear(cfg);
}
#[no_mangle]
pub unsafe extern "C" fn test_prg_range_once(mut bot: libc::c_int,
                                             mut limit: libc::c_int) {
    let mut rv: SECStatus = SECSuccess;
    let mut key: PrioPRGSeed = [0; 16];
    let mut lower: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut max: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut out: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut prg: PRG = 0 as PRG;
    lower.dp = 0 as *mut mp_digit;
    max.dp = 0 as *mut mp_digit;
    out.dp = 0 as *mut mp_digit;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = PrioPRGSeed_randomize(&mut key);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 370i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&key))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 370i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        let mut v: bool = false;
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (prg = PRG_new(key))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        prg = PRG_new(key.as_mut_ptr() as *const libc::c_uchar);
        v = !prg.is_null();
        if v {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 371i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/prg_test.c:%d: mu_check((v = (prg = PRG_new(key)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 371i32);
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
                                b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                373i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&max))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                373i32);
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
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    374i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    374i32);
                        }
                    }
                }
                if v_1 as libc::c_int != 0i32 {
                    rv = SECFailure
                } else {
                    let mut v_2: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (mp_init(&lower))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (mp_init(&lower))) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    v_2 = 0 != mp_init(&mut lower);
                    if v_2 as libc::c_int == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&lower))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 375i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/prg_test.c:%d: mu_check((v = (mp_init(&lower))) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 375i32);
                            }
                        }
                    }
                    if v_2 as libc::c_int != 0i32 {
                        rv = SECFailure
                    } else {
                        mp_set(&mut lower, bot as mp_digit);
                        mp_set(&mut max, limit as mp_digit);
                        let mut i: libc::c_int = 0i32;
                        while i < 100i32 {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((rv = (PRG_get_int_range(prg, &out, &lower, &max))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((rv = (PRG_get_int_range(prg, &out, &lower, &max))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            rv =
                                PRG_get_int_range(prg, &mut out, &mut lower,
                                                  &mut max);
                            if rv as libc::c_int == SECSuccess as libc::c_int
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
                                                b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int_range(prg, &out, &lower, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                381i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check((rv = (PRG_get_int_range(prg, &out, &lower, &max))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                381i32);
                                    }
                                }
                            }
                            if rv as libc::c_int != SECSuccess as libc::c_int
                               {
                                break ;
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp_d(&out, limit) == -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp_d(&mut out, limit as mp_digit) == -1i32
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
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                382i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&out, limit) == -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                382i32);
                                    }
                                }
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp_d(&out, bot) > -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp_d(&out, bot) > -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp_d(&mut out, bot as mp_digit) > -1i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&out, bot) > -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                383i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_d(&out, bot) > -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                383i32);
                                    }
                                }
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check(mp_cmp_z(&out) > -1)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if mp_cmp_z(&mut out) > -1i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                384i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/prg_test.c:%d: mu_check(mp_cmp_z(&out) > -1) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                384i32);
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
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 388i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/prg_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 388i32);
            }
        }
    }
    mp_clear(&mut lower);
    mp_clear(&mut max);
    mp_clear(&mut out);
    PRG_clear(prg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_prg_range__multiple_of_8() {
    test_prg_range_once(128i32, 256i32);
    test_prg_range_once(256i32, 256i32 * 256i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_prg_range__near_multiple_of_8() {
    test_prg_range_once(256i32, 256i32 + 1i32);
    test_prg_range_once(256i32 * 256i32, 256i32 * 256i32 + 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_prg_range__odd() {
    test_prg_range_once(23i32, 39i32);
    test_prg_range_once(7i32, 123i32);
    test_prg_range_once(99000i32, 993123i32);
}