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
    /*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
    pub type prg;
    /* Memory management       */
    #[no_mangle]
    fn mp_init(mp: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_copy(from: *const mp_int, to: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_clear(mp: *mut mp_int);
    #[no_mangle]
    fn mp_sub_d(a: *const mp_int, d: mp_digit, b: *mut mp_int) -> mp_err;
    /* Modular arithmetic      */
    #[no_mangle]
    fn mp_mod(a: *const mp_int, m: *const mp_int, c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_mulmod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                 c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn msgpack_unpacker_init(mpac: *mut msgpack_unpacker,
                             initial_buffer_size: size_t) -> bool;
    #[no_mangle]
    fn msgpack_unpacker_destroy(mpac: *mut msgpack_unpacker);
    #[no_mangle]
    fn BeaverTriple_clear(t: BeaverTriple);
    #[no_mangle]
    fn MPArray_clear(arr: MPArray);
    /*
 * Generate a new keypair for public-key encryption.
 */
    /*
 * Encrypt an arbitrary bitstring to the specified public key. The buffer
 * `output` should be large enough to store the ciphertext. Use the
 * `PublicKey_encryptSize()` function above to figure out how large of a buffer
 * you need.
 *
 * The value `inputLen` must be smaller than `MAX_ENCRYPT_LEN`.
 */
    #[no_mangle]
    fn PublicKey_encrypt(pubkey: PublicKey, output: *mut libc::c_uchar,
                         outputLen: *mut libc::c_uint,
                         maxOutputLen: libc::c_uint,
                         input: *const libc::c_uchar, inputLen: libc::c_uint)
     -> SECStatus;
    /*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
    /* ******
 * These functions attempt to implement CCA-secure public-key encryption using
 * the NSS library. We use hashed-ElGamal encryption with Curve25519 as the
 * underlying group and AES128-GCM as the bulk encryption mode of operation.
 *
 * I make no guarantees that I am using NSS correctly or that this encryption
 * scheme is actually CCA secure. As far as I can tell, NSS does not provide
 * any public-key hybrid encryption scheme out of the box, so I had to cook my
 * own. If you want to be really safe, you should use the NaCl Box routines
 * to implement these functions.
 */
    /*
 * Messages encrypted using this library must be smaller than MAX_ENCRYPT_LEN.
 * Enforcing this length limit helps avoid integer overflow.
 */
    /*
 * Write the number of bytes needed to store a ciphertext that encrypts a
 * plaintext message of length `inputLen` and authenticated data of length
 * `adLen` into the variable pointed to by `outputLen`. If `inputLen`
 * is too large (larger than `MAX_ENCRYPT_LEN`), this function returns
 * an error.
 */
    #[no_mangle]
    fn PublicKey_encryptSize(inputLen: libc::c_uint,
                             outputLen: *mut libc::c_uint) -> SECStatus;
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
    fn PRG_clear(prg: PRG);
    /*
 * Use secret sharing to split the int src into two shares.
 * Use PRG to generate the value `shareB`.
 * The mp_ints must be initialized.
 */
    #[no_mangle]
    fn PRG_share_int(prg: PRG, shareA: *mut mp_int, src: *const mp_int,
                     cfg: const_PrioConfig) -> SECStatus;
    /*
 * Use secret sharing to split the int src into two shares.
 * The mp_ints must be initialized.
 */
    #[no_mangle]
    fn share_int(cfg: const_PrioConfig, src: *const mp_int,
                 shareA: *mut mp_int, shareB: *mut mp_int) -> SECStatus;
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
 * Expands or shrinks the MPArray to the desired size. If shrinking,
 * will clear the values on the end of array.
 */
    #[no_mangle]
    fn MPArray_resize(arr: MPArray, newlen: libc::c_int) -> SECStatus;
    /*
 * Generate a random number x such that
 *    0 <= x < max
 * using the NSS random number generator.
 */
    #[no_mangle]
    fn rand_int(out: *mut mp_int, max: *const mp_int) -> SECStatus;
    /*
 * Initialize an array of `mp_int`s of the given length.
 */
    #[no_mangle]
    fn MPArray_new(len: libc::c_int) -> MPArray;
    /*
 * Initializes dst and creates a duplicate of the array in src.
 */
    #[no_mangle]
    fn MPArray_dup(src: const_MPArray) -> MPArray;
    /*
 * Secret shares the array in `src` into `arrA` using randomness
 * provided by `prgB`. The arrays `src` and `arrA` must be the same
 * length.
 */
    #[no_mangle]
    fn PRG_share_array(prgB: PRG, arrA: MPArray, src: const_MPArray,
                       cfg: const_PrioConfig) -> SECStatus;
    /*
 * Initializes array with 0/1 values specified in boolean array `data_in`
 */
    #[no_mangle]
    fn MPArray_new_bool(len: libc::c_int, data_in: *const bool) -> MPArray;
    #[no_mangle]
    fn BeaverTriple_set_rand(cfg: const_PrioConfig, triple_a: BeaverTriple,
                             triple_b: BeaverTriple) -> SECStatus;
    /*
 * Initialize or destroy a pseudo-random generator.
 */
    #[no_mangle]
    fn PRG_new(key: *const libc::c_uchar) -> PRG;
    /*
 * Generate a new PRG seed using the NSS global randomness source.
 * Use this routine to initialize the secret that the two Prio servers
 * share.
 */
    #[no_mangle]
    fn PrioPRGSeed_randomize(seed: *mut PrioPRGSeed) -> SECStatus;
    #[no_mangle]
    fn PrioConfig_hPoints(cfg: const_PrioConfig) -> libc::c_int;
    /*
 * Prio uses Beaver triples to implement one step of the
 * client data validation routine. A Beaver triple is just
 * a sharing of random values a, b, c such that
 *    a * b = c
 */
    #[no_mangle]
    fn BeaverTriple_new() -> BeaverTriple;
    /*
 * Return true iff the two arrays are equal in length
 * and contents. This comparison is NOT constant time.
 */
    #[no_mangle]
    fn MPArray_areEqual(arr1: const_MPArray, arr2: const_MPArray) -> bool;
    #[no_mangle]
    fn BeaverTriple_areEqual(t1: const_BeaverTriple, t2: const_BeaverTriple)
     -> bool;
    #[no_mangle]
    fn serial_read_packet_client(upk: *mut msgpack_unpacker,
                                 p: PrioPacketClient, cfg: const_PrioConfig)
     -> SECStatus;
    /*
 * Decrypt an arbitrary bitstring using the specified private key.  The output
 * buffer should be at least 16 bytes larger than the plaintext you expect. If
 * `outputLen` >= `inputLen`, you should be safe.
 */
    #[no_mangle]
    fn PrivateKey_decrypt(privkey: PrivateKey, output: *mut libc::c_uchar,
                          outputLen: *mut libc::c_uint,
                          maxOutputLen: libc::c_uint,
                          input: *const libc::c_uchar, inputLen: libc::c_uint)
     -> SECStatus;
}
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
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
pub type const_PrioConfig = *const prio_config;
pub type PrivateKey = *mut SECKEYPrivateKey;
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
pub type const_PrioPacketClient = *const prio_packet_client;
pub type PRG = *mut prg;
pub type const_MPArray = *const mparray;
pub type const_BeaverTriple = *const beaver_triple;
unsafe extern "C" fn msgpack_packer_init(mut pk: *mut msgpack_packer,
                                         mut data: *mut libc::c_void,
                                         mut callback: msgpack_packer_write) {
    (*pk).data = data;
    (*pk).callback = callback;
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
pub unsafe extern "C" fn PrioClient_encode(mut cfg: const_PrioConfig,
                                           mut data_in: *const bool,
                                           mut for_server_a:
                                               *mut *mut libc::c_uchar,
                                           mut aLen: *mut libc::c_uint,
                                           mut for_server_b:
                                               *mut *mut libc::c_uchar,
                                           mut bLen: *mut libc::c_uint)
 -> SECStatus {
    let mut writtenA: libc::c_uint = 0;
    let mut writtenB: libc::c_uint = 0;
    let mut rv: SECStatus = SECSuccess;
    let mut pA: PrioPacketClient = 0 as PrioPacketClient;
    let mut pB: PrioPacketClient = 0 as PrioPacketClient;
    *for_server_a = 0 as *mut libc::c_uchar;
    *for_server_b = 0 as *mut libc::c_uchar;
    let mut sbufA: msgpack_sbuffer =
        msgpack_sbuffer{size: 0, data: 0 as *mut libc::c_char, alloc: 0,};
    let mut sbufB: msgpack_sbuffer =
        msgpack_sbuffer{size: 0, data: 0 as *mut libc::c_char, alloc: 0,};
    let mut packerA: msgpack_packer =
        msgpack_packer{data: 0 as *mut libc::c_void, callback: None,};
    let mut packerB: msgpack_packer =
        msgpack_packer{data: 0 as *mut libc::c_void, callback: None,};
    msgpack_sbuffer_init(&mut sbufA);
    msgpack_sbuffer_init(&mut sbufB);
    msgpack_packer_init(&mut packerA,
                        &mut sbufA as *mut msgpack_sbuffer as
                            *mut libc::c_void, Some(msgpack_sbuffer_write));
    msgpack_packer_init(&mut packerB,
                        &mut sbufB as *mut msgpack_sbuffer as
                            *mut libc::c_void, Some(msgpack_sbuffer_write));
    pA = PrioPacketClient_new(cfg, PRIO_SERVER_A);
    if pA.is_null() {
        rv = SECFailure
    } else {
        pB = PrioPacketClient_new(cfg, PRIO_SERVER_B);
        if pB.is_null() {
            rv = SECFailure
        } else {
            rv = PrioPacketClient_set_data(cfg, data_in, pA, pB);
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                rv =
                    serial_write_packet_client(&mut packerA,
                                               pA as const_PrioPacketClient,
                                               cfg);
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    rv =
                        serial_write_packet_client(&mut packerB,
                                                   pB as
                                                       const_PrioPacketClient,
                                                   cfg);
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        rv =
                            PublicKey_encryptSize(sbufA.size as libc::c_uint,
                                                  aLen);
                        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                            rv =
                                PublicKey_encryptSize(sbufB.size as
                                                          libc::c_uint, bLen);
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                *for_server_a =
                                    malloc(*aLen as libc::c_ulong) as
                                        *mut libc::c_uchar;
                                if (*for_server_a).is_null() {
                                    rv = SECFailure
                                } else {
                                    *for_server_b =
                                        malloc(*bLen as libc::c_ulong) as
                                            *mut libc::c_uchar;
                                    if (*for_server_b).is_null() {
                                        rv = SECFailure
                                    } else {
                                        writtenA = 0;
                                        writtenB = 0;
                                        rv =
                                            PublicKey_encrypt((*cfg).server_a_pub,
                                                              *for_server_a,
                                                              &mut writtenA,
                                                              *aLen,
                                                              sbufA.data as
                                                                  *mut libc::c_uchar,
                                                              sbufA.size as
                                                                  libc::c_uint);
                                        if !(rv as libc::c_int !=
                                                 SECSuccess as libc::c_int) {
                                            rv =
                                                PublicKey_encrypt((*cfg).server_b_pub,
                                                                  *for_server_b,
                                                                  &mut writtenB,
                                                                  *bLen,
                                                                  sbufB.data
                                                                      as
                                                                      *mut libc::c_uchar,
                                                                  sbufB.size
                                                                      as
                                                                      libc::c_uint);
                                            if !(rv as libc::c_int !=
                                                     SECSuccess as
                                                         libc::c_int) {
                                                if !(writtenA == *aLen) {
                                                    rv = SECFailure
                                                } else if !(writtenB == *bLen)
                                                 {
                                                    rv = SECFailure
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
        if !(*for_server_a).is_null() {
            free(*for_server_a as *mut libc::c_void);
        }
        if !(*for_server_b).is_null() {
            free(*for_server_b as *mut libc::c_void);
        }
        *for_server_a = 0 as *mut libc::c_uchar;
        *for_server_b = 0 as *mut libc::c_uchar
    }
    PrioPacketClient_clear(pA);
    PrioPacketClient_clear(pB);
    msgpack_sbuffer_destroy(&mut sbufA);
    msgpack_sbuffer_destroy(&mut sbufB);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketClient_clear(mut p: PrioPacketClient) {
    if p.is_null() { return }
    if (*p).for_server as libc::c_uint ==
           PRIO_SERVER_A as libc::c_int as libc::c_uint {
        MPArray_clear((*p).shares.A.h_points);
        MPArray_clear((*p).shares.A.data_shares);
    }
    BeaverTriple_clear((*p).triple);
    mp_clear(&mut (*p).f0_share);
    mp_clear(&mut (*p).g0_share);
    mp_clear(&mut (*p).h0_share);
    free(p as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketClient_set_data(mut cfg: const_PrioConfig,
                                                   mut data_in: *const bool,
                                                   mut pA: PrioPacketClient,
                                                   mut pB: PrioPacketClient)
 -> SECStatus {
    let mut client_data: MPArray = 0 as MPArray;
    let mut prgB: PRG = 0 as PRG;
    let mut rv: SECStatus = SECSuccess;
    let data_len: libc::c_int = (*cfg).num_data_fields;
    if data_in.is_null() { return SECFailure }
    rv = PrioPRGSeed_randomize(&mut (*pB).shares.B.seed);
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        prgB =
            PRG_new((*pB).shares.B.seed.as_mut_ptr() as *const libc::c_uchar);
        if prgB.is_null() {
            rv = SECFailure
        } else {
            rv = BeaverTriple_set_rand(cfg, (*pA).triple, (*pB).triple);
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                client_data = MPArray_new_bool(data_len, data_in);
                if client_data.is_null() {
                    rv = SECFailure
                } else {
                    rv =
                        PRG_share_array(prgB, (*pA).shares.A.data_shares,
                                        client_data as const_MPArray, cfg);
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        rv =
                            share_polynomials(cfg,
                                              client_data as const_MPArray,
                                              pA, pB, prgB);
                        rv as libc::c_int != SECSuccess as libc::c_int;
                    }
                }
            }
        }
    }
    MPArray_clear(client_data);
    PRG_clear(prgB);
    return rv;
}
unsafe extern "C" fn share_polynomials(mut cfg: const_PrioConfig,
                                       mut data_in: const_MPArray,
                                       mut pA: PrioPacketClient,
                                       mut pB: PrioPacketClient,
                                       mut prgB: PRG) -> SECStatus {
    let mut j: libc::c_int = 0;
    let mut current_block: u64;
    let mut rv: SECStatus = SECSuccess;
    let mut mod_0: *const mp_int = &(*cfg).modulus;
    let mut points_f: const_MPArray = data_in;
    let mut f0: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut g0: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    f0.dp = 0 as *mut mp_digit;
    g0.dp = 0 as *mut mp_digit;
    let mut points_g: MPArray = 0 as MPArray;
    let mut evals_f_2N: MPArray = 0 as MPArray;
    let mut evals_g_2N: MPArray = 0 as MPArray;
    points_g = MPArray_dup(points_f);
    if points_g.is_null() {
        rv = SECFailure
    } else {
        evals_f_2N = MPArray_new(0i32);
        if evals_f_2N.is_null() {
            rv = SECFailure
        } else {
            evals_g_2N = MPArray_new(0i32);
            if evals_g_2N.is_null() {
                rv = SECFailure
            } else if mp_init(&mut f0) != 0i32 {
                rv = SECFailure
            } else if mp_init(&mut g0) != 0i32 {
                rv = SECFailure
            } else {
                let mut i: libc::c_int = 0i32;
                loop  {
                    if !(i < (*points_f).len) {
                        current_block = 6717214610478484138;
                        break ;
                    }
                    // For each input value x_i, we compute x_i * (x_i-1).
    //    f(i) = x_i
    //    g(i) = x_i - 1
                    if mp_sub_d(&mut *(*points_g).data.offset(i as isize),
                                1i32 as mp_digit,
                                &mut *(*points_g).data.offset(i as isize)) !=
                           0i32 {
                        rv = SECFailure;
                        current_block = 15267125253105024320;
                        break ;
                    } else if mp_mod(&mut *(*points_g).data.offset(i as
                                                                       isize),
                                     mod_0,
                                     &mut *(*points_g).data.offset(i as
                                                                       isize))
                                  != 0i32 {
                        rv = SECFailure;
                        current_block = 15267125253105024320;
                        break ;
                    } else { i += 1 }
                }
                match current_block {
                    15267125253105024320 => { }
                    _ => {
                        rv =
                            data_polynomial_evals(cfg, points_f, evals_f_2N,
                                                  &mut f0);
                        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                            rv =
                                data_polynomial_evals(cfg,
                                                      points_g as
                                                          const_MPArray,
                                                      evals_g_2N, &mut g0);
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                // The values f(0) and g(0) are set to random values.
  // We must send to each server a share of the points
  //    f(0),   g(0),   and   h(0) = f(0)*g(0)
                                rv =
                                    share_int(cfg, &mut f0,
                                              &mut (*pA).f0_share,
                                              &mut (*pB).f0_share);
                                if !(rv as libc::c_int !=
                                         SECSuccess as libc::c_int) {
                                    rv =
                                        share_int(cfg, &mut g0,
                                                  &mut (*pA).g0_share,
                                                  &mut (*pB).g0_share);
                                    if !(rv as libc::c_int !=
                                             SECSuccess as libc::c_int) {
                                        // Compute h(0) = f(0)*g(0).
                                        if mp_mulmod(&mut f0, &mut g0, mod_0,
                                                     &mut f0) != 0i32 {
                                            rv = SECFailure
                                        } else {
                                            // Give one share of h(0) to each server.
                                            rv =
                                                share_int(cfg, &mut f0,
                                                          &mut (*pA).h0_share,
                                                          &mut (*pB).h0_share);
                                            if !(rv as libc::c_int !=
                                                     SECSuccess as
                                                         libc::c_int) {
                                                // const int lenN = (evals_f_2N->len/2);
  // P_CHECKC (MPArray_resize (pA->shares.A.h_points, lenN));
                                                // We need to send to the servers the evaluations of
  //   f(r) * g(r)
  // for all 2N-th roots of unity r that are not also
  // N-th roots of unity.
  //
  // For each such root r, compute h(r) = f(r)*g(r) and
  // send a share of this value to each server.
                                                j = 0i32;
                                                let mut i_0: libc::c_int =
                                                    1i32;
                                                while i_0 < (*evals_f_2N).len
                                                      {
                                                    if mp_mulmod(&mut *(*evals_f_2N).data.offset(i_0
                                                                                                     as
                                                                                                     isize),
                                                                 &mut *(*evals_g_2N).data.offset(i_0
                                                                                                     as
                                                                                                     isize),
                                                                 mod_0,
                                                                 &mut f0) !=
                                                           0i32 {
                                                        rv = SECFailure;
                                                        break ;
                                                    } else {
                                                        rv =
                                                            PRG_share_int(prgB,
                                                                          &mut *(*(*pA).shares.A.h_points).data.offset(j
                                                                                                                           as
                                                                                                                           isize),
                                                                          &mut f0,
                                                                          cfg);
                                                        if rv as libc::c_int
                                                               !=
                                                               SECSuccess as
                                                                   libc::c_int
                                                           {
                                                            break ;
                                                        }
                                                        j += 1;
                                                        i_0 += 2i32
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
    MPArray_clear(evals_f_2N);
    MPArray_clear(evals_g_2N);
    MPArray_clear(points_g);
    mp_clear(&mut f0);
    mp_clear(&mut g0);
    return rv;
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
// Let the points of data_in be [x1, x2, x3, ... ].
// We construct the polynomial f such that
// (a)    f(0) = random,
// (b)    f(i) = x_i  for all i >= 1,
// (c)    degree(f)+1 is a power of two.
// We then evaluate f at the 2N-th roots of unity
// and we return these evaluations as `evals_out`
// and we return f(0) as `const_term`.
unsafe extern "C" fn data_polynomial_evals(mut cfg: const_PrioConfig,
                                           mut data_in: const_MPArray,
                                           mut evals_out: MPArray,
                                           mut const_term: *mut mp_int)
 -> SECStatus {
    let mut current_block: u64;
    let mut rv: SECStatus = SECSuccess;
    let mut mod_0: *const mp_int = &(*cfg).modulus;
    let mut points_f: MPArray = 0 as MPArray;
    let mut poly_f: MPArray = 0 as MPArray;
    // Number of multiplication gates in the Valid() circuit.
    let mul_gates: libc::c_int = (*cfg).num_data_fields;
    // Little n is the number of points on the polynomials.
  // The constant term is randomized, so it's (mul_gates + 1).
    let n: libc::c_int = mul_gates + 1i32;
    // Big N is n rounded up to a power of two.
    let N: libc::c_int = next_power_of_two(n);
    points_f = MPArray_new(N);
    if points_f.is_null() {
        rv = SECFailure
    } else {
        poly_f = MPArray_new(N);
        if poly_f.is_null() {
            rv = SECFailure
        } else {
            // Set constant term f(0) to random
            rv = rand_int(&mut *(*points_f).data.offset(0isize), mod_0);
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                if mp_copy(&mut *(*points_f).data.offset(0isize), const_term)
                       != 0i32 {
                    rv = SECFailure
                } else {
                    // Set other values of f(x)
                    let mut i: libc::c_int = 1i32;
                    loop  {
                        if !(i < n) {
                            current_block = 5494826135382683477;
                            break ;
                        }
                        if mp_copy(&mut *(*data_in).data.offset((i - 1i32) as
                                                                    isize),
                                   &mut *(*points_f).data.offset(i as isize))
                               != 0i32 {
                            rv = SECFailure;
                            current_block = 2166775942784062216;
                            break ;
                        } else { i += 1 }
                    }
                    match current_block {
                        2166775942784062216 => { }
                        _ => {
                            // Interpolate through the Nth roots of unity
                            rv =
                                poly_fft(poly_f, points_f as const_MPArray,
                                         cfg, 0 != 1i32);
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                // Evaluate at all 2N-th roots of unity.
  // To do so, first resize the eval arrays and fill upper
  // values with zeros.
                                rv = MPArray_resize(poly_f, 2i32 * N);
                                if !(rv as libc::c_int !=
                                         SECSuccess as libc::c_int) {
                                    rv = MPArray_resize(evals_out, 2i32 * N);
                                    if !(rv as libc::c_int !=
                                             SECSuccess as libc::c_int) {
                                        // Evaluate at the 2N-th roots of unity
                                        rv =
                                            poly_fft(evals_out,
                                                     poly_f as const_MPArray,
                                                     cfg, 0 != 0i32);
                                        rv as libc::c_int !=
                                            SECSuccess as libc::c_int;
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
    MPArray_clear(poly_f);
    return rv;
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
#[no_mangle]
pub unsafe extern "C" fn PrioPacketClient_new(mut cfg: const_PrioConfig,
                                              mut for_server: PrioServerId)
 -> PrioPacketClient {
    let mut current_block: u64;
    let mut rv: SECStatus = SECSuccess;
    let data_len: libc::c_int = (*cfg).num_data_fields;
    let mut p: PrioPacketClient = 0 as PrioPacketClient;
    p =
        malloc(::std::mem::size_of::<prio_packet_client>() as libc::c_ulong)
            as PrioPacketClient;
    if p.is_null() { return 0 as PrioPacketClient }
    (*p).for_server = for_server;
    (*p).triple = 0 as BeaverTriple;
    (*p).f0_share.dp = 0 as *mut mp_digit;
    (*p).g0_share.dp = 0 as *mut mp_digit;
    (*p).h0_share.dp = 0 as *mut mp_digit;
    match (*p).for_server as libc::c_uint {
        0 => {
            (*p).shares.A.data_shares = 0 as MPArray;
            (*p).shares.A.h_points = 0 as MPArray;
            current_block = 1054647088692577877;
        }
        1 => {
            memset((*p).shares.B.seed.as_mut_ptr() as *mut libc::c_void, 0i32,
                   16i32 as libc::c_ulong);
            current_block = 1054647088692577877;
        }
        _ => { rv = SECFailure; current_block = 15206252865582047828; }
    }
    match current_block {
        1054647088692577877 => {
            if mp_init(&mut (*p).f0_share) != 0i32 {
                rv = SECFailure
            } else if mp_init(&mut (*p).g0_share) != 0i32 {
                rv = SECFailure
            } else if mp_init(&mut (*p).h0_share) != 0i32 {
                rv = SECFailure
            } else {
                (*p).triple = BeaverTriple_new();
                if (*p).triple.is_null() {
                    rv = SECFailure
                } else if (*p).for_server as libc::c_uint ==
                              PRIO_SERVER_A as libc::c_int as libc::c_uint {
                    let num_h_points: libc::c_int = PrioConfig_hPoints(cfg);
                    (*p).shares.A.data_shares = MPArray_new(data_len);
                    if (*p).shares.A.data_shares.is_null() {
                        rv = SECFailure
                    } else {
                        (*p).shares.A.h_points = MPArray_new(num_h_points);
                        if (*p).shares.A.h_points.is_null() {
                            rv = SECFailure
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PrioPacketClient_clear(p);
        return 0 as PrioPacketClient
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketClient_decrypt(mut p: PrioPacketClient,
                                                  mut cfg: const_PrioConfig,
                                                  mut server_priv: PrivateKey,
                                                  mut data_in:
                                                      *const libc::c_uchar,
                                                  mut data_len: libc::c_uint)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let mut upk: msgpack_unpacker =
        msgpack_unpacker{buffer: 0 as *mut libc::c_char,
                         used: 0,
                         free: 0,
                         off: 0,
                         parsed: 0,
                         z: 0 as *mut msgpack_zone,
                         initial_buffer_size: 0,
                         ctx: 0 as *mut libc::c_void,};
    if !msgpack_unpacker_init(&mut upk, data_len as size_t) {
        return SECFailure
    }
    // Decrypt the ciphertext into dec_buf
    let mut bytes_decrypted: libc::c_uint = 0;
    rv =
        PrivateKey_decrypt(server_priv,
                           msgpack_unpacker_buffer(&mut upk) as
                               *mut libc::c_uchar, &mut bytes_decrypted,
                           data_len, data_in, data_len);
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        msgpack_unpacker_buffer_consumed(&mut upk, bytes_decrypted as size_t);
        rv = serial_read_packet_client(&mut upk, p, cfg);
        rv as libc::c_int != SECSuccess as libc::c_int;
    }
    msgpack_unpacker_destroy(&mut upk);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn PrioPacketClient_areEqual(mut p1:
                                                       const_PrioPacketClient,
                                                   mut p2:
                                                       const_PrioPacketClient)
 -> bool {
    if !BeaverTriple_areEqual((*p1).triple as const_BeaverTriple,
                              (*p2).triple as const_BeaverTriple) {
        return 0 != 0i32
    }
    if 0 != mp_cmp(&(*p1).f0_share, &(*p2).f0_share) { return 0 != 0i32 }
    if 0 != mp_cmp(&(*p1).g0_share, &(*p2).g0_share) { return 0 != 0i32 }
    if 0 != mp_cmp(&(*p1).h0_share, &(*p2).h0_share) { return 0 != 0i32 }
    if (*p1).for_server as libc::c_uint != (*p2).for_server as libc::c_uint {
        return 0 != 0i32
    }
    match (*p1).for_server as libc::c_uint {
        0 => {
            if !MPArray_areEqual((*p1).shares.A.data_shares as const_MPArray,
                                 (*p2).shares.A.data_shares as const_MPArray)
               {
                return 0 != 0i32
            }
            if !MPArray_areEqual((*p1).shares.A.h_points as const_MPArray,
                                 (*p2).shares.A.h_points as const_MPArray) {
                return 0 != 0i32
            }
        }
        1 => {
            if 0 !=
                   memcmp((*p1).shares.B.seed.as_ptr() as *const libc::c_void,
                          (*p2).shares.B.seed.as_ptr() as *const libc::c_void,
                          16i32 as libc::c_ulong) {
                return 0 != 0i32
            }
        }
        _ => { return 0 != 0i32 }
    }
    return 0 != 1i32;
}