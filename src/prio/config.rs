#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    pub type PK11SlotInfoStr;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    /*
 * Initialize or cleanup the global random number generator
 * state that NSS uses.
 */
    #[no_mangle]
    fn rand_init() -> SECStatus;
    #[no_mangle]
    fn rand_clear();
    #[no_mangle]
    fn mp_clear(mp: *mut mp_int);
    #[no_mangle]
    fn mp_invmod(a: *const mp_int, m: *const mp_int, c: *mut mp_int)
     -> mp_err;
    #[no_mangle]
    fn mp_set(mp: *mut mp_int, d: mp_digit);
    /* Memory management       */
    #[no_mangle]
    fn mp_init(mp: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_read_radix(mp: *mut mp_int, str: *const libc::c_char,
                     radix: libc::c_int) -> mp_err;
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
/* C99, Solaris */
/* MP_ULONG_LONG_MAX was defined to be ULLONG_MAX */
/* HPUX */
/* We only use unsigned long for mp_digit iff long is more than 32 bits. */
pub type mp_digit = libc::c_ulong;
pub type mp_size = libc::c_uint;
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
pub type PublicKey = *mut SECKEYPublicKey;
/*
 * Opaque types
 */
pub type PrioConfig = *mut prio_config;
pub type const_PrioConfig = *const prio_config;
pub type mp_err = libc::c_int;
/*
 * Initialize and clear random number generator state.
 * You must call Prio_init() before using the library.
 * To avoid memory leaks, call Prio_clear() afterwards.
 */
#[no_mangle]
pub unsafe extern "C" fn Prio_init() -> SECStatus { return rand_init(); }
#[no_mangle]
pub unsafe extern "C" fn Prio_clear() { rand_clear(); }
/*
 * PrioConfig holds the system parameters. The two relevant things determined
 * by the config object are:
 *    (1) the number of data fields we are collecting, and
 *    (2) the modulus we use for modular arithmetic.
 * The default configuration uses an 87-bit modulus.
 *
 * The value `nFields` must be in the range `0 < nFields <= max`, where `max`
 * is the value returned by the function `PrioConfig_maxDataFields()` below.
 *
 * The `batch_id` field specifies which "batch" of aggregate statistics we are
 * computing. For example, if the aggregate statistics are computed every 24
 * hours, the `batch_id` might be set to an encoding of the date. The clients
 * and servers must all use the same `batch_id` for each run of the protocol.
 * Each set of aggregate statistics should use a different `batch_id`.
 *
 * `PrioConfig_new` does not keep a pointer to the `batch_id` string that the
 * caller passes in, so you may free the `batch_id` string as soon as
 * `PrioConfig_new` returns.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioConfig_new(mut n_fields: libc::c_int,
                                        mut server_a: PublicKey,
                                        mut server_b: PublicKey,
                                        mut batch_id: *const libc::c_uchar,
                                        mut batch_id_len: libc::c_uint)
 -> PrioConfig {
    let mut rv: SECStatus = SECSuccess;
    let mut cfg: PrioConfig =
        malloc(::std::mem::size_of::<prio_config>() as libc::c_ulong) as
            PrioConfig;
    if cfg.is_null() { return 0 as PrioConfig }
    (*cfg).batch_id = 0 as *mut libc::c_uchar;
    (*cfg).batch_id_len = batch_id_len;
    (*cfg).server_a_pub = server_a;
    (*cfg).server_b_pub = server_b;
    (*cfg).num_data_fields = n_fields;
    (*cfg).n_roots = 1i32 << Generator2Order;
    (*cfg).modulus.dp = 0 as *mut mp_digit;
    (*cfg).inv2.dp = 0 as *mut mp_digit;
    (*cfg).generator.dp = 0 as *mut mp_digit;
    if !((*cfg).n_roots > 1i32) {
        rv = SECFailure
    } else if !((*cfg).num_data_fields <= PrioConfig_maxDataFields()) {
        rv = SECFailure
    } else {
        (*cfg).batch_id =
            malloc(batch_id_len as libc::c_ulong) as *mut libc::c_uchar;
        if (*cfg).batch_id.is_null() {
            rv = SECFailure
        } else {
            strncpy((*cfg).batch_id as *mut libc::c_char,
                    batch_id as *mut libc::c_char,
                    batch_id_len as libc::c_ulong);
            if mp_init(&mut (*cfg).modulus) != 0i32 {
                rv = SECFailure
            } else if mp_read_radix(&mut (*cfg).modulus, Modulus.as_ptr(),
                                    16i32) != 0i32 {
                rv = SECFailure
            } else if mp_init(&mut (*cfg).generator) != 0i32 {
                rv = SECFailure
            } else if mp_read_radix(&mut (*cfg).generator, Generator.as_ptr(),
                                    16i32) != 0i32 {
                rv = SECFailure
            } else if mp_init(&mut (*cfg).inv2) != 0i32 {
                rv = SECFailure
            } else {
                mp_set(&mut (*cfg).inv2, 2i32 as mp_digit);
                if mp_invmod(&mut (*cfg).inv2, &mut (*cfg).modulus,
                             &mut (*cfg).inv2) != 0i32 {
                    rv = SECFailure
                }
            }
        }
    }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PrioConfig_clear(cfg);
        return 0 as PrioConfig
    }
    return cfg;
}
#[no_mangle]
pub unsafe extern "C" fn PrioConfig_clear(mut cfg: PrioConfig) {
    if cfg.is_null() { return }
    if !(*cfg).batch_id.is_null() {
        free((*cfg).batch_id as *mut libc::c_void);
    }
    mp_clear(&mut (*cfg).modulus);
    mp_clear(&mut (*cfg).inv2);
    mp_clear(&mut (*cfg).generator);
    free(cfg as *mut libc::c_void);
}
// A generator g of a subgroup of Z*_p.
static mut Generator: [libc::c_char; 23] =
    [50, 53, 57, 55, 99, 49, 52, 102, 52, 56, 100, 53, 98, 54, 53, 101, 100,
     56, 100, 99, 99, 97, 0];
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
// A prime modulus p.
static mut Modulus: [libc::c_char; 23] =
    [56, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 56,
     48, 48, 48, 49, 0];
/*
 * Return the maximum number of data fields that the implementation supports.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioConfig_maxDataFields() -> libc::c_int {
    let n_roots: libc::c_int = 1i32 << Generator2Order;
    return (n_roots >> 1i32) - 1i32;
}
// The generator g generates a subgroup of
// order 2^Generator2Order in Z*_p.
static mut Generator2Order: libc::c_int = 19i32;
#[no_mangle]
pub unsafe extern "C" fn PrioConfig_numDataFields(mut cfg: const_PrioConfig)
 -> libc::c_int {
    return (*cfg).num_data_fields;
}
/*
 * Create a PrioConfig object with no encryption keys.  This routine is
 * useful for testing, but PrioClient_encode() will always fail when used with
 * this config.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioConfig_newTest(mut nFields: libc::c_int)
 -> PrioConfig {
    return PrioConfig_new(nFields, 0 as PublicKey, 0 as PublicKey,
                          b"testBatch\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_uchar, 9i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn PrioConfig_hPoints(mut cfg: const_PrioConfig)
 -> libc::c_int {
    let mul_gates: libc::c_int = (*cfg).num_data_fields + 1i32;
    let N: libc::c_int = next_power_of_two(mul_gates);
    return N;
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
// Minimum of two values
// Check a Prio error code and return failure if the call fails.
// Check an allocation that should not return NULL. If the allocation returns
// NULL, set the return value and jump to the cleanup label to free memory.
// Check a Prio library call that should return SECSuccess. If it doesn't,
// jump to the cleanup label.
// Check a boolean that should be true. If it not,
// jump to the cleanup label.
// Check an MPI library call and return failure if it fails.
// Check a msgpack object unpacked correctly
// Check an MPI library call. If it fails, set the return code and jump
// to the cleanup label.
unsafe extern "C" fn next_power_of_two(mut val: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = val;
    let mut out: libc::c_int = 0i32;
    while i > 0i32 { out += 1; i >>= 1i32 }
    let mut pow: libc::c_int = 1i32 << out;
    return if pow > 1i32 && pow / 2i32 == val { val } else { pow };
}