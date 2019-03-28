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
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    /*
 * Initialize and clear random number generator state.
 * You must call Prio_init() before using the library.
 * To avoid memory leaks, call Prio_clear() afterwards.
 */
    #[no_mangle]
    fn Prio_init() -> SECStatus;
    #[no_mangle]
    fn Prio_clear();
    /* print a message according to the verbosity level */
    /* print an error message */
    /* modify the internal state so a failure gets counted */
    /* modify the internal state so a success gets counted */
    /* !__cplusplus */
    /* __cplusplus */
    /* check that an expression evaluates to true, continue if the check fails */
    /* check that an expression evaluates to true, continue if the check fails */
    /*
 * ensure that an expression evaluates to true, abort the current test
 * case if the check fails
 */
    /* __cplusplus */
    /* we are using the C implementation */
    /*
 * this function implements the test suites execution, you should generate
 * a module with this function using mkmutest, or take a look to that script
 * if you want to implement your own customized version */
    #[no_mangle]
    fn mu_run_suites();
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
pub type unnamed = libc::c_uint;
/* shows the current running check */
pub const MU_CHECK: unnamed = 5;
/* shows test cases progress */
pub const MU_CASE: unnamed = 4;
/* shows test suites progress */
pub const MU_SUITE: unnamed = 3;
/* shows a summary */
pub const MU_SUMMARY: unnamed = 2;
/* shows errors only */
pub const MU_ERROR: unnamed = 1;
/* be completely quiet */
pub const MU_QUIET: unnamed = 0;
/* macro for running a single initialization function */
/* mu_run_init */
/* macro for running a single test case */
/* mu_run_case */
/* macro for running a single termination function */
/* mu_run_term */
/*
 * mutest exported variables for internal use, do not use directly unless you
 *  know what you're doing.
 */
#[no_mangle]
pub static mut mutest_suite_name: *const libc::c_char =
    0 as *const libc::c_char;
#[no_mangle]
pub static mut mutest_failed_suites: libc::c_int = 0;
#[no_mangle]
pub static mut mutest_passed_suites: libc::c_int = 0;
#[no_mangle]
pub static mut mutest_skipped_suites: libc::c_int = 0;
#[no_mangle]
pub static mut mutest_suite_failed: libc::c_int = 0;
/* test cases */
#[no_mangle]
pub static mut mutest_case_name: *const libc::c_char =
    0 as *const libc::c_char;
#[no_mangle]
pub static mut mutest_failed_cases: libc::c_int = 0;
#[no_mangle]
pub static mut mutest_passed_cases: libc::c_int = 0;
#[no_mangle]
pub static mut mutest_case_failed: libc::c_int = 0;
/* checks */
#[no_mangle]
pub static mut mutest_failed_checks: libc::c_int = 0;
#[no_mangle]
pub static mut mutest_passed_checks: libc::c_int = 0;
/* verbosity */
#[no_mangle]
pub static mut mutest_verbose_level: libc::c_int = 1i32;
/*
 * This file is part of mutest, a simple micro unit testing framework for C.
 *
 * mutest was written by Leandro Lucarella <llucax@gmail.com> and is released
 * under the BOLA license, please see the LICENSE file or visit:
 * http://blitiri.com.ar/p/bola/
 *
 * This is the main program, it runs all the test suites and shows the
 * results.  The main work (of running the test suite) is done by the (usually)
 * synthesized mu_run_suites() function, which can be generated using the
 * mkmutest script (or written manually).
 *
 * Please, read the README file for more details.
 */
/* printf(), fprintf() */
/* strncmp() */
/*
 * note that all global variables are public because they need to be accessed
 * from other modules, like the test suites or the module implementing
 * mu_run_suites()
 */
/* globals for managing test suites */
/* globals for managing test cases */
/* globals for managing checks */
/* verbosity level, see mutest.h */
/* exported for use in test suites */
/*
 * only -v is supported right now, both "-v -v" and "-vv" are accepted for
 * increasing the verbosity by 2.
 */
#[no_mangle]
pub unsafe extern "C" fn parse_args(mut argc: libc::c_int,
                                    mut argv: *mut *mut libc::c_char) {
    loop  {
        argv = argv.offset(1isize);
        if (*argv).is_null() { break ; }
        if strncmp(*argv, b"-v\x00" as *const u8 as *const libc::c_char,
                   2i32 as libc::c_ulong) == 0i32 {
            mutest_verbose_level += 1;
            let mut c: *mut libc::c_char = (*argv).offset(1isize);
            loop  {
                c = c.offset(1isize);
                if !(0 != *c) { break ; }
                if *c as libc::c_int != 'v' as i32 { break ; }
                mutest_verbose_level += 1
            }
        }
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    Prio_init();
    parse_args(argc, argv);
    mu_run_suites();
    Prio_clear();
    if mutest_verbose_level >= MU_SUMMARY as libc::c_int {
        if mutest_verbose_level == MU_ERROR as libc::c_int {
            fprintf(__stderrp,
                    b"\nTests done:\n\t%d test suite(s) passed, %d failed, %d skipped.\n\t%d test case(s) passed, %d failed.\n\t%d check(s) passed, %d failed.\n\n\x00"
                        as *const u8 as *const libc::c_char,
                    mutest_passed_suites, mutest_failed_suites,
                    mutest_skipped_suites, mutest_passed_cases,
                    mutest_failed_cases, mutest_passed_checks,
                    mutest_failed_checks);
        } else {
            fprintf(__stdoutp,
                    b"\nTests done:\n\t%d test suite(s) passed, %d failed, %d skipped.\n\t%d test case(s) passed, %d failed.\n\t%d check(s) passed, %d failed.\n\n\x00"
                        as *const u8 as *const libc::c_char,
                    mutest_passed_suites, mutest_failed_suites,
                    mutest_skipped_suites, mutest_passed_cases,
                    mutest_failed_cases, mutest_passed_checks,
                    mutest_failed_checks);
        }
    }
    return if 0 != mutest_failed_suites + mutest_skipped_suites {
               1i32
           } else { 0i32 };
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}