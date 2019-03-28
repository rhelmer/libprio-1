#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc)]
extern crate libc;
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
static mut TwoTo16: libc::c_double = 65536.0f64;
static mut TwoToMinus16: libc::c_double = 1.0f64 / 65536.0f64;
static mut Zero: libc::c_double = 0.0f64;
static mut TwoTo32: libc::c_double = 65536.0f64 * 65536.0f64;
static mut TwoToMinus32: libc::c_double = 1.0f64 / (65536.0f64 * 65536.0f64);
unsafe extern "C" fn upper32(mut x: libc::c_double) -> libc::c_double {
    return (x * TwoToMinus32) as libc::c_ulonglong as libc::c_double;
}
unsafe extern "C" fn lower32(mut x: libc::c_double, mut y: libc::c_double)
 -> libc::c_double {
    return x -
               TwoTo32 *
                   (x * TwoToMinus32) as libc::c_ulonglong as libc::c_double;
}
unsafe extern "C" fn mod_0(mut x: libc::c_double,
                           mut oneoverm: libc::c_double,
                           mut m: libc::c_double) -> libc::c_double {
    return x - m * (x * oneoverm) as libc::c_ulonglong as libc::c_double;
}
unsafe extern "C" fn cleanup(mut dt: *mut libc::c_double,
                             mut from: libc::c_int, mut tlen: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    tmp1 = Zero;
    tmp = tmp1;
    i = 2i32 * from;
    while i < 2i32 * tlen {
        x = *dt.offset(i as isize);
        x1 = *dt.offset((i + 1i32) as isize);
        *dt.offset(i as isize) = lower32(x, Zero) + tmp;
        *dt.offset((i + 1i32) as isize) = lower32(x1, Zero) + tmp1;
        tmp = upper32(x);
        tmp1 = upper32(x1);
        i += 2i32
    };
}
/* * end new code **/
#[no_mangle]
pub unsafe extern "C" fn conv_d16_to_i32(mut i32: *mut libc::c_uint,
                                         mut d16: *mut libc::c_double,
                                         mut tmp: *mut libc::c_longlong,
                                         mut ilen: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_longlong = 0;
    let mut t1: libc::c_longlong = 0;
    let mut a: libc::c_longlong = 0;
    let mut b: libc::c_longlong = 0;
    let mut c: libc::c_longlong = 0;
    let mut d: libc::c_longlong = 0;
    t1 = 0i32 as libc::c_longlong;
    a = *d16.offset(0isize) as libc::c_longlong;
    b = *d16.offset(1isize) as libc::c_longlong;
    i = 0i32;
    while i < ilen - 1i32 {
        c = *d16.offset((2i32 * i + 2i32) as isize) as libc::c_longlong;
        t1 += a as libc::c_uint as libc::c_longlong;
        t = a >> 32i32;
        d = *d16.offset((2i32 * i + 3i32) as isize) as libc::c_longlong;
        t1 += (b & 0xffffi32 as libc::c_longlong) << 16i32;
        t += (b >> 16i32) + (t1 >> 32i32);
        *i32.offset(i as isize) = t1 as libc::c_uint;
        t1 = t;
        a = c;
        b = d;
        i += 1
    }
    t1 += a as libc::c_uint as libc::c_longlong;
    t = a >> 32i32;
    t1 += (b & 0xffffi32 as libc::c_longlong) << 16i32;
    *i32.offset(i as isize) = t1 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn conv_i32_to_d32(mut d32: *mut libc::c_double,
                                         mut i32: *mut libc::c_uint,
                                         mut len: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < len {
        *d32.offset(i as isize) = *i32.offset(i as isize) as libc::c_double;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn conv_i32_to_d16(mut d16: *mut libc::c_double,
                                         mut i32: *mut libc::c_uint,
                                         mut len: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut a: libc::c_uint = 0;
    i = 0i32;
    while i < len {
        a = *i32.offset(i as isize);
        *d16.offset((2i32 * i) as isize) =
            (a & 0xffffi32 as libc::c_uint) as libc::c_double;
        *d16.offset((2i32 * i + 1i32) as isize) =
            (a >> 16i32) as libc::c_double;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn conv_i32_to_d32_and_d16(mut d32: *mut libc::c_double,
                                                 mut d16: *mut libc::c_double,
                                                 mut i32: *mut libc::c_uint,
                                                 mut len: libc::c_int) {
    let mut i: libc::c_int = 0i32;
    let mut a: libc::c_uint = 0;
    while i < len {
        a = *i32.offset(i as isize);
        *d32.offset(i as isize) = *i32.offset(i as isize) as libc::c_double;
        *d16.offset((2i32 * i) as isize) =
            (a & 0xffffi32 as libc::c_uint) as libc::c_double;
        *d16.offset((2i32 * i + 1i32) as isize) =
            (a >> 16i32) as libc::c_double;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn adjust_montf_result(mut i32: *mut libc::c_uint,
                                             mut nint: *mut libc::c_uint,
                                             mut len: libc::c_int) {
    let mut acc: libc::c_longlong = 0;
    let mut i: libc::c_int = 0;
    if *i32.offset(len as isize) > 0i32 as libc::c_uint {
        i = -1i32
    } else {
        i = len - 1i32;
        while i >= 0i32 {
            if *i32.offset(i as isize) != *nint.offset(i as isize) { break ; }
            i -= 1
        }
    }
    if i < 0i32 || *i32.offset(i as isize) > *nint.offset(i as isize) {
        acc = 0i32 as libc::c_longlong;
        i = 0i32;
        while i < len {
            acc =
                (acc as
                     libc::c_ulonglong).wrapping_add(*i32.offset(i as isize)
                                                         as
                                                         libc::c_ulonglong).wrapping_sub(*nint.offset(i
                                                                                                          as
                                                                                                          isize)
                                                                                             as
                                                                                             libc::c_ulonglong)
                    as libc::c_longlong;
            *i32.offset(i as isize) = acc as libc::c_uint;
            acc = acc >> 32i32;
            i += 1
        }
    };
}
/*
** the lengths of the input arrays should be at least the following:
** result[nlen+1], dm1[nlen], dm2[2*nlen+1], dt[4*nlen+2], dn[nlen], nint[nlen]
** all of them should be different from one another
**
*/
#[no_mangle]
pub unsafe extern "C" fn mont_mulf_noconv(mut result: *mut libc::c_uint,
                                          mut dm1: *mut libc::c_double,
                                          mut dm2: *mut libc::c_double,
                                          mut dt: *mut libc::c_double,
                                          mut dn: *mut libc::c_double,
                                          mut nint: *mut libc::c_uint,
                                          mut nlen: libc::c_int,
                                          mut dn0: libc::c_double) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut digit: libc::c_double = 0.;
    let mut m2j: libc::c_double = 0.;
    let mut nextm2j: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut dptmp: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdm1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdm2: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdn: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdtj: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdn_0: libc::c_double = 0.;
    let mut pdm1_0: libc::c_double = 0.;
    pdm1 = &mut *dm1.offset(0isize) as *mut libc::c_double;
    pdm2 = &mut *dm2.offset(0isize) as *mut libc::c_double;
    pdn = &mut *dn.offset(0isize) as *mut libc::c_double;
    *pdm2.offset((2i32 * nlen) as isize) = Zero;
    if nlen != 16i32 {
        i = 0i32;
        while i < 4i32 * nlen + 2i32 { *dt.offset(i as isize) = Zero; i += 1 }
        let ref mut fresh0 = *dt.offset(0isize);
        *fresh0 = *pdm1.offset(0isize) * *pdm2.offset(0isize);
        a = *fresh0;
        digit = mod_0(lower32(a, Zero) * dn0, TwoToMinus16, TwoTo16);
        pdtj = &mut *dt.offset(0isize) as *mut libc::c_double;
        jj = 0i32;
        j = jj;
        while j < 2i32 * nlen {
            m2j = *pdm2.offset(j as isize);
            a = *pdtj.offset(0isize) + *pdn.offset(0isize) * digit;
            b =
                *pdtj.offset(1isize) +
                    *pdm1.offset(0isize) * *pdm2.offset((j + 1i32) as isize) +
                    a * TwoToMinus16;
            *pdtj.offset(1isize) = b;
            i = 1i32;
            while i < nlen {
                *pdtj.offset((2i32 * i) as isize) +=
                    *pdm1.offset(i as isize) * m2j +
                        *pdn.offset(i as isize) * digit;
                i += 1
            }
            if jj == 30i32 {
                cleanup(dt, j / 2i32 + 1i32, 2i32 * nlen + 1i32);
                jj = 0i32
            }
            digit = mod_0(lower32(b, Zero) * dn0, TwoToMinus16, TwoTo16);
            j += 1;
            jj += 1;
            pdtj = pdtj.offset(1isize)
        }
    } else {
        let ref mut fresh1 = *dt.offset(0isize);
        *fresh1 = *pdm1.offset(0isize) * *pdm2.offset(0isize);
        a = *fresh1;
        let ref mut fresh65 = *dt.offset(64isize);
        let ref mut fresh64 = *dt.offset(63isize);
        let ref mut fresh63 = *dt.offset(62isize);
        let ref mut fresh62 = *dt.offset(61isize);
        let ref mut fresh61 = *dt.offset(60isize);
        let ref mut fresh60 = *dt.offset(59isize);
        let ref mut fresh59 = *dt.offset(58isize);
        let ref mut fresh58 = *dt.offset(57isize);
        let ref mut fresh57 = *dt.offset(56isize);
        let ref mut fresh56 = *dt.offset(55isize);
        let ref mut fresh55 = *dt.offset(54isize);
        let ref mut fresh54 = *dt.offset(53isize);
        let ref mut fresh53 = *dt.offset(52isize);
        let ref mut fresh52 = *dt.offset(51isize);
        let ref mut fresh51 = *dt.offset(50isize);
        let ref mut fresh50 = *dt.offset(49isize);
        let ref mut fresh49 = *dt.offset(48isize);
        let ref mut fresh48 = *dt.offset(47isize);
        let ref mut fresh47 = *dt.offset(46isize);
        let ref mut fresh46 = *dt.offset(45isize);
        let ref mut fresh45 = *dt.offset(44isize);
        let ref mut fresh44 = *dt.offset(43isize);
        let ref mut fresh43 = *dt.offset(42isize);
        let ref mut fresh42 = *dt.offset(41isize);
        let ref mut fresh41 = *dt.offset(40isize);
        let ref mut fresh40 = *dt.offset(39isize);
        let ref mut fresh39 = *dt.offset(38isize);
        let ref mut fresh38 = *dt.offset(37isize);
        let ref mut fresh37 = *dt.offset(36isize);
        let ref mut fresh36 = *dt.offset(35isize);
        let ref mut fresh35 = *dt.offset(34isize);
        let ref mut fresh34 = *dt.offset(33isize);
        let ref mut fresh33 = *dt.offset(32isize);
        let ref mut fresh32 = *dt.offset(31isize);
        let ref mut fresh31 = *dt.offset(30isize);
        let ref mut fresh30 = *dt.offset(29isize);
        let ref mut fresh29 = *dt.offset(28isize);
        let ref mut fresh28 = *dt.offset(27isize);
        let ref mut fresh27 = *dt.offset(26isize);
        let ref mut fresh26 = *dt.offset(25isize);
        let ref mut fresh25 = *dt.offset(24isize);
        let ref mut fresh24 = *dt.offset(23isize);
        let ref mut fresh23 = *dt.offset(22isize);
        let ref mut fresh22 = *dt.offset(21isize);
        let ref mut fresh21 = *dt.offset(20isize);
        let ref mut fresh20 = *dt.offset(19isize);
        let ref mut fresh19 = *dt.offset(18isize);
        let ref mut fresh18 = *dt.offset(17isize);
        let ref mut fresh17 = *dt.offset(16isize);
        let ref mut fresh16 = *dt.offset(15isize);
        let ref mut fresh15 = *dt.offset(14isize);
        let ref mut fresh14 = *dt.offset(13isize);
        let ref mut fresh13 = *dt.offset(12isize);
        let ref mut fresh12 = *dt.offset(11isize);
        let ref mut fresh11 = *dt.offset(10isize);
        let ref mut fresh10 = *dt.offset(9isize);
        let ref mut fresh9 = *dt.offset(8isize);
        let ref mut fresh8 = *dt.offset(7isize);
        let ref mut fresh7 = *dt.offset(6isize);
        let ref mut fresh6 = *dt.offset(5isize);
        let ref mut fresh5 = *dt.offset(4isize);
        let ref mut fresh4 = *dt.offset(3isize);
        let ref mut fresh3 = *dt.offset(2isize);
        let ref mut fresh2 = *dt.offset(1isize);
        *fresh2 = Zero;
        *fresh3 = *fresh2;
        *fresh4 = *fresh3;
        *fresh5 = *fresh4;
        *fresh6 = *fresh5;
        *fresh7 = *fresh6;
        *fresh8 = *fresh7;
        *fresh9 = *fresh8;
        *fresh10 = *fresh9;
        *fresh11 = *fresh10;
        *fresh12 = *fresh11;
        *fresh13 = *fresh12;
        *fresh14 = *fresh13;
        *fresh15 = *fresh14;
        *fresh16 = *fresh15;
        *fresh17 = *fresh16;
        *fresh18 = *fresh17;
        *fresh19 = *fresh18;
        *fresh20 = *fresh19;
        *fresh21 = *fresh20;
        *fresh22 = *fresh21;
        *fresh23 = *fresh22;
        *fresh24 = *fresh23;
        *fresh25 = *fresh24;
        *fresh26 = *fresh25;
        *fresh27 = *fresh26;
        *fresh28 = *fresh27;
        *fresh29 = *fresh28;
        *fresh30 = *fresh29;
        *fresh31 = *fresh30;
        *fresh32 = *fresh31;
        *fresh33 = *fresh32;
        *fresh34 = *fresh33;
        *fresh35 = *fresh34;
        *fresh36 = *fresh35;
        *fresh37 = *fresh36;
        *fresh38 = *fresh37;
        *fresh39 = *fresh38;
        *fresh40 = *fresh39;
        *fresh41 = *fresh40;
        *fresh42 = *fresh41;
        *fresh43 = *fresh42;
        *fresh44 = *fresh43;
        *fresh45 = *fresh44;
        *fresh46 = *fresh45;
        *fresh47 = *fresh46;
        *fresh48 = *fresh47;
        *fresh49 = *fresh48;
        *fresh50 = *fresh49;
        *fresh51 = *fresh50;
        *fresh52 = *fresh51;
        *fresh53 = *fresh52;
        *fresh54 = *fresh53;
        *fresh55 = *fresh54;
        *fresh56 = *fresh55;
        *fresh57 = *fresh56;
        *fresh58 = *fresh57;
        *fresh59 = *fresh58;
        *fresh60 = *fresh59;
        *fresh61 = *fresh60;
        *fresh62 = *fresh61;
        *fresh63 = *fresh62;
        *fresh64 = *fresh63;
        *fresh65 = *fresh64;
        *dt.offset(65isize) = *fresh65;
        pdn_0 = *pdn.offset(0isize);
        pdm1_0 = *pdm1.offset(0isize);
        digit = mod_0(lower32(a, Zero) * dn0, TwoToMinus16, TwoTo16);
        pdtj = &mut *dt.offset(0isize) as *mut libc::c_double;
        j = 0i32;
        while j < 32i32 {
            m2j = *pdm2.offset(j as isize);
            a = *pdtj.offset(0isize) + pdn_0 * digit;
            b =
                *pdtj.offset(1isize) +
                    pdm1_0 * *pdm2.offset((j + 1i32) as isize) +
                    a * TwoToMinus16;
            *pdtj.offset(1isize) = b;
            *pdtj.offset(2isize) +=
                *pdm1.offset(1isize) * m2j + *pdn.offset(1isize) * digit;
            *pdtj.offset(4isize) +=
                *pdm1.offset(2isize) * m2j + *pdn.offset(2isize) * digit;
            *pdtj.offset(6isize) +=
                *pdm1.offset(3isize) * m2j + *pdn.offset(3isize) * digit;
            *pdtj.offset(8isize) +=
                *pdm1.offset(4isize) * m2j + *pdn.offset(4isize) * digit;
            *pdtj.offset(10isize) +=
                *pdm1.offset(5isize) * m2j + *pdn.offset(5isize) * digit;
            *pdtj.offset(12isize) +=
                *pdm1.offset(6isize) * m2j + *pdn.offset(6isize) * digit;
            *pdtj.offset(14isize) +=
                *pdm1.offset(7isize) * m2j + *pdn.offset(7isize) * digit;
            *pdtj.offset(16isize) +=
                *pdm1.offset(8isize) * m2j + *pdn.offset(8isize) * digit;
            *pdtj.offset(18isize) +=
                *pdm1.offset(9isize) * m2j + *pdn.offset(9isize) * digit;
            *pdtj.offset(20isize) +=
                *pdm1.offset(10isize) * m2j + *pdn.offset(10isize) * digit;
            *pdtj.offset(22isize) +=
                *pdm1.offset(11isize) * m2j + *pdn.offset(11isize) * digit;
            *pdtj.offset(24isize) +=
                *pdm1.offset(12isize) * m2j + *pdn.offset(12isize) * digit;
            *pdtj.offset(26isize) +=
                *pdm1.offset(13isize) * m2j + *pdn.offset(13isize) * digit;
            *pdtj.offset(28isize) +=
                *pdm1.offset(14isize) * m2j + *pdn.offset(14isize) * digit;
            *pdtj.offset(30isize) +=
                *pdm1.offset(15isize) * m2j + *pdn.offset(15isize) * digit;
            digit = mod_0(lower32(b, Zero) * dn0, TwoToMinus16, TwoTo16);
            j += 1;
            pdtj = pdtj.offset(1isize)
        }
    }
    conv_d16_to_i32(result, dt.offset((2i32 * nlen) as isize),
                    dt as *mut libc::c_longlong, nlen + 1i32);
    adjust_montf_result(result, nint, nlen);
}