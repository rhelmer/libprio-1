#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    /*
 * Open the Cert, Key, and Security Module databases, read/write.
 * Initialize the Random Number Generator.
 * Does not initialize the cipher policies or enables.
 * Default policy settings disallow all ciphers.
 *
 * This allows using application defined prefixes for the cert and key db's
 * and an alternate name for the secmod database. NOTE: In future releases,
 * the database prefixes my not necessarily map to database names.
 *
 * configdir - base directory where all the cert, key, and module datbases live.
 * certPrefix - prefix added to the beginning of the cert database example: "
 *                      "https-server1-"
 * keyPrefix - prefix added to the beginning of the key database example: "
 *                      "https-server1-"
 * secmodName - name of the security module database (usually "secmod.db").
 * flags - change the open options of NSS_Initialize as follows:
 *      NSS_INIT_READONLY - Open the databases read only.
 *      NSS_INIT_NOCERTDB - Don't open the cert DB and key DB's, just
 *                      initialize the volatile certdb.
 *      NSS_INIT_NOMODDB  - Don't open the security module DB, just
 *                      initialize the  PKCS #11 module.
 *      NSS_INIT_FORCEOPEN - Continue to force initializations even if the
 *                      databases cannot be opened.
 *      NSS_INIT_NOROOTINIT - Don't try to look for the root certs module
 *                      automatically.
 *      NSS_INIT_OPTIMIZESPACE - Use smaller tables and caches.
 *      NSS_INIT_PK11THREADSAFE - only load PKCS#11 modules that are
 *                      thread-safe, ie. that support locking - either OS
 *                      locking or NSS-provided locks . If a PKCS#11
 *                      module isn't thread-safe, don't serialize its
 *                      calls; just don't load it instead. This is necessary
 *                      if another piece of code is using the same PKCS#11
 *                      modules that NSS is accessing without going through
 *                      NSS, for example the Java SunPKCS11 provider.
 *      NSS_INIT_PK11RELOAD - ignore the CKR_CRYPTOKI_ALREADY_INITIALIZED
 *                      error when loading PKCS#11 modules. This is necessary
 *                      if another piece of code is using the same PKCS#11
 *                      modules that NSS is accessing without going through
 *                      NSS, for example Java SunPKCS11 provider.
 *      NSS_INIT_NOPK11FINALIZE - never call C_Finalize on any
 *                      PKCS#11 module. This may be necessary in order to
 *                      ensure continuous operation and proper shutdown
 *                      sequence if another piece of code is using the same
 *                      PKCS#11 modules that NSS is accessing without going
 *                      through NSS, for example Java SunPKCS11 provider.
 *                      The following limitation applies when this is set :
 *                      SECMOD_WaitForAnyTokenEvent will not use
 *                      C_WaitForSlotEvent, in order to prevent the need for
 *                      C_Finalize. This call will be emulated instead.
 *      NSS_INIT_RESERVED - Currently has no effect, but may be used in the
 *                      future to trigger better cooperation between PKCS#11
 *                      modules used by both NSS and the Java SunPKCS11
 *                      provider. This should occur after a new flag is defined
 *                      for C_Initialize by the PKCS#11 working group.
 *      NSS_INIT_COOPERATE - Sets 4 recommended options for applications that
 *                      use both NSS and the Java SunPKCS11 provider.
 *
 * Also NOTE: This is not the recommended method for initializing NSS.
 * The preferred method is NSS_init().
 */
    pub type NSSInitContextStr;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn PK11_GenerateRandom(data: *mut libc::c_uchar, len: libc::c_int)
     -> SECStatus;
    /*
 * Returns whether NSS has already been initialized or not.
 */
    #[no_mangle]
    fn NSS_IsInitialized() -> PRBool;
    #[no_mangle]
    fn NSS_InitContext(configdir: *const libc::c_char,
                       certPrefix: *const libc::c_char,
                       keyPrefix: *const libc::c_char,
                       secmodName: *const libc::c_char,
                       initParams: *mut NSSInitParameters, flags: PRUint32)
     -> *mut NSSInitContext;
    #[no_mangle]
    fn NSS_ShutdownContext(_: *mut NSSInitContext) -> SECStatus;
    /*
 * Perform a graceful shutdown of NSPR.  PR_Cleanup() may be called by
 * the primordial thread near the end of the main() function.
 *
 * PR_Cleanup() attempts to synchronize the natural termination of
 * process.  It does that by blocking the caller, if and only if it is
 * the primordial thread, until the number of user threads has dropped
 * to zero.  When the primordial thread returns from main(), the process
 * will immediately and silently exit.  That is, it will (if necessary)
 * forcibly terminate any existing threads and exit without significant
 * blocking and there will be no error messages or core files.
 *
 * PR_Cleanup() returns PR_SUCCESS if NSPR is successfully shutdown,
 * or PR_FAILURE if the calling thread of this function is not the
 * primordial thread.
 */
    #[no_mangle]
    fn PR_Cleanup() -> PRStatus;
    #[no_mangle]
    fn mp_sub_d(a: *const mp_int, d: mp_digit, b: *mut mp_int) -> mp_err;
    /* MP_MODARITH */
    /* Comparisons             */
    #[no_mangle]
    fn mp_cmp_z(a: *const mp_int) -> libc::c_int;
    #[no_mangle]
    fn mp_cmp(a: *const mp_int, b: *const mp_int) -> libc::c_int;
    /* Octet string conversion functions */
    #[no_mangle]
    fn mp_read_unsigned_octets(mp: *mut mp_int, str: *const libc::c_uchar,
                               len: mp_size) -> mp_err;
    #[no_mangle]
    fn mp_unsigned_octet_size(mp: *const mp_int) -> libc::c_uint;
    #[no_mangle]
    fn mp_to_fixlen_octets(mp: *const mp_int, str: *mut libc::c_uchar,
                           len: mp_size) -> mp_err;
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
/*
** Status code used by some routines that have a single point of failure or
** special status return.
*/
pub type PRStatus = libc::c_int;
pub const PR_SUCCESS: PRStatus = 0;
pub const PR_FAILURE: PRStatus = -1;
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
/*
 * parameters used to initialize softoken. Mostly strings used to
 * internationalize softoken. Memory for the strings are owned by the caller,
 * who is free to free them once NSS_ContextInit returns. If the string
 * parameter is NULL (as opposed to empty, zero length), then the softoken
 * default is used. These are equivalent to the parameters for
 * PK11_ConfigurePKCS11().
 *
 * field names match their equivalent parameter names for softoken strings
 * documented at https://developer.mozilla.org/en/PKCS11_Module_Specs.
 *
 * minPWLen
 *     Minimum password length in bytes.
 * manufacturerID
 *     Override the default manufactureID value for the module returned in
 *     the CK_INFO, CK_SLOT_INFO, and CK_TOKEN_INFO structures with an
 *     internationalize string (UTF8). This value will be truncated at 32
 *     bytes (not including the trailing NULL, partial UTF8 characters will be
 *     dropped).
 * libraryDescription
 *     Override the default libraryDescription value for the module returned in
 *     the CK_INFO structure with an internationalize string (UTF8). This value
 *     will be truncated at 32 bytes(not including the trailing NULL, partial
 *     UTF8 characters will be dropped).
 * cryptoTokenDescription
 *     Override the default label value for the internal crypto token returned
 *     in the CK_TOKEN_INFO structure with an internationalize string (UTF8).
 *     This value will be truncated at 32 bytes (not including the trailing
 *     NULL, partial UTF8 characters will be dropped).
 * dbTokenDescription
 *     Override the default label value for the internal DB token returned in
 *     the CK_TOKEN_INFO structure with an internationalize string (UTF8). This
 *     value will be truncated at 32 bytes (not including the trailing NULL,
 *     partial UTF8 characters will be dropped).
 * FIPSTokenDescription
 *     Override the default label value for the internal FIPS token returned in
 *     the CK_TOKEN_INFO structure with an internationalize string (UTF8). This
 *     value will be truncated at 32 bytes (not including the trailing NULL,
 *     partial UTF8 characters will be dropped).
 * cryptoSlotDescription
 *     Override the default slotDescription value for the internal crypto token
 *     returned in the CK_SLOT_INFO structure with an internationalize string
 *     (UTF8). This value will be truncated at 64 bytes (not including the
 *     trailing NULL, partial UTF8 characters will be dropped).
 * dbSlotDescription
 *     Override the default slotDescription value for the internal DB token
 *     returned in the CK_SLOT_INFO structure with an internationalize string
 *     (UTF8). This value will be truncated at 64 bytes (not including the
 *     trailing NULL, partial UTF8 characters will be dropped).
 * FIPSSlotDescription
 *     Override the default slotDecription value for the internal FIPS token
 *     returned in the CK_SLOT_INFO structure with an internationalize string
 *     (UTF8). This value will be truncated at 64 bytes (not including the
 *     trailing NULL, partial UTF8 characters will be dropped).
 *
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct NSSInitParametersStr {
    pub length: libc::c_uint,
    pub passwordRequired: PRBool,
    pub minPWLen: libc::c_int,
    pub manufactureID: *mut libc::c_char,
    pub libraryDescription: *mut libc::c_char,
    pub cryptoTokenDescription: *mut libc::c_char,
    pub dbTokenDescription: *mut libc::c_char,
    pub FIPSTokenDescription: *mut libc::c_char,
    pub cryptoSlotDescription: *mut libc::c_char,
    pub dbSlotDescription: *mut libc::c_char,
    pub FIPSSlotDescription: *mut libc::c_char,
}
/*
 * NSS utility functions
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/* The private macro _NSS_CUSTOMIZED is for NSS internal use only. */
/*
 * NSS's major version, minor version, patch level, build number, and whether
 * this is a beta release.
 *
 * The format of the version string should be
 *     "<major version>.<minor version>[.<patch level>[.<build number>]][ <ECC>][ <Beta>]"
 */
pub type NSSInitParameters = NSSInitParametersStr;
pub type NSSInitContext = NSSInitContextStr;
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
 * Initialize or cleanup the global random number generator
 * state that NSS uses.
 */
#[no_mangle]
pub unsafe extern "C" fn rand_init() -> SECStatus {
    if !prioGlobalContext.is_null() { return SECSuccess }
    prioGlobalContext =
        NSS_InitContext(b"\x00" as *const u8 as *const libc::c_char,
                        b"\x00" as *const u8 as *const libc::c_char,
                        b"\x00" as *const u8 as *const libc::c_char,
                        b"\x00" as *const u8 as *const libc::c_char,
                        0 as *mut NSSInitParameters,
                        (0x1i32 | 0x2i32 | 0x4i32 | 0x8i32 | 0x10i32) as
                            PRUint32);
    return (if !prioGlobalContext.is_null() {
                SECSuccess as libc::c_int
            } else { SECFailure as libc::c_int }) as SECStatus;
}
/*
 * Copyright (c) 2018, Henry Corrigan-Gibbs
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
static mut prioGlobalContext: *mut NSSInitContext =
    0 as *const NSSInitContext as *mut NSSInitContext;
#[no_mangle]
pub unsafe extern "C" fn rand_clear() {
    if !prioGlobalContext.is_null() {
        NSS_ShutdownContext(prioGlobalContext);
        PR_Cleanup();
    }
    prioGlobalContext = 0 as *mut NSSInitContext;
}
/*
 * Generate the specified number of random bytes using the
 * NSS random number generator.
 */
#[no_mangle]
pub unsafe extern "C" fn rand_bytes(mut out: *mut libc::c_uchar,
                                    mut n_bytes: size_t) -> SECStatus {
    return rand_bytes_internal(0 as *mut libc::c_void, out, n_bytes);
}
unsafe extern "C" fn rand_bytes_internal(mut user_data: *mut libc::c_void,
                                         mut out: *mut libc::c_uchar,
                                         mut n_bytes: size_t) -> SECStatus {
    if !user_data.is_null() { return SECFailure }
    if 0 == NSS_IsInitialized() { return SECFailure }
    let mut rv: SECStatus = SECFailure;
    let mut to_go: libc::c_int = n_bytes as libc::c_int;
    let mut cp: *mut libc::c_uchar = out;
    while 0 != to_go {
        let mut to_gen: libc::c_int =
            if 8192i32 < to_go { 8192i32 } else { to_go };
        rv = PK11_GenerateRandom(cp, to_gen);
        if rv as libc::c_int != SECSuccess as libc::c_int {
            return SECFailure
        }
        cp = cp.offset(8192isize);
        to_go -= to_gen
    }
    return rv;
}
/*
 * Generate a random number x such that
 *    0 <= x < max
 * using the NSS random number generator.
 */
#[no_mangle]
pub unsafe extern "C" fn rand_int(mut out: *mut mp_int,
                                  mut max: *const mp_int) -> SECStatus {
    return rand_int_rng(out, max, Some(rand_bytes_internal),
                        0 as *mut libc::c_void);
}
/*
 * Generate a random number x such that
 *    0 <= x < max
 * using the specified randomness generator.
 *
 * The pointer user_data is passed to RandBytesFung `rng` as a first
 * argument.
 */
#[no_mangle]
pub unsafe extern "C" fn rand_int_rng(mut out: *mut mp_int,
                                      mut max: *const mp_int,
                                      mut rng_func: RandBytesFunc,
                                      mut user_data: *mut libc::c_void)
 -> SECStatus {
    let mut nbytes: libc::c_int = 0;
    let mut mask: libc::c_uchar = 0;
    let mut rv: SECStatus = SECSuccess;
    let mut max_bytes: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if mp_cmp_z(max) == 0i32 { return SECFailure }
    // Compute max-1, which tells us the largest
  // value we will ever need to generate.
    if mp_sub_d(max, 1i32 as mp_digit, out) != 0i32 {
        rv = SECFailure
    } else {
        nbytes = mp_unsigned_octet_size(out) as libc::c_int;
        // Figure out how many MSBs we need to get in the
  // most-significant byte.
        max_bytes =
            calloc(nbytes as libc::c_ulong,
                   ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong) as
                *mut libc::c_uchar;
        if max_bytes.is_null() {
            rv = SECFailure
        } else if mp_to_fixlen_octets(out, max_bytes, nbytes as mp_size) !=
                      0i32 {
            rv = SECFailure
        } else {
            mask = msb_mask(*max_bytes.offset(0isize));
            // Buffer to store the pseudo-random bytes
            buf =
                calloc(nbytes as libc::c_ulong,
                       ::std::mem::size_of::<libc::c_uchar>() as
                           libc::c_ulong) as *mut libc::c_uchar;
            if buf.is_null() {
                rv = SECFailure
            } else {
                loop  {
                    // Use  rejection sampling to find a value strictly less than max.
                    rv =
                        rng_func.expect("non-null function pointer")(user_data,
                                                                     buf,
                                                                     nbytes as
                                                                         size_t);
                    if rv as libc::c_int != SECSuccess as libc::c_int {
                        break ;
                    }
                    // Mask off high-order bits that we will never need.
                    rv =
                        rng_func.expect("non-null function pointer")(user_data,
                                                                     &mut *buf.offset(0isize),
                                                                     1i32 as
                                                                         size_t);
                    if rv as libc::c_int != SECSuccess as libc::c_int {
                        break ;
                    }
                    if 0 != mask {
                        let ref mut fresh0 = *buf.offset(0isize);
                        *fresh0 =
                            (*fresh0 as libc::c_int & mask as libc::c_int) as
                                libc::c_uchar
                    }
                    if mp_read_unsigned_octets(out, buf, nbytes as mp_size) !=
                           0i32 {
                        rv = SECFailure;
                        break ;
                    } else if !(mp_cmp(out, max) != -1i32) { break ; }
                }
            }
        }
    }
    if !max_bytes.is_null() { free(max_bytes as *mut libc::c_void); }
    if !buf.is_null() { free(buf as *mut libc::c_void); }
    return rv;
}
/*
 * Return a mask that masks out all of the zero bits
 */
unsafe extern "C" fn msb_mask(mut val: libc::c_uchar) -> libc::c_uchar {
    let mut mask: libc::c_uchar = 0;
    mask = 0i32 as libc::c_uchar;
    while val as libc::c_int & mask as libc::c_int != val as libc::c_int {
        mask = (((mask as libc::c_int) << 1i32) + 1i32) as libc::c_uchar
    }
    return mask;
}