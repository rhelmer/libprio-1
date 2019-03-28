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
    fn mp_submod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                 c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_mulmod(a: *const mp_int, b: *const mp_int, m: *const mp_int,
                 c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_exptmod_d(a: *const mp_int, d: mp_digit, m: *const mp_int,
                    c: *mut mp_int) -> mp_err;
    #[no_mangle]
    fn mp_invmod(a: *const mp_int, m: *const mp_int, c: *mut mp_int)
     -> mp_err;
    /*
 * Initialize an array of `mp_int`s of the given length.
 */
    #[no_mangle]
    fn MPArray_new(len: libc::c_int) -> MPArray;
    #[no_mangle]
    fn MPArray_clear(arr: MPArray);
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
pub unsafe extern "C" fn poly_fft(mut points_out: MPArray,
                                  mut points_in: const_MPArray,
                                  mut cfg: const_PrioConfig, mut invert: bool)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let n_points: libc::c_int = (*points_in).len;
    let mut scaled_roots: MPArray = 0 as MPArray;
    if (*points_out).len != (*points_in).len { return SECFailure }
    if n_points > (*cfg).n_roots { return SECFailure }
    if (*cfg).n_roots % n_points != 0i32 { return SECFailure }
    scaled_roots = MPArray_new(n_points);
    if scaled_roots.is_null() {
        rv = SECFailure
    } else {
        rv = poly_fft_get_roots(scaled_roots, n_points, cfg, invert);
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            rv =
                fft_interpolate_raw((*points_out).data, (*points_in).data,
                                    n_points, scaled_roots as const_MPArray,
                                    &(*cfg).modulus, invert);
            rv as libc::c_int != SECSuccess as libc::c_int;
        }
    }
    MPArray_clear(scaled_roots);
    return SECSuccess;
}
unsafe extern "C" fn fft_interpolate_raw(mut out: *mut mp_int,
                                         mut ys: *const mp_int,
                                         mut nPoints: libc::c_int,
                                         mut roots: const_MPArray,
                                         mut mod_0: *const mp_int,
                                         mut invert: bool) -> SECStatus {
    let mut n_inverse: mp_int =
        mp_int{sign: 0, alloc: 0, used: 0, dp: 0 as *mut mp_digit,};
    let mut rv: SECStatus = SECSuccess;
    let mut tmp: MPArray = 0 as MPArray;
    let mut ySub: MPArray = 0 as MPArray;
    let mut rootsSub: MPArray = 0 as MPArray;
    tmp = MPArray_new(nPoints);
    if tmp.is_null() {
        rv = SECFailure
    } else {
        ySub = MPArray_new(nPoints);
        if ySub.is_null() {
            rv = SECFailure
        } else {
            rootsSub = MPArray_new(nPoints);
            if rootsSub.is_null() {
                rv = SECFailure
            } else {
                n_inverse =
                    mp_int{sign: 0,
                           alloc: 0,
                           used: 0,
                           dp: 0 as *mut mp_digit,};
                n_inverse.dp = 0 as *mut mp_digit;
                if fft_recurse(out, mod_0, nPoints, (*roots).data, ys,
                               (*tmp).data, (*ySub).data, (*rootsSub).data) as
                       libc::c_int != 0i32 {
                    rv = SECFailure
                } else if invert {
                    if mp_init(&mut n_inverse) != 0i32 {
                        rv = SECFailure
                    } else {
                        mp_set(&mut n_inverse, nPoints as mp_digit);
                        if mp_invmod(&mut n_inverse, mod_0, &mut n_inverse) !=
                               0i32 {
                            rv = SECFailure
                        } else {
                            let mut i: libc::c_int = 0i32;
                            while i < nPoints {
                                if mp_mulmod(&mut *out.offset(i as isize),
                                             &mut n_inverse, mod_0,
                                             &mut *out.offset(i as isize)) !=
                                       0i32 {
                                    rv = SECFailure;
                                    break ;
                                } else { i += 1 }
                            }
                        }
                    }
                }
            }
        }
    }
    MPArray_clear(tmp);
    MPArray_clear(ySub);
    MPArray_clear(rootsSub);
    mp_clear(&mut n_inverse);
    return rv;
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
/*
 * A nice exposition of the recursive FFT/DFT algorithm we implement
 * is in the book:
 *
 *   "Modern Computer Algebra"
 *    by Von zur Gathen and Gerhard.
 *    Cambridge University Press, 2013.
 *
 * They present this algorithm as Algorithm 8.14.
 */
unsafe extern "C" fn fft_recurse(mut out: *mut mp_int,
                                 mut mod_0: *const mp_int, mut n: libc::c_int,
                                 mut roots: *const mp_int,
                                 mut ys: *const mp_int, mut tmp: *mut mp_int,
                                 mut ySub: *mut mp_int,
                                 mut rootsSub: *mut mp_int) -> SECStatus {
    if n == 1i32 {
        if mp_copy(&*ys.offset(0isize), &mut *out.offset(0isize)) != 0i32 {
            return SECFailure
        }
        return SECSuccess
    }
    let mut i: libc::c_int = 0i32;
    while i < n / 2i32 {
        if mp_addmod(&*ys.offset(i as isize),
                     &*ys.offset((i + n / 2i32) as isize), mod_0,
                     &mut *ySub.offset(i as isize)) != 0i32 {
            return SECFailure
        }
        if mp_copy(&*roots.offset((2i32 * i) as isize),
                   &mut *rootsSub.offset(i as isize)) != 0i32 {
            return SECFailure
        }
        i += 1
    }
    if fft_recurse(tmp, mod_0, n / 2i32, rootsSub, ySub,
                   &mut *tmp.offset((n / 2i32) as isize),
                   &mut *ySub.offset((n / 2i32) as isize),
                   &mut *rootsSub.offset((n / 2i32) as isize)) as libc::c_int
           != 0i32 {
        return SECFailure
    }
    let mut i_0: libc::c_int = 0i32;
    while i_0 < n / 2i32 {
        if mp_copy(&mut *tmp.offset(i_0 as isize),
                   &mut *out.offset((2i32 * i_0) as isize)) != 0i32 {
            return SECFailure
        }
        i_0 += 1
    }
    let mut i_1: libc::c_int = 0i32;
    while i_1 < n / 2i32 {
        if mp_submod(&*ys.offset(i_1 as isize),
                     &*ys.offset((i_1 + n / 2i32) as isize), mod_0,
                     &mut *ySub.offset(i_1 as isize)) != 0i32 {
            return SECFailure
        }
        if mp_mulmod(&mut *ySub.offset(i_1 as isize),
                     &*roots.offset(i_1 as isize), mod_0,
                     &mut *ySub.offset(i_1 as isize)) != 0i32 {
            return SECFailure
        }
        i_1 += 1
    }
    if fft_recurse(tmp, mod_0, n / 2i32, rootsSub, ySub,
                   &mut *tmp.offset((n / 2i32) as isize),
                   &mut *ySub.offset((n / 2i32) as isize),
                   &mut *rootsSub.offset((n / 2i32) as isize)) as libc::c_int
           != 0i32 {
        return SECFailure
    }
    let mut i_2: libc::c_int = 0i32;
    while i_2 < n / 2i32 {
        if mp_copy(&mut *tmp.offset(i_2 as isize),
                   &mut *out.offset((2i32 * i_2 + 1i32) as isize)) != 0i32 {
            return SECFailure
        }
        i_2 += 1
    }
    return SECSuccess;
}
/*
 * Get an array
 *    (r^0, r^1, r^2, ... )
 * where r is an n-th root of unity, for n a power of two
 * less than cfg->n_roots.
 *
 * Do NOT mp_clear() the mp_ints stored in roots_out.
 * These are owned by the PrioConfig object.
 */
#[no_mangle]
pub unsafe extern "C" fn poly_fft_get_roots(mut roots_out: MPArray,
                                            mut n_points: libc::c_int,
                                            mut cfg: const_PrioConfig,
                                            mut invert: bool) -> SECStatus {
    if n_points < 1i32 { return SECFailure }
    if n_points != (*roots_out).len { return SECFailure }
    if n_points > (*cfg).n_roots { return SECFailure }
    mp_set(&mut *(*roots_out).data.offset(0isize), 1i32 as mp_digit);
    if n_points == 1i32 { return SECSuccess }
    let step_size: libc::c_int = (*cfg).n_roots / n_points;
    let mut gen: *mut mp_int =
        &mut *(*roots_out).data.offset(1isize) as *mut mp_int;
    if mp_copy(&(*cfg).generator, gen) != 0i32 { return SECFailure }
    if invert {
        if mp_invmod(gen, &(*cfg).modulus, gen) != 0i32 { return SECFailure }
    }
    if mp_exptmod_d(gen, step_size as mp_digit, &(*cfg).modulus, gen) != 0i32
       {
        return SECFailure
    }
    let mut i: libc::c_int = 2i32;
    while i < n_points {
        if mp_mulmod(gen, &mut *(*roots_out).data.offset((i - 1i32) as isize),
                     &(*cfg).modulus,
                     &mut *(*roots_out).data.offset(i as isize)) != 0i32 {
            return SECFailure
        }
        i += 1
    }
    return SECSuccess;
}
/*
 * Evaluate the polynomial specified by the coefficients
 * at the point `eval_at` and return the result as `value`.
 */
#[no_mangle]
pub unsafe extern "C" fn poly_eval(mut value: *mut mp_int,
                                   mut coeffs: const_MPArray,
                                   mut eval_at: *const mp_int,
                                   mut cfg: const_PrioConfig) -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let n: libc::c_int = (*coeffs).len;
    if mp_copy(&mut *(*coeffs).data.offset((n - 1i32) as isize), value) !=
           0i32 {
        return SECFailure
    }
    let mut i: libc::c_int = n - 2i32;
    while i >= 0i32 {
        if mp_mulmod(value, eval_at, &(*cfg).modulus, value) != 0i32 {
            return SECFailure
        }
        if mp_addmod(value, &mut *(*coeffs).data.offset(i as isize),
                     &(*cfg).modulus, value) != 0i32 {
            return SECFailure
        }
        i -= 1
    }
    return rv;
}
/*
 * Interpolate the polynomial through the points
 *    (x_1, y_1), ..., (x_N, y_N),
 * where x_i is an N-th root of unity and the y_i values are
 * specified by `poly_points`. Evaluate the resulting polynomial
 * at the point `eval_at`. Return the result as `value`.
 */
#[no_mangle]
pub unsafe extern "C" fn poly_interp_evaluate(mut value: *mut mp_int,
                                              mut poly_points: const_MPArray,
                                              mut eval_at: *const mp_int,
                                              mut cfg: const_PrioConfig)
 -> SECStatus {
    let mut rv: SECStatus = SECSuccess;
    let mut coeffs: MPArray = 0 as MPArray;
    let N: libc::c_int = (*poly_points).len;
    coeffs = MPArray_new(N);
    if coeffs.is_null() {
        rv = SECFailure
    } else {
        // Interpolate polynomial through roots of unity
        rv = poly_fft(coeffs, poly_points, cfg, 0 != 1i32);
        if !(rv as libc::c_int != SECSuccess as libc::c_int) {
            rv = poly_eval(value, coeffs as const_MPArray, eval_at, cfg);
            rv as libc::c_int != SECSuccess as libc::c_int;
        }
    }
    MPArray_clear(coeffs);
    return rv;
}