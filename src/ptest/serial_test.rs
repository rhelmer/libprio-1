#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    pub type msgpack_zone_chunk;
    pub type __sFILEX;
    pub type PK11SlotInfoStr;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn msgpack_unpacker_init(mpac: *mut msgpack_unpacker,
                             initial_buffer_size: size_t) -> bool;
    #[no_mangle]
    fn msgpack_unpacker_destroy(mpac: *mut msgpack_unpacker);
    #[no_mangle]
    fn msgpack_unpacker_expand_buffer(mpac: *mut msgpack_unpacker,
                                      size: size_t) -> bool;
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
    /*
 * Create a PrioConfig object with no encryption keys.  This routine is
 * useful for testing, but PrioClient_encode() will always fail when used with
 * this config.
 */
    #[no_mangle]
    fn PrioConfig_newTest(nFields: libc::c_int) -> PrioConfig;
    /*
 * Generate the first packet that servers need to exchange to verify the
 * client's submission. This should be sent over a TLS connection between the
 * servers.
 */
    #[no_mangle]
    fn PrioPacketVerify1_new() -> PrioPacketVerify1;
    #[no_mangle]
    fn PrioPacketVerify1_clear(p1: PrioPacketVerify1);
    #[no_mangle]
    fn PrioPacketVerify1_write(p: const_PrioPacketVerify1,
                               pk: *mut msgpack_packer) -> SECStatus;
    #[no_mangle]
    fn PrioPacketVerify1_read(p: PrioPacketVerify1,
                              upk: *mut msgpack_unpacker,
                              cfg: const_PrioConfig) -> SECStatus;
    /*
 * Generate the second packet that the servers need to exchange to verify the
 * client's submission. The routine takes as input the PrioPacketVerify1
 * packets from both server A and server B.
 *
 * This should be sent over a TLS connection between the servers.
 */
    #[no_mangle]
    fn PrioPacketVerify2_new() -> PrioPacketVerify2;
    #[no_mangle]
    fn PrioPacketVerify2_clear(p: PrioPacketVerify2);
    #[no_mangle]
    fn PrioPacketVerify2_write(p: const_PrioPacketVerify2,
                               pk: *mut msgpack_packer) -> SECStatus;
    #[no_mangle]
    fn PrioPacketVerify2_read(p: PrioPacketVerify2,
                              upk: *mut msgpack_unpacker,
                              cfg: const_PrioConfig) -> SECStatus;
    /*
 * After the servers have aggregated data packets from "enough" clients
 * (this determines the anonymity set size), each server runs this routine
 * to get a share of the aggregate statistics.
 */
    #[no_mangle]
    fn PrioTotalShare_new() -> PrioTotalShare;
    #[no_mangle]
    fn PrioTotalShare_clear(t: PrioTotalShare);
    #[no_mangle]
    fn PrioTotalShare_write(t: const_PrioTotalShare, pk: *mut msgpack_packer)
     -> SECStatus;
    #[no_mangle]
    fn PrioTotalShare_read(t: PrioTotalShare, upk: *mut msgpack_unpacker,
                           cfg: const_PrioConfig) -> SECStatus;
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
    fn mp_set(mp: *mut mp_int, d: mp_digit);
    #[no_mangle]
    fn mp_cmp_d(a: *const mp_int, d: mp_digit) -> libc::c_int;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    /*
 * Expands or shrinks the MPArray to the desired size. If shrinking,
 * will clear the values on the end of array.
 */
    #[no_mangle]
    fn MPArray_resize(arr: MPArray, newlen: libc::c_int) -> SECStatus;
    /*
 * Return true iff the two arrays are equal in length
 * and contents. This comparison is NOT constant time.
 */
    #[no_mangle]
    fn MPArray_areEqual(arr1: const_MPArray, arr2: const_MPArray) -> bool;
    #[no_mangle]
    fn PrioPacketClient_new(cfg: const_PrioConfig, for_server: PrioServerId)
     -> PrioPacketClient;
    #[no_mangle]
    fn PrioPacketClient_clear(p: PrioPacketClient);
    #[no_mangle]
    fn PrioPacketClient_set_data(cfg: const_PrioConfig, data_in: *const bool,
                                 for_server_a: PrioPacketClient,
                                 for_server_b: PrioPacketClient) -> SECStatus;
    #[no_mangle]
    fn PrioPacketClient_areEqual(p1: const_PrioPacketClient,
                                 p2: const_PrioPacketClient) -> bool;
    /*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
    #[no_mangle]
    fn serial_write_packet_client(pk: *mut msgpack_packer,
                                  p: const_PrioPacketClient,
                                  cfg: const_PrioConfig) -> SECStatus;
    #[no_mangle]
    fn serial_read_packet_client(upk: *mut msgpack_unpacker,
                                 p: PrioPacketClient, cfg: const_PrioConfig)
     -> SECStatus;
}
pub type size_t = libc::c_ulong;
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
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_zone_finalizer {
    pub func: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub data: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_zone_finalizer_array {
    pub tail: *mut msgpack_zone_finalizer,
    pub end: *mut msgpack_zone_finalizer,
    pub array: *mut msgpack_zone_finalizer,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_zone_chunk_list {
    pub free: size_t,
    pub ptr: *mut libc::c_char,
    pub head: *mut msgpack_zone_chunk,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_zone {
    pub chunk_list: msgpack_zone_chunk_list,
    pub finalizer_array: msgpack_zone_finalizer_array,
    pub chunk_size: size_t,
}
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
pub type msgpack_packer_write
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: size_t) -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_packer {
    pub data: *mut libc::c_void,
    pub callback: msgpack_packer_write,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_unpacker {
    pub buffer: *mut libc::c_char,
    pub used: size_t,
    pub free: size_t,
    pub off: size_t,
    pub parsed: size_t,
    pub z: *mut msgpack_zone,
    pub initial_buffer_size: size_t,
    pub ctx: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_sbuffer {
    pub size: size_t,
    pub data: *mut libc::c_char,
    pub alloc: size_t,
}
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
pub type const_MPArray = *const mparray;
pub type const_PrioPacketClient = *const prio_packet_client;
unsafe extern "C" fn msgpack_packer_init(mut pk: *mut msgpack_packer,
                                         mut data: *mut libc::c_void,
                                         mut callback: msgpack_packer_write) {
    (*pk).data = data;
    (*pk).callback = callback;
}
unsafe extern "C" fn msgpack_unpacker_reserve_buffer(mut mpac:
                                                         *mut msgpack_unpacker,
                                                     mut size: size_t)
 -> bool {
    if (*mpac).free >= size { return 0 != 1i32 }
    return msgpack_unpacker_expand_buffer(mpac, size);
}
unsafe extern "C" fn msgpack_unpacker_buffer(mut mpac: *mut msgpack_unpacker)
 -> *mut libc::c_char {
    return (*mpac).buffer.offset((*mpac).used as isize);
}
unsafe extern "C" fn msgpack_unpacker_buffer_consumed(mut mpac:
                                                          *mut msgpack_unpacker,
                                                      mut size: size_t) {
    (*mpac).used =
        ((*mpac).used as libc::c_ulong).wrapping_add(size) as size_t as
            size_t;
    (*mpac).free =
        ((*mpac).free as libc::c_ulong).wrapping_sub(size) as size_t as
            size_t;
}
unsafe extern "C" fn msgpack_sbuffer_init(mut sbuf: *mut msgpack_sbuffer) {
    memset(sbuf as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<msgpack_sbuffer>() as libc::c_ulong);
}
unsafe extern "C" fn msgpack_sbuffer_destroy(mut sbuf: *mut msgpack_sbuffer) {
    free((*sbuf).data as *mut libc::c_void);
}
unsafe extern "C" fn msgpack_sbuffer_write(mut data: *mut libc::c_void,
                                           mut buf: *const libc::c_char,
                                           mut len: size_t) -> libc::c_int {
    let mut sbuf: *mut msgpack_sbuffer = data as *mut msgpack_sbuffer;
    if (*sbuf).alloc.wrapping_sub((*sbuf).size) < len {
        let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut nsize: size_t =
            if 0 != (*sbuf).alloc {
                (*sbuf).alloc.wrapping_mul(2i32 as libc::c_ulong)
            } else { 8192i32 as libc::c_ulong };
        while nsize < (*sbuf).size.wrapping_add(len) {
            let mut tmp_nsize: size_t =
                nsize.wrapping_mul(2i32 as libc::c_ulong);
            if tmp_nsize <= nsize {
                nsize = (*sbuf).size.wrapping_add(len);
                break ;
            } else { nsize = tmp_nsize }
        }
        tmp = realloc((*sbuf).data as *mut libc::c_void, nsize);
        if tmp.is_null() { return -1i32 }
        (*sbuf).data = tmp as *mut libc::c_char;
        (*sbuf).alloc = nsize
    }
    memcpy((*sbuf).data.offset((*sbuf).size as isize) as *mut libc::c_void,
           buf as *const libc::c_void, len);
    (*sbuf).size =
        ((*sbuf).size as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    return 0i32;
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn gen_client_packets(mut cfg: const_PrioConfig,
                                            mut pA: PrioPacketClient,
                                            mut pB: PrioPacketClient)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let ndata: libc::c_int = (*cfg).num_data_fields;
    let mut data_items: *mut bool = 0 as *mut bool;
    let mut v: bool = false;
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
               ::std::mem::size_of::<bool>() as libc::c_ulong) as *mut bool;
    v = !data_items.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 29i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 29i32);
            }
        }
    }
    if !v {
        rv = SECFailure
    } else {
        let mut i: libc::c_int = 0i32;
        while i < ndata {
            *data_items.offset(i as isize) =
                i % 3i32 == 1i32 || i % 5i32 == 3i32;
            i += 1
        }
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((rv = (PrioPacketClient_set_data(cfg, data_items, pA, pB))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((rv = (PrioPacketClient_set_data(cfg, data_items, pA, pB))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        rv = PrioPacketClient_set_data(cfg, data_items, pA, pB);
        if rv as libc::c_int == SECSuccess as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/serial_test.c:%d: mu_check((rv = (PrioPacketClient_set_data(cfg, data_items, pA, pB))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 35i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/serial_test.c:%d: mu_check((rv = (PrioPacketClient_set_data(cfg, data_items, pA, pB))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 35i32);
                }
            }
        }
        rv as libc::c_int != SECSuccess as libc::c_int;
    }
    if !data_items.is_null() { free(data_items as *mut libc::c_void); }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn serial_client(mut bad: libc::c_int) {
    let mut size_a: libc::c_int = 0;
    let mut size_b: libc::c_int = 0;
    let mut rv: SECStatus = SECSuccess;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut cfg2: PrioConfig = 0 as PrioConfig;
    let mut pA: PrioPacketClient = 0 as PrioPacketClient;
    let mut pB: PrioPacketClient = 0 as PrioPacketClient;
    let mut qA: PrioPacketClient = 0 as PrioPacketClient;
    let mut qB: PrioPacketClient = 0 as PrioPacketClient;
    let mut batch_id1: *const libc::c_uchar =
        b"my_test_prio_batch1\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_uchar;
    let mut batch_id2: *const libc::c_uchar =
        b"my_test_prio_batch2\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_uchar;
    let batch_id_len: libc::c_uint =
        strlen(batch_id1 as *mut libc::c_char) as libc::c_uint;
    let mut sbufA: msgpack_sbuffer =
        msgpack_sbuffer{size: 0, data: 0 as *mut libc::c_char, alloc: 0,};
    let mut sbufB: msgpack_sbuffer =
        msgpack_sbuffer{size: 0, data: 0 as *mut libc::c_char, alloc: 0,};
    let mut pkA: msgpack_packer =
        msgpack_packer{data: 0 as *mut libc::c_void, callback: None,};
    let mut pkB: msgpack_packer =
        msgpack_packer{data: 0 as *mut libc::c_void, callback: None,};
    let mut upkA: msgpack_unpacker =
        msgpack_unpacker{buffer: 0 as *mut libc::c_char,
                         used: 0,
                         free: 0,
                         off: 0,
                         parsed: 0,
                         z: 0 as *mut msgpack_zone,
                         initial_buffer_size: 0,
                         ctx: 0 as *mut libc::c_void,};
    let mut upkB: msgpack_unpacker =
        msgpack_unpacker{buffer: 0 as *mut libc::c_char,
                         used: 0,
                         free: 0,
                         off: 0,
                         parsed: 0,
                         z: 0 as *mut msgpack_zone,
                         initial_buffer_size: 0,
                         ctx: 0 as *mut libc::c_void,};
    msgpack_sbuffer_init(&mut sbufA);
    msgpack_packer_init(&mut pkA,
                        &mut sbufA as *mut msgpack_sbuffer as
                            *mut libc::c_void, Some(msgpack_sbuffer_write));
    msgpack_sbuffer_init(&mut sbufB);
    msgpack_packer_init(&mut pkB,
                        &mut sbufB as *mut msgpack_sbuffer as
                            *mut libc::c_void, Some(msgpack_sbuffer_write));
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_new(100, ((void*)0), ((void*)0), batch_id1, batch_id_len))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_new(100, ((void*)0), ((void*)0), batch_id1, batch_id_len))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg =
        PrioConfig_new(100i32, 0 as PublicKey, 0 as PublicKey, batch_id1,
                       batch_id_len);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (cfg = PrioConfig_new(100, ((void*)0), ((void*)0), batch_id1, batch_id_len)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 68i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (cfg = PrioConfig_new(100, ((void*)0), ((void*)0), batch_id1, batch_id_len)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 68i32);
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
                        b"\t\t* Checking mu_check((v = (cfg2 = PrioConfig_new(100, ((void*)0), ((void*)0), batch_id2, batch_id_len))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (cfg2 = PrioConfig_new(100, ((void*)0), ((void*)0), batch_id2, batch_id_len))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        cfg2 =
            PrioConfig_new(100i32, 0 as PublicKey, 0 as PublicKey, batch_id2,
                           batch_id_len);
        v_0 = !cfg2.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/serial_test.c:%d: mu_check((v = (cfg2 = PrioConfig_new(100, ((void*)0), ((void*)0), batch_id2, batch_id_len)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 69i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/serial_test.c:%d: mu_check((v = (cfg2 = PrioConfig_new(100, ((void*)0), ((void*)0), batch_id2, batch_id_len)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 69i32);
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
                            b"\t\t* Checking mu_check((v = (pA = PrioPacketClient_new(cfg, PRIO_SERVER_A))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (pA = PrioPacketClient_new(cfg, PRIO_SERVER_A))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            pA = PrioPacketClient_new(cfg as const_PrioConfig, PRIO_SERVER_A);
            v_1 = !pA.is_null();
            if v_1 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/serial_test.c:%d: mu_check((v = (pA = PrioPacketClient_new(cfg, PRIO_SERVER_A)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                70i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/serial_test.c:%d: mu_check((v = (pA = PrioPacketClient_new(cfg, PRIO_SERVER_A)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                70i32);
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
                                b"\t\t* Checking mu_check((v = (pB = PrioPacketClient_new(cfg, PRIO_SERVER_B))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((v = (pB = PrioPacketClient_new(cfg, PRIO_SERVER_B))))...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                pB =
                    PrioPacketClient_new(cfg as const_PrioConfig,
                                         PRIO_SERVER_B);
                v_2 = !pB.is_null();
                if v_2 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/serial_test.c:%d: mu_check((v = (pB = PrioPacketClient_new(cfg, PRIO_SERVER_B)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    71i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/serial_test.c:%d: mu_check((v = (pB = PrioPacketClient_new(cfg, PRIO_SERVER_B)))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    71i32);
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
                                    b"\t\t* Checking mu_check((v = (qA = PrioPacketClient_new(cfg, PRIO_SERVER_A))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (qA = PrioPacketClient_new(cfg, PRIO_SERVER_A))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    qA =
                        PrioPacketClient_new(cfg as const_PrioConfig,
                                             PRIO_SERVER_A);
                    v_3 = !qA.is_null();
                    if v_3 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/serial_test.c:%d: mu_check((v = (qA = PrioPacketClient_new(cfg, PRIO_SERVER_A)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 72i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/serial_test.c:%d: mu_check((v = (qA = PrioPacketClient_new(cfg, PRIO_SERVER_A)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 72i32);
                            }
                        }
                    }
                    if !v_3 {
                        rv = SECFailure
                    } else {
                        let mut v_4: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = (qB = PrioPacketClient_new(cfg, PRIO_SERVER_B))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (qB = PrioPacketClient_new(cfg, PRIO_SERVER_B))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        qB =
                            PrioPacketClient_new(cfg as const_PrioConfig,
                                                 PRIO_SERVER_B);
                        v_4 = !qB.is_null();
                        if v_4 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/serial_test.c:%d: mu_check((v = (qB = PrioPacketClient_new(cfg, PRIO_SERVER_B)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 73i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/serial_test.c:%d: mu_check((v = (qB = PrioPacketClient_new(cfg, PRIO_SERVER_B)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 73i32);
                                }
                            }
                        }
                        if !v_4 {
                            rv = SECFailure
                        } else {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((rv = (gen_client_packets(cfg, pA, pB))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((rv = (gen_client_packets(cfg, pA, pB))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            rv =
                                gen_client_packets(cfg as const_PrioConfig,
                                                   pA, pB);
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
                                                b"build/ptest/serial_test.c:%d: mu_check((rv = (gen_client_packets(cfg, pA, pB))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                75i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/serial_test.c:%d: mu_check((rv = (gen_client_packets(cfg, pA, pB))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                75i32);
                                    }
                                }
                            }
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check((rv = (serial_write_packet_client(&pkA, pA, cfg))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check((rv = (serial_write_packet_client(&pkA, pA, cfg))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                rv =
                                    serial_write_packet_client(&mut pkA,
                                                               pA as
                                                                   const_PrioPacketClient,
                                                               cfg as
                                                                   const_PrioConfig);
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
                                                    b"build/ptest/serial_test.c:%d: mu_check((rv = (serial_write_packet_client(&pkA, pA, cfg))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    77i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/serial_test.c:%d: mu_check((rv = (serial_write_packet_client(&pkA, pA, cfg))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    77i32);
                                        }
                                    }
                                }
                                if !(rv as libc::c_int !=
                                         SECSuccess as libc::c_int) {
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check((rv = (serial_write_packet_client(&pkB, pB, cfg))) == SECSuccess)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check((rv = (serial_write_packet_client(&pkB, pB, cfg))) == SECSuccess)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    rv =
                                        serial_write_packet_client(&mut pkB,
                                                                   pB as
                                                                       const_PrioPacketClient,
                                                                   cfg as
                                                                       const_PrioConfig);
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
                                                        b"build/ptest/serial_test.c:%d: mu_check((rv = (serial_write_packet_client(&pkB, pB, cfg))) == SECSuccess) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        78i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/serial_test.c:%d: mu_check((rv = (serial_write_packet_client(&pkB, pB, cfg))) == SECSuccess) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        78i32);
                                            }
                                        }
                                    }
                                    if !(rv as libc::c_int !=
                                             SECSuccess as libc::c_int) {
                                        if bad == 1i32 {
                                            sbufA.size = 1i32 as size_t
                                        }
                                        if bad == 2i32 {
                                            memset(sbufA.data as
                                                       *mut libc::c_void,
                                                   0i32, sbufA.size);
                                        }
                                        size_a = sbufA.size as libc::c_int;
                                        size_b = sbufB.size as libc::c_int;
                                        let mut v_5: bool = false;
                                        if mutest_verbose_level >=
                                               MU_CHECK as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upkA, 0))))...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upkA, 0))))...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            }
                                        }
                                        v_5 =
                                            msgpack_unpacker_init(&mut upkA,
                                                                  0i32 as
                                                                      size_t);
                                        if v_5 {
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
                                                            b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upkA, 0)))) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            91i32);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upkA, 0)))) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            91i32);
                                                }
                                            }
                                        }
                                        if v_5 {
                                            let mut v_6: bool = false;
                                            if mutest_verbose_level >=
                                                   MU_CHECK as libc::c_int {
                                                if mutest_verbose_level ==
                                                       MU_ERROR as libc::c_int
                                                   {
                                                    fprintf(__stderrp,
                                                            b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upkB, 0))))...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upkB, 0))))...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                }
                                            }
                                            v_6 =
                                                msgpack_unpacker_init(&mut upkB,
                                                                      0i32 as
                                                                          size_t);
                                            if v_6 {
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
                                                                b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upkB, 0)))) failed, resuming test case\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                92i32);
                                                    } else {
                                                        fprintf(__stdoutp,
                                                                b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upkB, 0)))) failed, resuming test case\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                92i32);
                                                    }
                                                }
                                            }
                                            if v_6 {
                                                let mut v_7: bool = false;
                                                if mutest_verbose_level >=
                                                       MU_CHECK as libc::c_int
                                                   {
                                                    if mutest_verbose_level ==
                                                           MU_ERROR as
                                                               libc::c_int {
                                                        fprintf(__stderrp,
                                                                b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upkA, size_a))))...\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                    } else {
                                                        fprintf(__stdoutp,
                                                                b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upkA, size_a))))...\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
                                                    }
                                                }
                                                v_7 =
                                                    msgpack_unpacker_reserve_buffer(&mut upkA,
                                                                                    size_a
                                                                                        as
                                                                                        size_t);
                                                if v_7 {
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
                                                                    b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upkA, size_a)))) failed, resuming test case\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    94i32);
                                                        } else {
                                                            fprintf(__stdoutp,
                                                                    b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upkA, size_a)))) failed, resuming test case\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    94i32);
                                                        }
                                                    }
                                                }
                                                if v_7 {
                                                    let mut v_8: bool = false;
                                                    if mutest_verbose_level >=
                                                           MU_CHECK as
                                                               libc::c_int {
                                                        if mutest_verbose_level
                                                               ==
                                                               MU_ERROR as
                                                                   libc::c_int
                                                           {
                                                            fprintf(__stderrp,
                                                                    b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upkB, size_b))))...\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                        } else {
                                                            fprintf(__stdoutp,
                                                                    b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upkB, size_b))))...\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                        }
                                                    }
                                                    v_8 =
                                                        msgpack_unpacker_reserve_buffer(&mut upkB,
                                                                                        size_b
                                                                                            as
                                                                                            size_t);
                                                    if v_8 {
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
                                                                        b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upkB, size_b)))) failed, resuming test case\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        95i32);
                                                            } else {
                                                                fprintf(__stdoutp,
                                                                        b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upkB, size_b)))) failed, resuming test case\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        95i32);
                                                            }
                                                        }
                                                    }
                                                    if v_8 {
                                                        memcpy(msgpack_unpacker_buffer(&mut upkA)
                                                                   as
                                                                   *mut libc::c_void,
                                                               sbufA.data as
                                                                   *const libc::c_void,
                                                               size_a as
                                                                   libc::c_ulong);
                                                        memcpy(msgpack_unpacker_buffer(&mut upkB)
                                                                   as
                                                                   *mut libc::c_void,
                                                               sbufB.data as
                                                                   *const libc::c_void,
                                                               size_b as
                                                                   libc::c_ulong);
                                                        msgpack_unpacker_buffer_consumed(&mut upkA,
                                                                                         size_a
                                                                                             as
                                                                                             size_t);
                                                        msgpack_unpacker_buffer_consumed(&mut upkB,
                                                                                         size_b
                                                                                             as
                                                                                             size_t);
                                                        rv =
                                                            serial_read_packet_client(&mut upkA,
                                                                                      qA,
                                                                                      cfg
                                                                                          as
                                                                                          const_PrioConfig);
                                                        if !(rv as libc::c_int
                                                                 !=
                                                                 SECSuccess as
                                                                     libc::c_int)
                                                           {
                                                            rv =
                                                                serial_read_packet_client(&mut upkB,
                                                                                          qB,
                                                                                          (if bad
                                                                                                  ==
                                                                                                  3i32
                                                                                              {
                                                                                               cfg2
                                                                                           } else {
                                                                                               cfg
                                                                                           })
                                                                                              as
                                                                                              const_PrioConfig);
                                                            if !(rv as
                                                                     libc::c_int
                                                                     !=
                                                                     SECSuccess
                                                                         as
                                                                         libc::c_int)
                                                               {
                                                                if 0 == bad {
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
                                                                                    b"\t\t* Checking mu_check(PrioPacketClient_areEqual(pA, qA))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"\t\t* Checking mu_check(PrioPacketClient_areEqual(pA, qA))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        }
                                                                    }
                                                                    if PrioPacketClient_areEqual(pA
                                                                                                     as
                                                                                                     const_PrioPacketClient,
                                                                                                 qA
                                                                                                     as
                                                                                                     const_PrioPacketClient)
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
                                                                                        b"build/ptest/serial_test.c:%d: mu_check(PrioPacketClient_areEqual(pA, qA)) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        107i32);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"build/ptest/serial_test.c:%d: mu_check(PrioPacketClient_areEqual(pA, qA)) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        107i32);
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
                                                                                    b"\t\t* Checking mu_check(PrioPacketClient_areEqual(pB, qB))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"\t\t* Checking mu_check(PrioPacketClient_areEqual(pB, qB))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        }
                                                                    }
                                                                    if PrioPacketClient_areEqual(pB
                                                                                                     as
                                                                                                     const_PrioPacketClient,
                                                                                                 qB
                                                                                                     as
                                                                                                     const_PrioPacketClient)
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
                                                                                        b"build/ptest/serial_test.c:%d: mu_check(PrioPacketClient_areEqual(pB, qB)) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        108i32);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"build/ptest/serial_test.c:%d: mu_check(PrioPacketClient_areEqual(pB, qB)) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        108i32);
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
                                                                                    b"\t\t* Checking mu_check(!PrioPacketClient_areEqual(pB, qA))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"\t\t* Checking mu_check(!PrioPacketClient_areEqual(pB, qA))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        }
                                                                    }
                                                                    if !PrioPacketClient_areEqual(pB
                                                                                                      as
                                                                                                      const_PrioPacketClient,
                                                                                                  qA
                                                                                                      as
                                                                                                      const_PrioPacketClient)
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
                                                                                        b"build/ptest/serial_test.c:%d: mu_check(!PrioPacketClient_areEqual(pB, qA)) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        109i32);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"build/ptest/serial_test.c:%d: mu_check(!PrioPacketClient_areEqual(pB, qA)) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        109i32);
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
                                                                                    b"\t\t* Checking mu_check(!PrioPacketClient_areEqual(pA, qB))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"\t\t* Checking mu_check(!PrioPacketClient_areEqual(pA, qB))...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        }
                                                                    }
                                                                    if !PrioPacketClient_areEqual(pA
                                                                                                      as
                                                                                                      const_PrioPacketClient,
                                                                                                  qB
                                                                                                      as
                                                                                                      const_PrioPacketClient)
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
                                                                                        b"build/ptest/serial_test.c:%d: mu_check(!PrioPacketClient_areEqual(pA, qB)) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        110i32);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"build/ptest/serial_test.c:%d: mu_check(!PrioPacketClient_areEqual(pA, qB)) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        110i32);
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
    PrioPacketClient_clear(pA);
    PrioPacketClient_clear(pB);
    PrioPacketClient_clear(qA);
    PrioPacketClient_clear(qB);
    PrioConfig_clear(cfg);
    PrioConfig_clear(cfg2);
    msgpack_sbuffer_destroy(&mut sbufA);
    msgpack_sbuffer_destroy(&mut sbufB);
    msgpack_unpacker_destroy(&mut upkA);
    msgpack_unpacker_destroy(&mut upkB);
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(bad ? rv == SECFailure : rv == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(bad ? rv == SECFailure : rv == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if 0 !=
           if 0 != bad {
               (rv as libc::c_int == SECFailure as libc::c_int) as libc::c_int
           } else {
               (rv as libc::c_int == SECSuccess as libc::c_int) as libc::c_int
           } {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/serial_test.c:%d: mu_check(bad ? rv == SECFailure : rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 124i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/serial_test.c:%d: mu_check(bad ? rv == SECFailure : rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 124i32);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__serial_client() { serial_client(0i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test__serial_client_bad1() {
    serial_client(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__serial_client_bad2() {
    serial_client(2i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test__serial_client_bad3() {
    serial_client(3i32);
}
#[no_mangle]
pub unsafe extern "C" fn test_verify1(mut bad: libc::c_int) {
    let mut sbuf: msgpack_sbuffer =
        msgpack_sbuffer{size: 0, data: 0 as *mut libc::c_char, alloc: 0,};
    let mut pk: msgpack_packer =
        msgpack_packer{data: 0 as *mut libc::c_void, callback: None,};
    let mut upk: msgpack_unpacker =
        msgpack_unpacker{buffer: 0 as *mut libc::c_char,
                         used: 0,
                         free: 0,
                         off: 0,
                         parsed: 0,
                         z: 0 as *mut msgpack_zone,
                         initial_buffer_size: 0,
                         ctx: 0 as *mut libc::c_void,};
    let mut rv: SECStatus = SECSuccess;
    let mut v1: PrioPacketVerify1 = 0 as PrioPacketVerify1;
    let mut v2: PrioPacketVerify1 = 0 as PrioPacketVerify1;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(1))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(1))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(1i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(1)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 159i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(1)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 159i32);
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
                        b"\t\t* Checking mu_check((v = (v1 = PrioPacketVerify1_new())))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (v1 = PrioPacketVerify1_new())))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        v1 = PrioPacketVerify1_new();
        v_0 = !v1.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/serial_test.c:%d: mu_check((v = (v1 = PrioPacketVerify1_new()))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 160i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/serial_test.c:%d: mu_check((v = (v1 = PrioPacketVerify1_new()))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 160i32);
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
                            b"\t\t* Checking mu_check((v = (v2 = PrioPacketVerify1_new())))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (v2 = PrioPacketVerify1_new())))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            v2 = PrioPacketVerify1_new();
            v_1 = !v2.is_null();
            if v_1 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/serial_test.c:%d: mu_check((v = (v2 = PrioPacketVerify1_new()))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                161i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/serial_test.c:%d: mu_check((v = (v2 = PrioPacketVerify1_new()))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                161i32);
                    }
                }
            }
            if !v_1 {
                rv = SECFailure
            } else {
                mp_set(&mut (*v1).share_d, 4i32 as mp_digit);
                mp_set(&mut (*v1).share_e, 10i32 as mp_digit);
                sbuf =
                    msgpack_sbuffer{size: 0,
                                    data: 0 as *mut libc::c_char,
                                    alloc: 0,};
                pk =
                    msgpack_packer{data: 0 as *mut libc::c_void,
                                   callback: None,};
                upk =
                    msgpack_unpacker{buffer: 0 as *mut libc::c_char,
                                     used: 0,
                                     free: 0,
                                     off: 0,
                                     parsed: 0,
                                     z: 0 as *mut msgpack_zone,
                                     initial_buffer_size: 0,
                                     ctx: 0 as *mut libc::c_void,};
                msgpack_sbuffer_init(&mut sbuf);
                msgpack_packer_init(&mut pk,
                                    &mut sbuf as *mut msgpack_sbuffer as
                                        *mut libc::c_void,
                                    Some(msgpack_sbuffer_write));
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((rv = (PrioPacketVerify1_write(v1, &pk))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((rv = (PrioPacketVerify1_write(v1, &pk))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                rv =
                    PrioPacketVerify1_write(v1 as const_PrioPacketVerify1,
                                            &mut pk);
                if rv as libc::c_int == SECSuccess as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/serial_test.c:%d: mu_check((rv = (PrioPacketVerify1_write(v1, &pk))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    172i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/serial_test.c:%d: mu_check((rv = (PrioPacketVerify1_write(v1, &pk))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    172i32);
                        }
                    }
                }
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    if bad == 1i32 {
                        mp_set(&mut (*cfg).modulus, 6i32 as mp_digit);
                    }
                    let mut v_2: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upk, 0))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upk, 0))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    v_2 = msgpack_unpacker_init(&mut upk, 0i32 as size_t);
                    if v_2 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upk, 0)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 178i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upk, 0)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 178i32);
                            }
                        }
                    }
                    if v_2 {
                        let mut v_3: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        v_3 =
                            msgpack_unpacker_reserve_buffer(&mut upk,
                                                            sbuf.size);
                        if v_3 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 179i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 179i32);
                                }
                            }
                        }
                        if v_3 {
                            memcpy(msgpack_unpacker_buffer(&mut upk) as
                                       *mut libc::c_void,
                                   sbuf.data as *const libc::c_void,
                                   sbuf.size);
                            msgpack_unpacker_buffer_consumed(&mut upk,
                                                             sbuf.size);
                            rv =
                                PrioPacketVerify1_read(v2, &mut upk,
                                                       cfg as
                                                           const_PrioConfig);
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(!mp_cmp(&v1->share_d, &v2->share_d))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(!mp_cmp(&v1->share_d, &v2->share_d))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if 0 ==
                                       mp_cmp(&mut (*v1).share_d,
                                              &mut (*v2).share_d) {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp(&v1->share_d, &v2->share_d)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    185i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp(&v1->share_d, &v2->share_d)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    185i32);
                                        }
                                    }
                                }
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(!mp_cmp(&v1->share_e, &v2->share_e))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(!mp_cmp(&v1->share_e, &v2->share_e))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if 0 ==
                                       mp_cmp(&mut (*v1).share_e,
                                              &mut (*v2).share_e) {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp(&v1->share_e, &v2->share_e)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    186i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp(&v1->share_e, &v2->share_e)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    186i32);
                                        }
                                    }
                                }
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(!mp_cmp_d(&v2->share_d, 4))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(!mp_cmp_d(&v2->share_d, 4))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if 0 ==
                                       mp_cmp_d(&mut (*v2).share_d,
                                                4i32 as mp_digit) {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp_d(&v2->share_d, 4)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    187i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp_d(&v2->share_d, 4)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    187i32);
                                        }
                                    }
                                }
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(!mp_cmp_d(&v2->share_e, 10))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(!mp_cmp_d(&v2->share_e, 10))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if 0 ==
                                       mp_cmp_d(&mut (*v2).share_e,
                                                10i32 as mp_digit) {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp_d(&v2->share_e, 10)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    188i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp_d(&v2->share_e, 10)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    188i32);
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
                    b"\t\t* Checking mu_check(bad ? rv == SECFailure : rv == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(bad ? rv == SECFailure : rv == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if 0 !=
           if 0 != bad {
               (rv as libc::c_int == SECFailure as libc::c_int) as libc::c_int
           } else {
               (rv as libc::c_int == SECSuccess as libc::c_int) as libc::c_int
           } {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/serial_test.c:%d: mu_check(bad ? rv == SECFailure : rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 191i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/serial_test.c:%d: mu_check(bad ? rv == SECFailure : rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 191i32);
            }
        }
    }
    PrioConfig_clear(cfg);
    PrioPacketVerify1_clear(v1);
    PrioPacketVerify1_clear(v2);
    msgpack_unpacker_destroy(&mut upk);
    msgpack_sbuffer_destroy(&mut sbuf);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_verify1_good() { test_verify1(0i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test_verify1_bad() { test_verify1(1i32); }
#[no_mangle]
pub unsafe extern "C" fn test_verify2(mut bad: libc::c_int) {
    let mut sbuf: msgpack_sbuffer =
        msgpack_sbuffer{size: 0, data: 0 as *mut libc::c_char, alloc: 0,};
    let mut pk: msgpack_packer =
        msgpack_packer{data: 0 as *mut libc::c_void, callback: None,};
    let mut upk: msgpack_unpacker =
        msgpack_unpacker{buffer: 0 as *mut libc::c_char,
                         used: 0,
                         free: 0,
                         off: 0,
                         parsed: 0,
                         z: 0 as *mut msgpack_zone,
                         initial_buffer_size: 0,
                         ctx: 0 as *mut libc::c_void,};
    let mut rv: SECStatus = SECSuccess;
    let mut v1: PrioPacketVerify2 = 0 as PrioPacketVerify2;
    let mut v2: PrioPacketVerify2 = 0 as PrioPacketVerify2;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(1))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(1))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(1i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(1)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 219i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(1)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 219i32);
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
                        b"\t\t* Checking mu_check((v = (v1 = PrioPacketVerify2_new())))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (v1 = PrioPacketVerify2_new())))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        v1 = PrioPacketVerify2_new();
        v_0 = !v1.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/serial_test.c:%d: mu_check((v = (v1 = PrioPacketVerify2_new()))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 220i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/serial_test.c:%d: mu_check((v = (v1 = PrioPacketVerify2_new()))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 220i32);
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
                            b"\t\t* Checking mu_check((v = (v2 = PrioPacketVerify2_new())))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (v2 = PrioPacketVerify2_new())))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            v2 = PrioPacketVerify2_new();
            v_1 = !v2.is_null();
            if v_1 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/serial_test.c:%d: mu_check((v = (v2 = PrioPacketVerify2_new()))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                221i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/serial_test.c:%d: mu_check((v = (v2 = PrioPacketVerify2_new()))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                221i32);
                    }
                }
            }
            if !v_1 {
                rv = SECFailure
            } else {
                mp_set(&mut (*v1).share_out, 4i32 as mp_digit);
                sbuf =
                    msgpack_sbuffer{size: 0,
                                    data: 0 as *mut libc::c_char,
                                    alloc: 0,};
                pk =
                    msgpack_packer{data: 0 as *mut libc::c_void,
                                   callback: None,};
                upk =
                    msgpack_unpacker{buffer: 0 as *mut libc::c_char,
                                     used: 0,
                                     free: 0,
                                     off: 0,
                                     parsed: 0,
                                     z: 0 as *mut msgpack_zone,
                                     initial_buffer_size: 0,
                                     ctx: 0 as *mut libc::c_void,};
                msgpack_sbuffer_init(&mut sbuf);
                msgpack_packer_init(&mut pk,
                                    &mut sbuf as *mut msgpack_sbuffer as
                                        *mut libc::c_void,
                                    Some(msgpack_sbuffer_write));
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((rv = (PrioPacketVerify2_write(v1, &pk))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((rv = (PrioPacketVerify2_write(v1, &pk))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                rv =
                    PrioPacketVerify2_write(v1 as const_PrioPacketVerify2,
                                            &mut pk);
                if rv as libc::c_int == SECSuccess as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/serial_test.c:%d: mu_check((rv = (PrioPacketVerify2_write(v1, &pk))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    231i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/serial_test.c:%d: mu_check((rv = (PrioPacketVerify2_write(v1, &pk))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    231i32);
                        }
                    }
                }
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    if bad == 1i32 {
                        mp_set(&mut (*cfg).modulus, 4i32 as mp_digit);
                    }
                    let mut v_2: bool = false;
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upk, 0))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upk, 0))))...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    v_2 = msgpack_unpacker_init(&mut upk, 0i32 as size_t);
                    if v_2 {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upk, 0)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 237i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upk, 0)))) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 237i32);
                            }
                        }
                    }
                    if v_2 {
                        let mut v_3: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        v_3 =
                            msgpack_unpacker_reserve_buffer(&mut upk,
                                                            sbuf.size);
                        if v_3 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 238i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 238i32);
                                }
                            }
                        }
                        if v_3 {
                            memcpy(msgpack_unpacker_buffer(&mut upk) as
                                       *mut libc::c_void,
                                   sbuf.data as *const libc::c_void,
                                   sbuf.size);
                            msgpack_unpacker_buffer_consumed(&mut upk,
                                                             sbuf.size);
                            rv =
                                PrioPacketVerify2_read(v2, &mut upk,
                                                       cfg as
                                                           const_PrioConfig);
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(!mp_cmp(&v1->share_out, &v2->share_out))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(!mp_cmp(&v1->share_out, &v2->share_out))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if 0 ==
                                       mp_cmp(&mut (*v1).share_out,
                                              &mut (*v2).share_out) {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp(&v1->share_out, &v2->share_out)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    244i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp(&v1->share_out, &v2->share_out)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    244i32);
                                        }
                                    }
                                }
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(!mp_cmp_d(&v2->share_out, 4))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(!mp_cmp_d(&v2->share_out, 4))...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if 0 ==
                                       mp_cmp_d(&mut (*v2).share_out,
                                                4i32 as mp_digit) {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp_d(&v2->share_out, 4)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    245i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/serial_test.c:%d: mu_check(!mp_cmp_d(&v2->share_out, 4)) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    245i32);
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
                    b"\t\t* Checking mu_check(bad ? rv == SECFailure : rv == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(bad ? rv == SECFailure : rv == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if 0 !=
           if 0 != bad {
               (rv as libc::c_int == SECFailure as libc::c_int) as libc::c_int
           } else {
               (rv as libc::c_int == SECSuccess as libc::c_int) as libc::c_int
           } {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/serial_test.c:%d: mu_check(bad ? rv == SECFailure : rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 248i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/serial_test.c:%d: mu_check(bad ? rv == SECFailure : rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 248i32);
            }
        }
    }
    PrioConfig_clear(cfg);
    PrioPacketVerify2_clear(v1);
    PrioPacketVerify2_clear(v2);
    msgpack_unpacker_destroy(&mut upk);
    msgpack_sbuffer_destroy(&mut sbuf);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_verify2_good() { test_verify2(0i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test_verify2_bad() { test_verify2(1i32); }
#[no_mangle]
pub unsafe extern "C" fn test_total_share(mut bad: libc::c_int) {
    let mut sbuf: msgpack_sbuffer =
        msgpack_sbuffer{size: 0, data: 0 as *mut libc::c_char, alloc: 0,};
    let mut pk: msgpack_packer =
        msgpack_packer{data: 0 as *mut libc::c_void, callback: None,};
    let mut upk: msgpack_unpacker =
        msgpack_unpacker{buffer: 0 as *mut libc::c_char,
                         used: 0,
                         free: 0,
                         off: 0,
                         parsed: 0,
                         z: 0 as *mut msgpack_zone,
                         initial_buffer_size: 0,
                         ctx: 0 as *mut libc::c_void,};
    let mut rv: SECStatus = SECSuccess;
    let mut t1: PrioTotalShare = 0 as PrioTotalShare;
    let mut t2: PrioTotalShare = 0 as PrioTotalShare;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest((bad == 2 ? 4 : 3)))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest((bad == 2 ? 4 : 3)))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(if bad == 2i32 { 4i32 } else { 3i32 });
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest((bad == 2 ? 4 : 3))))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 276i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/serial_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest((bad == 2 ? 4 : 3))))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 276i32);
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
                        b"\t\t* Checking mu_check((v = (t1 = PrioTotalShare_new())))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (t1 = PrioTotalShare_new())))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        t1 = PrioTotalShare_new();
        v_0 = !t1.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/serial_test.c:%d: mu_check((v = (t1 = PrioTotalShare_new()))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 277i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/serial_test.c:%d: mu_check((v = (t1 = PrioTotalShare_new()))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 277i32);
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
                            b"\t\t* Checking mu_check((v = (t2 = PrioTotalShare_new())))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (t2 = PrioTotalShare_new())))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            t2 = PrioTotalShare_new();
            v_1 = !t2.is_null();
            if v_1 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/serial_test.c:%d: mu_check((v = (t2 = PrioTotalShare_new()))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                278i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/serial_test.c:%d: mu_check((v = (t2 = PrioTotalShare_new()))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                278i32);
                    }
                }
            }
            if !v_1 {
                rv = SECFailure
            } else {
                (*t1).idx = PRIO_SERVER_A;
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((rv = (MPArray_resize(t1->data_shares, 3))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((rv = (MPArray_resize(t1->data_shares, 3))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                rv = MPArray_resize((*t1).data_shares, 3i32);
                if rv as libc::c_int == SECSuccess as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/serial_test.c:%d: mu_check((rv = (MPArray_resize(t1->data_shares, 3))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    281i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/serial_test.c:%d: mu_check((rv = (MPArray_resize(t1->data_shares, 3))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    281i32);
                        }
                    }
                }
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    mp_set(&mut *(*(*t1).data_shares).data.offset(0isize),
                           10i32 as mp_digit);
                    mp_set(&mut *(*(*t1).data_shares).data.offset(1isize),
                           20i32 as mp_digit);
                    mp_set(&mut *(*(*t1).data_shares).data.offset(2isize),
                           30i32 as mp_digit);
                    sbuf =
                        msgpack_sbuffer{size: 0,
                                        data: 0 as *mut libc::c_char,
                                        alloc: 0,};
                    pk =
                        msgpack_packer{data: 0 as *mut libc::c_void,
                                       callback: None,};
                    upk =
                        msgpack_unpacker{buffer: 0 as *mut libc::c_char,
                                         used: 0,
                                         free: 0,
                                         off: 0,
                                         parsed: 0,
                                         z: 0 as *mut msgpack_zone,
                                         initial_buffer_size: 0,
                                         ctx: 0 as *mut libc::c_void,};
                    msgpack_sbuffer_init(&mut sbuf);
                    msgpack_packer_init(&mut pk,
                                        &mut sbuf as *mut msgpack_sbuffer as
                                            *mut libc::c_void,
                                        Some(msgpack_sbuffer_write));
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((rv = (PrioTotalShare_write(t1, &pk))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((rv = (PrioTotalShare_write(t1, &pk))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    rv =
                        PrioTotalShare_write(t1 as const_PrioTotalShare,
                                             &mut pk);
                    if rv as libc::c_int == SECSuccess as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/serial_test.c:%d: mu_check((rv = (PrioTotalShare_write(t1, &pk))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 294i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/serial_test.c:%d: mu_check((rv = (PrioTotalShare_write(t1, &pk))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 294i32);
                            }
                        }
                    }
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        if bad == 1i32 {
                            mp_set(&mut (*cfg).modulus, 4i32 as mp_digit);
                        }
                        let mut v_2: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upk, 0))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = (msgpack_unpacker_init(&upk, 0))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        v_2 = msgpack_unpacker_init(&mut upk, 0i32 as size_t);
                        if v_2 {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upk, 0)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 300i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_init(&upk, 0)))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 300i32);
                                }
                            }
                        }
                        if v_2 {
                            let mut v_3: bool = false;
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size))))...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size))))...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            v_3 =
                                msgpack_unpacker_reserve_buffer(&mut upk,
                                                                sbuf.size);
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
                                                b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                301i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/serial_test.c:%d: mu_check((v = (msgpack_unpacker_reserve_buffer(&upk, sbuf.size)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                301i32);
                                    }
                                }
                            }
                            if v_3 {
                                memcpy(msgpack_unpacker_buffer(&mut upk) as
                                           *mut libc::c_void,
                                       sbuf.data as *const libc::c_void,
                                       sbuf.size);
                                msgpack_unpacker_buffer_consumed(&mut upk,
                                                                 sbuf.size);
                                rv =
                                    PrioTotalShare_read(t2, &mut upk,
                                                        cfg as
                                                            const_PrioConfig);
                                if !(rv as libc::c_int !=
                                         SECSuccess as libc::c_int) {
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check(t1->idx == t2->idx)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(t1->idx == t2->idx)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if (*t1).idx as libc::c_uint ==
                                           (*t2).idx as libc::c_uint {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/serial_test.c:%d: mu_check(t1->idx == t2->idx) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        307i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/serial_test.c:%d: mu_check(t1->idx == t2->idx) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        307i32);
                                            }
                                        }
                                    }
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check(MPArray_areEqual(t1->data_shares, t2->data_shares))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(MPArray_areEqual(t1->data_shares, t2->data_shares))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if MPArray_areEqual((*t1).data_shares as
                                                            const_MPArray,
                                                        (*t2).data_shares as
                                                            const_MPArray) {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/serial_test.c:%d: mu_check(MPArray_areEqual(t1->data_shares, t2->data_shares)) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        308i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/serial_test.c:%d: mu_check(MPArray_areEqual(t1->data_shares, t2->data_shares)) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        308i32);
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
                    b"\t\t* Checking mu_check(bad ? rv == SECFailure : rv == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(bad ? rv == SECFailure : rv == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if 0 !=
           if 0 != bad {
               (rv as libc::c_int == SECFailure as libc::c_int) as libc::c_int
           } else {
               (rv as libc::c_int == SECSuccess as libc::c_int) as libc::c_int
           } {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/serial_test.c:%d: mu_check(bad ? rv == SECFailure : rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 311i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/serial_test.c:%d: mu_check(bad ? rv == SECFailure : rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 311i32);
            }
        }
    }
    PrioConfig_clear(cfg);
    PrioTotalShare_clear(t1);
    PrioTotalShare_clear(t2);
    msgpack_unpacker_destroy(&mut upk);
    msgpack_sbuffer_destroy(&mut sbuf);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_total_good() { test_total_share(0i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test_total_bad1() { test_total_share(1i32); }
#[no_mangle]
pub unsafe extern "C" fn mu_test_total_bad2() { test_total_share(2i32); }