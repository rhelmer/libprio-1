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
 * Opaque types
 */
    pub type prio_config;
    pub type prio_server;
    pub type prio_verifier;
    pub type prio_packet_verify1;
    pub type prio_packet_verify2;
    pub type prio_total_share;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn puts(_: *const libc::c_char) -> libc::c_int;
    /*
 * Initialize and clear random number generator state.
 * You must call Prio_init() before using the library.
 * To avoid memory leaks, call Prio_clear() afterwards.
 */
    #[no_mangle]
    fn Prio_init() -> SECStatus;
    #[no_mangle]
    fn Prio_clear();
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
    /*
 * The PrioServer object holds the state of the Prio servers.
 * Pass in the _same_ secret PRGSeed when initializing the two servers.
 * The PRGSeed must remain secret to the two servers.
 */
    #[no_mangle]
    fn PrioServer_new(cfg: const_PrioConfig, serverIdx: PrioServerId,
                      serverPriv: PrivateKey,
                      serverSharedSecret: *const libc::c_uchar) -> PrioServer;
    #[no_mangle]
    fn PrioServer_clear(s: PrioServer);
    /*
 * After receiving a client packet, each of the servers generate
 * a PrioVerifier object that they use to check whether the client's
 * encoded packet is well formed.
 */
    #[no_mangle]
    fn PrioVerifier_new(s: PrioServer) -> PrioVerifier;
    #[no_mangle]
    fn PrioVerifier_clear(v: PrioVerifier);
    /*
 * Read in encrypted data from the client, decrypt it, and prepare to check the
 * request for validity.
 */
    #[no_mangle]
    fn PrioVerifier_set_data(v: PrioVerifier, data: *mut libc::c_uchar,
                             dataLen: libc::c_uint) -> SECStatus;
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
    fn PrioPacketVerify1_set_data(p1: PrioPacketVerify1,
                                  v: const_PrioVerifier) -> SECStatus;
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
    fn PrioPacketVerify2_set_data(p2: PrioPacketVerify2,
                                  v: const_PrioVerifier,
                                  p1A: const_PrioPacketVerify1,
                                  p1B: const_PrioPacketVerify1) -> SECStatus;
    /*
 * Use the PrioPacketVerify2s from both servers to check whether
 * the client's submission is well formed.
 */
    #[no_mangle]
    fn PrioVerifier_isValid(v: const_PrioVerifier,
                            pA: const_PrioPacketVerify2,
                            pB: const_PrioPacketVerify2) -> SECStatus;
    /*
 * Each of the two servers calls this routine to aggregate the data
 * submission from a client that is included in the PrioVerifier object.
 *
 * IMPORTANT: This routine does *not* check the validity of the client's
 * data packet. The servers must execute the verification checks
 * above before aggregating any client data.
 */
    #[no_mangle]
    fn PrioServer_aggregate(s: PrioServer, v: PrioVerifier) -> SECStatus;
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
    fn PrioTotalShare_set_data(t: PrioTotalShare, s: const_PrioServer)
     -> SECStatus;
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
    fn PrioTotalShare_final(cfg: const_PrioConfig,
                            output: *mut libc::c_ulonglong,
                            tA: const_PrioTotalShare,
                            tB: const_PrioTotalShare) -> SECStatus;
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
pub type PrioConfig = *mut prio_config;
pub type const_PrioConfig = *const prio_config;
pub type PrioServer = *mut prio_server;
pub type const_PrioServer = *const prio_server;
pub type PrioVerifier = *mut prio_verifier;
pub type const_PrioVerifier = *const prio_verifier;
pub type PrioPacketVerify1 = *mut prio_packet_verify1;
pub type const_PrioPacketVerify1 = *const prio_packet_verify1;
pub type PrioPacketVerify2 = *mut prio_packet_verify2;
pub type const_PrioPacketVerify2 = *const prio_packet_verify2;
pub type PrioTotalShare = *mut prio_total_share;
pub type const_PrioTotalShare = *const prio_total_share;
pub type PublicKey = *mut SECKEYPublicKey;
pub type PrivateKey = *mut SECKEYPrivateKey;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn verify_full() -> libc::c_int {
    let mut ndata: libc::c_int = 0;
    let mut nclients: libc::c_int = 0;
    let mut server_secret: PrioPRGSeed = [0; 16];
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
    let mut tA: PrioTotalShare = 0 as PrioTotalShare;
    let mut tB: PrioTotalShare = 0 as PrioTotalShare;
    let mut for_server_a: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut for_server_b: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut batch_id: *const libc::c_uchar =
        b"prio_batch_2018-04-17\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_uchar;
    let batch_id_len: libc::c_uint =
        strlen(batch_id as *mut libc::c_char) as libc::c_uint;
    let mut output: *mut libc::c_ulonglong = 0 as *mut libc::c_ulonglong;
    let mut data_items: *mut bool = 0 as *mut bool;
    // Initialize NSS random number generator.
    rv = Prio_init();
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        // Number of different boolean data fields we collect.
        ndata = 100i32;
        // Number of clients to simulate.
        nclients = 10i32;
        output =
            calloc(ndata as libc::c_ulong,
                   ::std::mem::size_of::<libc::c_ulonglong>() as
                       libc::c_ulong) as *mut libc::c_ulonglong;
        if output.is_null() {
            rv = SECFailure
        } else {
            data_items =
                calloc(ndata as libc::c_ulong,
                       ::std::mem::size_of::<bool>() as libc::c_ulong) as
                    *mut bool;
            if data_items.is_null() {
                rv = SECFailure
            } else {
                // Generate keypairs for servers
                rv = Keypair_new(&mut skA, &mut pkA);
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    rv = Keypair_new(&mut skB, &mut pkB);
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        // Use the default configuration parameters.
                        cfg =
                            PrioConfig_new(ndata, pkA, pkB, batch_id,
                                           batch_id_len);
                        if cfg.is_null() {
                            rv = SECFailure
                        } else {
                            server_secret = [0; 16];
                            rv = PrioPRGSeed_randomize(&mut server_secret);
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                // Initialize two server objects. The role of the servers need not
  // be symmetric. In a deployment, we envision that:
  //   * Server A is the main telemetry server that is always online.
  //     Clients send their encrypted data packets to Server A and
  //     Server A stores them.
  //   * Server B only comes online when the two servers want to compute
  //     the final aggregate statistics.
                                sA =
                                    PrioServer_new(cfg as const_PrioConfig,
                                                   PRIO_SERVER_A, skA,
                                                   server_secret.as_mut_ptr()
                                                       as
                                                       *const libc::c_uchar);
                                if sA.is_null() {
                                    rv = SECFailure
                                } else {
                                    sB =
                                        PrioServer_new(cfg as
                                                           const_PrioConfig,
                                                       PRIO_SERVER_B, skB,
                                                       server_secret.as_mut_ptr()
                                                           as
                                                           *const libc::c_uchar);
                                    if sB.is_null() {
                                        rv = SECFailure
                                    } else {
                                        // Initialize empty verifier objects
                                        vA = PrioVerifier_new(sA);
                                        if vA.is_null() {
                                            rv = SECFailure
                                        } else {
                                            vB = PrioVerifier_new(sB);
                                            if vB.is_null() {
                                                rv = SECFailure
                                            } else {
                                                // Initialize shares of final aggregate statistics
                                                tA = PrioTotalShare_new();
                                                if tA.is_null() {
                                                    rv = SECFailure
                                                } else {
                                                    tB = PrioTotalShare_new();
                                                    if tB.is_null() {
                                                        rv = SECFailure
                                                    } else {
                                                        // Initialize shares of verification packets
                                                        p1A =
                                                            PrioPacketVerify1_new();
                                                        if p1A.is_null() {
                                                            rv = SECFailure
                                                        } else {
                                                            p1B =
                                                                PrioPacketVerify1_new();
                                                            if p1B.is_null() {
                                                                rv =
                                                                    SECFailure
                                                            } else {
                                                                p2A =
                                                                    PrioPacketVerify2_new();
                                                                if p2A.is_null()
                                                                   {
                                                                    rv =
                                                                        SECFailure
                                                                } else {
                                                                    p2B =
                                                                        PrioPacketVerify2_new();
                                                                    if p2B.is_null()
                                                                       {
                                                                        rv =
                                                                            SECFailure
                                                                    } else {
                                                                        // Generate client data packets.
                                                                        let mut c:
                                                                                libc::c_int =
                                                                            0i32;
                                                                        while c
                                                                                  <
                                                                                  nclients
                                                                              {
                                                                            let mut i:
                                                                                    libc::c_int =
                                                                                0i32;
                                                                            while i
                                                                                      <
                                                                                      ndata
                                                                                  {
                                                                                *data_items.offset(i
                                                                                                       as
                                                                                                       isize)
                                                                                    =
                                                                                    i
                                                                                        %
                                                                                        3i32
                                                                                        ==
                                                                                        1i32
                                                                                        ||
                                                                                        c
                                                                                            %
                                                                                            5i32
                                                                                            ==
                                                                                            3i32;
                                                                                i
                                                                                    +=
                                                                                    1
                                                                            }
                                                                            // I. CLIENT DATA SUBMISSION.
    //
    // Construct the client data packets.
                                                                            let mut aLen:
                                                                                    libc::c_uint =
                                                                                0;
                                                                            let mut bLen:
                                                                                    libc::c_uint =
                                                                                0;
                                                                            rv
                                                                                =
                                                                                PrioClient_encode(cfg
                                                                                                      as
                                                                                                      const_PrioConfig,
                                                                                                  data_items,
                                                                                                  &mut for_server_a,
                                                                                                  &mut aLen,
                                                                                                  &mut for_server_b,
                                                                                                  &mut bLen);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            // The Prio servers A and B can come online later (e.g., at the end of
    // each day) to download the encrypted telemetry packets from the
    // telemetry server and run the protocol that computes the aggregate
    // statistics. In this way, the client only needs to send a
    // single message (the pair of encrypted ClientPacketData packets)
    // to a single server (the telemetry-data-collection server).
                                                                            // THE CLIENT'S JOB IS DONE. The rest of the processing just takes place
    // between the two servers A and B.
                                                                            // II. VALIDATION PROTOCOL. (at servers)
    //
    // The servers now run a short 2-step protocol to check each
    // client's packet:
    //    1) Servers A and B broadcast one message (PrioPacketVerify1)
    //       to each other.
    //    2) Servers A and B broadcast another message (PrioPacketVerify2)
    //       to each other.
    //    3) Servers A and B can both determine whether the client's data
    //       submission is well-formed (in which case they add it to their
    //       running total of aggregate statistics) or ill-formed
    //       (in which case they ignore it).
    // These messages must be sent over an authenticated channel, so
    // that each server is assured that every received message came
    // from its peer.
                                                                            // Set up a Prio verifier object.
                                                                            rv
                                                                                =
                                                                                PrioVerifier_set_data(vA,
                                                                                                      for_server_a,
                                                                                                      aLen);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            rv
                                                                                =
                                                                                PrioVerifier_set_data(vB,
                                                                                                      for_server_b,
                                                                                                      bLen);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            // Both servers produce a packet1. Server A sends p1A to Server B
    // and vice versa.
                                                                            rv
                                                                                =
                                                                                PrioPacketVerify1_set_data(p1A,
                                                                                                           vA
                                                                                                               as
                                                                                                               const_PrioVerifier);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            rv
                                                                                =
                                                                                PrioPacketVerify1_set_data(p1B,
                                                                                                           vB
                                                                                                               as
                                                                                                               const_PrioVerifier);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            // Both servers produce a packet2. Server A sends p2A to Server B
    // and vice versa.
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
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
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
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            // Using p2A and p2B, the servers can determine whether the request
    // is valid. (In fact, only Server A needs to perform this
    // check, since Server A can just tell Server B whether the check
    // succeeded or failed.)
                                                                            rv
                                                                                =
                                                                                PrioVerifier_isValid(vA
                                                                                                         as
                                                                                                         const_PrioVerifier,
                                                                                                     p2A
                                                                                                         as
                                                                                                         const_PrioPacketVerify2,
                                                                                                     p2B
                                                                                                         as
                                                                                                         const_PrioPacketVerify2);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            rv
                                                                                =
                                                                                PrioVerifier_isValid(vB
                                                                                                         as
                                                                                                         const_PrioVerifier,
                                                                                                     p2A
                                                                                                         as
                                                                                                         const_PrioPacketVerify2,
                                                                                                     p2B
                                                                                                         as
                                                                                                         const_PrioPacketVerify2);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            // If we get here, the client packet is valid, so add it to the aggregate
    // statistic counter for both servers.
                                                                            rv
                                                                                =
                                                                                PrioServer_aggregate(sA,
                                                                                                     vA);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            rv
                                                                                =
                                                                                PrioServer_aggregate(sB,
                                                                                                     vB);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            free(for_server_a
                                                                                     as
                                                                                     *mut libc::c_void);
                                                                            free(for_server_b
                                                                                     as
                                                                                     *mut libc::c_void);
                                                                            for_server_a
                                                                                =
                                                                                0
                                                                                    as
                                                                                    *mut libc::c_uchar;
                                                                            for_server_b
                                                                                =
                                                                                0
                                                                                    as
                                                                                    *mut libc::c_uchar;
                                                                            // The servers repeat the steps above for each client submission.
                                                                            // III. PRODUCTION OF AGGREGATE STATISTICS.
    //
    // After collecting aggregates from MANY clients, the servers can compute
    // their shares of the aggregate statistics.
    //
    // Server B can send tB to Server A.
                                                                            rv
                                                                                =
                                                                                PrioTotalShare_set_data(tA,
                                                                                                        sA
                                                                                                            as
                                                                                                            const_PrioServer);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            rv
                                                                                =
                                                                                PrioTotalShare_set_data(tB,
                                                                                                        sB
                                                                                                            as
                                                                                                            const_PrioServer);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            // Once Server A has tA and tB, it can learn the aggregate statistics
    // in the clear.
                                                                            rv
                                                                                =
                                                                                PrioTotalShare_final(cfg
                                                                                                         as
                                                                                                         const_PrioConfig,
                                                                                                     output,
                                                                                                     tA
                                                                                                         as
                                                                                                         const_PrioTotalShare,
                                                                                                     tB
                                                                                                         as
                                                                                                         const_PrioTotalShare);
                                                                            if rv
                                                                                   as
                                                                                   libc::c_int
                                                                                   !=
                                                                                   SECSuccess
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                break
                                                                                    ;
                                                                            }
                                                                            let mut i_0:
                                                                                    libc::c_int =
                                                                                0i32;
                                                                            while i_0
                                                                                      <
                                                                                      ndata
                                                                                  {
                                                                                printf(b"output[%d] = %llu\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const libc::c_char,
                                                                                       i_0,
                                                                                       *output.offset(i_0
                                                                                                          as
                                                                                                          isize));
                                                                                i_0
                                                                                    +=
                                                                                    1
                                                                            }
                                                                            c
                                                                                +=
                                                                                1
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
    if rv as libc::c_int != SECSuccess as libc::c_int {
        fprintf(__stderrp,
                b"Warning: unexpected failure.\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if !for_server_a.is_null() { free(for_server_a as *mut libc::c_void); }
    if !for_server_b.is_null() { free(for_server_b as *mut libc::c_void); }
    if !output.is_null() { free(output as *mut libc::c_void); }
    if !data_items.is_null() { free(data_items as *mut libc::c_void); }
    PrioTotalShare_clear(tA);
    PrioTotalShare_clear(tB);
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
    Prio_clear();
    return !(rv as libc::c_int == SECSuccess as libc::c_int) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    puts(b"This utility demonstrates how to invoke the Prio API.\x00" as
             *const u8 as *const libc::c_char);
    return verify_full();
}
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }