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
    pub type PK11SlotInfoStr;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn msgpack_zone_free(zone: *mut msgpack_zone);
    #[no_mangle]
    fn msgpack_unpacker_next(mpac: *mut msgpack_unpacker,
                             pac: *mut msgpack_unpacked)
     -> msgpack_unpack_return;
    #[no_mangle]
    fn mp_unsigned_octet_size(mp: *const mp_int) -> libc::c_uint;
    #[no_mangle]
    fn mp_to_fixlen_octets(mp: *const mp_int, str: *mut libc::c_uchar,
                           len: mp_size) -> mp_err;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    /* MP_MODARITH */
    /* Comparisons             */
    #[no_mangle]
    fn mp_cmp_z(a: *const mp_int) -> libc::c_int;
    /* Octet string conversion functions */
    #[no_mangle]
    fn mp_read_unsigned_octets(mp: *mut mp_int, str: *const libc::c_uchar,
                               len: mp_size) -> mp_err;
    /*
 * Expands or shrinks the MPArray to the desired size. If shrinking,
 * will clear the values on the end of array.
 */
    #[no_mangle]
    fn MPArray_resize(arr: MPArray, newlen: libc::c_int) -> SECStatus;
    #[no_mangle]
    fn PrioConfig_hPoints(cfg: const_PrioConfig) -> libc::c_int;
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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type int8_t = libc::c_schar;
pub type int16_t = libc::c_short;
pub type int32_t = libc::c_int;
pub type int64_t = libc::c_longlong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
pub type msgpack_object_type = libc::c_uint;
pub const MSGPACK_OBJECT_EXT: msgpack_object_type = 9;
pub const MSGPACK_OBJECT_BIN: msgpack_object_type = 8;
pub const MSGPACK_OBJECT_MAP: msgpack_object_type = 7;
pub const MSGPACK_OBJECT_ARRAY: msgpack_object_type = 6;
pub const MSGPACK_OBJECT_STR: msgpack_object_type = 5;
pub const MSGPACK_OBJECT_FLOAT: msgpack_object_type = 4;
pub const MSGPACK_OBJECT_FLOAT64: msgpack_object_type = 4;
pub const MSGPACK_OBJECT_FLOAT32: msgpack_object_type = 10;
pub const MSGPACK_OBJECT_NEGATIVE_INTEGER: msgpack_object_type = 3;
pub const MSGPACK_OBJECT_POSITIVE_INTEGER: msgpack_object_type = 2;
pub const MSGPACK_OBJECT_BOOLEAN: msgpack_object_type = 1;
pub const MSGPACK_OBJECT_NIL: msgpack_object_type = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_object {
    pub type_0: msgpack_object_type,
    pub via: msgpack_object_union,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union msgpack_object_union {
    pub boolean: bool,
    pub u64_0: uint64_t,
    pub i64_0: int64_t,
    pub f64_0: libc::c_double,
    pub array: msgpack_object_array,
    pub map: msgpack_object_map,
    pub str_0: msgpack_object_str,
    pub bin: msgpack_object_bin,
    pub ext: msgpack_object_ext,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_object_ext {
    pub type_0: int8_t,
    pub size: uint32_t,
    pub ptr: *const libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_object_bin {
    pub size: uint32_t,
    pub ptr: *const libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_object_str {
    pub size: uint32_t,
    pub ptr: *const libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_object_map {
    pub size: uint32_t,
    pub ptr: *mut msgpack_object_kv,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_object_kv {
    pub key: msgpack_object,
    pub val: msgpack_object,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct msgpack_object_array {
    pub size: uint32_t,
    pub ptr: *mut msgpack_object,
}
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
pub struct msgpack_unpacked {
    pub zone: *mut msgpack_zone,
    pub data: msgpack_object,
}
pub type msgpack_unpack_return = libc::c_int;
pub const MSGPACK_UNPACK_NOMEM_ERROR: msgpack_unpack_return = -2;
pub const MSGPACK_UNPACK_PARSE_ERROR: msgpack_unpack_return = -1;
pub const MSGPACK_UNPACK_CONTINUE: msgpack_unpack_return = 0;
pub const MSGPACK_UNPACK_EXTRA_BYTES: msgpack_unpack_return = 1;
pub const MSGPACK_UNPACK_SUCCESS: msgpack_unpack_return = 2;
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
pub type mp_err = libc::c_int;
pub type const_MPArray = *const mparray;
pub type const_BeaverTriple = *const beaver_triple;
pub type const_PrioPacketClient = *const prio_packet_client;
unsafe extern "C" fn _OSSwapInt16(mut _data: __uint16_t) -> __uint16_t {
    return ((_data as libc::c_int) << 8i32 | _data as libc::c_int >> 8i32) as
               __uint16_t;
}
unsafe extern "C" fn _OSSwapInt32(mut _data: __uint32_t) -> __uint32_t {
    return _data.swap_bytes();
}
unsafe extern "C" fn msgpack_pack_int(mut x: *mut msgpack_packer,
                                      mut d: libc::c_int) -> libc::c_int {
    if d < -(1i32 << 5i32) {
        if d < -(1i32 << 15i32) {
            let mut buf: [libc::c_uchar; 5] = [0; 5];
            buf[0usize] = 0xd2i32 as libc::c_uchar;
            let mut val: uint32_t =
                if 0 != 0 {
                    (d as __uint32_t & 0xff000000u32) >> 24i32 |
                        (d as __uint32_t & 0xff0000i32 as libc::c_uint) >>
                            8i32 |
                        (d as __uint32_t & 0xff00i32 as libc::c_uint) << 8i32
                        | (d as __uint32_t & 0xffi32 as libc::c_uint) << 24i32
                } else { _OSSwapInt32(d as __uint32_t) };
            memcpy(&mut buf[1usize] as *mut libc::c_uchar as
                       *mut libc::c_void,
                   &mut val as *mut uint32_t as *const libc::c_void,
                   4i32 as libc::c_ulong);
            return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                     buf.as_mut_ptr()
                                                                         as
                                                                         *const libc::c_char,
                                                                     5i32 as
                                                                         size_t)
        } else if d < -(1i32 << 7i32) {
            let mut buf_0: [libc::c_uchar; 3] = [0; 3];
            buf_0[0usize] = 0xd1i32 as libc::c_uchar;
            let mut val_0: uint16_t =
                (if 0 != 0 {
                     ((d as int16_t as __uint16_t as libc::c_int & 0xff00i32)
                          >> 8i32 |
                          (d as int16_t as __uint16_t as libc::c_int &
                               0xffi32) << 8i32) as __uint16_t as libc::c_int
                 } else {
                     _OSSwapInt16(d as int16_t as __uint16_t) as libc::c_int
                 }) as __uint16_t;
            memcpy(&mut buf_0[1usize] as *mut libc::c_uchar as
                       *mut libc::c_void,
                   &mut val_0 as *mut uint16_t as *const libc::c_void,
                   2i32 as libc::c_ulong);
            return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                     buf_0.as_mut_ptr()
                                                                         as
                                                                         *const libc::c_char,
                                                                     3i32 as
                                                                         size_t)
        } else {
            let mut buf_1: [libc::c_uchar; 2] =
                [0xd0i32 as libc::c_uchar,
                 *(&mut d as *mut libc::c_int as
                       *mut uint8_t).offset(0isize)];
            return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                     buf_1.as_mut_ptr()
                                                                         as
                                                                         *const libc::c_char,
                                                                     2i32 as
                                                                         size_t)
        }
    } else if d < 1i32 << 7i32 {
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 &mut *(&mut d
                                                                            as
                                                                            *mut libc::c_int
                                                                            as
                                                                            *mut uint8_t).offset(0isize)
                                                                     as
                                                                     *mut uint8_t
                                                                     as
                                                                     *const libc::c_char,
                                                                 1i32 as
                                                                     size_t)
    } else if d < 1i32 << 8i32 {
        let mut buf_2: [libc::c_uchar; 2] =
            [0xcci32 as libc::c_uchar,
             *(&mut d as *mut libc::c_int as *mut uint8_t).offset(0isize)];
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 buf_2.as_mut_ptr()
                                                                     as
                                                                     *const libc::c_char,
                                                                 2i32 as
                                                                     size_t)
    } else if d < 1i32 << 16i32 {
        let mut buf_3: [libc::c_uchar; 3] = [0; 3];
        buf_3[0usize] = 0xcdi32 as libc::c_uchar;
        let mut val_1: uint16_t =
            (if 0 != 0 {
                 ((d as uint16_t as libc::c_int & 0xff00i32) >> 8i32 |
                      (d as uint16_t as libc::c_int & 0xffi32) << 8i32) as
                     __uint16_t as libc::c_int
             } else { _OSSwapInt16(d as uint16_t) as libc::c_int }) as
                __uint16_t;
        memcpy(&mut buf_3[1usize] as *mut libc::c_uchar as *mut libc::c_void,
               &mut val_1 as *mut uint16_t as *const libc::c_void,
               2i32 as libc::c_ulong);
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 buf_3.as_mut_ptr()
                                                                     as
                                                                     *const libc::c_char,
                                                                 3i32 as
                                                                     size_t)
    } else {
        let mut buf_4: [libc::c_uchar; 5] = [0; 5];
        buf_4[0usize] = 0xcei32 as libc::c_uchar;
        let mut val_2: uint32_t =
            if 0 != 0 {
                (d as uint32_t & 0xff000000u32) >> 24i32 |
                    (d as uint32_t & 0xff0000i32 as libc::c_uint) >> 8i32 |
                    (d as uint32_t & 0xff00i32 as libc::c_uint) << 8i32 |
                    (d as uint32_t & 0xffi32 as libc::c_uint) << 24i32
            } else { _OSSwapInt32(d as uint32_t) };
        memcpy(&mut buf_4[1usize] as *mut libc::c_uchar as *mut libc::c_void,
               &mut val_2 as *mut uint32_t as *const libc::c_void,
               4i32 as libc::c_ulong);
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 buf_4.as_mut_ptr()
                                                                     as
                                                                     *const libc::c_char,
                                                                 5i32 as
                                                                     size_t)
    };
}
unsafe extern "C" fn msgpack_pack_array(mut x: *mut msgpack_packer,
                                        mut n: size_t) -> libc::c_int {
    if n < 16i32 as libc::c_ulong {
        let mut d: libc::c_uchar =
            (0x90i32 | n as uint8_t as libc::c_int) as libc::c_uchar;
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 &mut d as
                                                                     *mut libc::c_uchar
                                                                     as
                                                                     *const libc::c_char,
                                                                 1i32 as
                                                                     size_t)
    } else if n < 65536i32 as libc::c_ulong {
        let mut buf: [libc::c_uchar; 3] = [0; 3];
        buf[0usize] = 0xdci32 as libc::c_uchar;
        let mut val: uint16_t =
            (if 0 != 0 {
                 ((n as uint16_t as libc::c_int & 0xff00i32) >> 8i32 |
                      (n as uint16_t as libc::c_int & 0xffi32) << 8i32) as
                     __uint16_t as libc::c_int
             } else { _OSSwapInt16(n as uint16_t) as libc::c_int }) as
                __uint16_t;
        memcpy(&mut buf[1usize] as *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint16_t as *const libc::c_void,
               2i32 as libc::c_ulong);
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 buf.as_mut_ptr()
                                                                     as
                                                                     *const libc::c_char,
                                                                 3i32 as
                                                                     size_t)
    } else {
        let mut buf_0: [libc::c_uchar; 5] = [0; 5];
        buf_0[0usize] = 0xddi32 as libc::c_uchar;
        let mut val_0: uint32_t =
            if 0 != 0 {
                (n as uint32_t & 0xff000000u32) >> 24i32 |
                    (n as uint32_t & 0xff0000i32 as libc::c_uint) >> 8i32 |
                    (n as uint32_t & 0xff00i32 as libc::c_uint) << 8i32 |
                    (n as uint32_t & 0xffi32 as libc::c_uint) << 24i32
            } else { _OSSwapInt32(n as uint32_t) };
        memcpy(&mut buf_0[1usize] as *mut libc::c_uchar as *mut libc::c_void,
               &mut val_0 as *mut uint32_t as *const libc::c_void,
               4i32 as libc::c_ulong);
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 buf_0.as_mut_ptr()
                                                                     as
                                                                     *const libc::c_char,
                                                                 5i32 as
                                                                     size_t)
    };
}
unsafe extern "C" fn msgpack_pack_str(mut x: *mut msgpack_packer,
                                      mut l: size_t) -> libc::c_int {
    if l < 32i32 as libc::c_ulong {
        let mut d: libc::c_uchar =
            (0xa0i32 | l as uint8_t as libc::c_int) as libc::c_uchar;
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 &mut *(&mut d
                                                                            as
                                                                            *mut libc::c_uchar).offset(0isize)
                                                                     as
                                                                     *mut uint8_t
                                                                     as
                                                                     *const libc::c_char,
                                                                 1i32 as
                                                                     size_t)
    } else if l < 256i32 as libc::c_ulong {
        let mut buf: [libc::c_uchar; 2] = [0; 2];
        buf[0usize] = 0xd9i32 as libc::c_uchar;
        buf[1usize] = l as uint8_t;
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 buf.as_mut_ptr()
                                                                     as
                                                                     *const libc::c_char,
                                                                 2i32 as
                                                                     size_t)
    } else if l < 65536i32 as libc::c_ulong {
        let mut buf_0: [libc::c_uchar; 3] = [0; 3];
        buf_0[0usize] = 0xdai32 as libc::c_uchar;
        let mut val: uint16_t =
            (if 0 != 0 {
                 ((l as uint16_t as libc::c_int & 0xff00i32) >> 8i32 |
                      (l as uint16_t as libc::c_int & 0xffi32) << 8i32) as
                     __uint16_t as libc::c_int
             } else { _OSSwapInt16(l as uint16_t) as libc::c_int }) as
                __uint16_t;
        memcpy(&mut buf_0[1usize] as *mut libc::c_uchar as *mut libc::c_void,
               &mut val as *mut uint16_t as *const libc::c_void,
               2i32 as libc::c_ulong);
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 buf_0.as_mut_ptr()
                                                                     as
                                                                     *const libc::c_char,
                                                                 3i32 as
                                                                     size_t)
    } else {
        let mut buf_1: [libc::c_uchar; 5] = [0; 5];
        buf_1[0usize] = 0xdbi32 as libc::c_uchar;
        let mut val_0: uint32_t =
            if 0 != 0 {
                (l as uint32_t & 0xff000000u32) >> 24i32 |
                    (l as uint32_t & 0xff0000i32 as libc::c_uint) >> 8i32 |
                    (l as uint32_t & 0xff00i32 as libc::c_uint) << 8i32 |
                    (l as uint32_t & 0xffi32 as libc::c_uint) << 24i32
            } else { _OSSwapInt32(l as uint32_t) };
        memcpy(&mut buf_1[1usize] as *mut libc::c_uchar as *mut libc::c_void,
               &mut val_0 as *mut uint32_t as *const libc::c_void,
               4i32 as libc::c_ulong);
        return (*x).callback.expect("non-null function pointer")((*x).data,
                                                                 buf_1.as_mut_ptr()
                                                                     as
                                                                     *const libc::c_char,
                                                                 5i32 as
                                                                     size_t)
    };
}
unsafe extern "C" fn msgpack_pack_str_body(mut x: *mut msgpack_packer,
                                           mut b: *const libc::c_void,
                                           mut l: size_t) -> libc::c_int {
    return (*x).callback.expect("non-null function pointer")((*x).data,
                                                             b as
                                                                 *const libc::c_uchar
                                                                 as
                                                                 *const libc::c_char,
                                                             l);
}
unsafe extern "C" fn msgpack_unpacked_init(mut result:
                                               *mut msgpack_unpacked) {
    memset(result as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<msgpack_unpacked>() as libc::c_ulong);
}
unsafe extern "C" fn msgpack_unpacked_destroy(mut result:
                                                  *mut msgpack_unpacked) {
    if !(*result).zone.is_null() {
        msgpack_zone_free((*result).zone);
        (*result).zone = 0 as *mut msgpack_zone;
        memset(&mut (*result).data as *mut msgpack_object as
                   *mut libc::c_void, 0i32,
               ::std::mem::size_of::<msgpack_object>() as libc::c_ulong);
    };
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify1_write(mut p:
                                                     const_PrioPacketVerify1,
                                                 mut pk: *mut msgpack_packer)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if pk.is_null() {
        rv = SECFailure
    } else if p.is_null() {
        rv = SECFailure
    } else {
        rv = serial_write_mp_int(pk, &(*p).share_d);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_write_mp_int(pk, &(*p).share_e);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
unsafe extern "C" fn serial_write_mp_int(mut pk: *mut msgpack_packer,
                                         mut n: *const mp_int) -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let mut n_size: libc::c_uint = mp_unsigned_octet_size(n);
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    data =
        calloc(n_size as libc::c_ulong,
               ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong) as
            *mut libc::c_uchar;
    if data.is_null() {
        rv = SECFailure
    } else if mp_to_fixlen_octets(n, data, n_size) != 0i32 {
        rv = SECFailure
    } else {
        rv = msgpack_pack_str(pk, n_size as size_t) as SECStatus;
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            rv =
                msgpack_pack_str_body(pk, data as *const libc::c_void,
                                      n_size as size_t) as SECStatus;
            rv as libc::c_int != SECSuccess as libc::c_int;
        }
    }
    if !data.is_null() { free(data as *mut libc::c_void); }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify1_read(mut p: PrioPacketVerify1,
                                                mut upk:
                                                    *mut msgpack_unpacker,
                                                mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if upk.is_null() {
        rv = SECFailure
    } else if p.is_null() {
        rv = SECFailure
    } else {
        rv = serial_read_mp_int(upk, &mut (*p).share_d, &(*cfg).modulus);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_read_mp_int(upk, &mut (*p).share_e, &(*cfg).modulus);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
unsafe extern "C" fn serial_read_mp_int(mut upk: *mut msgpack_unpacker,
                                        mut n: *mut mp_int,
                                        mut max: *const mp_int) -> SECStatus {
    let mut obj: msgpack_object =
        msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                       via: msgpack_object_union{boolean: false,},};
    let mut rv: SECStatus = SECSuccess;
    let mut res: msgpack_unpacked =
        msgpack_unpacked{zone: 0 as *mut msgpack_zone,
                         data:
                             msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                                            via:
                                                msgpack_object_union{boolean:
                                                                         false,},},};
    msgpack_unpacked_init(&mut res);
    if upk.is_null() {
        rv = SECFailure
    } else if n.is_null() {
        rv = SECFailure
    } else if max.is_null() {
        rv = SECFailure
    } else {
        let mut r: libc::c_int =
            msgpack_unpacker_next(upk, &mut res) as libc::c_int;
        if r != MSGPACK_UNPACK_SUCCESS as libc::c_int &&
               r != MSGPACK_UNPACK_EXTRA_BYTES as libc::c_int {
            return SECFailure
        }
        obj = res.data;
        rv = object_to_mp_int(&mut obj, n, max);
        rv as libc::c_int != SECSuccess as libc::c_int;
    }
    msgpack_unpacked_destroy(&mut res);
    return rv;
}
unsafe extern "C" fn object_to_mp_int(mut obj: *mut msgpack_object,
                                      mut n: *mut mp_int,
                                      mut max: *const mp_int) -> SECStatus {
    let mut s: msgpack_object_str =
        msgpack_object_str{size: 0, ptr: 0 as *const libc::c_char,};
    let mut rv: SECStatus = SECSuccess;
    if obj.is_null() {
        rv = SECFailure
    } else if !((*obj).type_0 as libc::c_uint ==
                    MSGPACK_OBJECT_STR as libc::c_int as libc::c_uint) {
        rv = SECFailure
    } else if n.is_null() {
        rv = SECFailure
    } else {
        s = (*obj).via.str_0;
        if s.ptr.is_null() {
            rv = SECFailure
        } else if mp_read_unsigned_octets(n, s.ptr as *mut libc::c_uchar,
                                          s.size) != 0i32 {
            rv = SECFailure
        } else if !(mp_cmp_z(n) >= 0i32) {
            rv = SECFailure
        } else if !(mp_cmp(n, max) < 0i32) { rv = SECFailure }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify2_write(mut p:
                                                     const_PrioPacketVerify2,
                                                 mut pk: *mut msgpack_packer)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if pk.is_null() {
        rv = SECFailure
    } else if p.is_null() {
        rv = SECFailure
    } else {
        rv = serial_write_mp_int(pk, &(*p).share_out);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketVerify2_read(mut p: PrioPacketVerify2,
                                                mut upk:
                                                    *mut msgpack_unpacker,
                                                mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if upk.is_null() {
        rv = SECFailure
    } else if p.is_null() {
        rv = SECFailure
    } else {
        rv = serial_read_mp_int(upk, &mut (*p).share_out, &(*cfg).modulus);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn PrioTotalShare_write(mut t: const_PrioTotalShare,
                                              mut pk: *mut msgpack_packer)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if t.is_null() {
        rv = SECFailure
    } else if pk.is_null() {
        rv = SECFailure
    } else {
        rv = msgpack_pack_int(pk, (*t).idx as libc::c_int) as SECStatus;
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_write_mp_array(pk, (*t).data_shares as const_MPArray);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
unsafe extern "C" fn serial_write_mp_array(mut pk: *mut msgpack_packer,
                                           mut arr: const_MPArray)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if pk.is_null() {
        rv = SECFailure
    } else if arr.is_null() {
        rv = SECFailure
    } else {
        rv = msgpack_pack_array(pk, (*arr).len as size_t) as SECStatus;
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        let mut i: libc::c_int = 0i32;
        while i < (*arr).len {
            rv =
                serial_write_mp_int(pk, &mut *(*arr).data.offset(i as isize));
            if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
            i += 1
        }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn PrioTotalShare_read(mut t: PrioTotalShare,
                                             mut upk: *mut msgpack_unpacker,
                                             mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if t.is_null() {
        rv = SECFailure
    } else if upk.is_null() {
        rv = SECFailure
    } else {
        rv = serial_read_server_id(upk, &mut (*t).idx);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv =
            serial_read_mp_array(upk, (*t).data_shares,
                                 (*cfg).num_data_fields as size_t,
                                 &(*cfg).modulus);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
unsafe extern "C" fn serial_read_mp_array(mut upk: *mut msgpack_unpacker,
                                          mut arr: MPArray, mut len: size_t,
                                          mut max: *const mp_int)
 -> SECStatus {
    let mut obj: msgpack_object =
        msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                       via: msgpack_object_union{boolean: false,},};
    let mut objarr: msgpack_object_array =
        msgpack_object_array{size: 0, ptr: 0 as *mut msgpack_object,};
    let mut rv: SECStatus = SECSuccess;
    let mut res: msgpack_unpacked =
        msgpack_unpacked{zone: 0 as *mut msgpack_zone,
                         data:
                             msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                                            via:
                                                msgpack_object_union{boolean:
                                                                         false,},},};
    msgpack_unpacked_init(&mut res);
    if upk.is_null() {
        rv = SECFailure
    } else if arr.is_null() {
        rv = SECFailure
    } else if max.is_null() {
        rv = SECFailure
    } else {
        let mut r: libc::c_int =
            msgpack_unpacker_next(upk, &mut res) as libc::c_int;
        if r != MSGPACK_UNPACK_SUCCESS as libc::c_int &&
               r != MSGPACK_UNPACK_EXTRA_BYTES as libc::c_int {
            return SECFailure
        }
        obj = res.data;
        if !(obj.type_0 as libc::c_uint ==
                 MSGPACK_OBJECT_ARRAY as libc::c_int as libc::c_uint) {
            rv = SECFailure
        } else {
            objarr = obj.via.array;
            if !(objarr.size as libc::c_ulong == len) {
                rv = SECFailure
            } else {
                rv = MPArray_resize(arr, len as libc::c_int);
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    let mut i: libc::c_uint = 0i32 as libc::c_uint;
                    while (i as libc::c_ulong) < len {
                        rv =
                            object_to_mp_int(&mut *objarr.ptr.offset(i as
                                                                         isize),
                                             &mut *(*arr).data.offset(i as
                                                                          isize),
                                             max);
                        if rv as libc::c_int != SECSuccess as libc::c_int {
                            break ;
                        }
                        i = i.wrapping_add(1)
                    }
                }
            }
        }
    }
    msgpack_unpacked_destroy(&mut res);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn serial_read_server_id(mut upk: *mut msgpack_unpacker,
                                               mut s: *mut PrioServerId)
 -> SECStatus {
    let mut serv: libc::c_int = 0;
    let mut rv: SECStatus = SECSuccess;
    if upk.is_null() {
        rv = SECFailure
    } else if s.is_null() {
        rv = SECFailure
    } else {
        serv = 0;
        rv = serial_read_int(upk, &mut serv);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        if !(serv == PRIO_SERVER_A as libc::c_int ||
                 serv == PRIO_SERVER_B as libc::c_int) {
            rv = SECFailure
        } else { *s = serv as PrioServerId }
    }
    return rv;
}
unsafe extern "C" fn serial_read_int(mut upk: *mut msgpack_unpacker,
                                     mut n: *mut libc::c_int) -> SECStatus {
    let mut obj: msgpack_object =
        msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                       via: msgpack_object_union{boolean: false,},};
    let mut rv: SECStatus = SECSuccess;
    let mut res: msgpack_unpacked =
        msgpack_unpacked{zone: 0 as *mut msgpack_zone,
                         data:
                             msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                                            via:
                                                msgpack_object_union{boolean:
                                                                         false,},},};
    msgpack_unpacked_init(&mut res);
    if upk.is_null() {
        rv = SECFailure
    } else if n.is_null() {
        rv = SECFailure
    } else {
        let mut r: libc::c_int =
            msgpack_unpacker_next(upk, &mut res) as libc::c_int;
        if r != MSGPACK_UNPACK_SUCCESS as libc::c_int &&
               r != MSGPACK_UNPACK_EXTRA_BYTES as libc::c_int {
            return SECFailure
        }
        obj = res.data;
        if !(obj.type_0 as libc::c_uint ==
                 MSGPACK_OBJECT_POSITIVE_INTEGER as libc::c_int as
                     libc::c_uint) {
            rv = SECFailure
        } else { *n = obj.via.i64_0 as libc::c_int }
    }
    msgpack_unpacked_destroy(&mut res);
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
pub unsafe extern "C" fn serial_write_packet_client(mut pk:
                                                        *mut msgpack_packer,
                                                    mut p:
                                                        const_PrioPacketClient,
                                                    mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if pk.is_null() {
        rv = SECFailure
    } else if p.is_null() {
        rv = SECFailure
    } else {
        rv = msgpack_pack_str(pk, (*cfg).batch_id_len as size_t) as SECStatus;
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv =
            msgpack_pack_str_body(pk, (*cfg).batch_id as *const libc::c_void,
                                  (*cfg).batch_id_len as size_t) as SECStatus;
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv =
            serial_write_beaver_triple(pk, (*p).triple as const_BeaverTriple);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_write_mp_int(pk, &(*p).f0_share);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_write_mp_int(pk, &(*p).g0_share);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_write_mp_int(pk, &(*p).h0_share);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv =
            msgpack_pack_int(pk, (*p).for_server as libc::c_int) as SECStatus;
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        match (*p).for_server as libc::c_uint {
            0 => {
                rv = serial_write_server_a_data(pk, &(*p).shares.A);
                if rv as libc::c_int != SECSuccess as libc::c_int {
                    return rv
                }
            }
            1 => {
                rv = serial_write_server_b_data(pk, &(*p).shares.B);
                if rv as libc::c_int != SECSuccess as libc::c_int {
                    return rv
                }
            }
            _ => { return SECFailure }
        }
    }
    return rv;
}
unsafe extern "C" fn serial_write_server_b_data(mut pk: *mut msgpack_packer,
                                                mut B: *const server_b_data)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if pk.is_null() {
        rv = SECFailure
    } else if B.is_null() {
        rv = SECFailure
    } else { rv = serial_write_prg_seed(pk, &(*B).seed) }
    return rv;
}
unsafe extern "C" fn serial_write_prg_seed(mut pk: *mut msgpack_packer,
                                           mut seed: *const PrioPRGSeed)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if pk.is_null() {
        rv = SECFailure
    } else if seed.is_null() {
        rv = SECFailure
    } else {
        rv = msgpack_pack_str(pk, 16i32 as size_t) as SECStatus;
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv =
            msgpack_pack_str_body(pk, seed as *const libc::c_void,
                                  16i32 as size_t) as SECStatus;
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
unsafe extern "C" fn serial_write_server_a_data(mut pk: *mut msgpack_packer,
                                                mut A: *const server_a_data)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if pk.is_null() {
        rv = SECFailure
    } else if A.is_null() {
        rv = SECFailure
    } else {
        rv = serial_write_mp_array(pk, (*A).data_shares as const_MPArray);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_write_mp_array(pk, (*A).h_points as const_MPArray);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
unsafe extern "C" fn serial_write_beaver_triple(mut pk: *mut msgpack_packer,
                                                mut t: const_BeaverTriple)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if pk.is_null() {
        rv = SECFailure
    } else if t.is_null() {
        rv = SECFailure
    } else {
        rv = serial_write_mp_int(pk, &(*t).a);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_write_mp_int(pk, &(*t).b);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_write_mp_int(pk, &(*t).c);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn serial_read_packet_client(mut upk:
                                                       *mut msgpack_unpacker,
                                                   mut p: PrioPacketClient,
                                                   mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut obj: msgpack_object =
        msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                       via: msgpack_object_union{boolean: false,},};
    let mut s: msgpack_object_str =
        msgpack_object_str{size: 0, ptr: 0 as *const libc::c_char,};
    let mut rv: SECStatus = SECSuccess;
    let mut res: msgpack_unpacked =
        msgpack_unpacked{zone: 0 as *mut msgpack_zone,
                         data:
                             msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                                            via:
                                                msgpack_object_union{boolean:
                                                                         false,},},};
    msgpack_unpacked_init(&mut res);
    if upk.is_null() {
        rv = SECFailure
    } else if p.is_null() {
        rv = SECFailure
    } else {
        let mut r: libc::c_int =
            msgpack_unpacker_next(upk, &mut res) as libc::c_int;
        if r != MSGPACK_UNPACK_SUCCESS as libc::c_int &&
               r != MSGPACK_UNPACK_EXTRA_BYTES as libc::c_int {
            return SECFailure
        }
        obj = res.data;
        if !(obj.type_0 as libc::c_uint ==
                 MSGPACK_OBJECT_STR as libc::c_int as libc::c_uint) {
            rv = SECFailure
        } else {
            s = obj.via.str_0;
            if !(s.size == (*cfg).batch_id_len) {
                rv = SECFailure
            } else if 0 !=
                          memcmp(s.ptr as *const libc::c_void,
                                 (*cfg).batch_id as *mut libc::c_char as
                                     *const libc::c_void,
                                 (*cfg).batch_id_len as libc::c_ulong) {
                rv = SECFailure
            } else {
                rv =
                    serial_read_beaver_triple(upk, (*p).triple,
                                              &(*cfg).modulus);
                if rv as libc::c_int != SECSuccess as libc::c_int {
                    return rv
                }
                rv =
                    serial_read_mp_int(upk, &mut (*p).f0_share,
                                       &(*cfg).modulus);
                if rv as libc::c_int != SECSuccess as libc::c_int {
                    return rv
                }
                rv =
                    serial_read_mp_int(upk, &mut (*p).g0_share,
                                       &(*cfg).modulus);
                if rv as libc::c_int != SECSuccess as libc::c_int {
                    return rv
                }
                rv =
                    serial_read_mp_int(upk, &mut (*p).h0_share,
                                       &(*cfg).modulus);
                if rv as libc::c_int != SECSuccess as libc::c_int {
                    return rv
                }
                rv = serial_read_server_id(upk, &mut (*p).for_server);
                if rv as libc::c_int != SECSuccess as libc::c_int {
                    return rv
                }
                match (*p).for_server as libc::c_uint {
                    0 => {
                        rv =
                            serial_read_server_a_data(upk, &mut (*p).shares.A,
                                                      cfg);
                        if rv as libc::c_int != SECSuccess as libc::c_int {
                            return rv
                        }
                    }
                    1 => {
                        rv =
                            serial_read_server_b_data(upk,
                                                      &mut (*p).shares.B);
                        if rv as libc::c_int != SECSuccess as libc::c_int {
                            return rv
                        }
                    }
                    _ => { return SECFailure }
                }
            }
        }
    }
    msgpack_unpacked_destroy(&mut res);
    return rv;
}
unsafe extern "C" fn serial_read_server_b_data(mut upk: *mut msgpack_unpacker,
                                               mut B: *mut server_b_data)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if upk.is_null() {
        rv = SECFailure
    } else if B.is_null() {
        rv = SECFailure
    } else { rv = serial_read_prg_seed(upk, &mut (*B).seed) }
    return rv;
}
unsafe extern "C" fn serial_read_prg_seed(mut upk: *mut msgpack_unpacker,
                                          mut seed: *mut PrioPRGSeed)
 -> SECStatus {
    let mut obj: msgpack_object =
        msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                       via: msgpack_object_union{boolean: false,},};
    let mut s: msgpack_object_str =
        msgpack_object_str{size: 0, ptr: 0 as *const libc::c_char,};
    let mut rv: SECStatus = SECSuccess;
    let mut res: msgpack_unpacked =
        msgpack_unpacked{zone: 0 as *mut msgpack_zone,
                         data:
                             msgpack_object{type_0: MSGPACK_OBJECT_NIL,
                                            via:
                                                msgpack_object_union{boolean:
                                                                         false,},},};
    msgpack_unpacked_init(&mut res);
    if upk.is_null() {
        rv = SECFailure
    } else if seed.is_null() {
        rv = SECFailure
    } else {
        let mut r: libc::c_int =
            msgpack_unpacker_next(upk, &mut res) as libc::c_int;
        if r != MSGPACK_UNPACK_SUCCESS as libc::c_int &&
               r != MSGPACK_UNPACK_EXTRA_BYTES as libc::c_int {
            return SECFailure
        }
        obj = res.data;
        if !(obj.type_0 as libc::c_uint ==
                 MSGPACK_OBJECT_STR as libc::c_int as libc::c_uint) {
            rv = SECFailure
        } else {
            s = obj.via.str_0;
            if !(s.size == 16i32 as libc::c_uint) {
                rv = SECFailure
            } else {
                memcpy(seed as *mut libc::c_void,
                       s.ptr as *const libc::c_void, 16i32 as libc::c_ulong);
            }
        }
    }
    msgpack_unpacked_destroy(&mut res);
    return rv;
}
unsafe extern "C" fn serial_read_server_a_data(mut upk: *mut msgpack_unpacker,
                                               mut A: *mut server_a_data,
                                               mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if upk.is_null() {
        rv = SECFailure
    } else if A.is_null() {
        rv = SECFailure
    } else {
        rv =
            serial_read_mp_array(upk, (*A).data_shares,
                                 (*cfg).num_data_fields as size_t,
                                 &(*cfg).modulus);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv =
            serial_read_mp_array(upk, (*A).h_points,
                                 PrioConfig_hPoints(cfg) as size_t,
                                 &(*cfg).modulus);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}
unsafe extern "C" fn serial_read_beaver_triple(mut pk: *mut msgpack_unpacker,
                                               mut t: BeaverTriple,
                                               mut max: *const mp_int)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if pk.is_null() {
        rv = SECFailure
    } else if t.is_null() {
        rv = SECFailure
    } else if max.is_null() {
        rv = SECFailure
    } else {
        rv = serial_read_mp_int(pk, &mut (*t).a, max);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_read_mp_int(pk, &mut (*t).b, max);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        rv = serial_read_mp_int(pk, &mut (*t).c, max);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    }
    return rv;
}