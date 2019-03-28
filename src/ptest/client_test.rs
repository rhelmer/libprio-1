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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
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
 * Return the maximum number of data fields that the implementation supports.
 */
    #[no_mangle]
    fn PrioConfig_maxDataFields() -> libc::c_int;
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
    fn PrioPacketClient_new(cfg: const_PrioConfig, for_server: PrioServerId)
     -> PrioPacketClient;
    #[no_mangle]
    fn PrioPacketClient_clear(p: PrioPacketClient);
    #[no_mangle]
    fn PrioPacketClient_set_data(cfg: const_PrioConfig, data_in: *const bool,
                                 for_server_a: PrioPacketClient,
                                 for_server_b: PrioPacketClient) -> SECStatus;
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
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn mu_test_client__new() {
    let mut ndata: libc::c_int = 0;
    let mut rv: SECStatus = SECSuccess;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut pA: PrioPacketClient = 0 as PrioPacketClient;
    let mut pB: PrioPacketClient = 0 as PrioPacketClient;
    let mut data_items: *mut bool = 0 as *mut bool;
    let mut v: bool = false;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(23))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((v = (cfg = PrioConfig_newTest(23))))...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    cfg = PrioConfig_newTest(23i32);
    v = !cfg.is_null();
    if v {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/client_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(23)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 26i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/client_test.c:%d: mu_check((v = (cfg = PrioConfig_newTest(23)))) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 26i32);
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
                        b"\t\t* Checking mu_check((v = (pA = PrioPacketClient_new(cfg, PRIO_SERVER_A))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((v = (pA = PrioPacketClient_new(cfg, PRIO_SERVER_A))))...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        pA = PrioPacketClient_new(cfg as const_PrioConfig, PRIO_SERVER_A);
        v_0 = !pA.is_null();
        if v_0 {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/client_test.c:%d: mu_check((v = (pA = PrioPacketClient_new(cfg, PRIO_SERVER_A)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 27i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/client_test.c:%d: mu_check((v = (pA = PrioPacketClient_new(cfg, PRIO_SERVER_A)))) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 27i32);
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
                            b"\t\t* Checking mu_check((v = (pB = PrioPacketClient_new(cfg, PRIO_SERVER_B))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((v = (pB = PrioPacketClient_new(cfg, PRIO_SERVER_B))))...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            pB = PrioPacketClient_new(cfg as const_PrioConfig, PRIO_SERVER_B);
            v_1 = !pB.is_null();
            if v_1 {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/client_test.c:%d: mu_check((v = (pB = PrioPacketClient_new(cfg, PRIO_SERVER_B)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                28i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/client_test.c:%d: mu_check((v = (pB = PrioPacketClient_new(cfg, PRIO_SERVER_B)))) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                28i32);
                    }
                }
            }
            if !v_1 {
                rv = SECFailure
            } else {
                ndata = PrioConfig_numDataFields(cfg as const_PrioConfig);
                let mut v_2: bool = false;
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
                           ::std::mem::size_of::<bool>() as libc::c_ulong) as
                        *mut bool;
                v_2 = !data_items.is_null();
                if v_2 {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/client_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    31i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/client_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    31i32);
                        }
                    }
                }
                if !v_2 {
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
                    rv =
                        PrioPacketClient_set_data(cfg as const_PrioConfig,
                                                  data_items, pA, pB);
                    if rv as libc::c_int == SECSuccess as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/client_test.c:%d: mu_check((rv = (PrioPacketClient_set_data(cfg, data_items, pA, pB))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 38i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/client_test.c:%d: mu_check((rv = (PrioPacketClient_set_data(cfg, data_items, pA, pB))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 38i32);
                            }
                        }
                    }
                    rv as libc::c_int != SECSuccess as libc::c_int;
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
                        b"build/ptest/client_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 41i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/client_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 41i32);
            }
        }
    }
    if !data_items.is_null() { free(data_items as *mut libc::c_void); }
    PrioPacketClient_clear(pA);
    PrioPacketClient_clear(pB);
    PrioConfig_clear(cfg);
}
#[no_mangle]
pub unsafe extern "C" fn test_client_agg(mut nclients: libc::c_int,
                                         mut nfields: libc::c_int,
                                         mut config_is_okay: bool) {
    let mut ndata: libc::c_int = 0;
    let mut current_block: u64;
    let mut rv: SECStatus = SECSuccess;
    let mut pkA: PublicKey = 0 as PublicKey;
    let mut pkB: PublicKey = 0 as PublicKey;
    let mut skA: PrivateKey = 0 as PrivateKey;
    let mut skB: PrivateKey = 0 as PrivateKey;
    let mut cfg: PrioConfig = 0 as PrioConfig;
    let mut sA: PrioServer = 0 as PrioServer;
    let mut sB: PrioServer = 0 as PrioServer;
    let mut tA: PrioTotalShare = 0 as PrioTotalShare;
    let mut tB: PrioTotalShare = 0 as PrioTotalShare;
    let mut vA: PrioVerifier = 0 as PrioVerifier;
    let mut vB: PrioVerifier = 0 as PrioVerifier;
    let mut for_a: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut for_b: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut batch_id: *const libc::c_uchar =
        b"test_batch\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_uchar;
    let mut batch_id_len: libc::c_uint =
        strlen(batch_id as *mut libc::c_char) as libc::c_uint;
    let mut data_items: *mut bool = 0 as *mut bool;
    let mut output: *mut libc::c_ulonglong = 0 as *mut libc::c_ulonglong;
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
                        b"build/ptest/client_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 73i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/client_test.c:%d: mu_check((rv = (PrioPRGSeed_randomize(&seed))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 73i32);
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
                            b"build/ptest/client_test.c:%d: mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 75i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/client_test.c:%d: mu_check((rv = (Keypair_new(&skA, &pkA))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 75i32);
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
                                b"build/ptest/client_test.c:%d: mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                76i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/client_test.c:%d: mu_check((rv = (Keypair_new(&skB, &pkB))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                76i32);
                    }
                }
            }
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                cfg =
                    PrioConfig_new(nfields, pkA, pkB, batch_id, batch_id_len);
                if cfg.is_null() {
                    rv = SECFailure
                } else {
                    if !config_is_okay {
                        let mut v: bool = false;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((v = ((PrioConfig_new(nfields, pkA, pkB, batch_id, batch_id_len) == ((void*)0)))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((v = ((PrioConfig_new(nfields, pkA, pkB, batch_id, batch_id_len) == ((void*)0)))))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        v =
                            PrioConfig_new(nfields, pkA, pkB, batch_id,
                                           batch_id_len).is_null();
                        if v {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/client_test.c:%d: mu_check((v = ((PrioConfig_new(nfields, pkA, pkB, batch_id, batch_id_len) == ((void*)0))))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 80i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/client_test.c:%d: mu_check((v = ((PrioConfig_new(nfields, pkA, pkB, batch_id, batch_id_len) == ((void*)0))))) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 80i32);
                                }
                            }
                        }
                        if !v {
                            current_block = 15146464411692216834;
                        } else { current_block = 14648249180243006330; }
                    } else { current_block = 14648249180243006330; }
                    match current_block {
                        15146464411692216834 => { }
                        _ => {
                            let mut v_0: bool = false;
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
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
                            v_0 = !sA.is_null();
                            if v_0 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/client_test.c:%d: mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                82i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/client_test.c:%d: mu_check((v = (sA = PrioServer_new(cfg, 0, skA, seed)))) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                82i32);
                                    }
                                }
                            }
                            if !v_0 {
                                rv = SECFailure
                            } else {
                                let mut v_1: bool = false;
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
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
                                v_1 = !sB.is_null();
                                if v_1 {
                                    mutest_passed_checks += 1
                                } else {
                                    mutest_failed_checks += 1;
                                    mutest_case_failed = 1i32;
                                    if mutest_verbose_level >=
                                           MU_ERROR as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"build/ptest/client_test.c:%d: mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed)))) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    83i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/client_test.c:%d: mu_check((v = (sB = PrioServer_new(cfg, 1, skB, seed)))) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    83i32);
                                        }
                                    }
                                }
                                if !v_1 {
                                    rv = SECFailure
                                } else {
                                    let mut v_2: bool = false;
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check((v = (tA = PrioTotalShare_new())))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check((v = (tA = PrioTotalShare_new())))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    tA = PrioTotalShare_new();
                                    v_2 = !tA.is_null();
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
                                                        b"build/ptest/client_test.c:%d: mu_check((v = (tA = PrioTotalShare_new()))) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        84i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/client_test.c:%d: mu_check((v = (tA = PrioTotalShare_new()))) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        84i32);
                                            }
                                        }
                                    }
                                    if !v_2 {
                                        rv = SECFailure
                                    } else {
                                        let mut v_3: bool = false;
                                        if mutest_verbose_level >=
                                               MU_CHECK as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"\t\t* Checking mu_check((v = (tB = PrioTotalShare_new())))...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"\t\t* Checking mu_check((v = (tB = PrioTotalShare_new())))...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            }
                                        }
                                        tB = PrioTotalShare_new();
                                        v_3 = !tB.is_null();
                                        if v_3 {
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
                                                            b"build/ptest/client_test.c:%d: mu_check((v = (tB = PrioTotalShare_new()))) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            85i32);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"build/ptest/client_test.c:%d: mu_check((v = (tB = PrioTotalShare_new()))) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            85i32);
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
                                                       MU_ERROR as libc::c_int
                                                   {
                                                    fprintf(__stderrp,
                                                            b"\t\t* Checking mu_check((v = (vA = PrioVerifier_new(sA))))...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"\t\t* Checking mu_check((v = (vA = PrioVerifier_new(sA))))...\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                }
                                            }
                                            vA = PrioVerifier_new(sA);
                                            v_4 = !vA.is_null();
                                            if v_4 {
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
                                                                b"build/ptest/client_test.c:%d: mu_check((v = (vA = PrioVerifier_new(sA)))) failed, resuming test case\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                86i32);
                                                    } else {
                                                        fprintf(__stdoutp,
                                                                b"build/ptest/client_test.c:%d: mu_check((v = (vA = PrioVerifier_new(sA)))) failed, resuming test case\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                86i32);
                                                    }
                                                }
                                            }
                                            if !v_4 {
                                                rv = SECFailure
                                            } else {
                                                let mut v_5: bool = false;
                                                if mutest_verbose_level >=
                                                       MU_CHECK as libc::c_int
                                                   {
                                                    if mutest_verbose_level ==
                                                           MU_ERROR as
                                                               libc::c_int {
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
                                                v_5 = !vB.is_null();
                                                if v_5 {
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
                                                                    b"build/ptest/client_test.c:%d: mu_check((v = (vB = PrioVerifier_new(sB)))) failed, resuming test case\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    87i32);
                                                        } else {
                                                            fprintf(__stdoutp,
                                                                    b"build/ptest/client_test.c:%d: mu_check((v = (vB = PrioVerifier_new(sB)))) failed, resuming test case\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    87i32);
                                                        }
                                                    }
                                                }
                                                if !v_5 {
                                                    rv = SECFailure
                                                } else {
                                                    ndata =
                                                        PrioConfig_numDataFields(cfg
                                                                                     as
                                                                                     const_PrioConfig);
                                                    let mut v_6: bool = false;
                                                    if mutest_verbose_level >=
                                                           MU_CHECK as
                                                               libc::c_int {
                                                        if mutest_verbose_level
                                                               ==
                                                               MU_ERROR as
                                                                   libc::c_int
                                                           {
                                                            fprintf(__stderrp,
                                                                    b"\t\t* Checking mu_check((v = (data_items = calloc(ndata, sizeof(_Bool)))))...\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                        } else {
                                                            fprintf(__stdoutp,
                                                                    b"\t\t* Checking mu_check((v = (data_items = calloc(ndata, sizeof(_Bool)))))...\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                        }
                                                    }
                                                    data_items =
                                                        calloc(ndata as
                                                                   libc::c_ulong,
                                                               ::std::mem::size_of::<bool>()
                                                                   as
                                                                   libc::c_ulong)
                                                            as *mut bool;
                                                    v_6 =
                                                        !data_items.is_null();
                                                    if v_6 {
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
                                                                        b"build/ptest/client_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        91i32);
                                                            } else {
                                                                fprintf(__stdoutp,
                                                                        b"build/ptest/client_test.c:%d: mu_check((v = (data_items = calloc(ndata, sizeof(_Bool))))) failed, resuming test case\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        91i32);
                                                            }
                                                        }
                                                    }
                                                    if !v_6 {
                                                        rv = SECFailure
                                                    } else {
                                                        let mut i:
                                                                libc::c_int =
                                                            0i32;
                                                        while i < ndata {
                                                            *data_items.offset(i
                                                                                   as
                                                                                   isize)
                                                                =
                                                                i % 3i32 ==
                                                                    1i32 ||
                                                                    i % 5i32
                                                                        ==
                                                                        3i32;
                                                            i += 1
                                                        }
                                                        let mut i_0:
                                                                libc::c_int =
                                                            0i32;
                                                        loop  {
                                                            if !(i_0 <
                                                                     nclients)
                                                               {
                                                                current_block
                                                                    =
                                                                    7481698924059668625;
                                                                break ;
                                                            }
                                                            let mut aLen:
                                                                    libc::c_uint =
                                                                0;
                                                            let mut bLen:
                                                                    libc::c_uint =
                                                                0;
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
                                                                            b"\t\t* Checking mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                } else {
                                                                    fprintf(__stdoutp,
                                                                            b"\t\t* Checking mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                }
                                                            }
                                                            rv =
                                                                PrioClient_encode(cfg
                                                                                      as
                                                                                      const_PrioConfig,
                                                                                  data_items,
                                                                                  &mut for_a,
                                                                                  &mut aLen,
                                                                                  &mut for_b,
                                                                                  &mut bLen);
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
                                                                                b"build/ptest/client_test.c:%d: mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                99i32);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"build/ptest/client_test.c:%d: mu_check((rv = (PrioClient_encode(cfg, data_items, &for_a, &aLen, &for_b, &bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                99i32);
                                                                    }
                                                                }
                                                            }
                                                            if rv as
                                                                   libc::c_int
                                                                   !=
                                                                   SECSuccess
                                                                       as
                                                                       libc::c_int
                                                               {
                                                                current_block
                                                                    =
                                                                    15146464411692216834;
                                                                break ;
                                                            }
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
                                                                                b"build/ptest/client_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vA, for_a, aLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                101i32);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"build/ptest/client_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vA, for_a, aLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                101i32);
                                                                    }
                                                                }
                                                            }
                                                            if rv as
                                                                   libc::c_int
                                                                   !=
                                                                   SECSuccess
                                                                       as
                                                                       libc::c_int
                                                               {
                                                                current_block
                                                                    =
                                                                    15146464411692216834;
                                                                break ;
                                                            }
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
                                                                                b"build/ptest/client_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                102i32);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"build/ptest/client_test.c:%d: mu_check((rv = (PrioVerifier_set_data(vB, for_b, bLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                102i32);
                                                                    }
                                                                }
                                                            }
                                                            if rv as
                                                                   libc::c_int
                                                                   !=
                                                                   SECSuccess
                                                                       as
                                                                       libc::c_int
                                                               {
                                                                current_block
                                                                    =
                                                                    15146464411692216834;
                                                                break ;
                                                            }
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
                                                                            b"\t\t* Checking mu_check(PrioServer_aggregate(sA, vA) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                } else {
                                                                    fprintf(__stdoutp,
                                                                            b"\t\t* Checking mu_check(PrioServer_aggregate(sA, vA) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                }
                                                            }
                                                            if PrioServer_aggregate(sA,
                                                                                    vA)
                                                                   as
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
                                                                                b"build/ptest/client_test.c:%d: mu_check(PrioServer_aggregate(sA, vA) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                104i32);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"build/ptest/client_test.c:%d: mu_check(PrioServer_aggregate(sA, vA) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                104i32);
                                                                    }
                                                                }
                                                            }
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
                                                                            b"\t\t* Checking mu_check(PrioServer_aggregate(sB, vB) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                } else {
                                                                    fprintf(__stdoutp,
                                                                            b"\t\t* Checking mu_check(PrioServer_aggregate(sB, vB) == SECSuccess)...\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char);
                                                                }
                                                            }
                                                            if PrioServer_aggregate(sB,
                                                                                    vB)
                                                                   as
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
                                                                                b"build/ptest/client_test.c:%d: mu_check(PrioServer_aggregate(sB, vB) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                105i32);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"build/ptest/client_test.c:%d: mu_check(PrioServer_aggregate(sB, vB) == SECSuccess) failed, resuming test case\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                105i32);
                                                                    }
                                                                }
                                                            }
                                                            free(for_a as
                                                                     *mut libc::c_void);
                                                            free(for_b as
                                                                     *mut libc::c_void);
                                                            for_a =
                                                                0 as
                                                                    *mut libc::c_uchar;
                                                            for_b =
                                                                0 as
                                                                    *mut libc::c_uchar;
                                                            i_0 += 1
                                                        }
                                                        match current_block {
                                                            15146464411692216834
                                                            => {
                                                            }
                                                            _ => {
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
                                                                                b"\t\t* Checking mu_check(PrioTotalShare_set_data(tA, sA) == SECSuccess)...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"\t\t* Checking mu_check(PrioTotalShare_set_data(tA, sA) == SECSuccess)...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    }
                                                                }
                                                                if PrioTotalShare_set_data(tA,
                                                                                           sA
                                                                                               as
                                                                                               const_PrioServer)
                                                                       as
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
                                                                                    b"build/ptest/client_test.c:%d: mu_check(PrioTotalShare_set_data(tA, sA) == SECSuccess) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    114i32);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"build/ptest/client_test.c:%d: mu_check(PrioTotalShare_set_data(tA, sA) == SECSuccess) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    114i32);
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
                                                                                b"\t\t* Checking mu_check(PrioTotalShare_set_data(tB, sB) == SECSuccess)...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"\t\t* Checking mu_check(PrioTotalShare_set_data(tB, sB) == SECSuccess)...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    }
                                                                }
                                                                if PrioTotalShare_set_data(tB,
                                                                                           sB
                                                                                               as
                                                                                               const_PrioServer)
                                                                       as
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
                                                                                    b"build/ptest/client_test.c:%d: mu_check(PrioTotalShare_set_data(tB, sB) == SECSuccess) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    115i32);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"build/ptest/client_test.c:%d: mu_check(PrioTotalShare_set_data(tB, sB) == SECSuccess) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    115i32);
                                                                        }
                                                                    }
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
                                                                                b"\t\t* Checking mu_check((v = (output = calloc(ndata, sizeof(unsigned long)))))...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    } else {
                                                                        fprintf(__stdoutp,
                                                                                b"\t\t* Checking mu_check((v = (output = calloc(ndata, sizeof(unsigned long)))))...\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char);
                                                                    }
                                                                }
                                                                output =
                                                                    calloc(ndata
                                                                               as
                                                                               libc::c_ulong,
                                                                           ::std::mem::size_of::<libc::c_ulong>()
                                                                               as
                                                                               libc::c_ulong)
                                                                        as
                                                                        *mut libc::c_ulonglong;
                                                                v_7 =
                                                                    !output.is_null();
                                                                if v_7 {
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
                                                                                    b"build/ptest/client_test.c:%d: mu_check((v = (output = calloc(ndata, sizeof(unsigned long))))) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    117i32);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"build/ptest/client_test.c:%d: mu_check((v = (output = calloc(ndata, sizeof(unsigned long))))) failed, resuming test case\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    117i32);
                                                                        }
                                                                    }
                                                                }
                                                                if !v_7 {
                                                                    rv =
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
                                                                                    b"\t\t* Checking mu_check(PrioTotalShare_final(cfg, output, tA, tB) == SECSuccess)...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        } else {
                                                                            fprintf(__stdoutp,
                                                                                    b"\t\t* Checking mu_check(PrioTotalShare_final(cfg, output, tA, tB) == SECSuccess)...\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char);
                                                                        }
                                                                    }
                                                                    if PrioTotalShare_final(cfg
                                                                                                as
                                                                                                const_PrioConfig,
                                                                                            output,
                                                                                            tA
                                                                                                as
                                                                                                const_PrioTotalShare,
                                                                                            tB
                                                                                                as
                                                                                                const_PrioTotalShare)
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
                                                                                        b"build/ptest/client_test.c:%d: mu_check(PrioTotalShare_final(cfg, output, tA, tB) == SECSuccess) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        118i32);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"build/ptest/client_test.c:%d: mu_check(PrioTotalShare_final(cfg, output, tA, tB) == SECSuccess) failed, resuming test case\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        118i32);
                                                                            }
                                                                        }
                                                                    }
                                                                    let mut i_1:
                                                                            libc::c_int =
                                                                        0i32;
                                                                    while i_1
                                                                              <
                                                                              ndata
                                                                          {
                                                                        let mut v_8:
                                                                                libc::c_ulong =
                                                                            (i_1
                                                                                 %
                                                                                 3i32
                                                                                 ==
                                                                                 1i32
                                                                                 ||
                                                                                 i_1
                                                                                     %
                                                                                     5i32
                                                                                     ==
                                                                                     3i32)
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong;
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
                                                                                        b"\t\t* Checking mu_check(output[i] == v * nclients)...\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char);
                                                                            } else {
                                                                                fprintf(__stdoutp,
                                                                                        b"\t\t* Checking mu_check(output[i] == v * nclients)...\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char);
                                                                            }
                                                                        }
                                                                        if *output.offset(i_1
                                                                                              as
                                                                                              isize)
                                                                               ==
                                                                               v_8.wrapping_mul(nclients
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                                                   as
                                                                                   libc::c_ulonglong
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
                                                                                            b"build/ptest/client_test.c:%d: mu_check(output[i] == v * nclients) failed, resuming test case\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char,
                                                                                            121i32);
                                                                                } else {
                                                                                    fprintf(__stdoutp,
                                                                                            b"build/ptest/client_test.c:%d: mu_check(output[i] == v * nclients) failed, resuming test case\n\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char,
                                                                                            121i32);
                                                                                }
                                                                            }
                                                                        }
                                                                        i_1 +=
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
    if config_is_okay {
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
                            b"build/ptest/client_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 126i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/client_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 126i32);
                }
            }
        }
    } else {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check(rv == SECFailure)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check(rv == SECFailure)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        if rv as libc::c_int == SECFailure as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/client_test.c:%d: mu_check(rv == SECFailure) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 128i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/client_test.c:%d: mu_check(rv == SECFailure) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 128i32);
                }
            }
        }
    }
    if !data_items.is_null() { free(data_items as *mut libc::c_void); }
    if !output.is_null() { free(output as *mut libc::c_void); }
    if !for_a.is_null() { free(for_a as *mut libc::c_void); }
    if !for_b.is_null() { free(for_b as *mut libc::c_void); }
    PublicKey_clear(pkA);
    PublicKey_clear(pkB);
    PrivateKey_clear(skA);
    PrivateKey_clear(skB);
    PrioVerifier_clear(vA);
    PrioVerifier_clear(vB);
    PrioTotalShare_clear(tA);
    PrioTotalShare_clear(tB);
    PrioServer_clear(sA);
    PrioServer_clear(sB);
    PrioConfig_clear(cfg);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_client__agg_1() {
    test_client_agg(1i32, 133i32, 0 != 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_client__agg_2() {
    test_client_agg(2i32, 133i32, 0 != 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_client__agg_10() {
    test_client_agg(10i32, 133i32, 0 != 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_client__agg_max() {
    let mut max: libc::c_int =
        if PrioConfig_maxDataFields() < 4000i32 {
            PrioConfig_maxDataFields()
        } else { 4000i32 };
    test_client_agg(10i32, max, 0 != 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_client__agg_max_bad() {
    let mut max: libc::c_int = PrioConfig_maxDataFields();
    test_client_agg(10i32, max + 1i32, 0 != 0i32);
}