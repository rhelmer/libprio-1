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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    /* Memory management       */
    #[no_mangle]
    fn mp_init(mp: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_copy(from: *const mp_int, to: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_clear(mp: *mut mp_int);
    #[no_mangle]
    fn mp_set(mp: *mut mp_int, d: mp_digit);
    #[no_mangle]
    fn mp_addmod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                 c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    /*
 * Use secret sharing to split the int src into two shares.
 * The mp_ints must be initialized.
 */
    #[no_mangle]
    fn share_int(cfg: const_PrioConfig, src: *const mp_int,
                 shareA: *mut mp_int, shareB: *mut mp_int) -> SECStatus;
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
 * Initialize an array of `mp_int`s of the given length.
 */
#[no_mangle]
pub unsafe extern "C" fn MPArray_new(mut len: libc::c_int) -> MPArray {
    let mut rv: SECStatus = SECSuccess;
    let mut arr: MPArray =
        malloc(::std::mem::size_of::<mparray>() as libc::c_ulong) as MPArray;
    if arr.is_null() { return 0 as MPArray }
    (*arr).data = 0 as *mut mp_int;
    (*arr).len = len;
    (*arr).data =
        calloc(len as libc::c_ulong,
               ::std::mem::size_of::<mp_int>() as libc::c_ulong) as
            *mut mp_int;
    if (*arr).data.is_null() {
        rv = SECFailure
    } else {
        let mut i: libc::c_int = 0i32;
        while i < len {
            let ref mut fresh0 = (*(*arr).data.offset(i as isize)).dp;
            *fresh0 = 0 as *mut mp_digit;
            i += 1
        }
        let mut i_0: libc::c_int = 0i32;
        while i_0 < len {
            if mp_init(&mut *(*arr).data.offset(i_0 as isize)) != 0i32 {
                rv = SECFailure;
                break ;
            } else { i_0 += 1 }
        }
    }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        MPArray_clear(arr);
        return 0 as MPArray
    }
    return arr;
}
#[no_mangle]
pub unsafe extern "C" fn MPArray_clear(mut arr: MPArray) {
    if arr.is_null() { return }
    if !(*arr).data.is_null() {
        let mut i: libc::c_int = 0i32;
        while i < (*arr).len {
            mp_clear(&mut *(*arr).data.offset(i as isize));
            i += 1
        }
        free((*arr).data as *mut libc::c_void);
    }
    free(arr as *mut libc::c_void);
}
/*
 * Copies secret sharing of data from src into arrays
 * arrA and arrB. The lengths of the three input arrays
 * must be identical.
 */
#[no_mangle]
pub unsafe extern "C" fn MPArray_set_share(mut arrA: MPArray,
                                           mut arrB: MPArray,
                                           mut src: const_MPArray,
                                           mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    if (*arrA).len != (*src).len || (*arrB).len != (*src).len {
        return SECFailure
    }
    let len: libc::c_int = (*src).len;
    let mut i: libc::c_int = 0i32;
    while i < len {
        rv =
            share_int(cfg, &mut *(*src).data.offset(i as isize),
                      &mut *(*arrA).data.offset(i as isize),
                      &mut *(*arrB).data.offset(i as isize));
        if rv as libc::c_int != SECSuccess as libc::c_int { return rv }
        i += 1
    }
    return rv;
}
/*
 * Initializes array with 0/1 values specified in boolean array `data_in`
 */
#[no_mangle]
pub unsafe extern "C" fn MPArray_new_bool(mut len: libc::c_int,
                                          mut data_in: *const bool)
 -> MPArray {
    let mut arr: MPArray = MPArray_new(len);
    if arr.is_null() { return 0 as MPArray }
    let mut i: libc::c_int = 0i32;
    while i < len {
        mp_set(&mut *(*arr).data.offset(i as isize),
               *data_in.offset(i as isize) as mp_digit);
        i += 1
    }
    return arr;
}
/*
 * Expands or shrinks the MPArray to the desired size. If shrinking,
 * will clear the values on the end of array.
 */
#[no_mangle]
pub unsafe extern "C" fn MPArray_resize(mut arr: MPArray,
                                        mut newlen: libc::c_int)
 -> SECStatus {
    let mut current_block: u64;
    let mut rv: SECStatus = SECSuccess;
    let oldlen: libc::c_int = (*arr).len;
    if oldlen == newlen { return rv }
    // TODO: Use realloc for this?
    let mut newdata: *mut mp_int =
        calloc(newlen as libc::c_ulong,
               ::std::mem::size_of::<mp_int>() as libc::c_ulong) as
            *mut mp_int;
    if newdata.is_null() { return SECFailure }
    let mut i: libc::c_int = 0i32;
    while i < newlen {
        let ref mut fresh1 = (*newdata.offset(i as isize)).dp;
        *fresh1 = 0 as *mut mp_digit;
        i += 1
    }
    // Initialize new array
    let mut i_0: libc::c_int = 0i32;
    loop  {
        if !(i_0 < newlen) { current_block = 12039483399334584727; break ; }
        if mp_init(&mut *newdata.offset(i_0 as isize)) != 0i32 {
            rv = SECFailure;
            current_block = 10433015008054693148;
            break ;
        } else { i_0 += 1 }
    }
    match current_block {
        12039483399334584727 => {
            // Copy old data into new array
            let mut i_1: libc::c_int = 0i32;
            loop  {
                if !(i_1 < newlen && i_1 < oldlen) {
                    current_block = 10043043949733653460;
                    break ;
                }
                if mp_copy(&mut *(*arr).data.offset(i_1 as isize),
                           &mut *newdata.offset(i_1 as isize)) != 0i32 {
                    rv = SECFailure;
                    current_block = 10433015008054693148;
                    break ;
                } else { i_1 += 1 }
            }
            match current_block {
                10433015008054693148 => { }
                _ => {
                    let mut i_2: libc::c_int = 0i32;
                    while i_2 < oldlen {
                        mp_clear(&mut *(*arr).data.offset(i_2 as isize));
                        i_2 += 1
                    }
                    free((*arr).data as *mut libc::c_void);
                    (*arr).data = newdata;
                    (*arr).len = newlen
                }
            }
        }
        _ => { }
    }
    if rv as libc::c_int != SECSuccess as libc::c_int {
        let mut i_3: libc::c_int = 0i32;
        while i_3 < newlen {
            mp_clear(&mut *newdata.offset(i_3 as isize));
            i_3 += 1
        }
        free(newdata as *mut libc::c_void);
    }
    return rv;
}
/*
 * Initializes dst and creates a duplicate of the array in src.
 */
#[no_mangle]
pub unsafe extern "C" fn MPArray_dup(mut src: const_MPArray) -> MPArray {
    let mut dst: MPArray = MPArray_new((*src).len);
    if dst.is_null() { return 0 as MPArray }
    let mut rv: SECStatus = MPArray_copy(dst, src);
    if rv as libc::c_int == SECSuccess as libc::c_int {
        return dst
    } else { MPArray_clear(dst); return 0 as MPArray };
}
/*
 * Copies array from src to dst. Arrays must have the same length.
 */
#[no_mangle]
pub unsafe extern "C" fn MPArray_copy(mut dst: MPArray,
                                      mut src: const_MPArray) -> SECStatus {
    if (*dst).len != (*src).len { return SECFailure }
    let mut i: libc::c_int = 0i32;
    while i < (*src).len {
        if mp_copy(&mut *(*src).data.offset(i as isize),
                   &mut *(*dst).data.offset(i as isize)) != 0i32 {
            return SECFailure
        }
        i += 1
    }
    return SECSuccess;
}
/* For each index i into the array, set:
 *    dst[i] = dst[i] + to_add[i]   (modulo mod)
 */
#[no_mangle]
pub unsafe extern "C" fn MPArray_addmod(mut dst: MPArray,
                                        mut to_add: const_MPArray,
                                        mut mod_0: *const mp_int)
 -> SECStatus {
    if (*dst).len != (*to_add).len { return SECFailure }
    let mut i: libc::c_int = 0i32;
    while i < (*dst).len {
        if mp_addmod(&mut *(*dst).data.offset(i as isize),
                     &mut *(*to_add).data.offset(i as isize), mod_0,
                     &mut *(*dst).data.offset(i as isize)) != 0i32 {
            return SECFailure
        }
        i += 1
    }
    return SECSuccess;
}
/*
 * Return true iff the two arrays are equal in length
 * and contents. This comparison is NOT constant time.
 */
#[no_mangle]
pub unsafe extern "C" fn MPArray_areEqual(mut arr1: const_MPArray,
                                          mut arr2: const_MPArray) -> bool {
    if (*arr1).len != (*arr2).len { return 0 != 0i32 }
    let mut i: libc::c_int = 0i32;
    while i < (*arr1).len {
        if 0 !=
               mp_cmp(&mut *(*arr1).data.offset(i as isize),
                      &mut *(*arr2).data.offset(i as isize)) {
            return 0 != 0i32
        }
        i += 1
    }
    return 0 != 1i32;
}