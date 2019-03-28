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
    pub type PK11SymKeyStr;
    pub type PK11ContextStr;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn PK11_FreeSlot(slot: *mut PK11SlotInfo);
    #[no_mangle]
    fn PK11_GetInternalSlot() -> *mut PK11SlotInfo;
    /* *********************************************************************
 *                   Symmetric, Public, and Private Keys
 **********************************************************************/
    #[no_mangle]
    fn PK11_FreeSymKey(key: *mut PK11SymKey);
    #[no_mangle]
    fn PK11_ImportSymKey(slot: *mut PK11SlotInfo, type_0: CK_MECHANISM_TYPE,
                         origin: PK11Origin, operation: CK_ATTRIBUTE_TYPE,
                         key: *mut SECItem, wincx: *mut libc::c_void)
     -> *mut PK11SymKey;
    /* *********************************************************************
 *                   Crypto Contexts
 **********************************************************************/
    #[no_mangle]
    fn PK11_DestroyContext(context: *mut PK11Context, freeit: PRBool);
    #[no_mangle]
    fn PK11_CreateContextBySymKey(type_0: CK_MECHANISM_TYPE,
                                  operation: CK_ATTRIBUTE_TYPE,
                                  symKey: *mut PK11SymKey,
                                  param: *mut SECItem) -> *mut PK11Context;
    #[no_mangle]
    fn PK11_CipherOp(context: *mut PK11Context, out: *mut libc::c_uchar,
                     outlen: *mut libc::c_int, maxout: libc::c_int,
                     in_0: *const libc::c_uchar, inlen: libc::c_int)
     -> SECStatus;
    /*
 * Generate the specified number of random bytes using the
 * NSS random number generator.
 */
    #[no_mangle]
    fn rand_bytes(out: *mut libc::c_uchar, n_bytes: size_t) -> SECStatus;
    /* Memory management       */
    #[no_mangle]
    fn mp_init(mp: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_clear(mp: *mut mp_int);
    /* Full arithmetic         */
    #[no_mangle]
    fn mp_add(a: *const mp_int, b: *const mp_int, c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_sub(a: *const mp_int, b: *const mp_int, c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_submod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                 c: *mut mp_int) -> mp_err;
    /*
 * Generate a random number x such that
 *    0 <= x < max
 * using the specified randomness generator.
 *
 * The pointer user_data is passed to RandBytesFung `rng` as a first
 * argument.
 */
    #[no_mangle]
    fn rand_int_rng(out: *mut mp_int, max: *const mp_int, rng: RandBytesFunc,
                    user_data: *mut libc::c_void) -> SECStatus;
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
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* License to copy and use this software is granted provided that it is
 * identified as "RSA Security Inc. PKCS #11 Cryptographic Token Interface
 * (Cryptoki)" in all material mentioning or referencing this software.

 * License is also granted to make and use derivative works provided that
 * such works are identified as "derived from the RSA Security Inc. PKCS #11
 * Cryptographic Token Interface (Cryptoki)" in all material mentioning or
 * referencing the derived work.

 * RSA Security Inc. makes no representations concerning either the
 * merchantability of this software or the suitability of this software for
 * any particular purpose. It is provided "as is" without express or implied
 * warranty of any kind.
 */
/* an unsigned 8-bit value */
pub type CK_BYTE = libc::c_uchar;
/* an unsigned value, at least 32 bits long */
pub type CK_ULONG = libc::c_ulong;
/* CK_OBJECT_HANDLE is a token-specific identifier for an
 * object  */
pub type CK_OBJECT_HANDLE = CK_ULONG;
/* The following certificate types are defined: */
/* CKC_X_509_ATTR_CERT is new for v2.10 */
/* CKC_WTLS is new for v2.20 */
/* CK_ATTRIBUTE_TYPE is a value that identifies an attribute
 * type */
/* CK_ATTRIBUTE_TYPE was changed from CK_USHORT to CK_ULONG for
 * v2.0 */
pub type CK_ATTRIBUTE_TYPE = CK_ULONG;
/* CK_MECHANISM_TYPE is a value that identifies a mechanism
 * type */
/* CK_MECHANISM_TYPE was changed from CK_USHORT to CK_ULONG for
 * v2.0 */
pub type CK_MECHANISM_TYPE = CK_ULONG;
/* CK_AES_CTR_PARAMS is new for PKCS #11 v2.20 amendment 3 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CK_AES_CTR_PARAMS {
    pub ulCounterBits: CK_ULONG,
    pub cb: [CK_BYTE; 16],
}
/* defined in secmodti.h */
pub type PK11SlotInfo = PK11SlotInfoStr;
/* defined in secmodti.h */
pub type PK11SymKey = PK11SymKeyStr;
/* defined in secmodti.h */
pub type PK11Context = PK11ContextStr;
/*
 * PK11_ATTR_XXX
 *
 * The following PK11_ATTR_XXX bitflags are used to specify
 * PKCS #11 object attributes that have Boolean values.  Some NSS
 * functions have a "PK11AttrFlags attrFlags" parameter whose value
 * is the logical OR of these bitflags.  NSS use these bitflags on
 * private keys or secret keys.  Some of these bitflags also apply
 * to the public keys associated with the private keys.
 *
 * For each PKCS #11 object attribute, we need two bitflags to
 * specify not only "true" and "false" but also "default".  For
 * example, PK11_ATTR_PRIVATE and PK11_ATTR_PUBLIC control the
 * CKA_PRIVATE attribute.  If PK11_ATTR_PRIVATE is set, we add
 *     { CKA_PRIVATE, &cktrue, sizeof(CK_BBOOL) }
 * to the template.  If PK11_ATTR_PUBLIC is set, we add
 *     { CKA_PRIVATE, &ckfalse, sizeof(CK_BBOOL) }
 * to the template.  If neither flag is set, we don't add any
 * CKA_PRIVATE entry to the template.
 */
/*
 * Attributes for PKCS #11 storage objects, which include not only
 * keys but also certificates and domain parameters.
 */
/*
 * PK11_ATTR_TOKEN
 * PK11_ATTR_SESSION
 *
 * These two flags determine whether the object is a token or
 * session object.
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_TOKEN flag is set, the object is a token
 * object.  If the PK11_ATTR_SESSION flag is set, the object is
 * a session object.  If neither flag is set, the object is *by
 * default* a session object.
 *
 * These two flags specify the value of the PKCS #11 CKA_TOKEN
 * attribute.
 */
/*
 * PK11_ATTR_PRIVATE
 * PK11_ATTR_PUBLIC
 *
 * These two flags determine whether the object is a private or
 * public object.  A user may not access a private object until the
 * user has authenticated to the token.
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_PRIVATE flag is set, the object is a private
 * object.  If the PK11_ATTR_PUBLIC flag is set, the object is a
 * public object.  If neither flag is set, it is token-specific
 * whether the object is private or public.
 *
 * These two flags specify the value of the PKCS #11 CKA_PRIVATE
 * attribute.  NSS only uses this attribute on private and secret
 * keys, so public keys created by NSS get the token-specific
 * default value of the CKA_PRIVATE attribute.
 */
/*
 * PK11_ATTR_MODIFIABLE
 * PK11_ATTR_UNMODIFIABLE
 *
 * These two flags determine whether the object is modifiable or
 * read-only.
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_MODIFIABLE flag is set, the object can be
 * modified.  If the PK11_ATTR_UNMODIFIABLE flag is set, the object
 * is read-only.  If neither flag is set, the object is *by default*
 * modifiable.
 *
 * These two flags specify the value of the PKCS #11 CKA_MODIFIABLE
 * attribute.
 */
/* Attributes for PKCS #11 key objects. */
/*
 * PK11_ATTR_SENSITIVE
 * PK11_ATTR_INSENSITIVE
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_SENSITIVE flag is set, the key is sensitive.
 * If the PK11_ATTR_INSENSITIVE flag is set, the key is not
 * sensitive.  If neither flag is set, it is token-specific whether
 * the key is sensitive or not.
 *
 * If a key is sensitive, certain attributes of the key cannot be
 * revealed in plaintext outside the token.
 *
 * This flag specifies the value of the PKCS #11 CKA_SENSITIVE
 * attribute.  Although the default value of the CKA_SENSITIVE
 * attribute for secret keys is CK_FALSE per PKCS #11, some FIPS
 * tokens set the default value to CK_TRUE because only CK_TRUE
 * is allowed.  So in practice the default value of this attribute
 * is token-specific, hence the need for two bitflags.
 */
/*
 * PK11_ATTR_EXTRACTABLE
 * PK11_ATTR_UNEXTRACTABLE
 *
 * These two flags are related and cannot both be set.
 * If the PK11_ATTR_EXTRACTABLE flag is set, the key is extractable
 * and can be wrapped.  If the PK11_ATTR_UNEXTRACTABLE flag is set,
 * the key is not extractable, and certain attributes of the key
 * cannot be revealed in plaintext outside the token (just like a
 * sensitive key).  If neither flag is set, it is token-specific
 * whether the key is extractable or not.
 *
 * These two flags specify the value of the PKCS #11 CKA_EXTRACTABLE
 * attribute.
 */
/* Cryptographic module types */
/* external module */
/* internal default module */
/* internal fips module */
/* default module configuration strings */
/*
 * What is the origin of a given Key. Normally this doesn't matter, but
 * the fortezza code needs to know if it needs to invoke the SSL3 fortezza
 * hack.
 */
pub type PK11Origin = libc::c_uint;
/* Key was unwrapped or decrypted */
pub const PK11_OriginUnwrap: PK11Origin = 4;
/* Key was marked for fortezza hack */
pub const PK11_OriginFortezzaHack: PK11Origin = 3;
/* Key was generated (also PBE keys) */
pub const PK11_OriginGenerated: PK11Origin = 2;
/* Key was derived from some other key */
pub const PK11_OriginDerive: PK11Origin = 1;
/* There is not key, it's a null SymKey */
pub const PK11_OriginNULL: PK11Origin = 0;
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
pub type mp_err = libc::c_int;
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
pub struct prg {
    pub slot: *mut PK11SlotInfo,
    pub key: *mut PK11SymKey,
    pub ctx: *mut PK11Context,
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
pub type PRG = *mut prg;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
/*
 * Typedef for function pointer. A function pointer of type RandBytesFunc
 * points to a function that fills the buffer `out` of with `len` random bytes.
 */
pub type RandBytesFunc
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_uchar,
                                _: size_t) -> SECStatus>;
/*
 * Generate a new PRG seed using the NSS global randomness source.
 * Use this routine to initialize the secret that the two Prio servers
 * share.
 */
#[no_mangle]
pub unsafe extern "C" fn PrioPRGSeed_randomize(mut key: *mut PrioPRGSeed)
 -> SECStatus {
    return rand_bytes(key as *mut libc::c_uchar, 16i32 as size_t);
}
/*
 * Initialize or destroy a pseudo-random generator.
 */
#[no_mangle]
pub unsafe extern "C" fn PRG_new(mut key_in: *const libc::c_uchar) -> PRG {
    let mut key_mut: PrioPRGSeed = [0; 16];
    let mut keyItem: SECItem =
        SECItemStr{type_0: siBuffer, data: 0 as *mut libc::c_uchar, len: 0,};
    let mut param: CK_AES_CTR_PARAMS =
        CK_AES_CTR_PARAMS{ulCounterBits: 0, cb: [0; 16],};
    let mut paramItem: SECItem =
        SECItemStr{type_0: siBuffer, data: 0 as *mut libc::c_uchar, len: 0,};
    let mut prg: PRG =
        malloc(::std::mem::size_of::<prg>() as libc::c_ulong) as PRG;
    if prg.is_null() { return 0 as PRG }
    (*prg).slot = 0 as *mut PK11SlotInfo;
    (*prg).key = 0 as *mut PK11SymKey;
    (*prg).ctx = 0 as *mut PK11Context;
    let mut rv: SECStatus = SECSuccess;
    let cipher: CK_MECHANISM_TYPE = 0x1086i32 as CK_MECHANISM_TYPE;
    (*prg).slot = PK11_GetInternalSlot();
    if (*prg).slot.is_null() {
        rv = SECFailure
    } else {
        // Create a mutable copy of the key.
        key_mut = [0; 16];
        memcpy(key_mut.as_mut_ptr() as *mut libc::c_void,
               key_in as *const libc::c_void, 16i32 as libc::c_ulong);
        keyItem =
            SECItemStr{type_0: siBuffer,
                       data: key_mut.as_mut_ptr(),
                       len: 16i32 as libc::c_uint,};
        // The IV can be all zeros since we only encrypt once with
  // each AES key.
        param =
            CK_AES_CTR_PARAMS{ulCounterBits: 128i32 as CK_ULONG,
                              cb:
                                  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                   0, 0],};
        paramItem =
            SECItemStr{type_0: siBuffer,
                       data:
                           &mut param as *mut CK_AES_CTR_PARAMS as
                               *mut libc::c_void as *mut libc::c_uchar,
                       len:
                           ::std::mem::size_of::<CK_AES_CTR_PARAMS>() as
                               libc::c_ulong as libc::c_uint,};
        (*prg).key =
            PK11_ImportSymKey((*prg).slot, cipher, PK11_OriginUnwrap,
                              0x104i32 as CK_ATTRIBUTE_TYPE, &mut keyItem,
                              0 as *mut libc::c_void);
        if (*prg).key.is_null() {
            rv = SECFailure
        } else {
            (*prg).ctx =
                PK11_CreateContextBySymKey(cipher,
                                           0x104i32 as CK_ATTRIBUTE_TYPE,
                                           (*prg).key, &mut paramItem);
            if (*prg).ctx.is_null() { rv = SECFailure }
        }
    }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PRG_clear(prg);
        prg = 0 as PRG
    }
    return prg;
}
#[no_mangle]
pub unsafe extern "C" fn PRG_clear(mut prg: PRG) {
    if prg.is_null() { return }
    if !(*prg).key.is_null() { PK11_FreeSymKey((*prg).key); }
    if !(*prg).slot.is_null() { PK11_FreeSlot((*prg).slot); }
    if !(*prg).ctx.is_null() { PK11_DestroyContext((*prg).ctx, 1i32); }
    free(prg as *mut libc::c_void);
}
/*
 * Produce the next bytes of output from the PRG.
 */
#[no_mangle]
pub unsafe extern "C" fn PRG_get_bytes(mut prg: PRG,
                                       mut bytes: *mut libc::c_uchar,
                                       mut len: size_t) -> SECStatus {
    return PRG_get_bytes_internal(prg as *mut libc::c_void, bytes, len);
}
unsafe extern "C" fn PRG_get_bytes_internal(mut prg_vp: *mut libc::c_void,
                                            mut bytes: *mut libc::c_uchar,
                                            mut len: size_t) -> SECStatus {
    let mut outlen: libc::c_int = 0;
    let mut rv: SECStatus = SECSuccess;
    let mut prg: PRG = prg_vp as PRG;
    let mut in_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    in_0 =
        calloc(len, ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as *mut libc::c_uchar;
    if in_0.is_null() {
        rv = SECFailure
    } else {
        memset(in_0 as *mut libc::c_void, 0i32, len);
        outlen = 0;
        rv =
            PK11_CipherOp((*prg).ctx, bytes, &mut outlen, len as libc::c_int,
                          in_0, len as libc::c_int);
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            if !(outlen as size_t == len) { rv = SECFailure }
        }
    }
    if !in_0.is_null() { free(in_0 as *mut libc::c_void); }
    return rv;
}
/*
 * Use the PRG output to sample a big integer x in the range
 *    0 <= x < max.
 */
#[no_mangle]
pub unsafe extern "C" fn PRG_get_int(mut prg: PRG, mut out: *mut mp_int,
                                     mut max: *const mp_int) -> SECStatus {
    return rand_int_rng(out, max, Some(PRG_get_bytes_internal),
                        prg as *mut libc::c_void);
}
/*
 * Use the PRG output to sample a big integer x in the range
 *    lower <= x < max.
 */
#[no_mangle]
pub unsafe extern "C" fn PRG_get_int_range(mut prg: PRG, mut out: *mut mp_int,
                                           mut lower: *const mp_int,
                                           mut max: *const mp_int)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let mut width: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    width.dp = 0 as *mut mp_digit;
    if mp_init(&mut width) != 0i32 {
        rv = SECFailure
    } else if mp_sub(max, lower, &mut width) != 0i32 {
        rv = SECFailure
    } else {
        // Get an integer x in the range [0, width)
        rv = PRG_get_int(prg, out, &mut width);
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            // Set
  //    out = lower + x
  // which is in the range [lower, width+lower),
  // which is              [lower, max).
            if mp_add(lower, out, out) != 0i32 { rv = SECFailure }
        }
    }
    mp_clear(&mut width);
    return rv;
}
/*
 * Use secret sharing to split the int src into two shares.
 * Use PRG to generate the value `shareB`.
 * The mp_ints must be initialized.
 */
#[no_mangle]
pub unsafe extern "C" fn PRG_share_int(mut prgB: PRG, mut shareA: *mut mp_int,
                                       mut src: *const mp_int,
                                       mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let mut tmp: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    tmp.dp = 0 as *mut mp_digit;
    if mp_init(&mut tmp) != 0i32 {
        rv = SECFailure
    } else {
        rv = PRG_get_int(prgB, &mut tmp, &(*cfg).modulus);
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            if mp_submod(src, &mut tmp, &(*cfg).modulus, shareA) != 0i32 {
                rv = SECFailure
            }
        }
    }
    mp_clear(&mut tmp);
    return rv;
}
/*
 * Set each item in the array to a pseudorandom value in the range
 * [0, mod), where the values are generated using the PRG.
 */
#[no_mangle]
pub unsafe extern "C" fn PRG_get_array(mut prg: PRG, mut dst: MPArray,
                                       mut mod_0: *const mp_int)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let mut i: libc::c_int = 0i32;
    while i < (*dst).len {
        rv = PRG_get_int(prg, &mut *(*dst).data.offset(i as isize), mod_0);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        i += 1
    }
    return SECSuccess;
}
/*
 * Secret shares the array in `src` into `arrA` using randomness
 * provided by `prgB`. The arrays `src` and `arrA` must be the same
 * length.
 */
#[no_mangle]
pub unsafe extern "C" fn PRG_share_array(mut prgB: PRG, mut arrA: MPArray,
                                         mut src: const_MPArray,
                                         mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if (*arrA).len != (*src).len { return SECFailure }
    let len: libc::c_int = (*src).len;
    let mut i: libc::c_int = 0i32;
    while i < len {
        rv =
            PRG_share_int(prgB, &mut *(*arrA).data.offset(i as isize),
                          &mut *(*src).data.offset(i as isize), cfg);
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        i += 1
    }
    return rv;
}