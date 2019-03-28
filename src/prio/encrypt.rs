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
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    /*
** Zero and then free "zap". If freeit is PR_TRUE then "zap" itself is freed.
*/
    #[no_mangle]
    fn SECITEM_ZfreeItem(zap: *mut SECItem, freeit: PRBool);
    #[no_mangle]
    fn SECOID_FindOIDByTag(tagnum: SECOidTag) -> *mut SECOidData;
    /* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
    /*#include "secpkcs5.h" */
    /*
** Destroy a subject-public-key-info object.
*/
    #[no_mangle]
    fn SECKEY_DestroySubjectPublicKeyInfo(spki:
                                              *mut CERTSubjectPublicKeyInfo);
    /*
** Decode a DER encoded subject public key info into a
** CERTSubjectPublicKeyInfo structure.
*/
    #[no_mangle]
    fn SECKEY_DecodeDERSubjectPublicKeyInfo(spkider: *const SECItem)
     -> *mut CERTSubjectPublicKeyInfo;
    /*
 * extract the public key from a subject Public Key info structure.
 * (used by JSS).
 */
    #[no_mangle]
    fn SECKEY_ExtractPublicKey(_: *const CERTSubjectPublicKeyInfo)
     -> *mut SECKEYPublicKey;
    /*
** Destroy a private key object.
**	"key" the object
*/
    #[no_mangle]
    fn SECKEY_DestroyPrivateKey(key: *mut SECKEYPrivateKey);
    /*
** Destroy a public key object.
**	"key" the object
*/
    #[no_mangle]
    fn SECKEY_DestroyPublicKey(key: *mut SECKEYPublicKey);
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
    fn PK11_PubDeriveWithKDF(privKey: *mut SECKEYPrivateKey,
                             pubKey: *mut SECKEYPublicKey, isSender: PRBool,
                             randomA: *mut SECItem, randomB: *mut SECItem,
                             derive: CK_MECHANISM_TYPE,
                             target: CK_MECHANISM_TYPE,
                             operation: CK_ATTRIBUTE_TYPE,
                             keySize: libc::c_int, kdf: CK_ULONG,
                             sharedData: *mut SECItem,
                             wincx: *mut libc::c_void) -> *mut PK11SymKey;
    #[no_mangle]
    fn PK11_GenerateKeyPair(slot: *mut PK11SlotInfo,
                            type_0: CK_MECHANISM_TYPE,
                            param: *mut libc::c_void,
                            pubk: *mut *mut SECKEYPublicKey, isPerm: PRBool,
                            isSensitive: PRBool, wincx: *mut libc::c_void)
     -> *mut SECKEYPrivateKey;
    #[no_mangle]
    fn PK11_Decrypt(symkey: *mut PK11SymKey, mechanism: CK_MECHANISM_TYPE,
                    param: *mut SECItem, out: *mut libc::c_uchar,
                    outLen: *mut libc::c_uint, maxLen: libc::c_uint,
                    enc: *const libc::c_uchar, encLen: libc::c_uint)
     -> SECStatus;
    #[no_mangle]
    fn PK11_Encrypt(symKey: *mut PK11SymKey, mechanism: CK_MECHANISM_TYPE,
                    param: *mut SECItem, out: *mut libc::c_uchar,
                    outLen: *mut libc::c_uint, maxLen: libc::c_uint,
                    data: *const libc::c_uchar, dataLen: libc::c_uint)
     -> SECStatus;
    #[no_mangle]
    fn PK11_ImportDERPrivateKeyInfoAndReturnKey(slot: *mut PK11SlotInfo,
                                                derPKI: *mut SECItem,
                                                nickname: *mut SECItem,
                                                publicValue: *mut SECItem,
                                                isPerm: PRBool,
                                                isPrivate: PRBool,
                                                usage: libc::c_uint,
                                                privk:
                                                    *mut *mut SECKEYPrivateKey,
                                                wincx: *mut libc::c_void)
     -> SECStatus;
    /*
 * PK11_ReadRawAttribute and PK11_WriteRawAttribute are generic
 * functions to read and modify the actual PKCS #11 attributes of
 * the underlying pkcs #11 object.
 *
 * object is a pointer to an NSS object that represents the underlying
 *  PKCS #11 object. It's type must match the type of PK11ObjectType
 *  as follows:
 *
 *     type                           object
 *   PK11_TypeGeneric            PK11GenericObject *
 *   PK11_TypePrivKey            SECKEYPrivateKey *
 *   PK11_TypePubKey             SECKEYPublicKey *
 *   PK11_TypeSymKey             PK11SymKey *
 *
 *  All other types are considered invalid. If type does not match the object
 *  passed, unpredictable results will occur.
 *
 * PK11_ReadRawAttribute allocates the buffer for returning the attribute
 * value.  The caller of PK11_ReadRawAttribute should free the data buffer
 * pointed to by item using a SECITEM_FreeItem(item, PR_FALSE) or
 * PORT_Free(item->data) call.
 */
    #[no_mangle]
    fn PK11_ReadRawAttribute(type_0: PK11ObjectType,
                             object: *mut libc::c_void,
                             attr: CK_ATTRIBUTE_TYPE, item: *mut SECItem)
     -> SECStatus;
    /*
 * Generate the specified number of random bytes using the
 * NSS random number generator.
 */
    #[no_mangle]
    fn rand_bytes(out: *mut libc::c_uchar, n_bytes: size_t) -> SECStatus;
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
pub type uint8_t = libc::c_uchar;
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
pub struct SECOidDataStr {
    pub oid: SECItem,
    pub offset: SECOidTag,
    pub desc: *const libc::c_char,
    pub mechanism: libc::c_ulong,
    pub supportedExtension: SECSupportExtenTag,
}
/* fake OID for DSS sign/verify */
pub type SECSupportExtenTag = libc::c_uint;
pub const SUPPORTED_CERT_EXTENSION: SECSupportExtenTag = 2;
pub const UNSUPPORTED_CERT_EXTENSION: SECSupportExtenTag = 1;
pub const INVALID_CERT_EXTENSION: SECSupportExtenTag = 0;
/*
 * Misc object IDs - these numbers are for convenient handling.
 * They are mapped into real object IDs
 *
 * NOTE: the order of these entries must mach the array "oids" of SECOidData
 * in util/secoid.c.
 */
pub type SECOidTag = libc::c_uint;
pub const SEC_OID_TOTAL: SECOidTag = 361;
pub const SEC_OID_IPSEC_IKE_INTERMEDIATE: SECOidTag = 360;
pub const SEC_OID_IPSEC_IKE_END: SECOidTag = 359;
pub const SEC_OID_EXT_KEY_USAGE_IPSEC_IKE: SECOidTag = 358;
pub const SEC_OID_X509_ANY_EXT_KEY_USAGE: SECOidTag = 357;
pub const SEC_OID_TLS13_KEA_ANY: SECOidTag = 356;
pub const SEC_OID_CURVE25519: SECOidTag = 355;
pub const SEC_OID_TLS_DHE_CUSTOM: SECOidTag = 354;
pub const SEC_OID_TLS_FFDHE_8192: SECOidTag = 353;
pub const SEC_OID_TLS_FFDHE_6144: SECOidTag = 352;
pub const SEC_OID_TLS_FFDHE_4096: SECOidTag = 351;
pub const SEC_OID_TLS_FFDHE_3072: SECOidTag = 350;
pub const SEC_OID_TLS_FFDHE_2048: SECOidTag = 349;
pub const SEC_OID_TLS_DHE_PSK: SECOidTag = 348;
pub const SEC_OID_TLS_ECDHE_PSK: SECOidTag = 347;
pub const SEC_OID_CHACHA20_POLY1305: SECOidTag = 346;
pub const SEC_OID_APPLY_SSL_POLICY: SECOidTag = 345;
pub const SEC_OID_TLS_DH_ANON_EXPORT: SECOidTag = 344;
pub const SEC_OID_TLS_DH_DSS_EXPORT: SECOidTag = 343;
pub const SEC_OID_TLS_DH_RSA_EXPORT: SECOidTag = 342;
pub const SEC_OID_TLS_DHE_DSS_EXPORT: SECOidTag = 341;
pub const SEC_OID_TLS_DHE_RSA_EXPORT: SECOidTag = 340;
pub const SEC_OID_TLS_RSA_EXPORT: SECOidTag = 339;
pub const SEC_OID_TLS_ECDH_ANON: SECOidTag = 338;
pub const SEC_OID_TLS_ECDH_RSA: SECOidTag = 337;
pub const SEC_OID_TLS_ECDH_ECDSA: SECOidTag = 336;
pub const SEC_OID_TLS_ECDHE_RSA: SECOidTag = 335;
pub const SEC_OID_TLS_ECDHE_ECDSA: SECOidTag = 334;
pub const SEC_OID_TLS_DH_ANON: SECOidTag = 333;
pub const SEC_OID_TLS_DH_DSS: SECOidTag = 332;
pub const SEC_OID_TLS_DH_RSA: SECOidTag = 331;
pub const SEC_OID_TLS_DHE_DSS: SECOidTag = 330;
pub const SEC_OID_TLS_DHE_RSA: SECOidTag = 329;
pub const SEC_OID_TLS_RSA: SECOidTag = 328;
pub const SEC_OID_HMAC_MD5: SECOidTag = 327;
pub const SEC_OID_NULL_CIPHER: SECOidTag = 326;
pub const SEC_OID_RC4_56: SECOidTag = 325;
pub const SEC_OID_RC4_40: SECOidTag = 324;
pub const SEC_OID_DES_40_CBC: SECOidTag = 323;
/* pseudo - OIDs */
pub const SEC_OID_RC2_40_CBC: SECOidTag = 322;
pub const SEC_OID_IDEA_CBC: SECOidTag = 321;
pub const SEC_OID_AES_256_GCM: SECOidTag = 320;
pub const SEC_OID_AES_192_GCM: SECOidTag = 319;
pub const SEC_OID_AES_128_GCM: SECOidTag = 318;
/* The 'name' attribute type in X.520 */
pub const SEC_OID_AVA_NAME: SECOidTag = 317;
/* Microsoft Trust List Signing
     * szOID_KP_CTL_USAGE_SIGNING
     * where KP stands for Key Purpose
     */
pub const SEC_OID_MS_EXT_KEY_USAGE_CTL_SIGNING: SECOidTag = 316;
pub const SEC_OID_NIST_DSA_SIGNATURE_WITH_SHA256_DIGEST: SECOidTag = 315;
pub const SEC_OID_NIST_DSA_SIGNATURE_WITH_SHA224_DIGEST: SECOidTag = 314;
pub const SEC_OID_BUSINESS_CATEGORY: SECOidTag = 313;
pub const SEC_OID_EV_INCORPORATION_COUNTRY: SECOidTag = 312;
pub const SEC_OID_EV_INCORPORATION_STATE: SECOidTag = 311;
pub const SEC_OID_EV_INCORPORATION_LOCALITY: SECOidTag = 310;
pub const SEC_OID_SHA224: SECOidTag = 309;
pub const SEC_OID_PKCS1_SHA224_WITH_RSA_ENCRYPTION: SECOidTag = 308;
pub const SEC_OID_PKCS1_RSA_PSS_SIGNATURE: SECOidTag = 307;
pub const SEC_OID_PKCS1_PSPECIFIED: SECOidTag = 306;
pub const SEC_OID_PKCS1_MGF1: SECOidTag = 305;
pub const SEC_OID_PKCS1_RSA_OAEP_ENCRYPTION: SECOidTag = 304;
pub const SEC_OID_X509_ANY_POLICY: SECOidTag = 303;
pub const SEC_OID_SEED_CBC: SECOidTag = 302;
pub const SEC_OID_ISO_SHA1_WITH_RSA_SIGNATURE: SECOidTag = 301;
pub const SEC_OID_PKIX_CA_REPOSITORY: SECOidTag = 300;
pub const SEC_OID_PKIX_TIMESTAMPING: SECOidTag = 299;
pub const SEC_OID_HMAC_SHA512: SECOidTag = 298;
pub const SEC_OID_HMAC_SHA384: SECOidTag = 297;
pub const SEC_OID_HMAC_SHA256: SECOidTag = 296;
pub const SEC_OID_HMAC_SHA224: SECOidTag = 295;
pub const SEC_OID_HMAC_SHA1: SECOidTag = 294;
pub const SEC_OID_PKCS5_PBMAC1: SECOidTag = 293;
pub const SEC_OID_PKCS5_PBES2: SECOidTag = 292;
/* PKCS 5 V2 OIDS */
pub const SEC_OID_PKCS5_PBKDF2: SECOidTag = 291;
pub const SEC_OID_CAMELLIA_256_CBC: SECOidTag = 290;
pub const SEC_OID_CAMELLIA_192_CBC: SECOidTag = 289;
/* Camellia OIDs (RFC3657)*/
pub const SEC_OID_CAMELLIA_128_CBC: SECOidTag = 288;
pub const SEC_OID_X509_SUBJECT_INFO_ACCESS: SECOidTag = 287;
pub const SEC_OID_X509_INHIBIT_ANY_POLICY: SECOidTag = 286;
pub const SEC_OID_X509_FRESHEST_CRL: SECOidTag = 285;
pub const SEC_OID_X509_CERT_ISSUER: SECOidTag = 284;
pub const SEC_OID_X509_ISSUING_DISTRIBUTION_POINT: SECOidTag = 283;
pub const SEC_OID_X509_DELTA_CRL_INDICATOR: SECOidTag = 282;
/* More id-ce and id-pe OIDs from RFC 3280 */
pub const SEC_OID_X509_HOLD_INSTRUCTION_CODE: SECOidTag = 281;
pub const SEC_OID_ANSIX962_ECDSA_SHA512_SIGNATURE: SECOidTag = 280;
pub const SEC_OID_ANSIX962_ECDSA_SHA384_SIGNATURE: SECOidTag = 279;
pub const SEC_OID_ANSIX962_ECDSA_SHA256_SIGNATURE: SECOidTag = 278;
pub const SEC_OID_ANSIX962_ECDSA_SHA224_SIGNATURE: SECOidTag = 277;
pub const SEC_OID_ANSIX962_ECDSA_SIGNATURE_SPECIFIED_DIGEST: SECOidTag = 276;
/* new EC Signature oids */
pub const SEC_OID_ANSIX962_ECDSA_SIGNATURE_RECOMMENDED_DIGEST: SECOidTag =
    275;
pub const SEC_OID_PKCS9_EXTENSION_REQUEST: SECOidTag = 274;
/* More OIDs */
pub const SEC_OID_PKIX_CA_ISSUERS: SECOidTag = 273;
pub const SEC_OID_AVA_PSEUDONYM: SECOidTag = 272;
pub const SEC_OID_AVA_HOUSE_IDENTIFIER: SECOidTag = 271;
pub const SEC_OID_AVA_GENERATION_QUALIFIER: SECOidTag = 270;
pub const SEC_OID_AVA_INITIALS: SECOidTag = 269;
pub const SEC_OID_AVA_GIVEN_NAME: SECOidTag = 268;
pub const SEC_OID_AVA_POST_OFFICE_BOX: SECOidTag = 267;
pub const SEC_OID_AVA_POSTAL_CODE: SECOidTag = 266;
pub const SEC_OID_AVA_POSTAL_ADDRESS: SECOidTag = 265;
pub const SEC_OID_AVA_TITLE: SECOidTag = 264;
pub const SEC_OID_AVA_STREET_ADDRESS: SECOidTag = 263;
pub const SEC_OID_AVA_SERIAL_NUMBER: SECOidTag = 262;
pub const SEC_OID_AVA_SURNAME: SECOidTag = 261;
pub const SEC_OID_NETSCAPE_AOLSCREENNAME: SECOidTag = 260;
pub const SEC_OID_SECG_EC_SECT571R1: SECOidTag = 259;
pub const SEC_OID_SECG_EC_SECT571K1: SECOidTag = 258;
pub const SEC_OID_SECG_EC_SECT409R1: SECOidTag = 257;
pub const SEC_OID_SECG_EC_SECT409K1: SECOidTag = 256;
pub const SEC_OID_SECG_EC_SECT283R1: SECOidTag = 255;
pub const SEC_OID_SECG_EC_SECT283K1: SECOidTag = 254;
pub const SEC_OID_SECG_EC_SECT239K1: SECOidTag = 253;
pub const SEC_OID_SECG_EC_SECT233R1: SECOidTag = 252;
pub const SEC_OID_SECG_EC_SECT233K1: SECOidTag = 251;
pub const SEC_OID_SECG_EC_SECT193R2: SECOidTag = 250;
pub const SEC_OID_SECG_EC_SECT193R1: SECOidTag = 249;
pub const SEC_OID_SECG_EC_SECT163R2: SECOidTag = 248;
pub const SEC_OID_SECG_EC_SECT163R1: SECOidTag = 247;
pub const SEC_OID_SECG_EC_SECT163K1: SECOidTag = 246;
pub const SEC_OID_SECG_EC_SECT131R2: SECOidTag = 245;
pub const SEC_OID_SECG_EC_SECT131R1: SECOidTag = 244;
pub const SEC_OID_SECG_EC_SECT113R2: SECOidTag = 243;
/* SECG named elliptic curves (characteristic two field) */
pub const SEC_OID_SECG_EC_SECT113R1: SECOidTag = 242;
pub const SEC_OID_ANSIX962_EC_C2TNB431R1: SECOidTag = 241;
pub const SEC_OID_ANSIX962_EC_C2PNB368W1: SECOidTag = 240;
pub const SEC_OID_ANSIX962_EC_C2TNB359V1: SECOidTag = 239;
pub const SEC_OID_ANSIX962_EC_C2PNB304W1: SECOidTag = 238;
pub const SEC_OID_ANSIX962_EC_C2PNB272W1: SECOidTag = 237;
pub const SEC_OID_ANSIX962_EC_C2ONB239V5: SECOidTag = 236;
pub const SEC_OID_ANSIX962_EC_C2ONB239V4: SECOidTag = 235;
pub const SEC_OID_ANSIX962_EC_C2TNB239V3: SECOidTag = 234;
pub const SEC_OID_ANSIX962_EC_C2TNB239V2: SECOidTag = 233;
pub const SEC_OID_ANSIX962_EC_C2TNB239V1: SECOidTag = 232;
pub const SEC_OID_ANSIX962_EC_C2PNB208W1: SECOidTag = 231;
pub const SEC_OID_ANSIX962_EC_C2ONB191V5: SECOidTag = 230;
pub const SEC_OID_ANSIX962_EC_C2ONB191V4: SECOidTag = 229;
pub const SEC_OID_ANSIX962_EC_C2TNB191V3: SECOidTag = 228;
pub const SEC_OID_ANSIX962_EC_C2TNB191V2: SECOidTag = 227;
pub const SEC_OID_ANSIX962_EC_C2TNB191V1: SECOidTag = 226;
pub const SEC_OID_ANSIX962_EC_C2PNB176V1: SECOidTag = 225;
pub const SEC_OID_ANSIX962_EC_C2PNB163V3: SECOidTag = 224;
pub const SEC_OID_ANSIX962_EC_C2PNB163V2: SECOidTag = 223;
/* ANSI X9.62 named elliptic curves (characteristic two field) */
pub const SEC_OID_ANSIX962_EC_C2PNB163V1: SECOidTag = 222;
pub const SEC_OID_SECG_EC_SECP521R1: SECOidTag = 221;
/* SEC_OID_SECG_EC_SECP256R1 is SEC_OID_ANSIX962_EC_PRIME256V1 */
pub const SEC_OID_SECG_EC_SECP384R1: SECOidTag = 220;
pub const SEC_OID_SECG_EC_SECP256K1: SECOidTag = 219;
pub const SEC_OID_SECG_EC_SECP224R1: SECOidTag = 218;
/* SEC_OID_SECG_EC_SECP192R1 is SEC_OID_ANSIX962_EC_PRIME192V1 */
pub const SEC_OID_SECG_EC_SECP224K1: SECOidTag = 217;
pub const SEC_OID_SECG_EC_SECP192K1: SECOidTag = 216;
pub const SEC_OID_SECG_EC_SECP160R2: SECOidTag = 215;
pub const SEC_OID_SECG_EC_SECP160R1: SECOidTag = 214;
pub const SEC_OID_SECG_EC_SECP160K1: SECOidTag = 213;
pub const SEC_OID_SECG_EC_SECP128R2: SECOidTag = 212;
pub const SEC_OID_SECG_EC_SECP128R1: SECOidTag = 211;
pub const SEC_OID_SECG_EC_SECP112R2: SECOidTag = 210;
/* SECG named elliptic curves (prime field) */
pub const SEC_OID_SECG_EC_SECP112R1: SECOidTag = 209;
pub const SEC_OID_ANSIX962_EC_PRIME256V1: SECOidTag = 208;
pub const SEC_OID_ANSIX962_EC_PRIME239V3: SECOidTag = 207;
pub const SEC_OID_ANSIX962_EC_PRIME239V2: SECOidTag = 206;
pub const SEC_OID_ANSIX962_EC_PRIME239V1: SECOidTag = 205;
pub const SEC_OID_ANSIX962_EC_PRIME192V3: SECOidTag = 204;
pub const SEC_OID_ANSIX962_EC_PRIME192V2: SECOidTag = 203;
/* ANSI X9.62 named elliptic curves (prime field) */
pub const SEC_OID_ANSIX962_EC_PRIME192V1: SECOidTag = 202;
pub const SEC_OID_ANSIX962_ECDSA_SHA1_SIGNATURE: SECOidTag = 201;
/* Elliptic Curve Cryptography (ECC) OIDs */
pub const SEC_OID_ANSIX962_EC_PUBLIC_KEY: SECOidTag = 200;
pub const SEC_OID_AES_256_KEY_WRAP: SECOidTag = 199;
pub const SEC_OID_AES_192_KEY_WRAP: SECOidTag = 198;
pub const SEC_OID_AES_128_KEY_WRAP: SECOidTag = 197;
pub const SEC_OID_PKCS1_SHA512_WITH_RSA_ENCRYPTION: SECOidTag = 196;
pub const SEC_OID_PKCS1_SHA384_WITH_RSA_ENCRYPTION: SECOidTag = 195;
pub const SEC_OID_PKCS1_SHA256_WITH_RSA_ENCRYPTION: SECOidTag = 194;
pub const SEC_OID_SHA512: SECOidTag = 193;
pub const SEC_OID_SHA384: SECOidTag = 192;
pub const SEC_OID_SHA256: SECOidTag = 191;
pub const SEC_OID_MS_SMIME_ENCRYPTION_KEY_PREFERENCE: SECOidTag = 190;
pub const SEC_OID_SDN702_DSA_SIGNATURE: SECOidTag = 189;
pub const SEC_OID_AES_256_CBC: SECOidTag = 188;
pub const SEC_OID_AES_256_ECB: SECOidTag = 187;
pub const SEC_OID_AES_192_CBC: SECOidTag = 186;
pub const SEC_OID_AES_192_ECB: SECOidTag = 185;
pub const SEC_OID_AES_128_CBC: SECOidTag = 184;
/* AES OIDs */
pub const SEC_OID_AES_128_ECB: SECOidTag = 183;
/* SMIME attributes */
pub const SEC_OID_SMIME_ENCRYPTION_KEY_PREFERENCE: SECOidTag = 182;
pub const SEC_OID_CMS_RC2_KEY_WRAP: SECOidTag = 181;
pub const SEC_OID_CMS_3DES_KEY_WRAP: SECOidTag = 180;
/* CMS (RFC2630) OIDs */
pub const SEC_OID_CMS_EPHEMERAL_STATIC_DIFFIE_HELLMAN: SECOidTag = 179;
pub const SEC_OID_NS_CERT_EXT_SCOPE_OF_USE: SECOidTag = 178;
/* New PSM certificate management OIDs */
pub const SEC_OID_CERT_RENEWAL_LOCATOR: SECOidTag = 177;
/* Cert Server OIDS */
pub const SEC_OID_NETSCAPE_RECOVERY_REQUEST: SECOidTag = 176;
/* Netscape other name types */
    /* SEC_OID_NETSCAPE_NICKNAME is an otherName field of type IA5String
     * in the subjectAltName certificate extension.  NSS dropped support
     * for SEC_OID_NETSCAPE_NICKNAME in NSS 3.13. */
pub const SEC_OID_NETSCAPE_NICKNAME: SECOidTag = 175;
/*Diffe Helman OIDS */
pub const SEC_OID_X942_DIFFIE_HELMAN_KEY: SECOidTag = 174;
pub const SEC_OID_BOGUS_KEY_USAGE: SECOidTag = 173;
pub const SEC_OID_PKCS9_LOCAL_KEY_ID: SECOidTag = 172;
pub const SEC_OID_PKCS9_FRIENDLY_NAME: SECOidTag = 171;
pub const SEC_OID_PKCS9_X509_CRL: SECOidTag = 170;
pub const SEC_OID_PKCS9_SDSI_CERT: SECOidTag = 169;
pub const SEC_OID_PKCS9_X509_CERT: SECOidTag = 168;
pub const SEC_OID_PKCS12_V1_SAFE_CONTENTS_BAG_ID: SECOidTag = 167;
pub const SEC_OID_PKCS12_V1_SECRET_BAG_ID: SECOidTag = 166;
pub const SEC_OID_PKCS12_V1_CRL_BAG_ID: SECOidTag = 165;
pub const SEC_OID_PKCS12_V1_CERT_BAG_ID: SECOidTag = 164;
pub const SEC_OID_PKCS12_V1_PKCS8_SHROUDED_KEY_BAG_ID: SECOidTag = 163;
pub const SEC_OID_PKCS12_V1_KEY_BAG_ID: SECOidTag = 162;
pub const SEC_OID_PKCS12_PKCS8_SHROUDED_KEY_BAG_ID: SECOidTag = 161;
pub const SEC_OID_PKCS12_SAFE_CONTENTS_ID: SECOidTag = 160;
pub const SEC_OID_PKCS12_V2_PBE_WITH_SHA1_AND_40_BIT_RC2_CBC: SECOidTag = 159;
pub const SEC_OID_PKCS12_V2_PBE_WITH_SHA1_AND_128_BIT_RC2_CBC: SECOidTag =
    158;
pub const SEC_OID_PKCS12_V2_PBE_WITH_SHA1_AND_2KEY_TRIPLE_DES_CBC: SECOidTag =
    157;
pub const SEC_OID_PKCS12_V2_PBE_WITH_SHA1_AND_3KEY_TRIPLE_DES_CBC: SECOidTag =
    156;
pub const SEC_OID_PKCS12_V2_PBE_WITH_SHA1_AND_40_BIT_RC4: SECOidTag = 155;
/* PKCS 12 V2 oids */
pub const SEC_OID_PKCS12_V2_PBE_WITH_SHA1_AND_128_BIT_RC4: SECOidTag = 154;
/* Skipjack OID -- ### mwelch temporary */
pub const SEC_OID_FORTEZZA_SKIPJACK: SECOidTag = 153;
/* Netscape Algorithm OIDs */
pub const SEC_OID_NETSCAPE_SMIME_KEA: SECOidTag = 152;
pub const SEC_OID_OCSP_RESPONDER: SECOidTag = 151;
pub const SEC_OID_EXT_KEY_USAGE_TIME_STAMP: SECOidTag = 150;
pub const SEC_OID_EXT_KEY_USAGE_EMAIL_PROTECT: SECOidTag = 149;
pub const SEC_OID_EXT_KEY_USAGE_CODE_SIGN: SECOidTag = 148;
pub const SEC_OID_EXT_KEY_USAGE_CLIENT_AUTH: SECOidTag = 147;
pub const SEC_OID_EXT_KEY_USAGE_SERVER_AUTH: SECOidTag = 146;
pub const SEC_OID_PKIX_REGINFO_CERT_REQUEST: SECOidTag = 145;
pub const SEC_OID_PKIX_REGINFO_UTF8_PAIRS: SECOidTag = 144;
pub const SEC_OID_PKIX_REGCTRL_PROTOCOL_ENC_KEY: SECOidTag = 143;
pub const SEC_OID_PKIX_REGCTRL_OLD_CERT_ID: SECOidTag = 142;
pub const SEC_OID_PKIX_REGCTRL_PKI_ARCH_OPTIONS: SECOidTag = 141;
pub const SEC_OID_PKIX_REGCTRL_PKIPUBINFO: SECOidTag = 140;
pub const SEC_OID_PKIX_REGCTRL_AUTHENTICATOR: SECOidTag = 139;
pub const SEC_OID_PKIX_REGCTRL_REGTOKEN: SECOidTag = 138;
pub const SEC_OID_PKIX_OCSP_SERVICE_LOCATOR: SECOidTag = 137;
pub const SEC_OID_PKIX_OCSP_ARCHIVE_CUTOFF: SECOidTag = 136;
pub const SEC_OID_PKIX_OCSP_NO_CHECK: SECOidTag = 135;
pub const SEC_OID_PKIX_OCSP_RESPONSE: SECOidTag = 134;
pub const SEC_OID_PKIX_OCSP_CRL: SECOidTag = 133;
pub const SEC_OID_PKIX_OCSP_NONCE: SECOidTag = 132;
pub const SEC_OID_PKIX_OCSP_BASIC_RESPONSE: SECOidTag = 131;
pub const SEC_OID_PKIX_OCSP: SECOidTag = 130;
pub const SEC_OID_PKIX_USER_NOTICE_QUALIFIER: SECOidTag = 129;
/* PKIX OIDs */
pub const SEC_OID_PKIX_CPS_POINTER_QUALIFIER: SECOidTag = 128;
/* Verisign OIDs */
pub const SEC_OID_VERISIGN_USER_NOTICES: SECOidTag = 127;
pub const SEC_OID_BOGUS_DSA_SIGNATURE_WITH_SHA1_DIGEST: SECOidTag = 126;
pub const SEC_OID_ANSIX9_DSA_SIGNATURE_WITH_SHA1_DIGEST: SECOidTag = 125;
/* end of PKCS 12 additions */
/* DSA signatures */
pub const SEC_OID_ANSIX9_DSA_SIGNATURE: SECOidTag = 124;
pub const SEC_OID_PKCS12_RSA_SIGNATURE_WITH_SHA1_DIGEST: SECOidTag = 123;
pub const SEC_OID_PKCS12_RSA_ENCRYPTION_WITH_TRIPLE_DES: SECOidTag = 122;
pub const SEC_OID_PKCS12_RSA_ENCRYPTION_WITH_40_BIT_RC4: SECOidTag = 121;
pub const SEC_OID_PKCS12_RSA_ENCRYPTION_WITH_128_BIT_RC4: SECOidTag = 120;
pub const SEC_OID_PKCS12_PBE_WITH_SHA1_AND_40_BIT_RC2_CBC: SECOidTag = 119;
pub const SEC_OID_PKCS12_PBE_WITH_SHA1_AND_128_BIT_RC2_CBC: SECOidTag = 118;
pub const SEC_OID_PKCS12_PBE_WITH_SHA1_AND_TRIPLE_DES_CBC: SECOidTag = 117;
pub const SEC_OID_PKCS12_PBE_WITH_SHA1_AND_40_BIT_RC4: SECOidTag = 116;
pub const SEC_OID_PKCS12_PBE_WITH_SHA1_AND_128_BIT_RC4: SECOidTag = 115;
pub const SEC_OID_PKCS12_SDSI_CERT_BAG: SECOidTag = 114;
pub const SEC_OID_PKCS12_X509_CERT_CRL_BAG: SECOidTag = 113;
pub const SEC_OID_PKCS12_SECRET_BAG_ID: SECOidTag = 112;
pub const SEC_OID_PKCS12_CERT_AND_CRL_BAG_ID: SECOidTag = 111;
pub const SEC_OID_PKCS12_KEY_BAG_ID: SECOidTag = 110;
/* SEC_OID_PKCS12_OFFLINE_TRANSPORT_MODE,
    SEC_OID_PKCS12_ONLINE_TRANSPORT_MODE, */
pub const SEC_OID_PKCS12_PKCS8_KEY_SHROUDING: SECOidTag = 109;
pub const SEC_OID_PKCS12_ENVELOPING_IDS: SECOidTag = 108;
pub const SEC_OID_PKCS12_SIGNATURE_IDS: SECOidTag = 107;
pub const SEC_OID_PKCS12_PBE_IDS: SECOidTag = 106;
pub const SEC_OID_PKCS12_OIDS: SECOidTag = 105;
pub const SEC_OID_PKCS12_CERT_BAG_IDS: SECOidTag = 104;
pub const SEC_OID_PKCS12_BAG_IDS: SECOidTag = 103;
pub const SEC_OID_PKCS12_ESPVK_IDS: SECOidTag = 102;
pub const SEC_OID_PKCS12_MODE_IDS: SECOidTag = 101;
/* PKCS 12 additions */
pub const SEC_OID_PKCS12: SECOidTag = 100;
pub const SEC_OID_RFC1274_MAIL: SECOidTag = 99;
/* alg 1485 additions */
pub const SEC_OID_RFC1274_UID: SECOidTag = 98;
/* End of x.509 v3 Extensions */
pub const SEC_OID_X500_RSA_ENCRYPTION: SECOidTag = 97;
pub const SEC_OID_X509_INVALID_DATE: SECOidTag = 96;
pub const SEC_OID_X509_REASON_CODE: SECOidTag = 95;
pub const SEC_OID_X509_CRL_NUMBER: SECOidTag = 94;
pub const SEC_OID_X509_AUTH_INFO_ACCESS: SECOidTag = 93;
pub const SEC_OID_X509_EXT_KEY_USAGE: SECOidTag = 92;
pub const SEC_OID_X509_AUTH_KEY_ID: SECOidTag = 91;
pub const SEC_OID_X509_POLICY_CONSTRAINTS: SECOidTag = 90;
pub const SEC_OID_X509_POLICY_MAPPINGS: SECOidTag = 89;
pub const SEC_OID_X509_CERTIFICATE_POLICIES: SECOidTag = 88;
pub const SEC_OID_X509_CRL_DIST_POINTS: SECOidTag = 87;
pub const SEC_OID_X509_NAME_CONSTRAINTS: SECOidTag = 86;
pub const SEC_OID_X509_BASIC_CONSTRAINTS: SECOidTag = 85;
pub const SEC_OID_X509_ISSUER_ALT_NAME: SECOidTag = 84;
pub const SEC_OID_X509_SUBJECT_ALT_NAME: SECOidTag = 83;
pub const SEC_OID_X509_PRIVATE_KEY_USAGE_PERIOD: SECOidTag = 82;
pub const SEC_OID_X509_KEY_USAGE: SECOidTag = 81;
pub const SEC_OID_X509_SUBJECT_KEY_ID: SECOidTag = 80;
/* x.509 v3 Extensions */
pub const SEC_OID_X509_SUBJECT_DIRECTORY_ATTR: SECOidTag = 79;
pub const SEC_OID_NS_KEY_USAGE_GOVT_APPROVED: SECOidTag = 78;
pub const SEC_OID_NS_CERT_EXT_CERT_RENEWAL_TIME: SECOidTag = 77;
pub const SEC_OID_NS_CERT_EXT_LOST_PASSWORD_URL: SECOidTag = 76;
pub const SEC_OID_NS_CERT_EXT_COMMENT: SECOidTag = 75;
pub const SEC_OID_NS_CERT_EXT_SSL_SERVER_NAME: SECOidTag = 74;
pub const SEC_OID_NS_CERT_EXT_USER_PICTURE: SECOidTag = 73;
pub const SEC_OID_NS_CERT_EXT_ENTITY_LOGO: SECOidTag = 72;
pub const SEC_OID_NS_CERT_EXT_HOMEPAGE_URL: SECOidTag = 71;
pub const SEC_OID_NS_CERT_EXT_CA_POLICY_URL: SECOidTag = 70;
pub const SEC_OID_NS_CERT_EXT_CERT_RENEWAL_URL: SECOidTag = 69;
pub const SEC_OID_NS_CERT_EXT_CA_CERT_URL: SECOidTag = 68;
pub const SEC_OID_NS_CERT_EXT_CA_CRL_URL: SECOidTag = 67;
pub const SEC_OID_NS_CERT_EXT_CA_REVOCATION_URL: SECOidTag = 66;
pub const SEC_OID_NS_CERT_EXT_REVOCATION_URL: SECOidTag = 65;
pub const SEC_OID_NS_CERT_EXT_BASE_URL: SECOidTag = 64;
pub const SEC_OID_NS_CERT_EXT_CERT_TYPE: SECOidTag = 63;
pub const SEC_OID_NS_CERT_EXT_SUBJECT_LOGO: SECOidTag = 62;
pub const SEC_OID_NS_CERT_EXT_ISSUER_LOGO: SECOidTag = 61;
/* Netscape private certificate extensions */
pub const SEC_OID_NS_CERT_EXT_NETSCAPE_OK: SECOidTag = 60;
pub const SEC_OID_MISSI_ALT_KEA: SECOidTag = 59;
pub const SEC_OID_MISSI_KEA: SECOidTag = 58;
pub const SEC_OID_MISSI_DSS: SECOidTag = 57;
pub const SEC_OID_MISSI_KEA_DSS: SECOidTag = 56;
pub const SEC_OID_MISSI_DSS_OLD: SECOidTag = 55;
pub const SEC_OID_MISSI_KEA_DSS_OLD: SECOidTag = 54;
pub const SEC_OID_NS_TYPE_CERT_SEQUENCE: SECOidTag = 53;
pub const SEC_OID_NS_TYPE_HTML: SECOidTag = 52;
pub const SEC_OID_NS_TYPE_URL: SECOidTag = 51;
pub const SEC_OID_NS_TYPE_JPEG: SECOidTag = 50;
pub const SEC_OID_NS_TYPE_GIF: SECOidTag = 49;
pub const SEC_OID_AVA_DC: SECOidTag = 48;
pub const SEC_OID_AVA_DN_QUALIFIER: SECOidTag = 47;
pub const SEC_OID_AVA_ORGANIZATIONAL_UNIT_NAME: SECOidTag = 46;
pub const SEC_OID_AVA_ORGANIZATION_NAME: SECOidTag = 45;
pub const SEC_OID_AVA_STATE_OR_PROVINCE: SECOidTag = 44;
pub const SEC_OID_AVA_LOCALITY: SECOidTag = 43;
pub const SEC_OID_AVA_COUNTRY_NAME: SECOidTag = 42;
pub const SEC_OID_AVA_COMMON_NAME: SECOidTag = 41;
pub const SEC_OID_PKCS9_SMIME_CAPABILITIES: SECOidTag = 40;
pub const SEC_OID_PKCS9_EXTENDED_CERTIFICATE_ATTRIBUTES: SECOidTag = 39;
pub const SEC_OID_PKCS9_UNSTRUCTURED_ADDRESS: SECOidTag = 38;
pub const SEC_OID_PKCS9_CHALLENGE_PASSWORD: SECOidTag = 37;
pub const SEC_OID_PKCS9_COUNTER_SIGNATURE: SECOidTag = 36;
pub const SEC_OID_PKCS9_SIGNING_TIME: SECOidTag = 35;
pub const SEC_OID_PKCS9_MESSAGE_DIGEST: SECOidTag = 34;
pub const SEC_OID_PKCS9_CONTENT_TYPE: SECOidTag = 33;
pub const SEC_OID_PKCS9_UNSTRUCTURED_NAME: SECOidTag = 32;
pub const SEC_OID_PKCS9_EMAIL_ADDRESS: SECOidTag = 31;
pub const SEC_OID_PKCS7_ENCRYPTED_DATA: SECOidTag = 30;
pub const SEC_OID_PKCS7_DIGESTED_DATA: SECOidTag = 29;
pub const SEC_OID_PKCS7_SIGNED_ENVELOPED_DATA: SECOidTag = 28;
pub const SEC_OID_PKCS7_ENVELOPED_DATA: SECOidTag = 27;
pub const SEC_OID_PKCS7_SIGNED_DATA: SECOidTag = 26;
pub const SEC_OID_PKCS7_DATA: SECOidTag = 25;
pub const SEC_OID_PKCS7: SECOidTag = 24;
pub const SEC_OID_PKCS5_PBE_WITH_SHA1_AND_DES_CBC: SECOidTag = 23;
pub const SEC_OID_PKCS5_PBE_WITH_MD5_AND_DES_CBC: SECOidTag = 22;
pub const SEC_OID_PKCS5_PBE_WITH_MD2_AND_DES_CBC: SECOidTag = 21;
pub const SEC_OID_PKCS1_SHA1_WITH_RSA_ENCRYPTION: SECOidTag = 20;
pub const SEC_OID_PKCS1_MD5_WITH_RSA_ENCRYPTION: SECOidTag = 19;
pub const SEC_OID_PKCS1_MD4_WITH_RSA_ENCRYPTION: SECOidTag = 18;
pub const SEC_OID_PKCS1_MD2_WITH_RSA_ENCRYPTION: SECOidTag = 17;
pub const SEC_OID_PKCS1_RSA_ENCRYPTION: SECOidTag = 16;
pub const SEC_OID_ISO_SHA_WITH_RSA_SIGNATURE: SECOidTag = 15;
pub const SEC_OID_DES_EDE: SECOidTag = 14;
pub const SEC_OID_DES_MAC: SECOidTag = 13;
pub const SEC_OID_DES_CFB: SECOidTag = 12;
pub const SEC_OID_DES_OFB: SECOidTag = 11;
pub const SEC_OID_DES_CBC: SECOidTag = 10;
pub const SEC_OID_DES_ECB: SECOidTag = 9;
pub const SEC_OID_RC5_CBC_PAD: SECOidTag = 8;
pub const SEC_OID_DES_EDE3_CBC: SECOidTag = 7;
pub const SEC_OID_RC4: SECOidTag = 6;
pub const SEC_OID_RC2_CBC: SECOidTag = 5;
pub const SEC_OID_SHA1: SECOidTag = 4;
pub const SEC_OID_MD5: SECOidTag = 3;
pub const SEC_OID_MD4: SECOidTag = 2;
pub const SEC_OID_MD2: SECOidTag = 1;
pub const SEC_OID_UNKNOWN: SECOidTag = 0;
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
 * secoidt.h - public data structures for ASN.1 OID functions
 */
pub type SECOidData = SECOidDataStr;
/*
** An X.500 algorithm identifier
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SECAlgorithmIDStr {
    pub algorithm: SECItem,
    pub parameters: SECItem,
}
pub type SECAlgorithmID = SECAlgorithmIDStr;
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
/* some special values for certain CK_ULONG variables */
pub type CK_BYTE_PTR = *mut CK_BYTE;
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
/* CK_GCM_PARAMS is new for version 2.30 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CK_GCM_PARAMS {
    pub pIv: CK_BYTE_PTR,
    pub ulIvLen: CK_ULONG,
    pub pAAD: CK_BYTE_PTR,
    pub ulAADLen: CK_ULONG,
    pub ulTagBits: CK_ULONG,
}
/* defined in secmodti.h */
pub type PK11SlotInfo = PK11SlotInfoStr;
/* defined in secmodti.h */
pub type PK11SymKey = PK11SymKeyStr;
/* types of PKCS #11 objects
 * used to identify which NSS data structure is
 * passed to the PK11_Raw* functions. Types map as follows:
 *   PK11_TypeGeneric            PK11GenericObject *
 *   PK11_TypePrivKey            SECKEYPrivateKey *
 *   PK11_TypePubKey             SECKEYPublicKey *
 *   PK11_TypeSymKey             PK11SymKey *
 *   PK11_TypeCert               CERTCertificate * (currently not used).
 */
pub type PK11ObjectType = libc::c_uint;
pub const PK11_TypeSymKey: PK11ObjectType = 4;
pub const PK11_TypeCert: PK11ObjectType = 3;
pub const PK11_TypePubKey: PK11ObjectType = 2;
pub const PK11_TypePrivKey: PK11ObjectType = 1;
pub const PK11_TypeGeneric: PK11ObjectType = 0;
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
pub type CERTSubjectPublicKeyInfo = CERTSubjectPublicKeyInfoStr;
/*
** An X.509 subject-public-key-info object
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CERTSubjectPublicKeyInfoStr {
    pub arena: *mut PLArenaPool,
    pub algorithm: SECAlgorithmID,
    pub subjectPublicKey: SECItem,
}
pub type PublicKey = *mut SECKEYPublicKey;
pub type const_PublicKey = *const SECKEYPublicKey;
pub type PrivateKey = *mut SECKEYPrivateKey;
/*
 * We use the PublicKey and PrivateKey objects for public-key encryption. Each
 * Prio server has an associated public key, and the clients use these keys to
 * encrypt messages to the servers.
 */
#[no_mangle]
pub unsafe extern "C" fn Keypair_new(mut pvtkey: *mut PrivateKey,
                                     mut pubkey: *mut PublicKey)
 -> SECStatus {
    let mut oid_struct_len: libc::c_int = 0;
    if pvtkey.is_null() { return SECFailure }
    if pubkey.is_null() { return SECFailure }
    let mut rv: SECStatus = SECSuccess;
    let mut oid_data: *mut SECOidData = 0 as *mut SECOidData;
    *pubkey = 0 as PublicKey;
    *pvtkey = 0 as PrivateKey;
    let mut ecp: SECKEYECParams =
        SECItemStr{type_0: siBuffer, data: 0 as *mut libc::c_uchar, len: 0,};
    ecp.data = 0 as *mut libc::c_uchar;
    let mut slot: *mut PK11SlotInfo = 0 as *mut PK11SlotInfo;
    oid_data = SECOID_FindOIDByTag(SEC_OID_CURVE25519);
    if oid_data.is_null() {
        rv = SECFailure
    } else {
        oid_struct_len =
            (2i32 as libc::c_uint).wrapping_add((*oid_data).oid.len) as
                libc::c_int;
        ecp.data =
            malloc(oid_struct_len as libc::c_ulong) as *mut libc::c_uchar;
        if ecp.data.is_null() {
            rv = SECFailure
        } else {
            ecp.len = oid_struct_len as libc::c_uint;
            ecp.type_0 = siDEROID;
            *ecp.data.offset(0isize) = 0x6i32 as libc::c_uchar;
            *ecp.data.offset(1isize) = (*oid_data).oid.len as libc::c_uchar;
            memcpy(&mut *ecp.data.offset(2isize) as *mut libc::c_uchar as
                       *mut libc::c_void,
                   (*oid_data).oid.data as *const libc::c_void,
                   (*oid_data).oid.len as libc::c_ulong);
            slot = PK11_GetInternalSlot();
            if slot.is_null() {
                rv = SECFailure
            } else {
                *pvtkey =
                    PK11_GenerateKeyPair(slot, 0x1040i32 as CK_MECHANISM_TYPE,
                                         &mut ecp as *mut SECKEYECParams as
                                             *mut libc::c_void, pubkey, 0i32,
                                         0i32, 0 as *mut libc::c_void);
                if (*pvtkey).is_null() { rv = SECFailure }
            }
        }
    }
    if !slot.is_null() { PK11_FreeSlot(slot); }
    if !ecp.data.is_null() { free(ecp.data as *mut libc::c_void); }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PublicKey_clear(*pubkey);
        PrivateKey_clear(*pvtkey);
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn PrivateKey_clear(mut pvtkey: PrivateKey) {
    if !pvtkey.is_null() { SECKEY_DestroyPrivateKey(pvtkey); };
}
#[no_mangle]
pub unsafe extern "C" fn PublicKey_clear(mut pubkey: PublicKey) {
    if !pubkey.is_null() { SECKEY_DestroyPublicKey(pubkey); };
}
/*
 * Import a new curve25519 public/private key from the raw bytes given.  When
 * importing a private key, you must pass in the corresponding public key as
 * well. The byte arrays given as input should be of length
 * `CURVE25519_KEY_LEN`.
 *
 * These functions will allocate a new `PublicKey`/`PrivateKey` object, which
 * the caller must free using `PublicKey_clear`/`PrivateKey_clear`.
 */
#[no_mangle]
pub unsafe extern "C" fn PublicKey_import(mut pk: *mut PublicKey,
                                          mut data: *const libc::c_uchar,
                                          mut dataLen: libc::c_uint)
 -> SECStatus {
    let mut spki_len: libc::c_int = 0;
    let mut spki_item: SECItem =
        SECItemStr{type_0: siBuffer, data: 0 as *mut libc::c_uchar, len: 0,};
    let mut rv: SECStatus = SECSuccess;
    let mut pkinfo: *mut CERTSubjectPublicKeyInfo =
        0 as *mut CERTSubjectPublicKeyInfo;
    *pk = 0 as PublicKey;
    let mut key_bytes: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut spki_data: *mut uint8_t = 0 as *mut uint8_t;
    if dataLen != 32i32 as libc::c_uint { return SECFailure }
    key_bytes =
        calloc(dataLen as libc::c_ulong,
               ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong) as
            *mut libc::c_uchar;
    if key_bytes.is_null() {
        rv = SECFailure
    } else {
        memcpy(key_bytes as *mut libc::c_void, data as *const libc::c_void,
               dataLen as libc::c_ulong);
        spki_len =
            ::std::mem::size_of::<[uint8_t; 59]>() as libc::c_ulong as
                libc::c_int;
        spki_data =
            calloc(spki_len as libc::c_ulong,
                   ::std::mem::size_of::<uint8_t>() as libc::c_ulong) as
                *mut uint8_t;
        if spki_data.is_null() {
            rv = SECFailure
        } else {
            memcpy(spki_data as *mut libc::c_void,
                   curve25519_spki_zeros.as_ptr() as *const libc::c_void,
                   spki_len as libc::c_ulong);
            spki_item =
                SECItemStr{type_0: siBuffer,
                           data: spki_data,
                           len: spki_len as libc::c_uint,};
            // Import the all-zeros curve25519 public key.
            pkinfo = SECKEY_DecodeDERSubjectPublicKeyInfo(&mut spki_item);
            if pkinfo.is_null() {
                rv = SECFailure
            } else {
                *pk = SECKEY_ExtractPublicKey(pkinfo);
                if (*pk).is_null() {
                    rv = SECFailure
                } else {
                    memcpy((**pk).u.ec.publicValue.data as *mut libc::c_void,
                           data as *const libc::c_void,
                           32i32 as libc::c_ulong);
                }
            }
        }
    }
    if !key_bytes.is_null() { free(key_bytes as *mut libc::c_void); }
    if !spki_data.is_null() { free(spki_data as *mut libc::c_void); }
    if !pkinfo.is_null() { SECKEY_DestroySubjectPublicKeyInfo(pkinfo); }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PublicKey_clear(*pk);
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
// Use curve25519
// Use 96-bit IV
// Use 128-bit auth tag
// For an example of NSS curve25519 import/export code, see:
// https://searchfox.org/nss/rev/cfd5fcba7efbfe116e2c08848075240ec3a92718/gtests/pk11_gtest/pk11_curve25519_unittest.cc#66
// The all-zeros curve25519 public key, as DER-encoded SPKI blob.
static mut curve25519_spki_zeros: [uint8_t; 59] =
    [0x30i32 as uint8_t, 0x39i32 as uint8_t, 0x30i32 as uint8_t,
     0x14i32 as uint8_t, 0x6i32 as uint8_t, 0x7i32 as uint8_t,
     0x2ai32 as uint8_t, 0x86i32 as uint8_t, 0x48i32 as uint8_t,
     0xcei32 as uint8_t, 0x3di32 as uint8_t, 0x2i32 as uint8_t,
     0x1i32 as uint8_t, 0x6i32 as uint8_t, 0x9i32 as uint8_t,
     0x2bi32 as uint8_t, 0x6i32 as uint8_t, 0x1i32 as uint8_t,
     0x4i32 as uint8_t, 0x1i32 as uint8_t, 0xdai32 as uint8_t,
     0x47i32 as uint8_t, 0xfi32 as uint8_t, 0x1i32 as uint8_t,
     0x3i32 as uint8_t, 0x21i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t];
#[no_mangle]
pub unsafe extern "C" fn PrivateKey_import(mut sk: *mut PrivateKey,
                                           mut sk_data: *const libc::c_uchar,
                                           mut sk_data_len: libc::c_uint,
                                           mut pk_data: *const libc::c_uchar,
                                           mut pk_data_len: libc::c_uint)
 -> SECStatus {
    let mut zero_priv_item: SECItem =
        SECItemStr{type_0: siBuffer, data: 0 as *mut libc::c_uchar, len: 0,};
    if sk_data_len != 32i32 as libc::c_uint || sk_data.is_null() {
        return SECFailure
    }
    if pk_data_len != 32i32 as libc::c_uint || pk_data.is_null() {
        return SECFailure
    }
    let mut rv: SECStatus = SECSuccess;
    let mut slot: *mut PK11SlotInfo = 0 as *mut PK11SlotInfo;
    let mut zero_priv_data: *mut uint8_t = 0 as *mut uint8_t;
    *sk = 0 as PrivateKey;
    let zero_priv_len: libc::c_int =
        ::std::mem::size_of::<[uint8_t; 105]>() as libc::c_ulong as
            libc::c_int;
    slot = PK11_GetInternalSlot();
    if slot.is_null() {
        rv = SECFailure
    } else {
        zero_priv_data =
            calloc(zero_priv_len as libc::c_ulong,
                   ::std::mem::size_of::<uint8_t>() as libc::c_ulong) as
                *mut uint8_t;
        if zero_priv_data.is_null() {
            rv = SECFailure
        } else {
            zero_priv_item =
                SECItemStr{type_0: siBuffer,
                           data: zero_priv_data,
                           len: zero_priv_len as libc::c_uint,};
            memcpy(zero_priv_data as *mut libc::c_void,
                   curve25519_priv_zeros.as_ptr() as *const libc::c_void,
                   zero_priv_len as libc::c_ulong);
            memcpy(zero_priv_data.offset(curve25519_priv_sk_offset as isize)
                       as *mut libc::c_void, sk_data as *const libc::c_void,
                   sk_data_len as libc::c_ulong);
            memcpy(zero_priv_data.offset(curve25519_priv_pk_offset as isize)
                       as *mut libc::c_void, pk_data as *const libc::c_void,
                   pk_data_len as libc::c_ulong);
            rv =
                PK11_ImportDERPrivateKeyInfoAndReturnKey(slot,
                                                         &mut zero_priv_item,
                                                         0 as *mut SECItem,
                                                         0 as *mut SECItem,
                                                         0i32, 0i32,
                                                         (0x80i32 | 0x40i32 |
                                                              0x20i32 |
                                                              0x10i32 | 0x8i32
                                                              | 0x4i32 |
                                                              0x2i32 | 0x1i32)
                                                             as libc::c_uint,
                                                         sk,
                                                         0 as
                                                             *mut libc::c_void);
            rv as libc::c_int != SECSuccess as libc::c_int;
        }
    }
    if !slot.is_null() { PK11_FreeSlot(slot); }
    if !zero_priv_data.is_null() {
        free(zero_priv_data as *mut libc::c_void);
    }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        PrivateKey_clear(*sk);
    }
    return rv;
}
// The all-zeros curve25519 private key, as a PKCS#8 blob.
static mut curve25519_priv_zeros: [uint8_t; 105] =
    [0x30i32 as uint8_t, 0x67i32 as uint8_t, 0x2i32 as uint8_t,
     0x1i32 as uint8_t, 0i32 as uint8_t, 0x30i32 as uint8_t,
     0x14i32 as uint8_t, 0x6i32 as uint8_t, 0x7i32 as uint8_t,
     0x2ai32 as uint8_t, 0x86i32 as uint8_t, 0x48i32 as uint8_t,
     0xcei32 as uint8_t, 0x3di32 as uint8_t, 0x2i32 as uint8_t,
     0x1i32 as uint8_t, 0x6i32 as uint8_t, 0x9i32 as uint8_t,
     0x2bi32 as uint8_t, 0x6i32 as uint8_t, 0x1i32 as uint8_t,
     0x4i32 as uint8_t, 0x1i32 as uint8_t, 0xdai32 as uint8_t,
     0x47i32 as uint8_t, 0xfi32 as uint8_t, 0x1i32 as uint8_t,
     0x4i32 as uint8_t, 0x4ci32 as uint8_t, 0x30i32 as uint8_t,
     0x4ai32 as uint8_t, 0x2i32 as uint8_t, 0x1i32 as uint8_t,
     0x1i32 as uint8_t, 0x4i32 as uint8_t, 0x20i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0xa1i32 as uint8_t, 0x23i32 as uint8_t, 0x3i32 as uint8_t,
     0x21i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0x4i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t, 0i32 as uint8_t,
     0i32 as uint8_t, 0i32 as uint8_t];
// Index into `curve25519_priv_zeros` at which the public key begins.
static mut curve25519_priv_pk_offset: size_t = 73i32 as size_t;
/* Byte index 36:  32 bytes of curve25519 private key. */
/* misc type fields */
/* Byte index 73:  32 bytes of curve25519 public key. */
// Index into `curve25519_priv_zeros` at which the private key begins.
static mut curve25519_priv_sk_offset: size_t = 36i32 as size_t;
/*
 * Import a new curve25519 public/private key from a hex string that contains
 * only the characters 0-9a-fA-F.
 *
 * The hex strings passed in must each be of length `CURVE25519_KEY_LEN_HEX`.
 * These functions will allocate a new `PublicKey`/`PrivateKey` object, which
 * the caller must free using `PublicKey_clear`/`PrivateKey_clear`.
 */
#[no_mangle]
pub unsafe extern "C" fn PublicKey_import_hex(mut pk: *mut PublicKey,
                                              mut hexData:
                                                  *const libc::c_uchar,
                                              mut dataLen: libc::c_uint)
 -> SECStatus {
    let mut raw_bytes: [libc::c_uchar; 32] = [0; 32];
    if dataLen != 64i32 as libc::c_uint || hexData.is_null() {
        return SECFailure
    }
    if key_from_hex(raw_bytes.as_mut_ptr(), hexData) as libc::c_int !=
           SECSuccess as libc::c_int {
        return SECFailure
    }
    return PublicKey_import(pk, raw_bytes.as_mut_ptr(),
                            32i32 as libc::c_uint);
}
unsafe extern "C" fn key_from_hex(mut key_out: *mut libc::c_uchar,
                                  mut hex_in: *const libc::c_uchar)
 -> SECStatus {
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    while i < 64i32 as libc::c_uint {
        if 0 == is_hex_digit(*hex_in.offset(i as isize) as libc::c_char) {
            return SECFailure
        }
        i = i.wrapping_add(1)
    }
    let mut p: *const libc::c_uchar = hex_in;
    let mut i_0: libc::c_uint = 0i32 as libc::c_uint;
    while i_0 < 32i32 as libc::c_uint {
        let mut d0: uint8_t = hex_to_int(*p.offset(0isize) as libc::c_char);
        let mut d1: uint8_t = hex_to_int(*p.offset(1isize) as libc::c_char);
        *key_out.offset(i_0 as isize) =
            ((d0 as libc::c_int) << 4i32 | d1 as libc::c_int) as
                libc::c_uchar;
        p = p.offset(2isize);
        i_0 = i_0.wrapping_add(1)
    }
    return SECSuccess;
}
unsafe extern "C" fn hex_to_int(mut h: libc::c_char) -> uint8_t {
    return (if h as libc::c_int > '9' as i32 {
                to_upper(h) as libc::c_int - 'A' as i32 + 10i32
            } else { h as libc::c_int - '0' as i32 }) as uint8_t;
}
// Note that we do not use toupper because it is locale-dependent
// See: https://github.com/mozilla/libprio/issues/20
unsafe extern "C" fn to_upper(mut c: libc::c_char) -> libc::c_char {
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
        return (c as libc::c_int - 0x20i32) as libc::c_char
    } else { return c };
}
// Note that we do not use isxdigit because it is locale-dependent
// See: https://github.com/mozilla/libprio/issues/20
unsafe extern "C" fn is_hex_digit(mut c: libc::c_char) -> libc::c_char {
    return ('0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32
                ||
                'a' as i32 <= c as libc::c_int &&
                    c as libc::c_int <= 'f' as i32 ||
                'A' as i32 <= c as libc::c_int &&
                    c as libc::c_int <= 'F' as i32) as libc::c_int as
               libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn PrivateKey_import_hex(mut sk: *mut PrivateKey,
                                               mut privHexData:
                                                   *const libc::c_uchar,
                                               mut privDataLen: libc::c_uint,
                                               mut pubHexData:
                                                   *const libc::c_uchar,
                                               mut pubDataLen: libc::c_uint)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let mut raw_priv: [libc::c_uchar; 32] = [0; 32];
    let mut raw_pub: [libc::c_uchar; 32] = [0; 32];
    if privDataLen != 64i32 as libc::c_uint ||
           pubDataLen != 64i32 as libc::c_uint {
        return SECFailure
    }
    if privHexData.is_null() || pubHexData.is_null() { return SECFailure }
    rv = key_from_hex(raw_priv.as_mut_ptr(), privHexData);
    if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    rv = key_from_hex(raw_pub.as_mut_ptr(), pubHexData);
    if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
    return PrivateKey_import(sk, raw_priv.as_mut_ptr(), 32i32 as libc::c_uint,
                             raw_pub.as_mut_ptr(), 32i32 as libc::c_uint);
}
/*
 * Export a curve25519 key as a raw byte-array.
 *
 * The output buffer `data` must have length exactly `CURVE25519_KEY_LEN`.
 */
#[no_mangle]
pub unsafe extern "C" fn PublicKey_export(mut pk: const_PublicKey,
                                          mut data: *mut libc::c_uchar,
                                          mut dataLen: libc::c_uint)
 -> SECStatus {
    if pk.is_null() || dataLen != 32i32 as libc::c_uint { return SECFailure }
    let mut key: *const SECItem = &(*pk).u.ec.publicValue;
    if (*key).len != 32i32 as libc::c_uint { return SECFailure }
    memcpy(data as *mut libc::c_void, (*key).data as *const libc::c_void,
           (*key).len as libc::c_ulong);
    return SECSuccess;
}
#[no_mangle]
pub unsafe extern "C" fn PrivateKey_export(mut sk: PrivateKey,
                                           mut data: *mut libc::c_uchar,
                                           mut dataLen: libc::c_uint)
 -> SECStatus {
    let mut leading_zeros: size_t = 0;
    if sk.is_null() || dataLen != 32i32 as libc::c_uint { return SECFailure }
    let mut rv: SECStatus = SECSuccess;
    let mut item: SECItem =
        SECItemStr{type_0: siBuffer,
                   data: 0 as *mut libc::c_uchar,
                   len: 0i32 as libc::c_uint,};
    rv =
        PK11_ReadRawAttribute(PK11_TypePrivKey, sk as *mut libc::c_void,
                              0x11i32 as CK_ATTRIBUTE_TYPE, &mut item);
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        memset(data as *mut libc::c_void, 0i32, 32i32 as libc::c_ulong);
        if !(item.len <= 32i32 as libc::c_uint) {
            rv = SECFailure
        } else {
            // Copy into the low-order bytes of the output.
            leading_zeros =
                (32i32 as libc::c_uint).wrapping_sub(item.len) as size_t;
            memcpy(data.offset(leading_zeros as isize) as *mut libc::c_void,
                   item.data as *const libc::c_void,
                   item.len as libc::c_ulong);
        }
    }
    if !item.data.is_null() { SECITEM_ZfreeItem(&mut item, 0i32); }
    return rv;
}
/*
 * Export a curve25519 key as a NULL-terminated hex string.
 *
 * The output buffer `data` must have length exactly `CURVE25519_KEY_LEN_HEX +
 * 1`.
 */
#[no_mangle]
pub unsafe extern "C" fn PublicKey_export_hex(mut pk: const_PublicKey,
                                              mut data: *mut libc::c_uchar,
                                              mut dataLen: libc::c_uint)
 -> SECStatus {
    if dataLen != (64i32 + 1i32) as libc::c_uint { return SECFailure }
    let mut raw_data: [libc::c_uchar; 32] = [0; 32];
    if PublicKey_export(pk, raw_data.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                            libc::c_ulong as libc::c_uint) as libc::c_int !=
           SECSuccess as libc::c_int {
        return SECFailure
    }
    key_to_hex(raw_data.as_mut_ptr() as *const libc::c_uchar, data);
    return SECSuccess;
}
unsafe extern "C" fn key_to_hex(mut key_in: *const libc::c_uchar,
                                mut hex_out: *mut libc::c_uchar) {
    let mut p: *const libc::c_uchar = key_in;
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    while i < 32i32 as libc::c_uint {
        let mut bytel: libc::c_uchar =
            (*p.offset(0isize) as libc::c_int & 0xfi32) as libc::c_uchar;
        let mut byteu: libc::c_uchar =
            ((*p.offset(0isize) as libc::c_int & 0xf0i32) >> 4i32) as
                libc::c_uchar;
        *hex_out.offset((2i32 as libc::c_uint).wrapping_mul(i) as isize) =
            int_to_hex(byteu);
        *hex_out.offset((2i32 as
                             libc::c_uint).wrapping_mul(i).wrapping_add(1i32
                                                                            as
                                                                            libc::c_uint)
                            as isize) = int_to_hex(bytel);
        p = p.offset(1isize);
        i = i.wrapping_add(1)
    }
    *hex_out.offset((2i32 * 32i32) as isize) =
        '\u{0}' as i32 as libc::c_uchar;
}
unsafe extern "C" fn int_to_hex(mut i: uint8_t) -> libc::c_uchar {
    return (if i as libc::c_int > 0x9i32 {
                i as libc::c_int - 10i32 + 'A' as i32
            } else { i as libc::c_int + '0' as i32 }) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn PrivateKey_export_hex(mut sk: PrivateKey,
                                               mut data: *mut libc::c_uchar,
                                               mut dataLen: libc::c_uint)
 -> SECStatus {
    if dataLen != (64i32 + 1i32) as libc::c_uint { return SECFailure }
    let mut raw_data: [libc::c_uchar; 32] = [0; 32];
    if PrivateKey_export(sk, raw_data.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_uchar; 32]>() as
                             libc::c_ulong as libc::c_uint) as libc::c_int !=
           SECSuccess as libc::c_int {
        return SECFailure
    }
    key_to_hex(raw_data.as_mut_ptr() as *const libc::c_uchar, data);
    return SECSuccess;
}
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
pub unsafe extern "C" fn PublicKey_encryptSize(mut inputLen: libc::c_uint,
                                               mut outputLen:
                                                   *mut libc::c_uint)
 -> SECStatus {
    if outputLen.is_null() ||
           inputLen >= (2147483647i32 >> 3i32) as libc::c_uint {
        return SECFailure
    }
    *outputLen =
        ((32i32 + 12i32 + 16i32) as libc::c_uint).wrapping_add(inputLen);
    return SECSuccess;
}
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
pub unsafe extern "C" fn PublicKey_encrypt(mut pubkey: PublicKey,
                                           mut output: *mut libc::c_uchar,
                                           mut outputLen: *mut libc::c_uint,
                                           mut maxOutputLen: libc::c_uint,
                                           mut input: *const libc::c_uchar,
                                           mut inputLen: libc::c_uint)
 -> SECStatus {
    let mut param: CK_GCM_PARAMS =
        CK_GCM_PARAMS{pIv: 0 as *mut CK_BYTE,
                      ulIvLen: 0,
                      pAAD: 0 as *mut CK_BYTE,
                      ulAADLen: 0,
                      ulTagBits: 0,};
    let mut paramItem: SECItem =
        SECItemStr{type_0: siBuffer, data: 0 as *mut libc::c_uchar, len: 0,};
    let mut pk: *const SECItem = 0 as *const SECItem;
    let mut offset: libc::c_int = 0;
    if pubkey.is_null() { return SECFailure }
    if inputLen >= (2147483647i32 >> 3i32) as libc::c_uint {
        return SECFailure
    }
    let mut needLen: libc::c_uint = 0;
    if PublicKey_encryptSize(inputLen, &mut needLen) as libc::c_int !=
           SECSuccess as libc::c_int {
        return SECFailure
    }
    if maxOutputLen < needLen { return SECFailure }
    let mut rv: SECStatus = SECSuccess;
    let mut eph_pub: PublicKey = 0 as PublicKey;
    let mut eph_priv: PrivateKey = 0 as PrivateKey;
    let mut aes_key: *mut PK11SymKey = 0 as *mut PK11SymKey;
    let mut nonce: [libc::c_uchar; 12] = [0; 12];
    let mut aadBuf: [libc::c_uchar; 54] = [0; 54];
    rv = rand_bytes(nonce.as_mut_ptr(), 12i32 as size_t);
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        rv = Keypair_new(&mut eph_priv, &mut eph_pub);
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            rv = derive_dh_secret(&mut aes_key, eph_priv, pubkey);
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                param =
                    CK_GCM_PARAMS{pIv: 0 as *mut CK_BYTE,
                                  ulIvLen: 0,
                                  pAAD: 0 as *mut CK_BYTE,
                                  ulAADLen: 0,
                                  ulTagBits: 0,};
                paramItem =
                    SECItemStr{type_0: siBuffer,
                               data: 0 as *mut libc::c_uchar,
                               len: 0,};
                set_gcm_params(&mut paramItem, &mut param, nonce.as_mut_ptr(),
                               eph_pub as const_PublicKey,
                               aadBuf.as_mut_ptr());
                pk = PublicKey_toBytes(eph_pub as const_PublicKey);
                if !((*pk).len == 32i32 as libc::c_uint) {
                    rv = SECFailure
                } else {
                    memcpy(output as *mut libc::c_void,
                           (*pk).data as *const libc::c_void,
                           (*pk).len as libc::c_ulong);
                    memcpy(output.offset(32isize) as *mut libc::c_void,
                           param.pIv as *const libc::c_void, param.ulIvLen);
                    offset =
                        (32i32 as libc::c_ulong).wrapping_add(param.ulIvLen)
                            as libc::c_int;
                    rv =
                        PK11_Encrypt(aes_key, 0x1087i32 as CK_MECHANISM_TYPE,
                                     &mut paramItem,
                                     output.offset(offset as isize),
                                     outputLen,
                                     maxOutputLen.wrapping_sub(offset as
                                                                   libc::c_uint),
                                     input, inputLen);
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        *outputLen =
                            (*outputLen).wrapping_add(32i32 as
                                                          libc::c_uint).wrapping_add(12i32
                                                                                         as
                                                                                         libc::c_uint)
                    }
                }
            }
        }
    }
    PublicKey_clear(eph_pub);
    PrivateKey_clear(eph_priv);
    if !aes_key.is_null() { PK11_FreeSymKey(aes_key); }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn PublicKey_toBytes(mut pubkey: const_PublicKey)
 -> *const SECItem {
    return &(*pubkey).u.ec.publicValue;
}
unsafe extern "C" fn set_gcm_params(mut paramItem: *mut SECItem,
                                    mut param: *mut CK_GCM_PARAMS,
                                    mut nonce: *mut libc::c_uchar,
                                    mut pubkey: const_PublicKey,
                                    mut aadBuf: *mut libc::c_uchar) {
    let mut offset: libc::c_int = 0i32;
    memcpy(aadBuf as *mut libc::c_void,
           b"PrioPacket\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           strlen(b"PrioPacket\x00" as *const u8 as *const libc::c_char));
    offset =
        (offset as
             libc::c_ulong).wrapping_add(strlen(b"PrioPacket\x00" as *const u8
                                                    as *const libc::c_char))
            as libc::c_int as libc::c_int;
    memcpy(aadBuf.offset(offset as isize) as *mut libc::c_void,
           (*PublicKey_toBytes(pubkey)).data as *const libc::c_void,
           32i32 as libc::c_ulong);
    offset += 32i32;
    memcpy(aadBuf.offset(offset as isize) as *mut libc::c_void,
           nonce as *const libc::c_void, 12i32 as libc::c_ulong);
    (*param).pIv = nonce;
    (*param).ulIvLen = 12i32 as CK_ULONG;
    (*param).pAAD = aadBuf;
    (*param).ulAADLen =
        strlen(b"PrioPacket\x00" as *const u8 as
                   *const libc::c_char).wrapping_add(32i32 as
                                                         libc::c_ulong).wrapping_add(12i32
                                                                                         as
                                                                                         libc::c_ulong);
    (*param).ulTagBits = (16i32 * 8i32) as CK_ULONG;
    (*paramItem).type_0 = siBuffer;
    (*paramItem).data = param as *mut libc::c_void as *mut libc::c_uchar;
    (*paramItem).len =
        ::std::mem::size_of::<CK_GCM_PARAMS>() as libc::c_ulong as
            libc::c_uint;
}
unsafe extern "C" fn derive_dh_secret(mut shared_secret: *mut *mut PK11SymKey,
                                      mut priv_0: PrivateKey,
                                      mut pub_0: PublicKey) -> SECStatus {
    if priv_0.is_null() { return SECFailure }
    if pub_0.is_null() { return SECFailure }
    if shared_secret.is_null() { return SECFailure }
    let mut rv: SECStatus = SECSuccess;
    *shared_secret = 0 as *mut PK11SymKey;
    *shared_secret =
        PK11_PubDeriveWithKDF(priv_0, pub_0, 0i32, 0 as *mut SECItem,
                              0 as *mut SECItem,
                              0x1050i32 as CK_MECHANISM_TYPE,
                              0x1087i32 as CK_MECHANISM_TYPE,
                              (0x104i32 | 0x105i32) as CK_ATTRIBUTE_TYPE,
                              16i32, 0x6i32 as CK_ULONG, 0 as *mut SECItem,
                              0 as *mut libc::c_void);
    if (*shared_secret).is_null() { rv = SECFailure }
    return rv;
}
/*
 * Decrypt an arbitrary bitstring using the specified private key.  The output
 * buffer should be at least 16 bytes larger than the plaintext you expect. If
 * `outputLen` >= `inputLen`, you should be safe.
 */
#[no_mangle]
pub unsafe extern "C" fn PrivateKey_decrypt(mut privkey: PrivateKey,
                                            mut output: *mut libc::c_uchar,
                                            mut outputLen: *mut libc::c_uint,
                                            mut maxOutputLen: libc::c_uint,
                                            mut input: *const libc::c_uchar,
                                            mut inputLen: libc::c_uint)
 -> SECStatus {
    let mut nonce: [libc::c_uchar; 12] = [0; 12];
    let mut paramItem: SECItem =
        SECItemStr{type_0: siBuffer, data: 0 as *mut libc::c_uchar, len: 0,};
    let mut param: CK_GCM_PARAMS =
        CK_GCM_PARAMS{pIv: 0 as *mut CK_BYTE,
                      ulIvLen: 0,
                      pAAD: 0 as *mut CK_BYTE,
                      ulAADLen: 0,
                      ulTagBits: 0,};
    let mut offset: libc::c_int = 0;
    let mut aes_key: *mut PK11SymKey = 0 as *mut PK11SymKey;
    let mut eph_pub: PublicKey = 0 as PublicKey;
    let mut aad_buf: [libc::c_uchar; 54] = [0; 54];
    if privkey.is_null() { return SECFailure }
    let mut rv: SECStatus = SECSuccess;
    let mut headerLen: libc::c_uint = 0;
    if PublicKey_encryptSize(0i32 as libc::c_uint, &mut headerLen) as
           libc::c_int != SECSuccess as libc::c_int {
        return SECFailure
    }
    if inputLen < headerLen { return SECFailure }
    let msglen: libc::c_uint = inputLen.wrapping_sub(headerLen);
    if maxOutputLen < msglen ||
           msglen >= (2147483647i32 >> 3i32) as libc::c_uint {
        return SECFailure
    }
    rv = PublicKey_import(&mut eph_pub, input, 32i32 as libc::c_uint);
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        nonce = [0; 12];
        memcpy(nonce.as_mut_ptr() as *mut libc::c_void,
               input.offset(32isize) as *const libc::c_void,
               12i32 as libc::c_ulong);
        paramItem =
            SECItemStr{type_0: siBuffer,
                       data: 0 as *mut libc::c_uchar,
                       len: 0,};
        param =
            CK_GCM_PARAMS{pIv: 0 as *mut CK_BYTE,
                          ulIvLen: 0,
                          pAAD: 0 as *mut CK_BYTE,
                          ulAADLen: 0,
                          ulTagBits: 0,};
        set_gcm_params(&mut paramItem, &mut param, nonce.as_mut_ptr(),
                       eph_pub as const_PublicKey, aad_buf.as_mut_ptr());
        rv = derive_dh_secret(&mut aes_key, privkey, eph_pub);
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            offset = 32i32 + 12i32;
            rv =
                PK11_Decrypt(aes_key, 0x1087i32 as CK_MECHANISM_TYPE,
                             &mut paramItem, output, outputLen, maxOutputLen,
                             input.offset(offset as isize),
                             inputLen.wrapping_sub(offset as libc::c_uint));
            rv as libc::c_int != SECSuccess as libc::c_int;
        }
    }
    PublicKey_clear(eph_pub);
    if !aes_key.is_null() { PK11_FreeSymKey(aes_key); }
    return rv;
}