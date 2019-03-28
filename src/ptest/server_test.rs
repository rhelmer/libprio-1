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
    fn mp_copy(from: *const mp_int, to: *mut mp_int) -> mp_err;
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
    fn mp_sub(a: *const mp_int, b: *const mp_int, c: *mut mp_int) -> mp_err;
    /* Modular arithmetic      */
    #[no_mangle]
    fn mp_mod(a: *const mp_int, m: *const mp_int, c: *mut mp_int) -> mp_err;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
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
    fn PrioConfig_new(nFields: libc::c_int, serverA: PublicKey,
                      serverB: PublicKey, batchId: *const libc::c_uchar,
                      batchIdLen: libc::c_uint) -> PrioConfig;
    #[no_mangle]
    fn PrioConfig_clear(cfg: PrioConfig);
    #[no_mangle]
    fn PrioConfig_numDataFields(cfg: const_PrioConfig) -> libc::c_int;
    /*
 * Create a PrioConfig object with no encryption keys.  This routine is
 * useful for testing, but PrioClient_encode() will always fail when used with
 * this config.
 */
    #[no_mangle]
    fn PrioConfig_newTest(nFields: libc::c_int) -> PrioConfig;
    /*
 * We use the PublicKey and PrivateKey objects for public-key encryption. Each
 * Prio server has an associated public key, and the clients use these keys to
 * encrypt messages to the servers.
 */
    #[no_mangle]
    fn Keypair_new(pvtkey: *mut PrivateKey, pubkey: *mut PublicKey)
     -> SECStatus;
    #[no_mangle]
    fn PublicKey_clear(pubkey: PublicKey);
    #[no_mangle]
    fn PrivateKey_clear(pvtkey: PrivateKey);
    /*
 *  PrioPacketClient_encode
 *
 * Takes as input a pointer to an array (`data_in`) of boolean values
 * whose length is equal to the number of data fields specified in
 * the config. It then encodes the data for servers A and B into a
 * string.
 *
 * NOTE: The caller must free() the strings `for_server_a` and
 * `for_server_b` to avoid memory leaks.
 */
    #[no_mangle]
    fn PrioClient_encode(cfg: const_PrioConfig, data_in: *const bool,
                         forServerA: *mut *mut libc::c_uchar,
                         aLen: *mut libc::c_uint,
                         forServerB: *mut *mut libc::c_uchar,
                         bLen: *mut libc::c_uint) -> SECStatus;
    /*
 * Generate a new PRG seed using the NSS global randomness source.
 * Use this routine to initialize the secret that the two Prio servers
 * share.
 */
    #[no_mangle]
    fn PrioPRGSeed_randomize(seed: *mut PrioPRGSeed) -> SECStatus;
    #[no_mangle]
    fn MPArray_clear(arr: MPArray);
    #[no_mangle]
    fn PRG_clear(prg: PRG);
    /*
 * Initialize or destroy a pseudo-random generator.
 */
    #[no_mangle]
    fn PRG_new(key: *const libc::c_uchar) -> PRG;
    /*
 * Initialize an array of `mp_int`s of the given length.
 */
    #[no_mangle]
    fn MPArray_new(len: libc::c_int) -> MPArray;
    #[no_mangle]
    fn PrioPacketClient_clear(p: PrioPacketClient);
    #[no_mangle]
    fn PrioPacketClient_new(cfg: const_PrioConfig, for_server: PrioServerId)
     -> PrioPacketClient;
    /*
 * Interpolate the polynomial through the points
 *    (x_1, y_1), ..., (x_N, y_N),
 * where x_i is an N-th root of unity and the y_i values are
 * specified by `poly_points`. Evaluate the resulting polynomial
 * at the point `eval_at`. Return the result as `value`.
 */
    #[no_mangle]
    fn poly_interp_evaluate(value: *mut mp_int, poly_points: const_MPArray,
                            eval_at: *const mp_int, cfg: const_PrioConfig)
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
    #[no_mangle]
    fn PrioPacketClient_decrypt(p: PrioPacketClient, cfg: const_PrioConfig,
                                server_priv: PrivateKey,
                                data_in: *const libc::c_uchar,
                                data_len: libc::c_uint) -> SECStatus;
    /* For each index i into the array, set:
 *    dst[i] = dst[i] + to_add[i]   (modulo mod)
 */
    #[no_mangle]
    fn MPArray_addmod(dst: MPArray, to_add: const_MPArray,
                      mod_0: *const mp_int) -> SECStatus;
    /*
 * Copies array from src to dst. Arrays must have the same length.
 */
    #[no_mangle]
    fn MPArray_copy(dst: MPArray, src: const_MPArray) -> SECStatus;
    /*
 * Expands or shrinks the MPArray to the desired size. If shrinking,
 * will clear the values on the end of array.
 */
    #[no_mangle]
    fn MPArray_resize(arr: MPArray, newlen: libc::c_int) -> SECStatus;
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
 * Evaluate the polynomial specified by the coefficients
 * at the point `eval_at` and return the result as `value`.
 */
    #[no_mangle]
    fn poly_eval(value: *mut mp_int, coeffs: const_MPArray,
                 eval_at: *const mp_int, cfg: const_PrioConfig) -> SECStatus;
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
/* PR_BYTES_PER_LONG == 8 */
/* !HAVE_LONG_LONG */
/* !HAVE_LONG_LONG */
/* ***********************************************************************
** TYPES:       PRUintn
**              PRIntn
** DESCRIPTION:
**  The PRIntn types are most appropriate for automatic variables. They are
**      guaranteed to be at least 16 bits, though various architectures may
**      define them to be wider (e.g., 32 or even 64 bits). These types are
**      never valid for fields of a structure.
************************************************************************/
pub type PRIntn = libc::c_int;
/* ***********************************************************************
** TYPES:       PRBool
** DESCRIPTION:
**  Use PRBool for variables and parameter types. Use PR_FALSE and PR_TRUE
**      for clarity of target type in assignments and actual arguments. Use
**      'if (bool)', 'while (!bool)', '(bool) ? x : y' etc., to test booleans
**      just as you would C int-valued conditions.
************************************************************************/
pub type PRBool = PRIntn;
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
/* bit flag definitions for staticflags */
/* bit 0 states \
                                        whether attributes are cached */
/* bit 1 is the value of CKA_PRIVATE */
/*
** A generic key structure
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECKEYPrivateKeyStr {
    pub arena: *mut PLArenaPool,
    pub keyType: KeyType,
    pub pkcs11Slot: *mut PK11SlotInfo,
    pub pkcs11ID: CK_OBJECT_HANDLE,
    pub pkcs11IsTemp: PRBool,
    pub wincx: *mut libc::c_void,
    pub staticflags: PRUint32,
}
pub type SECKEYPrivateKey = SECKEYPrivateKeyStr;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* Seed for a pseudo-random generator (PRG). */
pub type PrioPRGSeed = [libc::c_uchar; 16];
/* Length of a raw curve25519 public key, in bytes. */
/* Length of a hex-encoded curve25519 public key, in bytes. */
/*
 * Type for each of the two servers.
 */
pub type PrioServerId = libc::c_uint;
pub const PRIO_SERVER_B: PrioServerId = 1;
pub const PRIO_SERVER_A: PrioServerId = 0;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct prio_server {
    pub cfg: const_PrioConfig,
    pub idx: PrioServerId,
    pub priv_key: PrivateKey,
    pub data_shares: MPArray,
    pub prg: PRG,
}
pub type PRG = *mut prg;
pub type MPArray = *mut mparray;
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
pub type PrivateKey = *mut SECKEYPrivateKey;
pub type PrioServer = *mut prio_server;
pub type const_PrioServer = *const prio_server;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct prio_verifier {
    pub s: PrioServer,
    pub clientp: PrioPacketClient,
    pub data_sharesB: MPArray,
    pub h_pointsB: MPArray,
    pub share_fR: mp_int,
    pub share_gR: mp_int,
    pub share_hR: mp_int,
    pub share_out: mp_int,
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
/*
 * The PrioPacketClient object holds the encoded client data.
 * The client sends one packet to server A and one packet to
 * server B. The `for_server` parameter determines which server
 * the packet is for.
 */
pub type PrioPacketClient = *mut prio_packet_client;
/*
 * The data that a Prio client sends to each server.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct prio_packet_client {
    pub triple: BeaverTriple,
    pub f0_share: mp_int,
    pub g0_share: mp_int,
    pub h0_share: mp_int,
    pub for_server: PrioServerId,
    pub shares: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    pub A: server_a_data,
    pub B: server_b_data,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct server_b_data {
    pub seed: PrioPRGSeed,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct server_a_data {
    pub data_shares: MPArray,
    pub h_points: MPArray,
}
pub type BeaverTriple = *mut beaver_triple;
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
pub type PrioVerifier = *mut prio_verifier;
pub type const_PrioVerifier = *const prio_verifier;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct prio_packet_verify1 {
    pub share_d: mp_int,
    pub share_e: mp_int,
}
pub type PrioPacketVerify1 = *mut prio_packet_verify1;
pub type const_PrioPacketVerify1 = *const prio_packet_verify1;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct prio_packet_verify2 {
    pub share_out: mp_int,
}
pub type PrioPacketVerify2 = *mut prio_packet_verify2;
pub type const_PrioPacketVerify2 = *const prio_packet_verify2;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct prio_total_share {
    pub idx: PrioServerId,
    pub data_shares: MPArray,
}
pub type PrioTotalShare = *mut prio_total_share;
pub type const_PrioTotalShare = *const prio_total_share;
pub type const_PrioPacketClient = *const prio_packet_client;
pub type const_MPArray = *const mparray;
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
pub type unnamed_1 = libc::c_uint;
/* shows the current running check */
pub const MU_CHECK: unnamed_1 = 5;
/* shows test cases progress */
pub const MU_CASE: unnamed_1 = 4;
/* shows test suites progress */
pub const MU_SUITE: unnamed_1 = 3;
/* shows a summary */
pub const MU_SUMMARY: unnamed_1 = 2;
/* shows errors only */
pub const MU_ERROR: unnamed_1 = 1;
/* be completely quiet */
pub const MU_QUIET: unnamed_1 = 0;
/*
 * The PrioServer object holds the state of the Prio servers.
 * Pass in the _same_ secret PRGSeed when initializing the two servers.
 * The PRGSeed must remain secret to the two servers.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioServer_new(mut cfg: const_PrioConfig,
                                        mut server_idx: PrioServerId,
                                        mut server_priv: PrivateKey,
                                        mut seed: *const libc::c_uchar)
 -> PrioServer {
    let mut rv: SECStatus = SECSuccess;
    let mut s: PrioServer =
        malloc(::std::mem::size_of::<prio_server>() as libc::c_ulong) as
            PrioServer;
    if s.is_null() { return 0 as PrioServer }
    (*s).cfg = cfg;
    (*s).idx = server_idx;
    (*s).priv_key = server_priv;
    (*s).data_shares = 0 as MPArray;
    (*s).prg = 0 as PRG;
    (*s).data_shares = MPArray_new((*(*s).cfg).num_data_fields);
    if (*s).data_shares.is_null() {
        rv = SECFailure
    } else {
        (*s).prg = PRG_new(seed);
        if (*s).prg.is_null() { rv = SECFailure }
    }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PrioServer_clear(s);
        return 0 as PrioServer
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn PrioServer_clear(mut s: PrioServer) {
    if s.is_null() { return }
    PRG_clear((*s).prg);
    MPArray_clear((*s).data_shares);
    free(s as *mut libc::c_void);
}
/*
 * After receiving a client packet, each of the servers generate
 * a PrioVerifier object that they use to check whether the client's
 * encoded packet is well formed.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioVerifier_new(mut s: PrioServer) -> PrioVerifier {
    let mut N: libc::c_int = 0;
    let mut rv: SECStatus = SECSuccess;
    let mut v: PrioVerifier =
        malloc(::std::mem::size_of::<prio_verifier>() as libc::c_ulong) as
            PrioVerifier;
    if v.is_null() { return 0 as PrioVerifier }
    (*v).s = s;
    (*v).clientp = 0 as PrioPacketClient;
    (*v).data_sharesB = 0 as MPArray;
    (*v).h_pointsB = 0 as MPArray;
    (*v).share_fR.dp = 0 as *mut mp_digit;
    (*v).share_gR.dp = 0 as *mut mp_digit;
    (*v).share_hR.dp = 0 as *mut mp_digit;
    if mp_init(&mut (*v).share_fR) != 0i32 {
        rv = SECFailure
    } else if mp_init(&mut (*v).share_gR) != 0i32 {
        rv = SECFailure
    } else if mp_init(&mut (*v).share_hR) != 0i32 {
        rv = SECFailure
    } else {
        (*v).clientp = PrioPacketClient_new((*s).cfg, (*s).idx);
        if (*v).clientp.is_null() {
            rv = SECFailure
        } else {
            N = next_power_of_two((*(*s).cfg).num_data_fields + 1i32);
            if (*(*v).s).idx as libc::c_uint ==
                   PRIO_SERVER_B as libc::c_int as libc::c_uint {
                (*v).data_sharesB =
                    MPArray_new((*(*(*v).s).cfg).num_data_fields);
                if (*v).data_sharesB.is_null() {
                    rv = SECFailure
                } else {
                    (*v).h_pointsB = MPArray_new(N);
                    if (*v).h_pointsB.is_null() { rv = SECFailure }
                }
            }
        }
    }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PrioVerifier_clear(v);
        return 0 as PrioVerifier
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn PrioVerifier_clear(mut v: PrioVerifier) {
    if v.is_null() { return }
    PrioPacketClient_clear((*v).clientp);
    MPArray_clear((*v).data_sharesB);
    MPArray_clear((*v).h_pointsB);
    mp_clear(&mut (*v).share_fR);
    mp_clear(&mut (*v).share_gR);
    mp_clear(&mut (*v).share_hR);
    free(v as *mut libc::c_void);
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
/*
 * Read in encrypted data from the client, decrypt it, and prepare to check the
 * request for validity.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioVerifier_set_data(mut v: PrioVerifier,
                                               mut data: *mut libc::c_uchar,
                                               mut data_len: libc::c_uint)
 -> SECStatus {
    let mut p: PrioPacketClient = 0 as *mut prio_packet_client;
    let mut N: libc::c_int = 0;
    let mut current_block: u64;
    let mut rv: SECStatus = SECSuccess;
    let mut prgB: PRG = 0 as PRG;
    rv =
        PrioPacketClient_decrypt((*v).clientp, (*(*v).s).cfg,
                                 (*(*v).s).priv_key, data, data_len);
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        p = (*v).clientp;
        if (*p).for_server as libc::c_uint != (*(*v).s).idx as libc::c_uint {
            return SECFailure
        }
        N = next_power_of_two((*(*(*v).s).cfg).num_data_fields + 1i32);
        if (*(*v).s).idx as libc::c_uint ==
               PRIO_SERVER_A as libc::c_int as libc::c_uint {
            if (*(*p).shares.A.data_shares).len !=
                   (*(*(*v).s).cfg).num_data_fields {
                return SECFailure
            }
            if (*(*p).shares.A.h_points).len != N { return SECFailure }
        }
        if (*(*v).s).idx as libc::c_uint ==
               PRIO_SERVER_B as libc::c_int as libc::c_uint {
            prgB =
                PRG_new((*(*v).clientp).shares.B.seed.as_mut_ptr() as
                            *const libc::c_uchar);
            if prgB.is_null() {
                rv = SECFailure;
                current_block = 15979401701922058556;
            } else {
                rv =
                    PRG_get_array(prgB, (*v).data_sharesB,
                                  &(*(*(*v).s).cfg).modulus);
                if rv as libc::c_int != SECSuccess as libc::c_int {
                    current_block = 15979401701922058556;
                } else {
                    rv =
                        PRG_get_array(prgB, (*v).h_pointsB,
                                      &(*(*(*v).s).cfg).modulus);
                    if rv as libc::c_int != SECSuccess as libc::c_int {
                        current_block = 15979401701922058556;
                    } else { current_block = 16924917904204750491; }
                }
            }
        } else { current_block = 16924917904204750491; }
        match current_block {
            15979401701922058556 => { }
            _ => {
                // TODO: This can be done much faster by using the combined
  // interpolate-and-evaluate optimization described in the
  // Prio paper.
  //
  // Compute share of f(r), g(r), h(r)
                rv = compute_shares(v, p as const_PrioPacketClient);
                rv as libc::c_int != SECSuccess as libc::c_int;
            }
        }
    }
    PRG_clear(prgB);
    return rv;
}
/*
 * Build shares of the polynomials f, g, and h used in the Prio verification
 * routine and evalute these polynomials at a random point determined
 * by the shared secret. Store the evaluations in the verifier object.
 */
unsafe extern "C" fn compute_shares(mut v: PrioVerifier,
                                    mut p: const_PrioPacketClient)
 -> SECStatus {
    let mut j: libc::c_int = 0;
    let mut current_block: u64;
    let mut rv: SECStatus = SECSuccess;
    let n: libc::c_int = (*(*(*v).s).cfg).num_data_fields + 1i32;
    let N: libc::c_int = next_power_of_two(n);
    let mut eval_at: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut lower: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    eval_at.dp = 0 as *mut mp_digit;
    lower.dp = 0 as *mut mp_digit;
    let mut points_f: MPArray = 0 as MPArray;
    let mut points_g: MPArray = 0 as MPArray;
    let mut points_h: MPArray = 0 as MPArray;
    if mp_init(&mut eval_at) != 0i32 {
        rv = SECFailure
    } else if mp_init(&mut lower) != 0i32 {
        rv = SECFailure
    } else {
        points_f = MPArray_new(N);
        if points_f.is_null() {
            rv = SECFailure
        } else {
            points_g = MPArray_new(N);
            if points_g.is_null() {
                rv = SECFailure
            } else {
                points_h = MPArray_new(2i32 * N);
                if points_h.is_null() {
                    rv = SECFailure
                } else {
                    mp_set(&mut lower, (n + 1i32) as mp_digit);
                    rv =
                        PRG_get_int_range((*(*v).s).prg, &mut eval_at,
                                          &mut lower,
                                          &(*(*(*v).s).cfg).modulus);
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        // Reduce value into the field we're using. This
  // doesn't yield exactly a uniformly random point,
  // but for values this large, it will be close
  // enough.
                        if mp_mod(&mut eval_at, &(*(*(*v).s).cfg).modulus,
                                  &mut eval_at) != 0i32 {
                            rv = SECFailure
                        } else if mp_copy(&(*p).f0_share,
                                          &mut *(*points_f).data.offset(0isize))
                                      != 0i32 {
                            rv = SECFailure
                        } else if mp_copy(&(*p).g0_share,
                                          &mut *(*points_g).data.offset(0isize))
                                      != 0i32 {
                            rv = SECFailure
                        } else if mp_copy(&(*p).h0_share,
                                          &mut *(*points_h).data.offset(0isize))
                                      != 0i32 {
                            rv = SECFailure
                        } else {
                            let mut i: libc::c_int = 1i32;
                            loop  {
                                if !(i < n) {
                                    current_block = 8716029205547827362;
                                    break ;
                                }
                                // [f](i) = i-th data share
                                let mut data_i_minus_1: *const mp_int =
                                    get_data_share(v as const_PrioVerifier,
                                                   i - 1i32);
                                if mp_copy(data_i_minus_1,
                                           &mut *(*points_f).data.offset(i as
                                                                             isize))
                                       != 0i32 {
                                    rv = SECFailure;
                                    current_block = 18436038072878015465;
                                    break ;
                                } else if mp_copy(&mut *(*points_f).data.offset(i
                                                                                    as
                                                                                    isize),
                                                  &mut *(*points_g).data.offset(i
                                                                                    as
                                                                                    isize))
                                              != 0i32 {
                                    rv = SECFailure;
                                    current_block = 18436038072878015465;
                                    break ;
                                } else {
                                    if 0 == (*(*v).s).idx as u64 {
                                        if mp_sub_d(&mut *(*points_g).data.offset(i
                                                                                      as
                                                                                      isize),
                                                    1i32 as mp_digit,
                                                    &mut *(*points_g).data.offset(i
                                                                                      as
                                                                                      isize))
                                               != 0i32 {
                                            rv = SECFailure;
                                            current_block =
                                                18436038072878015465;
                                            break ;
                                        } else if mp_mod(&mut *(*points_g).data.offset(i
                                                                                           as
                                                                                           isize),
                                                         &(*(*(*v).s).cfg).modulus,
                                                         &mut *(*points_g).data.offset(i
                                                                                           as
                                                                                           isize))
                                                      != 0i32 {
                                            rv = SECFailure;
                                            current_block =
                                                18436038072878015465;
                                            break ;
                                        }
                                    }
                                    i += 1
                                }
                            }
                            match current_block {
                                18436038072878015465 => { }
                                _ => {
                                    j = 0i32;
                                    let mut i_0: libc::c_int = 1i32;
                                    loop  {
                                        if !(i_0 < 2i32 * N) {
                                            current_block =
                                                5023038348526654800;
                                            break ;
                                        }
                                        let fresh0 = j;
                                        j = j + 1;
                                        let mut h_point_j: *const mp_int =
                                            get_h_share(v as
                                                            const_PrioVerifier,
                                                        fresh0);
                                        if mp_copy(h_point_j,
                                                   &mut *(*points_h).data.offset(i_0
                                                                                     as
                                                                                     isize))
                                               != 0i32 {
                                            rv = SECFailure;
                                            current_block =
                                                18436038072878015465;
                                            break ;
                                        } else { i_0 += 2i32 }
                                    }
                                    match current_block {
                                        18436038072878015465 => { }
                                        _ => {
                                            rv =
                                                poly_interp_evaluate(&mut (*v).share_fR,
                                                                     points_f
                                                                         as
                                                                         const_MPArray,
                                                                     &mut eval_at,
                                                                     (*(*v).s).cfg);
                                            if !(rv as libc::c_int !=
                                                     SECSuccess as
                                                         libc::c_int) {
                                                rv =
                                                    poly_interp_evaluate(&mut (*v).share_gR,
                                                                         points_g
                                                                             as
                                                                             const_MPArray,
                                                                         &mut eval_at,
                                                                         (*(*v).s).cfg);
                                                if !(rv as libc::c_int !=
                                                         SECSuccess as
                                                             libc::c_int) {
                                                    rv =
                                                        poly_interp_evaluate(&mut (*v).share_hR,
                                                                             points_h
                                                                                 as
                                                                                 const_MPArray,
                                                                             &mut eval_at,
                                                                             (*(*v).s).cfg);
                                                    rv as libc::c_int !=
                                                        SECSuccess as
                                                            libc::c_int;
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
    }
    MPArray_clear(points_f);
    MPArray_clear(points_g);
    MPArray_clear(points_h);
    mp_clear(&mut eval_at);
    mp_clear(&mut lower);
    return rv;
}
unsafe extern "C" fn get_h_share(mut v: const_PrioVerifier,
                                 mut i: libc::c_int) -> *mut mp_int {
    match (*(*v).s).idx as libc::c_uint {
        0 => {
            return &mut *(*(*(*v).clientp).shares.A.h_points).data.offset(i as
                                                                              isize)
                       as *mut mp_int
        }
        1 => {
            return &mut *(*(*v).h_pointsB).data.offset(i as isize) as
                       *mut mp_int
        }
        _ => { }
    }
    return 0 as *mut mp_int;
}
unsafe extern "C" fn get_data_share(mut v: const_PrioVerifier,
                                    mut i: libc::c_int) -> *mut mp_int {
    match (*(*v).s).idx as libc::c_uint {
        0 => {
            return &mut *(*(*(*v).clientp).shares.A.data_shares).data.offset(i
                                                                                 as
                                                                                 isize)
                       as *mut mp_int
        }
        1 => {
            return &mut *(*(*v).data_sharesB).data.offset(i as isize) as
                       *mut mp_int
        }
        _ => { }
    }
    return 0 as *mut mp_int;
}
/*
 * Generate the first packet that servers need to exchange to verify the
 * client's submission. This should be sent over a TLS connection between the
 * servers.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify1_new() -> PrioPacketVerify1 {
    let mut rv: SECStatus = SECSuccess;
    let mut p: PrioPacketVerify1 =
        malloc(::std::mem::size_of::<prio_packet_verify1>() as libc::c_ulong)
            as PrioPacketVerify1;
    if p.is_null() { return 0 as PrioPacketVerify1 }
    (*p).share_d.dp = 0 as *mut mp_digit;
    (*p).share_e.dp = 0 as *mut mp_digit;
    if mp_init(&mut (*p).share_d) != 0i32 {
        rv = SECFailure
    } else if mp_init(&mut (*p).share_e) != 0i32 { rv = SECFailure }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PrioPacketVerify1_clear(p);
        return 0 as PrioPacketVerify1
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify1_clear(mut p: PrioPacketVerify1) {
    if p.is_null() { return }
    mp_clear(&mut (*p).share_d);
    mp_clear(&mut (*p).share_e);
    free(p as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify1_set_data(mut p1: PrioPacketVerify1,
                                                    mut v: const_PrioVerifier)
 -> SECStatus {
    // See the Prio paper for details on how this works.
  // Appendix C descrives the MPC protocol used here.
    let mut rv: SECStatus = SECSuccess;
    if mp_sub(&(*v).share_fR, &mut (*(*(*v).clientp).triple).a,
              &mut (*p1).share_d) != 0i32 {
        return SECFailure
    }
    if mp_mod(&mut (*p1).share_d, &(*(*(*v).s).cfg).modulus,
              &mut (*p1).share_d) != 0i32 {
        return SECFailure
    }
    if mp_sub(&(*v).share_gR, &mut (*(*(*v).clientp).triple).b,
              &mut (*p1).share_e) != 0i32 {
        return SECFailure
    }
    if mp_mod(&mut (*p1).share_e, &(*(*(*v).s).cfg).modulus,
              &mut (*p1).share_e) != 0i32 {
        return SECFailure
    }
    return rv;
}
/*
 * Generate the second packet that the servers need to exchange to verify the
 * client's submission. The routine takes as input the PrioPacketVerify1
 * packets from both server A and server B.
 *
 * This should be sent over a TLS connection between the servers.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify2_new() -> PrioPacketVerify2 {
    let mut rv: SECStatus = SECSuccess;
    let mut p: PrioPacketVerify2 =
        malloc(::std::mem::size_of::<prio_packet_verify2>() as libc::c_ulong)
            as PrioPacketVerify2;
    if p.is_null() { return 0 as PrioPacketVerify2 }
    (*p).share_out.dp = 0 as *mut mp_digit;
    if mp_init(&mut (*p).share_out) != 0i32 { rv = SECFailure }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PrioPacketVerify2_clear(p);
        return 0 as PrioPacketVerify2
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify2_clear(mut p: PrioPacketVerify2) {
    if p.is_null() { return }
    mp_clear(&mut (*p).share_out);
    free(p as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify2_set_data(mut p2: PrioPacketVerify2,
                                                    mut v: const_PrioVerifier,
                                                    mut p1A:
                                                        const_PrioPacketVerify1,
                                                    mut p1B:
                                                        const_PrioPacketVerify1)
 -> SECStatus {
    let mut mod_0: *const mp_int = 0 as *const mp_int;
    let mut rv: SECStatus = SECSuccess;
    let mut d: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut e: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    d.dp = 0 as *mut mp_digit;
    e.dp = 0 as *mut mp_digit;
    tmp.dp = 0 as *mut mp_digit;
    if mp_init(&mut d) != 0i32 {
        rv = SECFailure
    } else if mp_init(&mut e) != 0i32 {
        rv = SECFailure
    } else if mp_init(&mut tmp) != 0i32 {
        rv = SECFailure
    } else {
        mod_0 = &(*(*(*v).s).cfg).modulus;
        // Compute share of f(r)*g(r)
  //    [f(r)*g(r)] = [d*e/2] + d[b] + e[a] + [c]
        // Compute d
        if mp_addmod(&(*p1A).share_d, &(*p1B).share_d, mod_0, &mut d) != 0i32
           {
            rv = SECFailure
        } else if mp_addmod(&(*p1A).share_e, &(*p1B).share_e, mod_0, &mut e)
                      != 0i32 {
            rv = SECFailure
        } else if mp_mulmod(&mut d, &mut e, mod_0, &mut (*p2).share_out) !=
                      0i32 {
            rv = SECFailure
        } else if mp_mulmod(&mut (*p2).share_out, &(*(*(*v).s).cfg).inv2,
                            mod_0, &mut (*p2).share_out) != 0i32 {
            rv = SECFailure
        } else if mp_mulmod(&mut d, &mut (*(*(*v).clientp).triple).b, mod_0,
                            &mut tmp) != 0i32 {
            rv = SECFailure
        } else if mp_addmod(&mut (*p2).share_out, &mut tmp, mod_0,
                            &mut (*p2).share_out) != 0i32 {
            rv = SECFailure
        } else if mp_mulmod(&mut e, &mut (*(*(*v).clientp).triple).a, mod_0,
                            &mut tmp) != 0i32 {
            rv = SECFailure
        } else if mp_addmod(&mut (*p2).share_out, &mut tmp, mod_0,
                            &mut (*p2).share_out) != 0i32 {
            rv = SECFailure
        } else if mp_addmod(&mut (*p2).share_out,
                            &mut (*(*(*v).clientp).triple).c, mod_0,
                            &mut (*p2).share_out) != 0i32 {
            rv = SECFailure
        } else if mp_sub(&mut (*p2).share_out, &(*v).share_hR,
                         &mut (*p2).share_out) != 0i32 {
            rv = SECFailure
        } else if mp_mod(&mut (*p2).share_out, mod_0, &mut (*p2).share_out) !=
                      0i32 {
            rv = SECFailure
        }
    }
    mp_clear(&mut d);
    mp_clear(&mut e);
    mp_clear(&mut tmp);
    return rv;
}
/*
 * Use the PrioPacketVerify2s from both servers to check whether
 * the client's submission is well formed.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioVerifier_isValid(mut v: const_PrioVerifier,
                                              mut pA: const_PrioPacketVerify2,
                                              mut pB: const_PrioPacketVerify2)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let mut res: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    res.dp = 0 as *mut mp_digit;
    if mp_init(&mut res) != 0i32 {
        rv = SECFailure
    } else if mp_addmod(&(*pA).share_out, &(*pB).share_out,
                        &(*(*(*v).s).cfg).modulus, &mut res) != 0i32 {
        rv = SECFailure
    } else {
        rv =
            (if mp_cmp_d(&mut res, 0i32 as mp_digit) == 0i32 {
                 SECSuccess as libc::c_int
             } else { SECFailure as libc::c_int }) as SECStatus
    }
    mp_clear(&mut res);
    return rv as libc::c_int;
}
/*
 * Each of the two servers calls this routine to aggregate the data
 * submission from a client that is included in the PrioVerifier object.
 *
 * IMPORTANT: This routine does *not* check the validity of the client's
 * data packet. The servers must execute the verification checks
 * above before aggregating any client data.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioServer_aggregate(mut s: PrioServer,
                                              mut v: PrioVerifier)
 -> SECStatus {
    let mut arr: MPArray = 0 as MPArray;
    match (*s).idx as libc::c_uint {
        0 => { arr = (*(*v).clientp).shares.A.data_shares }
        1 => { arr = (*v).data_sharesB }
        _ => { return SECFailure }
    }
    return MPArray_addmod((*s).data_shares, arr as const_MPArray,
                          &(*(*s).cfg).modulus);
}
/*
 * After the servers have aggregated data packets from "enough" clients
 * (this determines the anonymity set size), each server runs this routine
 * to get a share of the aggregate statistics.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioTotalShare_new() -> PrioTotalShare {
    let mut t: PrioTotalShare =
        malloc(::std::mem::size_of::<prio_total_share>() as libc::c_ulong) as
            PrioTotalShare;
    if t.is_null() { return 0 as PrioTotalShare }
    (*t).data_shares = MPArray_new(0i32);
    if (*t).data_shares.is_null() {
        free(t as *mut libc::c_void);
        return 0 as PrioTotalShare
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn PrioTotalShare_clear(mut t: PrioTotalShare) {
    if t.is_null() { return }
    MPArray_clear((*t).data_shares);
    free(t as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn PrioTotalShare_set_data(mut t: PrioTotalShare,
                                                 mut s: const_PrioServer)
 -> SECStatus {
    (*t).idx = (*s).idx;
    let mut rv: SECStatus = SECSuccess;
    rv = MPArray_resize((*t).data_shares, (*(*s).data_shares).len);
    if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    rv = MPArray_copy((*t).data_shares, (*s).data_shares as const_MPArray);
    if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    return rv;
}
/*
 * Read the output data into an array of unsigned longs. You should
 * be sure that each data value can fit into a single `unsigned long`
 * and that the pointer `output` points to a buffer large enough to
 * store one long per data field.
 *
 * This function returns failure if some final data value is too
 * long to fit in an `unsigned long`.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioTotalShare_final(mut cfg: const_PrioConfig,
                                              mut output:
                                                  *mut libc::c_ulonglong,
                                              mut tA: const_PrioTotalShare,
                                              mut tB: const_PrioTotalShare)
 -> SECStatus {
    if (*(*tA).data_shares).len != (*cfg).num_data_fields {
        return SECFailure
    }
    if (*(*tA).data_shares).len != (*(*tB).data_shares).len {
        return SECFailure
    }
    if (*tA).idx as libc::c_uint !=
           PRIO_SERVER_A as libc::c_int as libc::c_uint ||
           (*tB).idx as libc::c_uint !=
               PRIO_SERVER_B as libc::c_int as libc::c_uint {
        return SECFailure
    }
    let mut rv: SECStatus = SECSuccess;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    tmp.dp = 0 as *mut mp_digit;
    if mp_init(&mut tmp) != 0i32 {
        rv = SECFailure
    } else {
        let mut i: libc::c_int = 0i32;
        while i < (*cfg).num_data_fields {
            if mp_addmod(&mut *(*(*tA).data_shares).data.offset(i as isize),
                         &mut *(*(*tB).data_shares).data.offset(i as isize),
                         &(*cfg).modulus, &mut tmp) != 0i32 {
                rv = SECFailure;
                break ;
            } else {
                if tmp.used > 1i32 as libc::c_uint {
                    if 0 == 0i32 { rv = SECFailure; break ; }
                }
                *output.offset(i as isize) =
                    *tmp.dp.offset(0isize) as libc::c_ulonglong;
                i += 1
            }
        }
    }
    mp_clear(&mut tmp);
    return rv;
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn mu_test__eval_poly() {
    let mut val: libc::c_int = 0;
    let mut rv: SECStatus = SECSuccess;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut coeffs: MPArray = 0 as MPArray;
    let mut eval_at: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut out: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    eval_at.dp = 0 as *mut mp_digit;
    out.dp = 0 as *mut mp_digit;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(54))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(54))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(54i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/server_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(54)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 29i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/server_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(54)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 29i32);
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
                        b"\t\t* Checking mu_check((v = (coeffs = MPArray_new(3))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (coeffs = MPArray_new(3))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        coeffs = MPArray_new(3i32);
        v_0 = !coeffs.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/server_test.c:%d: mu_check((v = (coeffs = MPArray_new(3)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 30i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/server_test.c:%d: mu_check((v = (coeffs = MPArray_new(3)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 30i32);
                }
            }
        }
        if !v_0 {
            rv = SECFailure
        } else {
            mp_set(&mut *(*coeffs).data.offset(0isize), 2i32 as mp_digit);
            mp_set(&mut *(*coeffs).data.offset(1isize), 8i32 as mp_digit);
            mp_set(&mut *(*coeffs).data.offset(2isize), 3i32 as mp_digit);
            let mut v_1: bool = false;
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((v = (mp_init(&eval_at))) == 0)...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (mp_init(&eval_at))) == 0)...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            v_1 = 0 != mp_init(&mut eval_at);
            if v_1 as libc::c_int == 0i32 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&eval_at))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                36i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&eval_at))) == 0) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                36i32);
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
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    37i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&out))) == 0) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    37i32);
                        }
                    }
                }
                if v_2 as libc::c_int != 0i32 {
                    rv = SECFailure
                } else {
                    mp_set(&mut eval_at, 7i32 as mp_digit);
                    val = 3i32 * 7i32 * 7i32 + 8i32 * 7i32 + 2i32;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(poly_eval(&out, coeffs, &eval_at, cfg) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(poly_eval(&out, coeffs, &eval_at, cfg) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if poly_eval(&mut out, coeffs as const_MPArray,
                                 &mut eval_at, cfg as const_PrioConfig) as
                           libc::c_int == SECSuccess as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/server_test.c:%d: mu_check(poly_eval(&out, coeffs, &eval_at, cfg) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 41i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/server_test.c:%d: mu_check(poly_eval(&out, coeffs, &eval_at, cfg) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 41i32);
                            }
                        }
                    }
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(mp_cmp_d(&out, val) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(mp_cmp_d(&out, val) == 0)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if mp_cmp_d(&mut out, val as mp_digit) == 0i32 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/server_test.c:%d: mu_check(mp_cmp_d(&out, val) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 42i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/server_test.c:%d: mu_check(mp_cmp_d(&out, val) == 0) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 42i32);
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
                        b"build/ptest/server_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 45i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/server_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 45i32);
            }
        }
    }
    mp_clear(&mut out);
    mp_clear(&mut eval_at);
    MPArray_clear(coeffs);
    PrioConfig_clear(cfg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__verify_new() {
    let mut ndata: libc::c_int = 0;
    let mut aLen: libc::c_uint = 0;
    let mut bLen: libc::c_uint = 0;
    let mut pA: PrioPacketClient = 0 as *mut prio_packet_client;
    let mut pB: PrioPacketClient = 0 as *mut prio_packet_client;
    let mut rv: SECStatus = SECSuccess;
    let mut pkA: PublicKey = 0 as PublicKey;
    let mut pkB: PublicKey = 0 as PublicKey;
    let mut skA: PrivateKey = 0 as PrivateKey;
    let mut skB: PrivateKey = 0 as PrivateKey;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut sA: PrioServer = 0 as PrioServer;
    let mut sB: PrioServer = 0 as PrioServer;
    let mut vA: PrioVerifier = 0 as PrioVerifier;
    let mut vB: PrioVerifier = 0 as PrioVerifier;
    let mut for_a: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut for_b: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut data_items: *mut bool = 0 as *mut bool;
    let mut fR: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut gR: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut hR: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    fR.dp = 0 as *mut mp_digit;
    gR.dp = 0 as *mut mp_digit;
    hR.dp = 0 as *mut mp_digit;
    let mut seed: PrioPRGSeed = [0; 16];
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
                        b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 75i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 75i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        rv = Keypair_new(&mut skA, &mut pkA);
        if rv as libc::c_int == SECSuccess as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/server_test.c:%d: mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 77i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/server_test.c:%d: mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 77i32);
                }
            }
        }
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            rv = Keypair_new(&mut skB, &mut pkB);
            if rv as libc::c_int == SECSuccess as libc::c_int {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/server_test.c:%d: mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                78i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/server_test.c:%d: mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                78i32);
                    }
                }
            }
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                let mut v: bool = false;
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((v = (cfg = PrioConfig_new(214, pkA, pkB, (unsigned char*)\"testbatch\", 9))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (cfg = PrioConfig_new(214, pkA, pkB, (unsigned char*)\"testbatch\", 9))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                cfg =
                    PrioConfig_new(214i32, pkA, pkB,
                                   b"testbatch\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_uchar,
                                   9i32 as libc::c_uint);
                v = !cfg.is_null();
                if v {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/server_test.c:%d: mu_check((v = (cfg = PrioConfig_new(214, pkA, pkB, (unsigned char*)\"testbatch\", 9)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    80i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/server_test.c:%d: mu_check((v = (cfg = PrioConfig_new(214, pkA, pkB, (unsigned char*)\"testbatch\", 9)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    80i32);
                        }
                    }
                }
                if !v {
                    rv = SECFailure
                } else {
                    ndata = PrioConfig_numDataFields(cfg as const_PrioConfig);
                    let mut v_0: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (data_items = calloc(ndata, sizeof(_Bool)))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (data_items = calloc(ndata, sizeof(_Bool)))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    data_items =
                        calloc(ndata as libc::c_ulong,
                               ::std::mem::size_of::<bool>() as libc::c_ulong)
                            as *mut bool;
                    v_0 = !data_items.is_null();
                    if v_0 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/server_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 83i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/server_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 83i32);
                            }
                        }
                    }
                    if !v_0 {
                        rv = SECFailure
                    } else {
                        let mut i: libc::c_int = 0i32;
                        while i < ndata {
                            *data_items.offset(i as isize) =
                                i % 3i32 == 1i32 || i % 5i32 == 3i32;
                            i += 1
                        }
                        let mut v_1: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        sA =
                            PrioServer_new(cfg as const_PrioConfig,
                                           PRIO_SERVER_A, skA,
                                           seed.as_mut_ptr() as
                                               *const libc::c_uchar);
                        v_1 = !sA.is_null();
                        if v_1 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/server_test.c:%d: mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 89i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/server_test.c:%d: mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 89i32);
                                }
                            }
                        }
                        if !v_1 {
                            rv = SECFailure
                        } else {
                            let mut v_2: bool = false;
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed))))...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed))))...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            sB =
                                PrioServer_new(cfg as const_PrioConfig,
                                               PRIO_SERVER_B, skB,
                                               seed.as_mut_ptr() as
                                                   *const libc::c_uchar);
                            v_2 = !sB.is_null();
                            if v_2 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/server_test.c:%d: mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                90i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/server_test.c:%d: mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                90i32);
                                    }
                                }
                            }
                            if !v_2 {
                                rv = SECFailure
                            } else {
                                aLen = 0;
                                bLen = 0;
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                rv =
                                    PrioClient_encode(cfg as const_PrioConfig,
                                                      data_items, &mut for_a,
                                                      &mut aLen, &mut for_b,
                                                      &mut bLen);
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
                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    93i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    93i32);
                                        }
                                    }
                                }
                                if !(rv as libc::c_int !=
                                         SECSuccess as libc::c_int) {
                                    let mut v_3: bool = false;
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check((v = (mp_init(&fR))) == 0)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check((v = (mp_init(&fR))) == 0)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    v_3 = 0 != mp_init(&mut fR);
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
                                                        b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&fR))) == 0) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        95i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&fR))) == 0) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        95i32);
                                            }
                                        }
                                    }
                                    if v_3 as libc::c_int != 0i32 {
                                        rv = SECFailure
                                    } else {
                                        let mut v_4: bool = false;
                                        if mutest_verbose_level >=
                                               MU_CHECK as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"\t\t* Checking mu_check((v = (mp_init(&gR))) == 0)...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"\t\t* Checking mu_check((v = (mp_init(&gR))) == 0)...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            }
                                        }
                                        v_4 = 0 != mp_init(&mut gR);
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
                                                            b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&gR))) == 0) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            96i32);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&gR))) == 0) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            96i32);
                                                }
                                            }
                                        }
                                        if v_4 as libc::c_int != 0i32 {
                                            rv = SECFailure
                                        } else {
                                            let mut v_5: bool = false;
                                            if mutest_verbose_level >=
                                                   MU_CHECK as libc::c_int {
                                                if mutest_verbose_level ==
                                                       MU_ERROR as libc::c_int
                                                   {
                                                    fprintf(__stderrp,
                                                            b"\t\t* Checking mu_check((v = (mp_init(&hR))) == 0)...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"\t\t* Checking mu_check((v = (mp_init(&hR))) == 0)...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                }
                                            }
                                            v_5 = 0 != mp_init(&mut hR);
                                            if v_5 as libc::c_int == 0i32 {
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
                                                                b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&hR))) == 0) failed, resuming test case\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                97i32);
                                                    } else {
                                                        fprintf(__stdoutp,
                                                                b"build/ptest/server_test.c:%d: mu_check((v = (mp_init(&hR))) == 0) failed, resuming test case\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                97i32);
                                                    }
                                                }
                                            }
                                            if v_5 as libc::c_int != 0i32 {
                                                rv = SECFailure
                                            } else {
                                                let mut v_6: bool = false;
                                                if mutest_verbose_level >=
                                                       MU_CHECK as libc::c_int
                                                   {
                                                    if mutest_verbose_level ==
                                                           MU_ERROR as
                                                               libc::c_int {
                                                        fprintf(__stderrp,
                                                                b"\t\t* Checking mu_check((v = (vA = PrioVerifier_new(sA))))...\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                    } else {
                                                        fprintf(__stdoutp,
                                                                b"\t\t* Checking mu_check((v = (vA = PrioVerifier_new(sA))))...\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                    }
                                                }
                                                vA = PrioVerifier_new(sA);
                                                v_6 = !vA.is_null();
                                                if v_6 {
                                                    mutest_passed_checks += 1
                                                } else {
                                                    mutest_failed_checks += 1;
                                                    mutest_case_failed = 1i32;
                                                    if mutest_verbose_level >=
                                                           MU_ERROR as
                                                               libc::c_int {
                                                        if mutest_verbose_level
                                                               ==
                                                               MU_ERROR as
                                                                   libc::c_int
                                                           {
                                                            fprintf(__stderrp,
                                                                    b"build/ptest/server_test.c:%d: mu_check((v = (vA = PrioVerifier_new(sA)))) failed, resuming test case\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    99i32);
                                                        } else {
                                                            fprintf(__stdoutp,
                                                                    b"build/ptest/server_test.c:%d: mu_check((v = (vA = PrioVerifier_new(sA)))) failed, resuming test case\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    99i32);
                                                        }
                                                    }
                                                }
                                                if !v_6 {
                                                    rv = SECFailure
                                                } else {
                                                    let mut v_7: bool = false;
                                                    if mutest_verbose_level >=
                                                           MU_CHECK as
                                                               libc::c_int {
                                                        if mutest_verbose_level
                                                               ==
                                                               MU_ERROR as
                                                                   libc::c_int
                                                           {
                                                            fprintf(__stderrp,
                                                                    b"\t\t* Checking mu_check((v = (vB = PrioVerifier_new(sB))))...\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                        } else {
                                                            fprintf(__stdoutp,
                                                                    b"\t\t* Checking mu_check((v = (vB = PrioVerifier_new(sB))))...\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                        }
                                                    }
                                                    vB = PrioVerifier_new(sB);
                                                    v_7 = !vB.is_null();
                                                    if v_7 {
                                                        mutest_passed_checks
                                                            += 1
                                                    } else {
                                                        mutest_failed_checks
                                                            += 1;
                                                        mutest_case_failed =
                                                            1i32;
                                                        if mutest_verbose_level
                                                               >=
                                                               MU_ERROR as
                                                                   libc::c_int
                                                           {
                                                            if mutest_verbose_level
                                                                   ==
                                                                   MU_ERROR as
                                                                       libc::c_int
                                                               {
                                                                fprintf(__stderrp,
                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (vB = PrioVerifier_new(sB)))) failed, resuming test case\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        100i32);
                                                            } else {
                                                                fprintf(__stdoutp,
                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (vB = PrioVerifier_new(sB)))) failed, resuming test case\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        100i32);
                                                            }
                                                        }
                                                    }
                                                    if !v_7 {
                                                        rv = SECFailure
                                                    } else {
                                                        if mutest_verbose_level
                                                               >=
                                                               MU_CHECK as
                                                                   libc::c_int
                                                           {
                                                            if mutest_verbose_level
                                                                   ==
                                                                   MU_ERROR as
                                                                       libc::c_int
                                                               {
                                                                fprintf(__stderrp,
                                                                        b"\t\t* Checking mu_check((rv = (PrioVerifier_set_data(vA, for_a, aLen))) == SECSuccess)...\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char);
                                                            } else {
                                                                fprintf(__stdoutp,
                                                                        b"\t\t* Checking mu_check((rv = (PrioVerifier_set_data(vA, for_a, aLen))) == SECSuccess)...\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char);
                                                            }
                                                        }
                                                        rv =
                                                            PrioVerifier_set_data(vA,
                                                                                  for_a,
                                                                                  aLen);
                                                        if rv as libc::c_int
                                                               ==
                                                               SECSuccess as
                                                                   libc::c_int
                                                           {
                                                            mutest_passed_checks
                                                                += 1
                                                        } else {
                                                            mutest_failed_checks
                                                                += 1;
                                                            mutest_case_failed
                                                                = 1i32;
                                                            if mutest_verbose_level
                                                                   >=
                                                                   MU_ERROR as
                                                                       libc::c_int
                                                               {
                                                                if mutest_verbose_level
                                                                       ==
                                                                       MU_ERROR
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    fprintf(__stderrp,
                                                                            b"build/ptest/server_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vA, for_a, aLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            101i32);
                                                                } else {
                                                                    fprintf(__stdoutp,
                                                                            b"build/ptest/server_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vA, for_a, aLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            101i32);
                                                                }
                                                            }
                                                        }
                                                        if !(rv as libc::c_int
                                                                 !=
                                                                 SECSuccess as
                                                                     libc::c_int)
                                                           {
                                                            if mutest_verbose_level
                                                                   >=
                                                                   MU_CHECK as
                                                                       libc::c_int
                                                               {
                                                                if mutest_verbose_level
                                                                       ==
                                                                       MU_ERROR
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    fprintf(__stderrp,
                                                                            b"\t\t* Checking mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                } else {
                                                                    fprintf(__stdoutp,
                                                                            b"\t\t* Checking mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                }
                                                            }
                                                            rv =
                                                                PrioVerifier_set_data(vB,
                                                                                      for_b,
                                                                                      bLen);
                                                            if rv as
                                                                   libc::c_int
                                                                   ==
                                                                   SECSuccess
                                                                       as
                                                                       libc::c_int
                                                               {
                                                                mutest_passed_checks
                                                                    += 1
                                                            } else {
                                                                mutest_failed_checks
                                                                    += 1;
                                                                mutest_case_failed
                                                                    = 1i32;
                                                                if mutest_verbose_level
                                                                       >=
                                                                       MU_ERROR
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    if mutest_verbose_level
                                                                           ==
                                                                           MU_ERROR
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        fprintf(__stderrp,
                                                                                b"build/ptest/server_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                102i32);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"build/ptest/server_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                102i32);
                                                                    }
                                                                }
                                                            }
                                                            if !(rv as
                                                                     libc::c_int
                                                                     !=
                                                                     SECSuccess
                                                                         as
                                                                         libc::c_int)
                                                               {
                                                                pA =
                                                                    (*vA).clientp;
                                                                pB =
                                                                    (*vB).clientp;
                                                                let mut v_8:
                                                                        bool =
                                                                    false;
                                                                if mutest_verbose_level
                                                                       >=
                                                                       MU_CHECK
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    if mutest_verbose_level
                                                                           ==
                                                                           MU_ERROR
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        fprintf(__stderrp,
                                                                                b"\t\t* Checking mu_check((v = (mp_addmod(&pA->f0_share, &pB->f0_share, &cfg->modulus, &fR))) == 0)...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"\t\t* Checking mu_check((v = (mp_addmod(&pA->f0_share, &pB->f0_share, &cfg->modulus, &fR))) == 0)...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    }
                                                                }
                                                                v_8 =
                                                                    0 !=
                                                                        mp_addmod(&mut (*pA).f0_share,
                                                                                  &mut (*pB).f0_share,
                                                                                  &mut (*cfg).modulus,
                                                                                  &mut fR);
                                                                if v_8 as
                                                                       libc::c_int
                                                                       == 0i32
                                                                   {
                                                                    mutest_passed_checks
                                                                        += 1
                                                                } else {
                                                                    mutest_failed_checks
                                                                        += 1;
                                                                    mutest_case_failed
                                                                        =
                                                                        1i32;
                                                                    if mutest_verbose_level
                                                                           >=
                                                                           MU_ERROR
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        if mutest_verbose_level
                                                                               ==
                                                                               MU_ERROR
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            fprintf(__stderrp,
                                                                                    b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&pA->f0_share, &pB->f0_share, &cfg->modulus, &fR))) == 0) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    106i32);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&pA->f0_share, &pB->f0_share, &cfg->modulus, &fR))) == 0) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    106i32);
                                                                        }
                                                                    }
                                                                }
                                                                if v_8 as
                                                                       libc::c_int
                                                                       != 0i32
                                                                   {
                                                                    rv =
                                                                        SECFailure
                                                                } else {
                                                                    let mut v_9:
                                                                            bool =
                                                                        false;
                                                                    if mutest_verbose_level
                                                                           >=
                                                                           MU_CHECK
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        if mutest_verbose_level
                                                                               ==
                                                                               MU_ERROR
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            fprintf(__stderrp,
                                                                                    b"\t\t* Checking mu_check((v = (mp_addmod(&pA->g0_share, &pB->g0_share, &cfg->modulus, &gR))) == 0)...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"\t\t* Checking mu_check((v = (mp_addmod(&pA->g0_share, &pB->g0_share, &cfg->modulus, &gR))) == 0)...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        }
                                                                    }
                                                                    v_9 =
                                                                        0 !=
                                                                            mp_addmod(&mut (*pA).g0_share,
                                                                                      &mut (*pB).g0_share,
                                                                                      &mut (*cfg).modulus,
                                                                                      &mut gR);
                                                                    if v_9 as
                                                                           libc::c_int
                                                                           ==
                                                                           0i32
                                                                       {
                                                                        mutest_passed_checks
                                                                            +=
                                                                            1
                                                                    } else {
                                                                        mutest_failed_checks
                                                                            +=
                                                                            1;
                                                                        mutest_case_failed
                                                                            =
                                                                            1i32;
                                                                        if mutest_verbose_level
                                                                               >=
                                                                               MU_ERROR
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            if mutest_verbose_level
                                                                                   ==
                                                                                   MU_ERROR
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                fprintf(__stderrp,
                                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&pA->g0_share, &pB->g0_share, &cfg->modulus, &gR))) == 0) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        107i32);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&pA->g0_share, &pB->g0_share, &cfg->modulus, &gR))) == 0) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        107i32);
                                                                            }
                                                                        }
                                                                    }
                                                                    if v_9 as
                                                                           libc::c_int
                                                                           !=
                                                                           0i32
                                                                       {
                                                                        rv =
                                                                            SECFailure
                                                                    } else {
                                                                        let mut v_10:
                                                                                bool =
                                                                            false;
                                                                        if mutest_verbose_level
                                                                               >=
                                                                               MU_CHECK
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            if mutest_verbose_level
                                                                                   ==
                                                                                   MU_ERROR
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                fprintf(__stderrp,
                                                                                        b"\t\t* Checking mu_check((v = (mp_addmod(&pA->h0_share, &pB->h0_share, &cfg->modulus, &hR))) == 0)...\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"\t\t* Checking mu_check((v = (mp_addmod(&pA->h0_share, &pB->h0_share, &cfg->modulus, &hR))) == 0)...\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char);
                                                                            }
                                                                        }
                                                                        v_10 =
                                                                            0
                                                                                !=
                                                                                mp_addmod(&mut (*pA).h0_share,
                                                                                          &mut (*pB).h0_share,
                                                                                          &mut (*cfg).modulus,
                                                                                          &mut hR);
                                                                        if v_10
                                                                               as
                                                                               libc::c_int
                                                                               ==
                                                                               0i32
                                                                           {
                                                                            mutest_passed_checks
                                                                                +=
                                                                                1
                                                                        } else {
                                                                            mutest_failed_checks
                                                                                +=
                                                                                1;
                                                                            mutest_case_failed
                                                                                =
                                                                                1i32;
                                                                            if mutest_verbose_level
                                                                                   >=
                                                                                   MU_ERROR
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                if mutest_verbose_level
                                                                                       ==
                                                                                       MU_ERROR
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    fprintf(__stderrp,
                                                                                            b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&pA->h0_share, &pB->h0_share, &cfg->modulus, &hR))) == 0) failed, resuming test case\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char,
                                                                                            108i32);
                                                                                } else {
                                                                                    fprintf(__stdoutp,
                                                                                            b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&pA->h0_share, &pB->h0_share, &cfg->modulus, &hR))) == 0) failed, resuming test case\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char,
                                                                                            108i32);
                                                                                }
                                                                            }
                                                                        }
                                                                        if v_10
                                                                               as
                                                                               libc::c_int
                                                                               !=
                                                                               0i32
                                                                           {
                                                                            rv
                                                                                =
                                                                                SECFailure
                                                                        } else {
                                                                            let mut v_11:
                                                                                    bool =
                                                                                false;
                                                                            if mutest_verbose_level
                                                                                   >=
                                                                                   MU_CHECK
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                if mutest_verbose_level
                                                                                       ==
                                                                                       MU_ERROR
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    fprintf(__stderrp,
                                                                                            b"\t\t* Checking mu_check((v = (mp_mulmod(&fR, &gR, &cfg->modulus, &fR))) == 0)...\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char);
                                                                                } else {
                                                                                    fprintf(__stdoutp,
                                                                                            b"\t\t* Checking mu_check((v = (mp_mulmod(&fR, &gR, &cfg->modulus, &fR))) == 0)...\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char);
                                                                                }
                                                                            }
                                                                            v_11
                                                                                =
                                                                                0
                                                                                    !=
                                                                                    mp_mulmod(&mut fR,
                                                                                              &mut gR,
                                                                                              &mut (*cfg).modulus,
                                                                                              &mut fR);
                                                                            if v_11
                                                                                   as
                                                                                   libc::c_int
                                                                                   ==
                                                                                   0i32
                                                                               {
                                                                                mutest_passed_checks
                                                                                    +=
                                                                                    1
                                                                            } else {
                                                                                mutest_failed_checks
                                                                                    +=
                                                                                    1;
                                                                                mutest_case_failed
                                                                                    =
                                                                                    1i32;
                                                                                if mutest_verbose_level
                                                                                       >=
                                                                                       MU_ERROR
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    if mutest_verbose_level
                                                                                           ==
                                                                                           MU_ERROR
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        fprintf(__stderrp,
                                                                                                b"build/ptest/server_test.c:%d: mu_check((v = (mp_mulmod(&fR, &gR, &cfg->modulus, &fR))) == 0) failed, resuming test case\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                110i32);
                                                                                    } else {
                                                                                        fprintf(__stdoutp,
                                                                                                b"build/ptest/server_test.c:%d: mu_check((v = (mp_mulmod(&fR, &gR, &cfg->modulus, &fR))) == 0) failed, resuming test case\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                110i32);
                                                                                    }
                                                                                }
                                                                            }
                                                                            if v_11
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   0i32
                                                                               {
                                                                                rv
                                                                                    =
                                                                                    SECFailure
                                                                            } else {
                                                                                if mutest_verbose_level
                                                                                       >=
                                                                                       MU_CHECK
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    if mutest_verbose_level
                                                                                           ==
                                                                                           MU_ERROR
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        fprintf(__stderrp,
                                                                                                b"\t\t* Checking mu_check(mp_cmp(&fR, &hR) == 0)...\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char);
                                                                                    } else {
                                                                                        fprintf(__stdoutp,
                                                                                                b"\t\t* Checking mu_check(mp_cmp(&fR, &hR) == 0)...\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char);
                                                                                    }
                                                                                }
                                                                                if mp_cmp(&mut fR,
                                                                                          &mut hR)
                                                                                       ==
                                                                                       0i32
                                                                                   {
                                                                                    mutest_passed_checks
                                                                                        +=
                                                                                        1
                                                                                } else {
                                                                                    mutest_failed_checks
                                                                                        +=
                                                                                        1;
                                                                                    mutest_case_failed
                                                                                        =
                                                                                        1i32;
                                                                                    if mutest_verbose_level
                                                                                           >=
                                                                                           MU_ERROR
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        if mutest_verbose_level
                                                                                               ==
                                                                                               MU_ERROR
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            fprintf(__stderrp,
                                                                                                    b"build/ptest/server_test.c:%d: mu_check(mp_cmp(&fR, &hR) == 0) failed, resuming test case\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char,
                                                                                                    111i32);
                                                                                        } else {
                                                                                            fprintf(__stdoutp,
                                                                                                    b"build/ptest/server_test.c:%d: mu_check(mp_cmp(&fR, &hR) == 0) failed, resuming test case\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char,
                                                                                                    111i32);
                                                                                        }
                                                                                    }
                                                                                }
                                                                                let mut v_12:
                                                                                        bool =
                                                                                    false;
                                                                                if mutest_verbose_level
                                                                                       >=
                                                                                       MU_CHECK
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    if mutest_verbose_level
                                                                                           ==
                                                                                           MU_ERROR
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        fprintf(__stderrp,
                                                                                                b"\t\t* Checking mu_check((v = (mp_addmod(&vA->share_fR, &vB->share_fR, &cfg->modulus, &fR))) == 0)...\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char);
                                                                                    } else {
                                                                                        fprintf(__stdoutp,
                                                                                                b"\t\t* Checking mu_check((v = (mp_addmod(&vA->share_fR, &vB->share_fR, &cfg->modulus, &fR))) == 0)...\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char);
                                                                                    }
                                                                                }
                                                                                v_12
                                                                                    =
                                                                                    0
                                                                                        !=
                                                                                        mp_addmod(&mut (*vA).share_fR,
                                                                                                  &mut (*vB).share_fR,
                                                                                                  &mut (*cfg).modulus,
                                                                                                  &mut fR);
                                                                                if v_12
                                                                                       as
                                                                                       libc::c_int
                                                                                       ==
                                                                                       0i32
                                                                                   {
                                                                                    mutest_passed_checks
                                                                                        +=
                                                                                        1
                                                                                } else {
                                                                                    mutest_failed_checks
                                                                                        +=
                                                                                        1;
                                                                                    mutest_case_failed
                                                                                        =
                                                                                        1i32;
                                                                                    if mutest_verbose_level
                                                                                           >=
                                                                                           MU_ERROR
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        if mutest_verbose_level
                                                                                               ==
                                                                                               MU_ERROR
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            fprintf(__stderrp,
                                                                                                    b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&vA->share_fR, &vB->share_fR, &cfg->modulus, &fR))) == 0) failed, resuming test case\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char,
                                                                                                    113i32);
                                                                                        } else {
                                                                                            fprintf(__stdoutp,
                                                                                                    b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&vA->share_fR, &vB->share_fR, &cfg->modulus, &fR))) == 0) failed, resuming test case\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char,
                                                                                                    113i32);
                                                                                        }
                                                                                    }
                                                                                }
                                                                                if v_12
                                                                                       as
                                                                                       libc::c_int
                                                                                       !=
                                                                                       0i32
                                                                                   {
                                                                                    rv
                                                                                        =
                                                                                        SECFailure
                                                                                } else {
                                                                                    let mut v_13:
                                                                                            bool =
                                                                                        false;
                                                                                    if mutest_verbose_level
                                                                                           >=
                                                                                           MU_CHECK
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        if mutest_verbose_level
                                                                                               ==
                                                                                               MU_ERROR
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            fprintf(__stderrp,
                                                                                                    b"\t\t* Checking mu_check((v = (mp_addmod(&vA->share_gR, &vB->share_gR, &cfg->modulus, &gR))) == 0)...\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char);
                                                                                        } else {
                                                                                            fprintf(__stdoutp,
                                                                                                    b"\t\t* Checking mu_check((v = (mp_addmod(&vA->share_gR, &vB->share_gR, &cfg->modulus, &gR))) == 0)...\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char);
                                                                                        }
                                                                                    }
                                                                                    v_13
                                                                                        =
                                                                                        0
                                                                                            !=
                                                                                            mp_addmod(&mut (*vA).share_gR,
                                                                                                      &mut (*vB).share_gR,
                                                                                                      &mut (*cfg).modulus,
                                                                                                      &mut gR);
                                                                                    if v_13
                                                                                           as
                                                                                           libc::c_int
                                                                                           ==
                                                                                           0i32
                                                                                       {
                                                                                        mutest_passed_checks
                                                                                            +=
                                                                                            1
                                                                                    } else {
                                                                                        mutest_failed_checks
                                                                                            +=
                                                                                            1;
                                                                                        mutest_case_failed
                                                                                            =
                                                                                            1i32;
                                                                                        if mutest_verbose_level
                                                                                               >=
                                                                                               MU_ERROR
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            if mutest_verbose_level
                                                                                                   ==
                                                                                                   MU_ERROR
                                                                                                       as
                                                                                                       libc::c_int
                                                                                               {
                                                                                                fprintf(__stderrp,
                                                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&vA->share_gR, &vB->share_gR, &cfg->modulus, &gR))) == 0) failed, resuming test case\n\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char,
                                                                                                        114i32);
                                                                                            } else {
                                                                                                fprintf(__stdoutp,
                                                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&vA->share_gR, &vB->share_gR, &cfg->modulus, &gR))) == 0) failed, resuming test case\n\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char,
                                                                                                        114i32);
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    if v_13
                                                                                           as
                                                                                           libc::c_int
                                                                                           !=
                                                                                           0i32
                                                                                       {
                                                                                        rv
                                                                                            =
                                                                                            SECFailure
                                                                                    } else {
                                                                                        let mut v_14:
                                                                                                bool =
                                                                                            false;
                                                                                        if mutest_verbose_level
                                                                                               >=
                                                                                               MU_CHECK
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            if mutest_verbose_level
                                                                                                   ==
                                                                                                   MU_ERROR
                                                                                                       as
                                                                                                       libc::c_int
                                                                                               {
                                                                                                fprintf(__stderrp,
                                                                                                        b"\t\t* Checking mu_check((v = (mp_addmod(&vA->share_hR, &vB->share_hR, &cfg->modulus, &hR))) == 0)...\n\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char);
                                                                                            } else {
                                                                                                fprintf(__stdoutp,
                                                                                                        b"\t\t* Checking mu_check((v = (mp_addmod(&vA->share_hR, &vB->share_hR, &cfg->modulus, &hR))) == 0)...\n\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char);
                                                                                            }
                                                                                        }
                                                                                        v_14
                                                                                            =
                                                                                            0
                                                                                                !=
                                                                                                mp_addmod(&mut (*vA).share_hR,
                                                                                                          &mut (*vB).share_hR,
                                                                                                          &mut (*cfg).modulus,
                                                                                                          &mut hR);
                                                                                        if v_14
                                                                                               as
                                                                                               libc::c_int
                                                                                               ==
                                                                                               0i32
                                                                                           {
                                                                                            mutest_passed_checks
                                                                                                +=
                                                                                                1
                                                                                        } else {
                                                                                            mutest_failed_checks
                                                                                                +=
                                                                                                1;
                                                                                            mutest_case_failed
                                                                                                =
                                                                                                1i32;
                                                                                            if mutest_verbose_level
                                                                                                   >=
                                                                                                   MU_ERROR
                                                                                                       as
                                                                                                       libc::c_int
                                                                                               {
                                                                                                if mutest_verbose_level
                                                                                                       ==
                                                                                                       MU_ERROR
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                   {
                                                                                                    fprintf(__stderrp,
                                                                                                            b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&vA->share_hR, &vB->share_hR, &cfg->modulus, &hR))) == 0) failed, resuming test case\n\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char,
                                                                                                            115i32);
                                                                                                } else {
                                                                                                    fprintf(__stdoutp,
                                                                                                            b"build/ptest/server_test.c:%d: mu_check((v = (mp_addmod(&vA->share_hR, &vB->share_hR, &cfg->modulus, &hR))) == 0) failed, resuming test case\n\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char,
                                                                                                            115i32);
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        if v_14
                                                                                               as
                                                                                               libc::c_int
                                                                                               !=
                                                                                               0i32
                                                                                           {
                                                                                            rv
                                                                                                =
                                                                                                SECFailure
                                                                                        } else {
                                                                                            let mut v_15:
                                                                                                    bool =
                                                                                                false;
                                                                                            if mutest_verbose_level
                                                                                                   >=
                                                                                                   MU_CHECK
                                                                                                       as
                                                                                                       libc::c_int
                                                                                               {
                                                                                                if mutest_verbose_level
                                                                                                       ==
                                                                                                       MU_ERROR
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                   {
                                                                                                    fprintf(__stderrp,
                                                                                                            b"\t\t* Checking mu_check((v = (mp_mulmod(&fR, &gR, &cfg->modulus, &fR))) == 0)...\n\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char);
                                                                                                } else {
                                                                                                    fprintf(__stdoutp,
                                                                                                            b"\t\t* Checking mu_check((v = (mp_mulmod(&fR, &gR, &cfg->modulus, &fR))) == 0)...\n\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char);
                                                                                                }
                                                                                            }
                                                                                            v_15
                                                                                                =
                                                                                                0
                                                                                                    !=
                                                                                                    mp_mulmod(&mut fR,
                                                                                                              &mut gR,
                                                                                                              &mut (*cfg).modulus,
                                                                                                              &mut fR);
                                                                                            if v_15
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   ==
                                                                                                   0i32
                                                                                               {
                                                                                                mutest_passed_checks
                                                                                                    +=
                                                                                                    1
                                                                                            } else {
                                                                                                mutest_failed_checks
                                                                                                    +=
                                                                                                    1;
                                                                                                mutest_case_failed
                                                                                                    =
                                                                                                    1i32;
                                                                                                if mutest_verbose_level
                                                                                                       >=
                                                                                                       MU_ERROR
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                   {
                                                                                                    if mutest_verbose_level
                                                                                                           ==
                                                                                                           MU_ERROR
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                       {
                                                                                                        fprintf(__stderrp,
                                                                                                                b"build/ptest/server_test.c:%d: mu_check((v = (mp_mulmod(&fR, &gR, &cfg->modulus, &fR))) == 0) failed, resuming test case\n\x00"
                                                                                                                    as
                                                                                                                    *const u8
                                                                                                                    as
                                                                                                                    *const libc::c_char,
                                                                                                                117i32);
                                                                                                    } else {
                                                                                                        fprintf(__stdoutp,
                                                                                                                b"build/ptest/server_test.c:%d: mu_check((v = (mp_mulmod(&fR, &gR, &cfg->modulus, &fR))) == 0) failed, resuming test case\n\x00"
                                                                                                                    as
                                                                                                                    *const u8
                                                                                                                    as
                                                                                                                    *const libc::c_char,
                                                                                                                117i32);
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            if v_15
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   !=
                                                                                                   0i32
                                                                                               {
                                                                                                rv
                                                                                                    =
                                                                                                    SECFailure
                                                                                            } else {
                                                                                                if mutest_verbose_level
                                                                                                       >=
                                                                                                       MU_CHECK
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                   {
                                                                                                    if mutest_verbose_level
                                                                                                           ==
                                                                                                           MU_ERROR
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                       {
                                                                                                        fprintf(__stderrp,
                                                                                                                b"\t\t* Checking mu_check(mp_cmp(&fR, &hR) == 0)...\n\x00"
                                                                                                                    as
                                                                                                                    *const u8
                                                                                                                    as
                                                                                                                    *const libc::c_char);
                                                                                                    } else {
                                                                                                        fprintf(__stdoutp,
                                                                                                                b"\t\t* Checking mu_check(mp_cmp(&fR, &hR) == 0)...\n\x00"
                                                                                                                    as
                                                                                                                    *const u8
                                                                                                                    as
                                                                                                                    *const libc::c_char);
                                                                                                    }
                                                                                                }
                                                                                                if mp_cmp(&mut fR,
                                                                                                          &mut hR)
                                                                                                       ==
                                                                                                       0i32
                                                                                                   {
                                                                                                    mutest_passed_checks
                                                                                                        +=
                                                                                                        1
                                                                                                } else {
                                                                                                    mutest_failed_checks
                                                                                                        +=
                                                                                                        1;
                                                                                                    mutest_case_failed
                                                                                                        =
                                                                                                        1i32;
                                                                                                    if mutest_verbose_level
                                                                                                           >=
                                                                                                           MU_ERROR
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                       {
                                                                                                        if mutest_verbose_level
                                                                                                               ==
                                                                                                               MU_ERROR
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                           {
                                                                                                            fprintf(__stderrp,
                                                                                                                    b"build/ptest/server_test.c:%d: mu_check(mp_cmp(&fR, &hR) == 0) failed, resuming test case\n\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char,
                                                                                                                    118i32);
                                                                                                        } else {
                                                                                                            fprintf(__stdoutp,
                                                                                                                    b"build/ptest/server_test.c:%d: mu_check(mp_cmp(&fR, &hR) == 0) failed, resuming test case\n\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char,
                                                                                                                    118i32);
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
                        b"build/ptest/server_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 121i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/server_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 121i32);
            }
        }
    }
    if !data_items.is_null() { free(data_items as *mut libc::c_void); }
    if !for_a.is_null() { free(for_a as *mut libc::c_void); }
    if !for_b.is_null() { free(for_b as *mut libc::c_void); }
    mp_clear(&mut fR);
    mp_clear(&mut gR);
    mp_clear(&mut hR);
    PrioVerifier_clear(vA);
    PrioVerifier_clear(vB);
    PrioServer_clear(sA);
    PrioServer_clear(sB);
    PrioConfig_clear(cfg);
    PublicKey_clear(pkA);
    PublicKey_clear(pkB);
    PrivateKey_clear(skA);
    PrivateKey_clear(skB);
}
#[no_mangle]
pub unsafe extern "C" fn verify_full(mut tweak: libc::c_int) {
    let mut ndata: libc::c_int = 0;
    let mut aLen: libc::c_uint = 0;
    let mut bLen: libc::c_uint = 0;
    let mut shouldBe: libc::c_int = 0;
    let mut rv: SECStatus = SECSuccess;
    let mut pkA: PublicKey = 0 as PublicKey;
    let mut pkB: PublicKey = 0 as PublicKey;
    let mut skA: PrivateKey = 0 as PrivateKey;
    let mut skB: PrivateKey = 0 as PrivateKey;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut sA: PrioServer = 0 as PrioServer;
    let mut sB: PrioServer = 0 as PrioServer;
    let mut vA: PrioVerifier = 0 as PrioVerifier;
    let mut vB: PrioVerifier = 0 as PrioVerifier;
    let mut p1A: PrioPacketVerify1 = 0 as PrioPacketVerify1;
    let mut p1B: PrioPacketVerify1 = 0 as PrioPacketVerify1;
    let mut p2A: PrioPacketVerify2 = 0 as PrioPacketVerify2;
    let mut p2B: PrioPacketVerify2 = 0 as PrioPacketVerify2;
    let mut for_a: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut for_b: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut data_items: *mut bool = 0 as *mut bool;
    let mut fR: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut gR: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut hR: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    fR.dp = 0 as *mut mp_digit;
    gR.dp = 0 as *mut mp_digit;
    hR.dp = 0 as *mut mp_digit;
    let mut seed: PrioPRGSeed = [0; 16];
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
                        b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 174i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 174i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        rv = Keypair_new(&mut skA, &mut pkA);
        if rv as libc::c_int == SECSuccess as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/server_test.c:%d: mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 176i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/server_test.c:%d: mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 176i32);
                }
            }
        }
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            rv = Keypair_new(&mut skB, &mut pkB);
            if rv as libc::c_int == SECSuccess as libc::c_int {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/server_test.c:%d: mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                177i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/server_test.c:%d: mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                177i32);
                    }
                }
            }
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                let mut v: bool = false;
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((v = (cfg = PrioConfig_new(47, pkA, pkB, (unsigned char*)\"test4\", 5))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (cfg = PrioConfig_new(47, pkA, pkB, (unsigned char*)\"test4\", 5))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                cfg =
                    PrioConfig_new(47i32, pkA, pkB,
                                   b"test4\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_uchar,
                                   5i32 as libc::c_uint);
                v = !cfg.is_null();
                if v {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/server_test.c:%d: mu_check((v = (cfg = PrioConfig_new(47, pkA, pkB, (unsigned char*)\"test4\", 5)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    178i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/server_test.c:%d: mu_check((v = (cfg = PrioConfig_new(47, pkA, pkB, (unsigned char*)\"test4\", 5)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    178i32);
                        }
                    }
                }
                if !v {
                    rv = SECFailure
                } else {
                    ndata = PrioConfig_numDataFields(cfg as const_PrioConfig);
                    let mut v_0: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (data_items = calloc(ndata, sizeof(_Bool)))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (data_items = calloc(ndata, sizeof(_Bool)))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    data_items =
                        calloc(ndata as libc::c_ulong,
                               ::std::mem::size_of::<bool>() as libc::c_ulong)
                            as *mut bool;
                    v_0 = !data_items.is_null();
                    if v_0 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/server_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 181i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/server_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 181i32);
                            }
                        }
                    }
                    if !v_0 {
                        rv = SECFailure
                    } else {
                        let mut i: libc::c_int = 0i32;
                        while i < ndata {
                            *data_items.offset(i as isize) =
                                i % 3i32 == 1i32 || i % 5i32 == 3i32;
                            i += 1
                        }
                        let mut v_1: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        sA =
                            PrioServer_new(cfg as const_PrioConfig,
                                           PRIO_SERVER_A, skA,
                                           seed.as_mut_ptr() as
                                               *const libc::c_uchar);
                        v_1 = !sA.is_null();
                        if v_1 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/server_test.c:%d: mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 187i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/server_test.c:%d: mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 187i32);
                                }
                            }
                        }
                        if !v_1 {
                            rv = SECFailure
                        } else {
                            let mut v_2: bool = false;
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed))))...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed))))...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            sB =
                                PrioServer_new(cfg as const_PrioConfig,
                                               PRIO_SERVER_B, skB,
                                               seed.as_mut_ptr() as
                                                   *const libc::c_uchar);
                            v_2 = !sB.is_null();
                            if v_2 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/server_test.c:%d: mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                188i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/server_test.c:%d: mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                188i32);
                                    }
                                }
                            }
                            if !v_2 {
                                rv = SECFailure
                            } else {
                                aLen = 0;
                                bLen = 0;
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                rv =
                                    PrioClient_encode(cfg as const_PrioConfig,
                                                      data_items, &mut for_a,
                                                      &mut aLen, &mut for_b,
                                                      &mut bLen);
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
                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    191i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    191i32);
                                        }
                                    }
                                }
                                if !(rv as libc::c_int !=
                                         SECSuccess as libc::c_int) {
                                    if tweak == 5i32 {
                                        *for_a.offset(3isize) =
                                            3i32 as libc::c_uchar;
                                        *for_a.offset(4isize) =
                                            4i32 as libc::c_uchar
                                    }
                                    let mut v_3: bool = false;
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check((v = (vA = PrioVerifier_new(sA))))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check((v = (vA = PrioVerifier_new(sA))))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    vA = PrioVerifier_new(sA);
                                    v_3 = !vA.is_null();
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
                                                        b"build/ptest/server_test.c:%d: mu_check((v = (vA = PrioVerifier_new(sA)))) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        198i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/server_test.c:%d: mu_check((v = (vA = PrioVerifier_new(sA)))) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        198i32);
                                            }
                                        }
                                    }
                                    if !v_3 {
                                        rv = SECFailure
                                    } else {
                                        let mut v_4: bool = false;
                                        if mutest_verbose_level >=
                                               MU_CHECK as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"\t\t* Checking mu_check((v = (vB = PrioVerifier_new(sB))))...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"\t\t* Checking mu_check((v = (vB = PrioVerifier_new(sB))))...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            }
                                        }
                                        vB = PrioVerifier_new(sB);
                                        v_4 = !vB.is_null();
                                        if v_4 {
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
                                                            b"build/ptest/server_test.c:%d: mu_check((v = (vB = PrioVerifier_new(sB)))) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            199i32);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"build/ptest/server_test.c:%d: mu_check((v = (vB = PrioVerifier_new(sB)))) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            199i32);
                                                }
                                            }
                                        }
                                        if !v_4 {
                                            rv = SECFailure
                                        } else {
                                            rv =
                                                PrioVerifier_set_data(vA,
                                                                      for_a,
                                                                      aLen);
                                            if !(rv as libc::c_int !=
                                                     SECSuccess as
                                                         libc::c_int) {
                                                if mutest_verbose_level >=
                                                       MU_CHECK as libc::c_int
                                                   {
                                                    if mutest_verbose_level ==
                                                           MU_ERROR as
                                                               libc::c_int {
                                                        fprintf(__stderrp,
                                                                b"\t\t* Checking mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess)...\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                    } else {
                                                        fprintf(__stdoutp,
                                                                b"\t\t* Checking mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess)...\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                    }
                                                }
                                                rv =
                                                    PrioVerifier_set_data(vB,
                                                                          for_b,
                                                                          bLen);
                                                if rv as libc::c_int ==
                                                       SECSuccess as
                                                           libc::c_int {
                                                    mutest_passed_checks += 1
                                                } else {
                                                    mutest_failed_checks += 1;
                                                    mutest_case_failed = 1i32;
                                                    if mutest_verbose_level >=
                                                           MU_ERROR as
                                                               libc::c_int {
                                                        if mutest_verbose_level
                                                               ==
                                                               MU_ERROR as
                                                                   libc::c_int
                                                           {
                                                            fprintf(__stderrp,
                                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    201i32);
                                                        } else {
                                                            fprintf(__stdoutp,
                                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    201i32);
                                                        }
                                                    }
                                                }
                                                if !(rv as libc::c_int !=
                                                         SECSuccess as
                                                             libc::c_int) {
                                                    if tweak == 3i32 {
                                                        mp_add_d(&mut (*vA).share_fR,
                                                                 1i32 as
                                                                     mp_digit,
                                                                 &mut (*vA).share_fR);
                                                    }
                                                    if tweak == 4i32 {
                                                        mp_add_d(&mut (*vB).share_gR,
                                                                 1i32 as
                                                                     mp_digit,
                                                                 &mut (*vB).share_gR);
                                                    }
                                                    let mut v_5: bool = false;
                                                    if mutest_verbose_level >=
                                                           MU_CHECK as
                                                               libc::c_int {
                                                        if mutest_verbose_level
                                                               ==
                                                               MU_ERROR as
                                                                   libc::c_int
                                                           {
                                                            fprintf(__stderrp,
                                                                    b"\t\t* Checking mu_check((v = (p1A = PrioPacketVerify1_new())))...\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                        } else {
                                                            fprintf(__stdoutp,
                                                                    b"\t\t* Checking mu_check((v = (p1A = PrioPacketVerify1_new())))...\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                        }
                                                    }
                                                    p1A =
                                                        PrioPacketVerify1_new();
                                                    v_5 = !p1A.is_null();
                                                    if v_5 {
                                                        mutest_passed_checks
                                                            += 1
                                                    } else {
                                                        mutest_failed_checks
                                                            += 1;
                                                        mutest_case_failed =
                                                            1i32;
                                                        if mutest_verbose_level
                                                               >=
                                                               MU_ERROR as
                                                                   libc::c_int
                                                           {
                                                            if mutest_verbose_level
                                                                   ==
                                                                   MU_ERROR as
                                                                       libc::c_int
                                                               {
                                                                fprintf(__stderrp,
                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (p1A = PrioPacketVerify1_new()))) failed, resuming test case\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        211i32);
                                                            } else {
                                                                fprintf(__stdoutp,
                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (p1A = PrioPacketVerify1_new()))) failed, resuming test case\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        211i32);
                                                            }
                                                        }
                                                    }
                                                    if !v_5 {
                                                        rv = SECFailure
                                                    } else {
                                                        let mut v_6: bool =
                                                            false;
                                                        if mutest_verbose_level
                                                               >=
                                                               MU_CHECK as
                                                                   libc::c_int
                                                           {
                                                            if mutest_verbose_level
                                                                   ==
                                                                   MU_ERROR as
                                                                       libc::c_int
                                                               {
                                                                fprintf(__stderrp,
                                                                        b"\t\t* Checking mu_check((v = (p1B = PrioPacketVerify1_new())))...\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char);
                                                            } else {
                                                                fprintf(__stdoutp,
                                                                        b"\t\t* Checking mu_check((v = (p1B = PrioPacketVerify1_new())))...\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char);
                                                            }
                                                        }
                                                        p1B =
                                                            PrioPacketVerify1_new();
                                                        v_6 = !p1B.is_null();
                                                        if v_6 {
                                                            mutest_passed_checks
                                                                += 1
                                                        } else {
                                                            mutest_failed_checks
                                                                += 1;
                                                            mutest_case_failed
                                                                = 1i32;
                                                            if mutest_verbose_level
                                                                   >=
                                                                   MU_ERROR as
                                                                       libc::c_int
                                                               {
                                                                if mutest_verbose_level
                                                                       ==
                                                                       MU_ERROR
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    fprintf(__stderrp,
                                                                            b"build/ptest/server_test.c:%d: mu_check((v = (p1B = PrioPacketVerify1_new()))) failed, resuming test case\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            212i32);
                                                                } else {
                                                                    fprintf(__stdoutp,
                                                                            b"build/ptest/server_test.c:%d: mu_check((v = (p1B = PrioPacketVerify1_new()))) failed, resuming test case\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            212i32);
                                                                }
                                                            }
                                                        }
                                                        if !v_6 {
                                                            rv = SECFailure
                                                        } else {
                                                            if mutest_verbose_level
                                                                   >=
                                                                   MU_CHECK as
                                                                       libc::c_int
                                                               {
                                                                if mutest_verbose_level
                                                                       ==
                                                                       MU_ERROR
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    fprintf(__stderrp,
                                                                            b"\t\t* Checking mu_check((rv = (PrioPacketVerify1_set_data(p1A, vA))) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                } else {
                                                                    fprintf(__stdoutp,
                                                                            b"\t\t* Checking mu_check((rv = (PrioPacketVerify1_set_data(p1A, vA))) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                }
                                                            }
                                                            rv =
                                                                PrioPacketVerify1_set_data(p1A,
                                                                                           vA
                                                                                               as
                                                                                               const_PrioVerifier);
                                                            if rv as
                                                                   libc::c_int
                                                                   ==
                                                                   SECSuccess
                                                                       as
                                                                       libc::c_int
                                                               {
                                                                mutest_passed_checks
                                                                    += 1
                                                            } else {
                                                                mutest_failed_checks
                                                                    += 1;
                                                                mutest_case_failed
                                                                    = 1i32;
                                                                if mutest_verbose_level
                                                                       >=
                                                                       MU_ERROR
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    if mutest_verbose_level
                                                                           ==
                                                                           MU_ERROR
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        fprintf(__stderrp,
                                                                                b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPacketVerify1_set_data(p1A, vA))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                214i32);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPacketVerify1_set_data(p1A, vA))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                214i32);
                                                                    }
                                                                }
                                                            }
                                                            if !(rv as
                                                                     libc::c_int
                                                                     !=
                                                                     SECSuccess
                                                                         as
                                                                         libc::c_int)
                                                               {
                                                                if mutest_verbose_level
                                                                       >=
                                                                       MU_CHECK
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    if mutest_verbose_level
                                                                           ==
                                                                           MU_ERROR
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        fprintf(__stderrp,
                                                                                b"\t\t* Checking mu_check((rv = (PrioPacketVerify1_set_data(p1B, vB))) == SECSuccess)...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"\t\t* Checking mu_check((rv = (PrioPacketVerify1_set_data(p1B, vB))) == SECSuccess)...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    }
                                                                }
                                                                rv =
                                                                    PrioPacketVerify1_set_data(p1B,
                                                                                               vB
                                                                                                   as
                                                                                                   const_PrioVerifier);
                                                                if rv as
                                                                       libc::c_int
                                                                       ==
                                                                       SECSuccess
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    mutest_passed_checks
                                                                        += 1
                                                                } else {
                                                                    mutest_failed_checks
                                                                        += 1;
                                                                    mutest_case_failed
                                                                        =
                                                                        1i32;
                                                                    if mutest_verbose_level
                                                                           >=
                                                                           MU_ERROR
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        if mutest_verbose_level
                                                                               ==
                                                                               MU_ERROR
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            fprintf(__stderrp,
                                                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPacketVerify1_set_data(p1B, vB))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    215i32);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPacketVerify1_set_data(p1B, vB))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    215i32);
                                                                        }
                                                                    }
                                                                }
                                                                if !(rv as
                                                                         libc::c_int
                                                                         !=
                                                                         SECSuccess
                                                                             as
                                                                             libc::c_int)
                                                                   {
                                                                    if tweak
                                                                           ==
                                                                           1i32
                                                                       {
                                                                        mp_add_d(&mut (*p1B).share_d,
                                                                                 1i32
                                                                                     as
                                                                                     mp_digit,
                                                                                 &mut (*p1B).share_d);
                                                                    }
                                                                    let mut v_7:
                                                                            bool =
                                                                        false;
                                                                    if mutest_verbose_level
                                                                           >=
                                                                           MU_CHECK
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        if mutest_verbose_level
                                                                               ==
                                                                               MU_ERROR
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            fprintf(__stderrp,
                                                                                    b"\t\t* Checking mu_check((v = (p2A = PrioPacketVerify2_new())))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"\t\t* Checking mu_check((v = (p2A = PrioPacketVerify2_new())))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        }
                                                                    }
                                                                    p2A =
                                                                        PrioPacketVerify2_new();
                                                                    v_7 =
                                                                        !p2A.is_null();
                                                                    if v_7 {
                                                                        mutest_passed_checks
                                                                            +=
                                                                            1
                                                                    } else {
                                                                        mutest_failed_checks
                                                                            +=
                                                                            1;
                                                                        mutest_case_failed
                                                                            =
                                                                            1i32;
                                                                        if mutest_verbose_level
                                                                               >=
                                                                               MU_ERROR
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            if mutest_verbose_level
                                                                                   ==
                                                                                   MU_ERROR
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                fprintf(__stderrp,
                                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (p2A = PrioPacketVerify2_new()))) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        221i32);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"build/ptest/server_test.c:%d: mu_check((v = (p2A = PrioPacketVerify2_new()))) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        221i32);
                                                                            }
                                                                        }
                                                                    }
                                                                    if !v_7 {
                                                                        rv =
                                                                            SECFailure
                                                                    } else {
                                                                        let mut v_8:
                                                                                bool =
                                                                            false;
                                                                        if mutest_verbose_level
                                                                               >=
                                                                               MU_CHECK
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            if mutest_verbose_level
                                                                                   ==
                                                                                   MU_ERROR
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                fprintf(__stderrp,
                                                                                        b"\t\t* Checking mu_check((v = (p2B = PrioPacketVerify2_new())))...\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"\t\t* Checking mu_check((v = (p2B = PrioPacketVerify2_new())))...\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char);
                                                                            }
                                                                        }
                                                                        p2B =
                                                                            PrioPacketVerify2_new();
                                                                        v_8 =
                                                                            !p2B.is_null();
                                                                        if v_8
                                                                           {
                                                                            mutest_passed_checks
                                                                                +=
                                                                                1
                                                                        } else {
                                                                            mutest_failed_checks
                                                                                +=
                                                                                1;
                                                                            mutest_case_failed
                                                                                =
                                                                                1i32;
                                                                            if mutest_verbose_level
                                                                                   >=
                                                                                   MU_ERROR
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                if mutest_verbose_level
                                                                                       ==
                                                                                       MU_ERROR
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    fprintf(__stderrp,
                                                                                            b"build/ptest/server_test.c:%d: mu_check((v = (p2B = PrioPacketVerify2_new()))) failed, resuming test case\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char,
                                                                                            222i32);
                                                                                } else {
                                                                                    fprintf(__stdoutp,
                                                                                            b"build/ptest/server_test.c:%d: mu_check((v = (p2B = PrioPacketVerify2_new()))) failed, resuming test case\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char,
                                                                                            222i32);
                                                                                }
                                                                            }
                                                                        }
                                                                        if !v_8
                                                                           {
                                                                            rv
                                                                                =
                                                                                SECFailure
                                                                        } else {
                                                                            if mutest_verbose_level
                                                                                   >=
                                                                                   MU_CHECK
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                if mutest_verbose_level
                                                                                       ==
                                                                                       MU_ERROR
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    fprintf(__stderrp,
                                                                                            b"\t\t* Checking mu_check((rv = (PrioPacketVerify2_set_data(p2A, vA, p1A, p1B))) == SECSuccess)...\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char);
                                                                                } else {
                                                                                    fprintf(__stdoutp,
                                                                                            b"\t\t* Checking mu_check((rv = (PrioPacketVerify2_set_data(p2A, vA, p1A, p1B))) == SECSuccess)...\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char);
                                                                                }
                                                                            }
                                                                            rv
                                                                                =
                                                                                PrioPacketVerify2_set_data(p2A,
                                                                                                           vA
                                                                                                               as
                                                                                                               const_PrioVerifier,
                                                                                                           p1A
                                                                                                               as
                                                                                                               const_PrioPacketVerify1,
                                                                                                           p1B
                                                                                                               as
                                                                                                               const_PrioPacketVerify1);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   ==
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                mutest_passed_checks
                                                                                    +=
                                                                                    1
                                                                            } else {
                                                                                mutest_failed_checks
                                                                                    +=
                                                                                    1;
                                                                                mutest_case_failed
                                                                                    =
                                                                                    1i32;
                                                                                if mutest_verbose_level
                                                                                       >=
                                                                                       MU_ERROR
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    if mutest_verbose_level
                                                                                           ==
                                                                                           MU_ERROR
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        fprintf(__stderrp,
                                                                                                b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPacketVerify2_set_data(p2A, vA, p1A, p1B))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                223i32);
                                                                                    } else {
                                                                                        fprintf(__stdoutp,
                                                                                                b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPacketVerify2_set_data(p2A, vA, p1A, p1B))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char,
                                                                                                223i32);
                                                                                    }
                                                                                }
                                                                            }
                                                                            if !(rv
                                                                                     as
                                                                                     libc::c_int
                                                                                     !=
                                                                                     SECSuccess
                                                                                         as
                                                                                         libc::c_int)
                                                                               {
                                                                                if mutest_verbose_level
                                                                                       >=
                                                                                       MU_CHECK
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    if mutest_verbose_level
                                                                                           ==
                                                                                           MU_ERROR
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        fprintf(__stderrp,
                                                                                                b"\t\t* Checking mu_check((rv = (PrioPacketVerify2_set_data(p2B, vB, p1A, p1B))) == SECSuccess)...\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char);
                                                                                    } else {
                                                                                        fprintf(__stdoutp,
                                                                                                b"\t\t* Checking mu_check((rv = (PrioPacketVerify2_set_data(p2B, vB, p1A, p1B))) == SECSuccess)...\n\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char);
                                                                                    }
                                                                                }
                                                                                rv
                                                                                    =
                                                                                    PrioPacketVerify2_set_data(p2B,
                                                                                                               vB
                                                                                                                   as
                                                                                                                   const_PrioVerifier,
                                                                                                               p1A
                                                                                                                   as
                                                                                                                   const_PrioPacketVerify1,
                                                                                                               p1B
                                                                                                                   as
                                                                                                                   const_PrioPacketVerify1);
                                                                                if rv
                                                                                       as
                                                                                       libc::c_int
                                                                                       ==
                                                                                       SECSuccess
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    mutest_passed_checks
                                                                                        +=
                                                                                        1
                                                                                } else {
                                                                                    mutest_failed_checks
                                                                                        +=
                                                                                        1;
                                                                                    mutest_case_failed
                                                                                        =
                                                                                        1i32;
                                                                                    if mutest_verbose_level
                                                                                           >=
                                                                                           MU_ERROR
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        if mutest_verbose_level
                                                                                               ==
                                                                                               MU_ERROR
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            fprintf(__stderrp,
                                                                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPacketVerify2_set_data(p2B, vB, p1A, p1B))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char,
                                                                                                    224i32);
                                                                                        } else {
                                                                                            fprintf(__stdoutp,
                                                                                                    b"build/ptest/server_test.c:%d: mu_check((rv = (PrioPacketVerify2_set_data(p2B, vB, p1A, p1B))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char,
                                                                                                    224i32);
                                                                                        }
                                                                                    }
                                                                                }
                                                                                if !(rv
                                                                                         as
                                                                                         libc::c_int
                                                                                         !=
                                                                                         SECSuccess
                                                                                             as
                                                                                             libc::c_int)
                                                                                   {
                                                                                    if tweak
                                                                                           ==
                                                                                           2i32
                                                                                       {
                                                                                        mp_add_d(&mut (*p2A).share_out,
                                                                                                 1i32
                                                                                                     as
                                                                                                     mp_digit,
                                                                                                 &mut (*p2B).share_out);
                                                                                    }
                                                                                    shouldBe
                                                                                        =
                                                                                        if 0
                                                                                               !=
                                                                                               tweak
                                                                                           {
                                                                                            SECFailure
                                                                                                as
                                                                                                libc::c_int
                                                                                        } else {
                                                                                            SECSuccess
                                                                                                as
                                                                                                libc::c_int
                                                                                        };
                                                                                    if mutest_verbose_level
                                                                                           >=
                                                                                           MU_CHECK
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        if mutest_verbose_level
                                                                                               ==
                                                                                               MU_ERROR
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            fprintf(__stderrp,
                                                                                                    b"\t\t* Checking mu_check(PrioVerifier_isValid(vA, p2A, p2B) == shouldBe)...\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char);
                                                                                        } else {
                                                                                            fprintf(__stdoutp,
                                                                                                    b"\t\t* Checking mu_check(PrioVerifier_isValid(vA, p2A, p2B) == shouldBe)...\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char);
                                                                                        }
                                                                                    }
                                                                                    if PrioVerifier_isValid(vA
                                                                                                                as
                                                                                                                const_PrioVerifier,
                                                                                                            p2A
                                                                                                                as
                                                                                                                const_PrioPacketVerify2,
                                                                                                            p2B
                                                                                                                as
                                                                                                                const_PrioPacketVerify2)
                                                                                           ==
                                                                                           shouldBe
                                                                                       {
                                                                                        mutest_passed_checks
                                                                                            +=
                                                                                            1
                                                                                    } else {
                                                                                        mutest_failed_checks
                                                                                            +=
                                                                                            1;
                                                                                        mutest_case_failed
                                                                                            =
                                                                                            1i32;
                                                                                        if mutest_verbose_level
                                                                                               >=
                                                                                               MU_ERROR
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            if mutest_verbose_level
                                                                                                   ==
                                                                                                   MU_ERROR
                                                                                                       as
                                                                                                       libc::c_int
                                                                                               {
                                                                                                fprintf(__stderrp,
                                                                                                        b"build/ptest/server_test.c:%d: mu_check(PrioVerifier_isValid(vA, p2A, p2B) == shouldBe) failed, resuming test case\n\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char,
                                                                                                        231i32);
                                                                                            } else {
                                                                                                fprintf(__stdoutp,
                                                                                                        b"build/ptest/server_test.c:%d: mu_check(PrioVerifier_isValid(vA, p2A, p2B) == shouldBe) failed, resuming test case\n\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char,
                                                                                                        231i32);
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    if mutest_verbose_level
                                                                                           >=
                                                                                           MU_CHECK
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        if mutest_verbose_level
                                                                                               ==
                                                                                               MU_ERROR
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            fprintf(__stderrp,
                                                                                                    b"\t\t* Checking mu_check(PrioVerifier_isValid(vB, p2A, p2B) == shouldBe)...\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char);
                                                                                        } else {
                                                                                            fprintf(__stdoutp,
                                                                                                    b"\t\t* Checking mu_check(PrioVerifier_isValid(vB, p2A, p2B) == shouldBe)...\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char);
                                                                                        }
                                                                                    }
                                                                                    if PrioVerifier_isValid(vB
                                                                                                                as
                                                                                                                const_PrioVerifier,
                                                                                                            p2A
                                                                                                                as
                                                                                                                const_PrioPacketVerify2,
                                                                                                            p2B
                                                                                                                as
                                                                                                                const_PrioPacketVerify2)
                                                                                           ==
                                                                                           shouldBe
                                                                                       {
                                                                                        mutest_passed_checks
                                                                                            +=
                                                                                            1
                                                                                    } else {
                                                                                        mutest_failed_checks
                                                                                            +=
                                                                                            1;
                                                                                        mutest_case_failed
                                                                                            =
                                                                                            1i32;
                                                                                        if mutest_verbose_level
                                                                                               >=
                                                                                               MU_ERROR
                                                                                                   as
                                                                                                   libc::c_int
                                                                                           {
                                                                                            if mutest_verbose_level
                                                                                                   ==
                                                                                                   MU_ERROR
                                                                                                       as
                                                                                                       libc::c_int
                                                                                               {
                                                                                                fprintf(__stderrp,
                                                                                                        b"build/ptest/server_test.c:%d: mu_check(PrioVerifier_isValid(vB, p2A, p2B) == shouldBe) failed, resuming test case\n\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char,
                                                                                                        232i32);
                                                                                            } else {
                                                                                                fprintf(__stdoutp,
                                                                                                        b"build/ptest/server_test.c:%d: mu_check(PrioVerifier_isValid(vB, p2A, p2B) == shouldBe) failed, resuming test case\n\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char,
                                                                                                        232i32);
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
    }
    if 0 == tweak {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check(rv == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check(rv == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
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
                            b"build/ptest/server_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 236i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/server_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 236i32);
                }
            }
        }
    }
    if !data_items.is_null() { free(data_items as *mut libc::c_void); }
    if !for_a.is_null() { free(for_a as *mut libc::c_void); }
    if !for_b.is_null() { free(for_b as *mut libc::c_void); }
    PrioPacketVerify2_clear(p2A);
    PrioPacketVerify2_clear(p2B);
    PrioPacketVerify1_clear(p1A);
    PrioPacketVerify1_clear(p1B);
    PrioVerifier_clear(vA);
    PrioVerifier_clear(vB);
    PrioServer_clear(sA);
    PrioServer_clear(sB);
    PrioConfig_clear(cfg);
    PublicKey_clear(pkA);
    PublicKey_clear(pkB);
    PrivateKey_clear(skA);
    PrivateKey_clear(skB);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__verify_full_good() { verify_full(0i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test__verify_full_bad1() { verify_full(1i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test__verify_full_bad2() { verify_full(2i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test__verify_full_bad3() { verify_full(3i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test__verify_full_bad4() { verify_full(4i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test__verify_full_bad5() { verify_full(5i32); }