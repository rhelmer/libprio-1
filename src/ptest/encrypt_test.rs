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
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    /*
** Return the strength of the public key in bytes
*/
    #[no_mangle]
    fn SECKEY_PublicKeyStrength(pubk: *const SECKEYPublicKey) -> libc::c_uint;
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
 * We use the PublicKey and PrivateKey objects for public-key encryption. Each
 * Prio server has an associated public key, and the clients use these keys to
 * encrypt messages to the servers.
 */
    #[no_mangle]
    fn Keypair_new(pvtkey: *mut PrivateKey, pubkey: *mut PublicKey)
     -> SECStatus;
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
    fn PublicKey_import(pk: *mut PublicKey, data: *const libc::c_uchar,
                        dataLen: libc::c_uint) -> SECStatus;
    #[no_mangle]
    fn PrivateKey_import(sk: *mut PrivateKey, privData: *const libc::c_uchar,
                         privDataLen: libc::c_uint,
                         pubData: *const libc::c_uchar,
                         pubDataLen: libc::c_uint) -> SECStatus;
    /*
 * Import a new curve25519 public/private key from a hex string that contains
 * only the characters 0-9a-fA-F.
 *
 * The hex strings passed in must each be of length `CURVE25519_KEY_LEN_HEX`.
 * These functions will allocate a new `PublicKey`/`PrivateKey` object, which
 * the caller must free using `PublicKey_clear`/`PrivateKey_clear`.
 */
    #[no_mangle]
    fn PublicKey_import_hex(pk: *mut PublicKey, hexData: *const libc::c_uchar,
                            dataLen: libc::c_uint) -> SECStatus;
    #[no_mangle]
    fn PrivateKey_import_hex(sk: *mut PrivateKey,
                             privHexData: *const libc::c_uchar,
                             privDataLen: libc::c_uint,
                             pubHexData: *const libc::c_uchar,
                             pubDataLen: libc::c_uint) -> SECStatus;
    /*
 * Export a curve25519 key as a raw byte-array.
 *
 * The output buffer `data` must have length exactly `CURVE25519_KEY_LEN`.
 */
    #[no_mangle]
    fn PublicKey_export(pk: const_PublicKey, data: *mut libc::c_uchar,
                        dataLen: libc::c_uint) -> SECStatus;
    #[no_mangle]
    fn PrivateKey_export(sk: PrivateKey, data: *mut libc::c_uchar,
                         dataLen: libc::c_uint) -> SECStatus;
    /*
 * Export a curve25519 key as a NULL-terminated hex string.
 *
 * The output buffer `data` must have length exactly `CURVE25519_KEY_LEN_HEX +
 * 1`.
 */
    #[no_mangle]
    fn PublicKey_export_hex(pk: const_PublicKey, data: *mut libc::c_uchar,
                            dataLen: libc::c_uint) -> SECStatus;
    #[no_mangle]
    fn PrivateKey_export_hex(sk: PrivateKey, data: *mut libc::c_uchar,
                             dataLen: libc::c_uint) -> SECStatus;
    #[no_mangle]
    fn PublicKey_clear(pubkey: PublicKey);
    #[no_mangle]
    fn PrivateKey_clear(pvtkey: PrivateKey);
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
pub type unnamed_0 = libc::c_uint;
/* shows the current running check */
pub const MU_CHECK: unnamed_0 = 5;
/* shows test cases progress */
pub const MU_CASE: unnamed_0 = 4;
/* shows test suites progress */
pub const MU_SUITE: unnamed_0 = 3;
/* shows a summary */
pub const MU_SUMMARY: unnamed_0 = 2;
/* shows errors only */
pub const MU_ERROR: unnamed_0 = 1;
/* be completely quiet */
pub const MU_QUIET: unnamed_0 = 0;
pub type PublicKey = *mut SECKEYPublicKey;
pub type const_PublicKey = *const SECKEYPublicKey;
pub type PrivateKey = *mut SECKEYPrivateKey;
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[no_mangle]
pub unsafe extern "C" fn mu_test_keygen() {
    let mut rv: SECStatus = SECSuccess;
    let mut pubkey: PublicKey = 0 as PublicKey;
    let mut pvtkey: PrivateKey = 0 as PrivateKey;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = Keypair_new(&mut pvtkey, &mut pubkey);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 29i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 29i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check(SECKEY_PublicKeyStrength(pubkey) == 32)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check(SECKEY_PublicKeyStrength(pubkey) == 32)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        if SECKEY_PublicKeyStrength(pubkey as *const SECKEYPublicKey) ==
               32i32 as libc::c_uint {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/encrypt_test.c:%d: mu_check(SECKEY_PublicKeyStrength(pubkey) == 32) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 30i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/encrypt_test.c:%d: mu_check(SECKEY_PublicKeyStrength(pubkey) == 32) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 30i32);
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
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 33i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 33i32);
            }
        }
    }
    PublicKey_clear(pubkey);
    PrivateKey_clear(pvtkey);
}
#[no_mangle]
pub unsafe extern "C" fn test_encrypt_once(mut bad: libc::c_int,
                                           mut inlen: libc::c_uint) {
    let mut declen: libc::c_uint = 0;
    let mut encryptedBytes: libc::c_uint = 0;
    let mut decryptedBytes: libc::c_uint = 0;
    let mut key_to_use: PrivateKey = 0 as *mut SECKEYPrivateKey;
    let mut rv: SECStatus = SECSuccess;
    let mut pubkey: PublicKey = 0 as PublicKey;
    let mut pvtkey: PrivateKey = 0 as PrivateKey;
    let mut pubkey2: PublicKey = 0 as PublicKey;
    let mut pvtkey2: PrivateKey = 0 as PrivateKey;
    let mut bytes_in: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bytes_enc: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bytes_dec: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut enclen: libc::c_uint = 0;
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PublicKey_encryptSize(inlen, &enclen))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PublicKey_encryptSize(inlen, &enclen))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = PublicKey_encryptSize(inlen, &mut enclen);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_encryptSize(inlen, &enclen))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 53i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_encryptSize(inlen, &enclen))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 53i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        declen = enclen;
        bytes_in = malloc(inlen as libc::c_ulong) as *mut libc::c_uchar;
        if bytes_in.is_null() {
            rv = SECFailure
        } else {
            bytes_enc = malloc(enclen as libc::c_ulong) as *mut libc::c_uchar;
            if bytes_enc.is_null() {
                rv = SECFailure
            } else {
                bytes_dec =
                    malloc(enclen as libc::c_ulong) as *mut libc::c_uchar;
                if bytes_dec.is_null() {
                    rv = SECFailure
                } else {
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((rv = (rand_bytes(bytes_in, inlen))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((rv = (rand_bytes(bytes_in, inlen))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    rv = rand_bytes(bytes_in, inlen as size_t);
                    if rv as libc::c_int == SECSuccess as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (rand_bytes(bytes_in, inlen))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 59i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (rand_bytes(bytes_in, inlen))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 59i32);
                            }
                        }
                    }
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        memset(bytes_dec as *mut libc::c_void, 0i32,
                               declen as libc::c_ulong);
                        encryptedBytes = 0;
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        rv = Keypair_new(&mut pvtkey, &mut pubkey);
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
                                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 64i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 64i32);
                                }
                            }
                        }
                        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey2, &pubkey2))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey2, &pubkey2))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            rv = Keypair_new(&mut pvtkey2, &mut pubkey2);
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
                                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey2, &pubkey2))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                65i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey2, &pubkey2))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                65i32);
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
                                                b"\t\t* Checking mu_check((rv = (PublicKey_encrypt(pubkey, bytes_enc, &encryptedBytes, enclen, bytes_in, inlen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check((rv = (PublicKey_encrypt(pubkey, bytes_enc, &encryptedBytes, enclen, bytes_in, inlen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                rv =
                                    PublicKey_encrypt(pubkey, bytes_enc,
                                                      &mut encryptedBytes,
                                                      enclen, bytes_in,
                                                      inlen);
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
                                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_encrypt(pubkey, bytes_enc, &encryptedBytes, enclen, bytes_in, inlen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    67i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_encrypt(pubkey, bytes_enc, &encryptedBytes, enclen, bytes_in, inlen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    67i32);
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
                                                    b"\t\t* Checking mu_check(encryptedBytes == enclen)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(encryptedBytes == enclen)...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if encryptedBytes == enclen {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(encryptedBytes == enclen) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        68i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(encryptedBytes == enclen) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        68i32);
                                            }
                                        }
                                    }
                                    if bad == 1i32 {
                                        enclen = 30i32 as libc::c_uint
                                    }
                                    if bad == 2i32 {
                                        *bytes_enc.offset(4isize) =
                                            6i32 as libc::c_uchar;
                                        *bytes_enc.offset(5isize) =
                                            0i32 as libc::c_uchar
                                    }
                                    if bad == 3i32 {
                                        *bytes_enc.offset(40isize) =
                                            6i32 as libc::c_uchar;
                                        *bytes_enc.offset(41isize) =
                                            0i32 as libc::c_uchar
                                    }
                                    decryptedBytes = 0;
                                    key_to_use =
                                        if bad == 4i32 {
                                            pvtkey2
                                        } else { pvtkey };
                                    rv =
                                        PrivateKey_decrypt(key_to_use,
                                                           bytes_dec,
                                                           &mut decryptedBytes,
                                                           declen, bytes_enc,
                                                           enclen);
                                    if !(rv as libc::c_int !=
                                             SECSuccess as libc::c_int) {
                                        if mutest_verbose_level >=
                                               MU_CHECK as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"\t\t* Checking mu_check(decryptedBytes == inlen)...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"\t\t* Checking mu_check(decryptedBytes == inlen)...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            }
                                        }
                                        if decryptedBytes == inlen {
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
                                                            b"build/ptest/encrypt_test.c:%d: mu_check(decryptedBytes == inlen) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            87i32);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"build/ptest/encrypt_test.c:%d: mu_check(decryptedBytes == inlen) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            87i32);
                                                }
                                            }
                                        }
                                        if mutest_verbose_level >=
                                               MU_CHECK as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"\t\t* Checking mu_check(!strncmp((char*)bytes_in, (char*)bytes_dec, inlen))...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"\t\t* Checking mu_check(!strncmp((char*)bytes_in, (char*)bytes_dec, inlen))...\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                            }
                                        }
                                        if 0 ==
                                               strncmp(bytes_in as
                                                           *mut libc::c_char,
                                                       bytes_dec as
                                                           *mut libc::c_char,
                                                       inlen as libc::c_ulong)
                                           {
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
                                                            b"build/ptest/encrypt_test.c:%d: mu_check(!strncmp((char*)bytes_in, (char*)bytes_dec, inlen)) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            88i32);
                                                } else {
                                                    fprintf(__stdoutp,
                                                            b"build/ptest/encrypt_test.c:%d: mu_check(!strncmp((char*)bytes_in, (char*)bytes_dec, inlen)) failed, resuming test case\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            88i32);
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
                    b"\t\t* Checking mu_check(bad ? (rv == SECFailure) : (rv == SECSuccess))...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(bad ? (rv == SECFailure) : (rv == SECSuccess))...\n\x00"
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
                        b"build/ptest/encrypt_test.c:%d: mu_check(bad ? (rv == SECFailure) : (rv == SECSuccess)) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 91i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(bad ? (rv == SECFailure) : (rv == SECSuccess)) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 91i32);
            }
        }
    }
    if !bytes_in.is_null() { free(bytes_in as *mut libc::c_void); }
    if !bytes_enc.is_null() { free(bytes_enc as *mut libc::c_void); }
    if !bytes_dec.is_null() { free(bytes_dec as *mut libc::c_void); }
    PublicKey_clear(pubkey);
    PrivateKey_clear(pvtkey);
    PublicKey_clear(pubkey2);
    PrivateKey_clear(pvtkey2);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_encrypt_good() {
    test_encrypt_once(0i32, 100i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_encrypt_good_long() {
    test_encrypt_once(0i32, 1000000i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_encrypt_too_short() {
    test_encrypt_once(1i32, 87i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_encrypt_garbage() {
    test_encrypt_once(2i32, 10023i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_encrypt_garbage2() {
    test_encrypt_once(3i32, 8123i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_decrypt_wrong_key() {
    test_encrypt_once(4i32, 81230i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_export_pubkey() {
    let mut rv: SECStatus = SECSuccess;
    let mut pubkey: PublicKey = 0 as PublicKey;
    let mut raw_bytes: [libc::c_uchar; 32] = [0; 32];
    let mut raw_bytes2: [libc::c_uchar; 32] = [0; 32];
    let mut i: libc::c_int = 0i32;
    while i < 32i32 {
        raw_bytes[i as usize] =
            ((3i32 * i + 7i32) % 0xffi32) as libc::c_uchar;
        i += 1
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PublicKey_import(&pubkey, raw_bytes, 32))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PublicKey_import(&pubkey, raw_bytes, 32))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv =
        PublicKey_import(&mut pubkey, raw_bytes.as_mut_ptr(),
                         32i32 as libc::c_uint);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import(&pubkey, raw_bytes, 32))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 154i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import(&pubkey, raw_bytes, 32))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 154i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, raw_bytes2, 32))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, raw_bytes2, 32))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        rv =
            PublicKey_export(pubkey as const_PublicKey,
                             raw_bytes2.as_mut_ptr(), 32i32 as libc::c_uint);
        if rv as libc::c_int == SECSuccess as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, raw_bytes2, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 155i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, raw_bytes2, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 155i32);
                }
            }
        }
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            let mut i_0: libc::c_int = 0i32;
            while i_0 < 32i32 {
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check(raw_bytes[i] == raw_bytes2[i])...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check(raw_bytes[i] == raw_bytes2[i])...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                if raw_bytes[i_0 as usize] as libc::c_int ==
                       raw_bytes2[i_0 as usize] as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check(raw_bytes[i] == raw_bytes2[i]) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    158i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check(raw_bytes[i] == raw_bytes2[i]) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    158i32);
                        }
                    }
                }
                i_0 += 1
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
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 162i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 162i32);
            }
        }
    }
    PublicKey_clear(pubkey);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_export_pubkey_zeros() {
    let mut rv: SECStatus = SECSuccess;
    let mut pubkey: PublicKey = 0 as PublicKey;
    let mut raw_bytes: [libc::c_uchar; 32] = [0; 32];
    let mut raw_bytes2: [libc::c_uchar; 32] = [0; 32];
    let mut i: libc::c_int = 0i32;
    while i < 32i32 {
        raw_bytes[i as usize] =
            ((3i32 * i + 7i32) % 0xffi32) as libc::c_uchar;
        i += 1
    }
    let mut i_0: libc::c_int = 0i32;
    while i_0 < 15i32 {
        raw_bytes[i_0 as usize] = 0i32 as libc::c_uchar;
        i_0 += 1
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PublicKey_import(&pubkey, raw_bytes, 32))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PublicKey_import(&pubkey, raw_bytes, 32))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv =
        PublicKey_import(&mut pubkey, raw_bytes.as_mut_ptr(),
                         32i32 as libc::c_uint);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import(&pubkey, raw_bytes, 32))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 183i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import(&pubkey, raw_bytes, 32))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 183i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, raw_bytes2, 32))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, raw_bytes2, 32))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        rv =
            PublicKey_export(pubkey as const_PublicKey,
                             raw_bytes2.as_mut_ptr(), 32i32 as libc::c_uint);
        if rv as libc::c_int == SECSuccess as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, raw_bytes2, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 184i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, raw_bytes2, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 184i32);
                }
            }
        }
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            let mut i_1: libc::c_int = 0i32;
            while i_1 < 32i32 {
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check(raw_bytes[i] == raw_bytes2[i])...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check(raw_bytes[i] == raw_bytes2[i])...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                if raw_bytes[i_1 as usize] as libc::c_int ==
                       raw_bytes2[i_1 as usize] as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check(raw_bytes[i] == raw_bytes2[i]) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    187i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check(raw_bytes[i] == raw_bytes2[i]) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    187i32);
                        }
                    }
                }
                i_1 += 1
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
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 191i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 191i32);
            }
        }
    }
    PublicKey_clear(pubkey);
}
#[no_mangle]
pub unsafe extern "C" fn test_export_privkey(mut zeros: libc::c_int) {
    let mut rv: SECStatus = SECSuccess;
    let mut pubkey: PublicKey = 0 as PublicKey;
    let mut pvtkey: PrivateKey = 0 as PrivateKey;
    let mut pvtkey_imp: PrivateKey = 0 as PrivateKey;
    let mut privData: [libc::c_uchar; 32] = [0; 32];
    let mut privData2: [libc::c_uchar; 32] = [0; 32];
    let mut pubData: [libc::c_uchar; 32] = [0; 32];
    let mut input: [libc::c_uchar; 67] = [0; 67];
    let mut output: [libc::c_uchar; 256] = [0; 256];
    let mut decrypt: [libc::c_uchar; 256] = [0; 256];
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) <
              ::std::mem::size_of::<[libc::c_uchar; 67]>() as libc::c_ulong {
        input[i as usize] = i as libc::c_uchar;
        i = i.wrapping_add(1)
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = Keypair_new(&mut pvtkey, &mut pubkey);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 214i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 214i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((rv = (PrivateKey_export(pvtkey, privData, 32))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((rv = (PrivateKey_export(pvtkey, privData, 32))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        rv =
            PrivateKey_export(pvtkey, privData.as_mut_ptr(),
                              32i32 as libc::c_uint);
        if rv as libc::c_int == SECSuccess as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_export(pvtkey, privData, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 215i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_export(pvtkey, privData, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 215i32);
                }
            }
        }
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, pubData, 32))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, pubData, 32))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            rv =
                PublicKey_export(pubkey as const_PublicKey,
                                 pubData.as_mut_ptr(), 32i32 as libc::c_uint);
            if rv as libc::c_int == SECSuccess as libc::c_int {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, pubData, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                216i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, pubData, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                216i32);
                    }
                }
            }
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                if 0 != zeros {
                    memset(privData.as_mut_ptr() as *mut libc::c_void, 0i32,
                           5i32 as libc::c_ulong);
                }
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((rv = (PrivateKey_import(&pvtkey_imp, privData, 32, pubData, 32))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((rv = (PrivateKey_import(&pvtkey_imp, privData, 32, pubData, 32))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                rv =
                    PrivateKey_import(&mut pvtkey_imp, privData.as_mut_ptr(),
                                      32i32 as libc::c_uint,
                                      pubData.as_mut_ptr(),
                                      32i32 as libc::c_uint);
                if rv as libc::c_int == SECSuccess as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_import(&pvtkey_imp, privData, 32, pubData, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    224i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_import(&pvtkey_imp, privData, 32, pubData, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    224i32);
                        }
                    }
                }
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((rv = (PrivateKey_export(pvtkey_imp, privData2, 32))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((rv = (PrivateKey_export(pvtkey_imp, privData2, 32))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    rv =
                        PrivateKey_export(pvtkey_imp, privData2.as_mut_ptr(),
                                          32i32 as libc::c_uint);
                    if rv as libc::c_int == SECSuccess as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_export(pvtkey_imp, privData2, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 225i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_export(pvtkey_imp, privData2, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 225i32);
                            }
                        }
                    }
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check(!memcmp(privData, privData2, 32))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check(!memcmp(privData, privData2, 32))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        if 0 ==
                               memcmp(privData.as_mut_ptr() as
                                          *const libc::c_void,
                                      privData2.as_mut_ptr() as
                                          *const libc::c_void,
                                      32i32 as libc::c_ulong) {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/encrypt_test.c:%d: mu_check(!memcmp(privData, privData2, 32)) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 227i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/encrypt_test.c:%d: mu_check(!memcmp(privData, privData2, 32)) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 227i32);
                                }
                            }
                        }
                        if 0 == zeros {
                            let mut outputLen: libc::c_uint = 0;
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((rv = (PublicKey_encrypt(pubkey, output, &outputLen, sizeof(output), input, sizeof(input)))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((rv = (PublicKey_encrypt(pubkey, output, &outputLen, sizeof(output), input, sizeof(input)))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            rv =
                                PublicKey_encrypt(pubkey, output.as_mut_ptr(),
                                                  &mut outputLen,
                                                  ::std::mem::size_of::<[libc::c_uchar; 256]>()
                                                      as libc::c_ulong as
                                                      libc::c_uint,
                                                  input.as_mut_ptr(),
                                                  ::std::mem::size_of::<[libc::c_uchar; 67]>()
                                                      as libc::c_ulong as
                                                      libc::c_uint);
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
                                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_encrypt(pubkey, output, &outputLen, sizeof(output), input, sizeof(input)))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                232i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_encrypt(pubkey, output, &outputLen, sizeof(output), input, sizeof(input)))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                232i32);
                                    }
                                }
                            }
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                // Check that can decrypt with imported private key.
                                let mut plainLen: libc::c_uint = 0;
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check((rv = (PrivateKey_decrypt(pvtkey_imp, decrypt, &plainLen, sizeof(decrypt), output, outputLen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check((rv = (PrivateKey_decrypt(pvtkey_imp, decrypt, &plainLen, sizeof(decrypt), output, outputLen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                rv =
                                    PrivateKey_decrypt(pvtkey_imp,
                                                       decrypt.as_mut_ptr(),
                                                       &mut plainLen,
                                                       ::std::mem::size_of::<[libc::c_uchar; 256]>()
                                                           as libc::c_ulong as
                                                           libc::c_uint,
                                                       output.as_mut_ptr(),
                                                       outputLen);
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
                                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_decrypt(pvtkey_imp, decrypt, &plainLen, sizeof(decrypt), output, outputLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    237i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_decrypt(pvtkey_imp, decrypt, &plainLen, sizeof(decrypt), output, outputLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    237i32);
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
                                                    b"\t\t* Checking mu_check(plainLen == sizeof(input))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(plainLen == sizeof(input))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if plainLen as libc::c_ulong ==
                                           ::std::mem::size_of::<[libc::c_uchar; 67]>()
                                               as libc::c_ulong {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(plainLen == sizeof(input)) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        239i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(plainLen == sizeof(input)) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        239i32);
                                            }
                                        }
                                    }
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check(!memcmp(input, decrypt, sizeof(input)))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(!memcmp(input, decrypt, sizeof(input)))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if 0 ==
                                           memcmp(input.as_mut_ptr() as
                                                      *const libc::c_void,
                                                  decrypt.as_mut_ptr() as
                                                      *const libc::c_void,
                                                  ::std::mem::size_of::<[libc::c_uchar; 67]>()
                                                      as libc::c_ulong) {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(!memcmp(input, decrypt, sizeof(input))) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        240i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(!memcmp(input, decrypt, sizeof(input))) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        240i32);
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
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 244i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 244i32);
            }
        }
    }
    PublicKey_clear(pubkey);
    PrivateKey_clear(pvtkey);
    PrivateKey_clear(pvtkey_imp);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_export_privkey() {
    test_export_privkey(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_export_privkey_zeros() {
    test_export_privkey(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn test_export_hex_privkey(mut zeros: libc::c_int) {
    let mut rv: SECStatus = SECSuccess;
    let mut pubkey: PublicKey = 0 as PublicKey;
    let mut pvtkey: PrivateKey = 0 as PrivateKey;
    let mut pvtkey_imp: PrivateKey = 0 as PrivateKey;
    let mut privData: [libc::c_uchar; 65] = [0; 65];
    let mut privData2: [libc::c_uchar; 65] = [0; 65];
    let mut pubData: [libc::c_uchar; 65] = [0; 65];
    let mut input: [libc::c_uchar; 67] = [0; 67];
    let mut output: [libc::c_uchar; 256] = [0; 256];
    let mut decrypt: [libc::c_uchar; 256] = [0; 256];
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) <
              ::std::mem::size_of::<[libc::c_uchar; 67]>() as libc::c_ulong {
        input[i as usize] = i as libc::c_uchar;
        i = i.wrapping_add(1)
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv = Keypair_new(&mut pvtkey, &mut pubkey);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 281i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (Keypair_new(&pvtkey, &pubkey))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 281i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((rv = (PrivateKey_export_hex(pvtkey, privData, 64 + 1))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((rv = (PrivateKey_export_hex(pvtkey, privData, 64 + 1))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        rv =
            PrivateKey_export_hex(pvtkey, privData.as_mut_ptr(),
                                  (64i32 + 1i32) as libc::c_uint);
        if rv as libc::c_int == SECSuccess as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_export_hex(pvtkey, privData, 64 + 1))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 283i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_export_hex(pvtkey, privData, 64 + 1))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 283i32);
                }
            }
        }
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((rv = (PublicKey_export_hex(pubkey, pubData, 64 + 1))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((rv = (PublicKey_export_hex(pubkey, pubData, 64 + 1))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            rv =
                PublicKey_export_hex(pubkey as const_PublicKey,
                                     pubData.as_mut_ptr(),
                                     (64i32 + 1i32) as libc::c_uint);
            if rv as libc::c_int == SECSuccess as libc::c_int {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export_hex(pubkey, pubData, 64 + 1))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                284i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export_hex(pubkey, pubData, 64 + 1))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                284i32);
                    }
                }
            }
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                if 0 != zeros {
                    memset(privData.as_mut_ptr() as *mut libc::c_void,
                           '0' as i32, 5i32 as libc::c_ulong);
                }
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((rv = (PrivateKey_import_hex(&pvtkey_imp, privData, 64, pubData, 64))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((rv = (PrivateKey_import_hex(&pvtkey_imp, privData, 64, pubData, 64))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                rv =
                    PrivateKey_import_hex(&mut pvtkey_imp,
                                          privData.as_mut_ptr(),
                                          64i32 as libc::c_uint,
                                          pubData.as_mut_ptr(),
                                          64i32 as libc::c_uint);
                if rv as libc::c_int == SECSuccess as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_import_hex(&pvtkey_imp, privData, 64, pubData, 64))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    292i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_import_hex(&pvtkey_imp, privData, 64, pubData, 64))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    292i32);
                        }
                    }
                }
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((rv = (PrivateKey_export_hex(pvtkey_imp, privData2, 64 + 1))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((rv = (PrivateKey_export_hex(pvtkey_imp, privData2, 64 + 1))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    rv =
                        PrivateKey_export_hex(pvtkey_imp,
                                              privData2.as_mut_ptr(),
                                              (64i32 + 1i32) as libc::c_uint);
                    if rv as libc::c_int == SECSuccess as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_export_hex(pvtkey_imp, privData2, 64 + 1))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 294i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_export_hex(pvtkey_imp, privData2, 64 + 1))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 294i32);
                            }
                        }
                    }
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check(!memcmp(privData, privData2, 32))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check(!memcmp(privData, privData2, 32))...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        if 0 ==
                               memcmp(privData.as_mut_ptr() as
                                          *const libc::c_void,
                                      privData2.as_mut_ptr() as
                                          *const libc::c_void,
                                      32i32 as libc::c_ulong) {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/encrypt_test.c:%d: mu_check(!memcmp(privData, privData2, 32)) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 296i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/encrypt_test.c:%d: mu_check(!memcmp(privData, privData2, 32)) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 296i32);
                                }
                            }
                        }
                        if 0 == zeros {
                            let mut outputLen: libc::c_uint = 0;
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_check((rv = (PublicKey_encrypt(pubkey, output, &outputLen, sizeof(output), input, sizeof(input)))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_check((rv = (PublicKey_encrypt(pubkey, output, &outputLen, sizeof(output), input, sizeof(input)))) == SECSuccess)...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            rv =
                                PublicKey_encrypt(pubkey, output.as_mut_ptr(),
                                                  &mut outputLen,
                                                  ::std::mem::size_of::<[libc::c_uchar; 256]>()
                                                      as libc::c_ulong as
                                                      libc::c_uint,
                                                  input.as_mut_ptr(),
                                                  ::std::mem::size_of::<[libc::c_uchar; 67]>()
                                                      as libc::c_ulong as
                                                      libc::c_uint);
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
                                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_encrypt(pubkey, output, &outputLen, sizeof(output), input, sizeof(input)))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                301i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_encrypt(pubkey, output, &outputLen, sizeof(output), input, sizeof(input)))) == SECSuccess) failed, resuming test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                301i32);
                                    }
                                }
                            }
                            if !(rv as libc::c_int !=
                                     SECSuccess as libc::c_int) {
                                // Check that can decrypt with imported private key.
                                let mut plainLen: libc::c_uint = 0;
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check((rv = (PrivateKey_decrypt(pvtkey_imp, decrypt, &plainLen, sizeof(decrypt), output, outputLen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check((rv = (PrivateKey_decrypt(pvtkey_imp, decrypt, &plainLen, sizeof(decrypt), output, outputLen))) == SECSuccess)...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                rv =
                                    PrivateKey_decrypt(pvtkey_imp,
                                                       decrypt.as_mut_ptr(),
                                                       &mut plainLen,
                                                       ::std::mem::size_of::<[libc::c_uchar; 256]>()
                                                           as libc::c_ulong as
                                                           libc::c_uint,
                                                       output.as_mut_ptr(),
                                                       outputLen);
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
                                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_decrypt(pvtkey_imp, decrypt, &plainLen, sizeof(decrypt), output, outputLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    306i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PrivateKey_decrypt(pvtkey_imp, decrypt, &plainLen, sizeof(decrypt), output, outputLen))) == SECSuccess) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    306i32);
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
                                                    b"\t\t* Checking mu_check(plainLen == sizeof(input))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(plainLen == sizeof(input))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if plainLen as libc::c_ulong ==
                                           ::std::mem::size_of::<[libc::c_uchar; 67]>()
                                               as libc::c_ulong {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(plainLen == sizeof(input)) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        308i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(plainLen == sizeof(input)) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        308i32);
                                            }
                                        }
                                    }
                                    if mutest_verbose_level >=
                                           MU_CHECK as libc::c_int {
                                        if mutest_verbose_level ==
                                               MU_ERROR as libc::c_int {
                                            fprintf(__stderrp,
                                                    b"\t\t* Checking mu_check(!memcmp(input, decrypt, sizeof(input)))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"\t\t* Checking mu_check(!memcmp(input, decrypt, sizeof(input)))...\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                        }
                                    }
                                    if 0 ==
                                           memcmp(input.as_mut_ptr() as
                                                      *const libc::c_void,
                                                  decrypt.as_mut_ptr() as
                                                      *const libc::c_void,
                                                  ::std::mem::size_of::<[libc::c_uchar; 67]>()
                                                      as libc::c_ulong) {
                                        mutest_passed_checks += 1
                                    } else {
                                        mutest_failed_checks += 1;
                                        mutest_case_failed = 1i32;
                                        if mutest_verbose_level >=
                                               MU_ERROR as libc::c_int {
                                            if mutest_verbose_level ==
                                                   MU_ERROR as libc::c_int {
                                                fprintf(__stderrp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(!memcmp(input, decrypt, sizeof(input))) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        309i32);
                                            } else {
                                                fprintf(__stdoutp,
                                                        b"build/ptest/encrypt_test.c:%d: mu_check(!memcmp(input, decrypt, sizeof(input))) failed, resuming test case\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        309i32);
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
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 313i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 313i32);
            }
        }
    }
    PublicKey_clear(pubkey);
    PrivateKey_clear(pvtkey);
    PrivateKey_clear(pvtkey_imp);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_export_hex_privkey() {
    test_export_hex_privkey(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_export_hex_privkey_zeros() {
    test_export_hex_privkey(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_import_hex_wrongsize() {
    let mut pvtkey: PrivateKey = 0 as PrivateKey;
    let mut privData: [libc::c_uchar; 33] = [0; 33];
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, privData, 7, privData, 8) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, privData, 7, privData, 8) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if PrivateKey_import_hex(&mut pvtkey, privData.as_mut_ptr(),
                             7i32 as libc::c_uint, privData.as_mut_ptr(),
                             8i32 as libc::c_uint) as libc::c_int ==
           SECFailure as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, privData, 7, privData, 8) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 339i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, privData, 7, privData, 8) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 339i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, privData, 32, privData, 1024) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, privData, 32, privData, 1024) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if PrivateKey_import_hex(&mut pvtkey, privData.as_mut_ptr(),
                             32i32 as libc::c_uint, privData.as_mut_ptr(),
                             1024i32 as libc::c_uint) as libc::c_int ==
           SECFailure as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, privData, 32, privData, 1024) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 341i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, privData, 32, privData, 1024) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 341i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, privData, 31, privData, 32) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, privData, 31, privData, 32) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if PrivateKey_import_hex(&mut pvtkey, privData.as_mut_ptr(),
                             31i32 as libc::c_uint, privData.as_mut_ptr(),
                             32i32 as libc::c_uint) as libc::c_int ==
           SECFailure as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, privData, 31, privData, 32) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 343i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, privData, 31, privData, 32) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 343i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, ((void*)0), 32, privData, 32) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, ((void*)0), 32, privData, 32) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if PrivateKey_import_hex(&mut pvtkey, 0 as *const libc::c_uchar,
                             32i32 as libc::c_uint, privData.as_mut_ptr(),
                             32i32 as libc::c_uint) as libc::c_int ==
           SECFailure as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, ((void*)0), 32, privData, 32) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 345i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, ((void*)0), 32, privData, 32) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 345i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, privData, 32, ((void*)0), 32) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(PrivateKey_import_hex(&pvtkey, privData, 32, ((void*)0), 32) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if PrivateKey_import_hex(&mut pvtkey, privData.as_mut_ptr(),
                             32i32 as libc::c_uint, 0 as *const libc::c_uchar,
                             32i32 as libc::c_uint) as libc::c_int ==
           SECFailure as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, privData, 32, ((void*)0), 32) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 347i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PrivateKey_import_hex(&pvtkey, privData, 32, ((void*)0), 32) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 347i32);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mu_test_export_hex() {
    let mut rv: SECStatus = SECSuccess;
    let mut pubkey: PublicKey = 0 as PublicKey;
    let hex_bytes: [libc::c_uchar; 64] =
        *::std::mem::transmute::<&[u8; 64],
                                 &[libc::c_uchar; 64]>(b"102030405060708090A0B0C0D0E0F00000FFEEDDCCBBAA998877665544332211");
    let hex_bytesl: [libc::c_uchar; 64] =
        *::std::mem::transmute::<&[u8; 64],
                                 &[libc::c_uchar; 64]>(b"102030405060708090a0B0C0D0E0F00000FfeEddcCbBaa998877665544332211");
    let raw_bytes_should: [libc::c_uchar; 32] =
        [0x10i32 as libc::c_uchar, 0x20i32 as libc::c_uchar,
         0x30i32 as libc::c_uchar, 0x40i32 as libc::c_uchar,
         0x50i32 as libc::c_uchar, 0x60i32 as libc::c_uchar,
         0x70i32 as libc::c_uchar, 0x80i32 as libc::c_uchar,
         0x90i32 as libc::c_uchar, 0xa0i32 as libc::c_uchar,
         0xb0i32 as libc::c_uchar, 0xc0i32 as libc::c_uchar,
         0xd0i32 as libc::c_uchar, 0xe0i32 as libc::c_uchar,
         0xf0i32 as libc::c_uchar, 0i32 as libc::c_uchar,
         0i32 as libc::c_uchar, 0xffi32 as libc::c_uchar,
         0xeei32 as libc::c_uchar, 0xddi32 as libc::c_uchar,
         0xcci32 as libc::c_uchar, 0xbbi32 as libc::c_uchar,
         0xaai32 as libc::c_uchar, 0x99i32 as libc::c_uchar,
         0x88i32 as libc::c_uchar, 0x77i32 as libc::c_uchar,
         0x66i32 as libc::c_uchar, 0x55i32 as libc::c_uchar,
         0x44i32 as libc::c_uchar, 0x33i32 as libc::c_uchar,
         0x22i32 as libc::c_uchar, 0x11i32 as libc::c_uchar];
    let mut raw_bytes: [libc::c_uchar; 32] = [0; 32];
    let mut hex_bytes2: [libc::c_uchar; 65] = [0; 65];
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32 - 1) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32 - 1) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if PublicKey_import_hex(&mut pubkey, hex_bytes.as_ptr(),
                            (2i32 * 32i32 - 1i32) as libc::c_uint) as
           libc::c_int == SECFailure as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32 - 1) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 372i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32 - 1) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 372i32);
            }
        }
    }
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check(PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32 + 1) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check(PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32 + 1) == SECFailure)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if PublicKey_import_hex(&mut pubkey, hex_bytes.as_ptr(),
                            (2i32 * 32i32 + 1i32) as libc::c_uint) as
           libc::c_int == SECFailure as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32 + 1) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 374i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32 + 1) == SECFailure) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 374i32);
            }
        }
    }
    // Import a key in upper-case hex
    if mutest_verbose_level >= MU_CHECK as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\t\t* Checking mu_check((rv = (PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(__stdoutp,
                    b"\t\t* Checking mu_check((rv = (PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32))) == SECSuccess)...\n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    rv =
        PublicKey_import_hex(&mut pubkey, hex_bytes.as_ptr(),
                             (2i32 * 32i32) as libc::c_uint);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        mutest_passed_checks += 1
    } else {
        mutest_failed_checks += 1;
        mutest_case_failed = 1i32;
        if mutest_verbose_level >= MU_ERROR as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 377i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import_hex(&pubkey, hex_bytes, 2 * 32))) == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 377i32);
            }
        }
    }
    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
        if mutest_verbose_level >= MU_CHECK as libc::c_int {
            if mutest_verbose_level == MU_ERROR as libc::c_int {
                fprintf(__stderrp,
                        b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, raw_bytes, 32))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                fprintf(__stdoutp,
                        b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, raw_bytes, 32))) == SECSuccess)...\n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        rv =
            PublicKey_export(pubkey as const_PublicKey,
                             raw_bytes.as_mut_ptr(), 32i32 as libc::c_uint);
        if rv as libc::c_int == SECSuccess as libc::c_int {
            mutest_passed_checks += 1
        } else {
            mutest_failed_checks += 1;
            mutest_case_failed = 1i32;
            if mutest_verbose_level >= MU_ERROR as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, raw_bytes, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 378i32);
                } else {
                    fprintf(__stdoutp,
                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, raw_bytes, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                as *const u8 as *const libc::c_char, 378i32);
                }
            }
        }
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            PublicKey_clear(pubkey);
            pubkey = 0 as PublicKey;
            let mut i: libc::c_int = 0i32;
            while i < 32i32 {
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check(raw_bytes[i] == raw_bytes_should[i])...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check(raw_bytes[i] == raw_bytes_should[i])...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                if raw_bytes[i as usize] as libc::c_int ==
                       raw_bytes_should[i as usize] as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check(raw_bytes[i] == raw_bytes_should[i]) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    383i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check(raw_bytes[i] == raw_bytes_should[i]) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    383i32);
                        }
                    }
                }
                i += 1
            }
            // Import a key in mixed-case hex
            if mutest_verbose_level >= MU_CHECK as libc::c_int {
                if mutest_verbose_level == MU_ERROR as libc::c_int {
                    fprintf(__stderrp,
                            b"\t\t* Checking mu_check((rv = (PublicKey_import_hex(&pubkey, hex_bytesl, 2 * 32))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                } else {
                    fprintf(__stdoutp,
                            b"\t\t* Checking mu_check((rv = (PublicKey_import_hex(&pubkey, hex_bytesl, 2 * 32))) == SECSuccess)...\n\x00"
                                as *const u8 as *const libc::c_char);
                }
            }
            rv =
                PublicKey_import_hex(&mut pubkey, hex_bytesl.as_ptr(),
                                     (2i32 * 32i32) as libc::c_uint);
            if rv as libc::c_int == SECSuccess as libc::c_int {
                mutest_passed_checks += 1
            } else {
                mutest_failed_checks += 1;
                mutest_case_failed = 1i32;
                if mutest_verbose_level >= MU_ERROR as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import_hex(&pubkey, hex_bytesl, 2 * 32))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                387i32);
                    } else {
                        fprintf(__stdoutp,
                                b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import_hex(&pubkey, hex_bytesl, 2 * 32))) == SECSuccess) failed, resuming test case\n\x00"
                                    as *const u8 as *const libc::c_char,
                                387i32);
                    }
                }
            }
            if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                if mutest_verbose_level >= MU_CHECK as libc::c_int {
                    if mutest_verbose_level == MU_ERROR as libc::c_int {
                        fprintf(__stderrp,
                                b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, raw_bytes, 32))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    } else {
                        fprintf(__stdoutp,
                                b"\t\t* Checking mu_check((rv = (PublicKey_export(pubkey, raw_bytes, 32))) == SECSuccess)...\n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                }
                rv =
                    PublicKey_export(pubkey as const_PublicKey,
                                     raw_bytes.as_mut_ptr(),
                                     32i32 as libc::c_uint);
                if rv as libc::c_int == SECSuccess as libc::c_int {
                    mutest_passed_checks += 1
                } else {
                    mutest_failed_checks += 1;
                    mutest_case_failed = 1i32;
                    if mutest_verbose_level >= MU_ERROR as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, raw_bytes, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    388i32);
                        } else {
                            fprintf(__stdoutp,
                                    b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export(pubkey, raw_bytes, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    388i32);
                        }
                    }
                }
                if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                    PublicKey_clear(pubkey);
                    pubkey = 0 as PublicKey;
                    let mut i_0: libc::c_int = 0i32;
                    while i_0 < 32i32 {
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check(raw_bytes[i] == raw_bytes_should[i])...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check(raw_bytes[i] == raw_bytes_should[i])...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        if raw_bytes[i_0 as usize] as libc::c_int ==
                               raw_bytes_should[i_0 as usize] as libc::c_int {
                            mutest_passed_checks += 1
                        } else {
                            mutest_failed_checks += 1;
                            mutest_case_failed = 1i32;
                            if mutest_verbose_level >= MU_ERROR as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"build/ptest/encrypt_test.c:%d: mu_check(raw_bytes[i] == raw_bytes_should[i]) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 393i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/encrypt_test.c:%d: mu_check(raw_bytes[i] == raw_bytes_should[i]) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 393i32);
                                }
                            }
                        }
                        i_0 += 1
                    }
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(PublicKey_import(&pubkey, raw_bytes_should, 32 - 1) == SECFailure)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(PublicKey_import(&pubkey, raw_bytes_should, 32 - 1) == SECFailure)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if PublicKey_import(&mut pubkey,
                                        raw_bytes_should.as_ptr(),
                                        (32i32 - 1i32) as libc::c_uint) as
                           libc::c_int == SECFailure as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check(PublicKey_import(&pubkey, raw_bytes_should, 32 - 1) == SECFailure) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 397i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check(PublicKey_import(&pubkey, raw_bytes_should, 32 - 1) == SECFailure) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 397i32);
                            }
                        }
                    }
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check(PublicKey_import(&pubkey, raw_bytes_should, 32 + 1) == SECFailure)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check(PublicKey_import(&pubkey, raw_bytes_should, 32 + 1) == SECFailure)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    if PublicKey_import(&mut pubkey,
                                        raw_bytes_should.as_ptr(),
                                        (32i32 + 1i32) as libc::c_uint) as
                           libc::c_int == SECFailure as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check(PublicKey_import(&pubkey, raw_bytes_should, 32 + 1) == SECFailure) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 399i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check(PublicKey_import(&pubkey, raw_bytes_should, 32 + 1) == SECFailure) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 399i32);
                            }
                        }
                    }
                    // Import a raw key and export as hex
                    if mutest_verbose_level >= MU_CHECK as libc::c_int {
                        if mutest_verbose_level == MU_ERROR as libc::c_int {
                            fprintf(__stderrp,
                                    b"\t\t* Checking mu_check((rv = (PublicKey_import(&pubkey, raw_bytes_should, 32))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        } else {
                            fprintf(__stdoutp,
                                    b"\t\t* Checking mu_check((rv = (PublicKey_import(&pubkey, raw_bytes_should, 32))) == SECSuccess)...\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                    }
                    rv =
                        PublicKey_import(&mut pubkey,
                                         raw_bytes_should.as_ptr(),
                                         32i32 as libc::c_uint);
                    if rv as libc::c_int == SECSuccess as libc::c_int {
                        mutest_passed_checks += 1
                    } else {
                        mutest_failed_checks += 1;
                        mutest_case_failed = 1i32;
                        if mutest_verbose_level >= MU_ERROR as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import(&pubkey, raw_bytes_should, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 402i32);
                            } else {
                                fprintf(__stdoutp,
                                        b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_import(&pubkey, raw_bytes_should, 32))) == SECSuccess) failed, resuming test case\n\x00"
                                            as *const u8 as
                                            *const libc::c_char, 402i32);
                            }
                        }
                    }
                    if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                        if mutest_verbose_level >= MU_CHECK as libc::c_int {
                            if mutest_verbose_level == MU_ERROR as libc::c_int
                               {
                                fprintf(__stderrp,
                                        b"\t\t* Checking mu_check((rv = (PublicKey_export_hex(pubkey, hex_bytes2, 64 + 1))) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            } else {
                                fprintf(__stdoutp,
                                        b"\t\t* Checking mu_check((rv = (PublicKey_export_hex(pubkey, hex_bytes2, 64 + 1))) == SECSuccess)...\n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                        }
                        rv =
                            PublicKey_export_hex(pubkey as const_PublicKey,
                                                 hex_bytes2.as_mut_ptr(),
                                                 (64i32 + 1i32) as
                                                     libc::c_uint);
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
                                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export_hex(pubkey, hex_bytes2, 64 + 1))) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 404i32);
                                } else {
                                    fprintf(__stdoutp,
                                            b"build/ptest/encrypt_test.c:%d: mu_check((rv = (PublicKey_export_hex(pubkey, hex_bytes2, 64 + 1))) == SECSuccess) failed, resuming test case\n\x00"
                                                as *const u8 as
                                                *const libc::c_char, 404i32);
                                }
                            }
                        }
                        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
                            let mut i_1: libc::c_int = 0i32;
                            while i_1 < 2i32 * 32i32 {
                                if mutest_verbose_level >=
                                       MU_CHECK as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"\t\t* Checking mu_check(hex_bytes[i] == hex_bytes2[i])...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"\t\t* Checking mu_check(hex_bytes[i] == hex_bytes2[i])...\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                    }
                                }
                                if hex_bytes[i_1 as usize] as libc::c_int ==
                                       hex_bytes2[i_1 as usize] as libc::c_int
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
                                                    b"build/ptest/encrypt_test.c:%d: mu_check(hex_bytes[i] == hex_bytes2[i]) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    407i32);
                                        } else {
                                            fprintf(__stdoutp,
                                                    b"build/ptest/encrypt_test.c:%d: mu_check(hex_bytes[i] == hex_bytes2[i]) failed, resuming test case\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    407i32);
                                        }
                                    }
                                }
                                i_1 += 1
                            }
                            if mutest_verbose_level >= MU_CHECK as libc::c_int
                               {
                                if mutest_verbose_level ==
                                       MU_ERROR as libc::c_int {
                                    fprintf(__stderrp,
                                            b"\t\t* Checking mu_ensure(hex_bytes2[2 * 32] == \'\\0\')...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                } else {
                                    fprintf(__stdoutp,
                                            b"\t\t* Checking mu_ensure(hex_bytes2[2 * 32] == \'\\0\')...\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                            }
                            if hex_bytes2[(2i32 * 32i32) as usize] as
                                   libc::c_int == '\u{0}' as i32 {
                                mutest_passed_checks += 1
                            } else {
                                mutest_failed_checks += 1;
                                mutest_case_failed = 1i32;
                                if mutest_verbose_level >=
                                       MU_ERROR as libc::c_int {
                                    if mutest_verbose_level ==
                                           MU_ERROR as libc::c_int {
                                        fprintf(__stderrp,
                                                b"build/ptest/encrypt_test.c:%d: mu_ensure(hex_bytes2[2 * 32] == \'\\0\') failed, aborting test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                409i32);
                                    } else {
                                        fprintf(__stdoutp,
                                                b"build/ptest/encrypt_test.c:%d: mu_ensure(hex_bytes2[2 * 32] == \'\\0\') failed, aborting test case\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                409i32);
                                    }
                                }
                                return
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
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 412i32);
            } else {
                fprintf(__stdoutp,
                        b"build/ptest/encrypt_test.c:%d: mu_check(rv == SECSuccess) failed, resuming test case\n\x00"
                            as *const u8 as *const libc::c_char, 412i32);
            }
        }
    }
    PublicKey_clear(pubkey);
}