// pathfinder/simd/src/x86.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::arch::x86_64::{self, __m128, __m128i, _MM_FROUND_TO_NEAREST_INT};
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Formatter};
use std::mem;
use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Mul, Not, Shr, Sub};

mod swizzle_f32x4;
mod swizzle_i32x4;

// Two 32-bit floats

#[derive(Clone, Copy)]
pub struct F32x2(pub u64);

impl F32x2 {
    // Constructors

    #[inline]
    pub fn new(a: f32, b: f32) -> F32x2 {
        unsafe {
            let a = mem::transmute::<*const f32, *const u32>(&a);
            let b = mem::transmute::<*const f32, *const u32>(&b);
            F32x2((*a as u64) | ((*b as u64) << 32))
        }
    }

    #[inline]
    pub fn splat(x: f32) -> F32x2 {
        F32x2::new(x, x)
    }

    // Basic operations

    #[inline]
    pub fn approx_recip(self) -> F32x2 {
        self.to_f32x4().approx_recip().xy()
    }

    #[inline]
    pub fn min(self, other: F32x2) -> F32x2 {
        self.to_f32x4().min(other.to_f32x4()).xy()
    }

    #[inline]
    pub fn max(self, other: F32x2) -> F32x2 {
        self.to_f32x4().max(other.to_f32x4()).xy()
    }

    #[inline]
    pub fn clamp(self, min: F32x2, max: F32x2) -> F32x2 {
        self.to_f32x4().clamp(min.to_f32x4(), max.to_f32x4()).xy()
    }

    #[inline]
    pub fn abs(self) -> F32x2 {
        self.to_f32x4().abs().xy()
    }

    #[inline]
    pub fn floor(self) -> F32x2 {
        self.to_f32x4().floor().xy()
    }

    #[inline]
    pub fn ceil(self) -> F32x2 {
        self.to_f32x4().ceil().xy()
    }

    #[inline]
    pub fn round(self) -> F32x2 {
        self.to_f32x4().round().xy()
    }

    #[inline]
    pub fn sqrt(self) -> F32x2 {
        self.to_f32x4().sqrt().xy()
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: F32x2) -> U32x2 {
        self.to_f32x4().packed_eq(other.to_f32x4()).xy()
    }

    #[inline]
    pub fn packed_gt(self, other: F32x2) -> U32x2 {
        self.to_f32x4().packed_gt(other.to_f32x4()).xy()
    }

    #[inline]
    pub fn packed_lt(self, other: F32x2) -> U32x2 {
        self.to_f32x4().packed_lt(other.to_f32x4()).xy()
    }

    #[inline]
    pub fn packed_le(self, other: F32x2) -> U32x2 {
        self.to_f32x4().packed_le(other.to_f32x4()).xy()
    }

    // Conversions

    #[inline]
    pub fn to_f32x4(self) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_castsi128_ps(x86_64::_mm_cvtsi64_si128(self.0 as i64))) }
    }

    #[inline]
    pub fn to_i32x2(self) -> I32x2 {
        self.to_i32x4().xy()
    }

    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        self.to_f32x4().to_i32x4()
    }

    // Swizzle

    #[inline]
    pub fn yx(self) -> F32x2 {
        self.to_f32x4().yx()
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: F32x2) -> F32x4 {
        self.to_f32x4().concat_xy_xy(other.to_f32x4())
    }
}

impl Default for F32x2 {
    #[inline]
    fn default() -> F32x2 {
        F32x2(0)
    }
}

impl Index<usize> for F32x2 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        unsafe { &mem::transmute::<&u64, &[f32; 2]>(&self.0)[index] }
    }
}

impl IndexMut<usize> for F32x2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        unsafe { &mut mem::transmute::<&mut u64, &mut [f32; 2]>(&mut self.0)[index] }
    }
}

impl Debug for F32x2 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}>", self[0], self[1])
    }
}

impl PartialEq for F32x2 {
    #[inline]
    fn eq(&self, other: &F32x2) -> bool {
        self.packed_eq(*other).is_all_ones()
    }
}

impl Add<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn add(self, other: F32x2) -> F32x2 {
        (self.to_f32x4() + other.to_f32x4()).xy()
    }
}

impl Mul<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn mul(self, other: F32x2) -> F32x2 {
        (self.to_f32x4() * other.to_f32x4()).xy()
    }
}

impl Sub<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn sub(self, other: F32x2) -> F32x2 {
        (self.to_f32x4() - other.to_f32x4()).xy()
    }
}

// Four 32-bit floats

#[derive(Clone, Copy)]
pub struct F32x4(pub __m128);

impl F32x4 {
    // Constructors

    #[inline]
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> F32x4 {
        unsafe {
            let vector = [a, b, c, d];
            F32x4(x86_64::_mm_loadu_ps(vector.as_ptr()))
        }
    }

    #[inline]
    pub fn splat(x: f32) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_set1_ps(x)) }
    }

    // Basic operations

    #[inline]
    pub fn approx_recip(self) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_rcp_ps(self.0)) }
    }

    #[inline]
    pub fn min(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_min_ps(self.0, other.0)) }
    }

    #[inline]
    pub fn max(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_max_ps(self.0, other.0)) }
    }

    #[inline]
    pub fn clamp(self, min: F32x4, max: F32x4) -> F32x4 {
        self.max(min).min(max)
    }

    #[inline]
    pub fn abs(self) -> F32x4 {
        unsafe {
            let tmp = x86_64::_mm_srli_epi32(I32x4::splat(-1).0, 1);
            F32x4(x86_64::_mm_and_ps(x86_64::_mm_castsi128_ps(tmp), self.0))
        }
    }

    #[inline]
    pub fn floor(self) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_floor_ps(self.0)) }
    }

    #[inline]
    pub fn ceil(self) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_ceil_ps(self.0)) }
    }

    #[inline]
    pub fn round(self) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_round_ps(self.0, _MM_FROUND_TO_NEAREST_INT)) }
    }

    #[inline]
    pub fn sqrt(self) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_sqrt_ps(self.0)) }
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: F32x4) -> U32x4 {
        unsafe {
            U32x4(x86_64::_mm_castps_si128(x86_64::_mm_cmpeq_ps(
                self.0, other.0,
            )))
        }
    }

    #[inline]
    pub fn packed_gt(self, other: F32x4) -> U32x4 {
        unsafe {
            U32x4(x86_64::_mm_castps_si128(x86_64::_mm_cmpgt_ps(
                self.0, other.0,
            )))
        }
    }

    #[inline]
    pub fn packed_lt(self, other: F32x4) -> U32x4 {
        other.packed_gt(self)
    }

    #[inline]
    pub fn packed_le(self, other: F32x4) -> U32x4 {
        !self.packed_gt(other)
    }

    // Conversions

    /// Converts these packed floats to integers.
    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_cvtps_epi32(self.0)) }
    }

    // Extraction

    #[inline]
    pub fn xy(self) -> F32x2 {
        unsafe { F32x2(x86_64::_mm_cvtsi128_si64(x86_64::_mm_castps_si128(self.0)) as u64) }
    }

    #[inline]
    pub fn xw(self) -> F32x2 {
        unsafe { F32x2(x86_64::_mm_cvtsi128_si64(x86_64::_mm_castps_si128(self.xwyz().0)) as u64) }
    }

    #[inline]
    pub fn yx(self) -> F32x2 {
        unsafe { F32x2(x86_64::_mm_cvtsi128_si64(x86_64::_mm_castps_si128(self.yxwz().0)) as u64) }
    }

    #[inline]
    pub fn zy(self) -> F32x2 {
        unsafe { F32x2(x86_64::_mm_cvtsi128_si64(x86_64::_mm_castps_si128(self.zyxw().0)) as u64) }
    }

    #[inline]
    pub fn zw(self) -> F32x2 {
        unsafe { F32x2(x86_64::_mm_cvtsi128_si64(x86_64::_mm_castps_si128(self.zwxy().0)) as u64) }
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: F32x4) -> F32x4 {
        unsafe {
            let this = x86_64::_mm_castps_pd(self.0);
            let other = x86_64::_mm_castps_pd(other.0);
            let result = x86_64::_mm_unpacklo_pd(this, other);
            F32x4(x86_64::_mm_castpd_ps(result))
        }
    }

    #[inline]
    pub fn concat_xy_zw(self, other: F32x4) -> F32x4 {
        unsafe {
            let this = x86_64::_mm_castps_pd(self.0);
            let other = x86_64::_mm_castps_pd(other.0);
            let result = x86_64::_mm_shuffle_pd(this, other, 0b10);
            F32x4(x86_64::_mm_castpd_ps(result))
        }
    }

    #[inline]
    pub fn concat_zw_zw(self, other: F32x4) -> F32x4 {
        unsafe {
            let this = x86_64::_mm_castps_pd(self.0);
            let other = x86_64::_mm_castps_pd(other.0);
            let result = x86_64::_mm_unpackhi_pd(this, other);
            F32x4(x86_64::_mm_castpd_ps(result))
        }
    }

    #[inline]
    pub fn concat_wz_yx(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_shuffle_ps(self.0, other.0, 0b0001_1011)) }
    }
}

impl Default for F32x4 {
    #[inline]
    fn default() -> F32x4 {
        unsafe { F32x4(x86_64::_mm_setzero_ps()) }
    }
}

impl Index<usize> for F32x4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        unsafe { &mem::transmute::<&__m128, &[f32; 4]>(&self.0)[index] }
    }
}

impl IndexMut<usize> for F32x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        unsafe { &mut mem::transmute::<&mut __m128, &mut [f32; 4]>(&mut self.0)[index] }
    }
}

impl Debug for F32x4 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}, {}, {}>", self[0], self[1], self[2], self[3])
    }
}

impl PartialEq for F32x4 {
    #[inline]
    fn eq(&self, other: &F32x4) -> bool {
        self.packed_eq(*other).is_all_ones()
    }
}

impl Add<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn add(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_add_ps(self.0, other.0)) }
    }
}

impl Mul<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn mul(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_mul_ps(self.0, other.0)) }
    }
}

impl Sub<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn sub(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_sub_ps(self.0, other.0)) }
    }
}

// Two 32-bit signed integers

#[derive(Clone, Copy)]
pub struct I32x2(pub u64);

impl I32x2 {
    // Constructors

    #[inline]
    pub fn new(a: i32, b: i32) -> I32x2 {
        unsafe {
            let a = mem::transmute::<*const i32, *const u32>(&a);
            let b = mem::transmute::<*const i32, *const u32>(&b);
            I32x2((*a as u64) | ((*b as u64) << 32))
        }
    }

    #[inline]
    pub fn splat(x: i32) -> I32x2 {
        I32x2::new(x, x)
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: I32x2) -> I32x4 {
        self.to_i32x4().concat_xy_xy(other.to_i32x4())
    }

    // Conversions

    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_cvtsi64_si128(self.0 as i64)) }
    }

    #[inline]
    pub fn to_f32x4(self) -> F32x4 {
        self.to_i32x4().to_f32x4()
    }

    /// Converts these packed integers to floats.
    #[inline]
    pub fn to_f32x2(self) -> F32x2 {
        self.to_f32x4().xy()
    }

    // Basic operations

    #[inline]
    pub fn min(self, other: I32x2) -> I32x2 {
        self.to_i32x4().min(other.to_i32x4()).xy()
    }

    // Comparisons

    // TODO(pcwalton): Make a `U32x2` type and use that!
    #[inline]
    pub fn packed_eq(self, other: I32x2) -> U32x4 {
        self.to_i32x4().packed_eq(other.to_i32x4())
    }

    #[inline]
    pub fn packed_gt(self, other: I32x2) -> U32x4 {
        self.to_i32x4().packed_gt(other.to_i32x4())
    }

    #[inline]
    pub fn packed_le(self, other: I32x2) -> U32x4 {
        self.to_i32x4().packed_le(other.to_i32x4())
    }
}

impl Default for I32x2 {
    #[inline]
    fn default() -> I32x2 {
        I32x2(0)
    }
}

impl Index<usize> for I32x2 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &i32 {
        unsafe { &mem::transmute::<&u64, &[i32; 2]>(&self.0)[index] }
    }
}

impl IndexMut<usize> for I32x2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        unsafe { &mut mem::transmute::<&mut u64, &mut [i32; 2]>(&mut self.0)[index] }
    }
}

impl Add<I32x2> for I32x2 {
    type Output = I32x2;
    #[inline]
    fn add(self, other: I32x2) -> I32x2 {
        (self.to_i32x4() + other.to_i32x4()).xy()
    }
}

impl Sub<I32x2> for I32x2 {
    type Output = I32x2;
    #[inline]
    fn sub(self, other: I32x2) -> I32x2 {
        (self.to_i32x4() - other.to_i32x4()).xy()
    }
}

impl Mul<I32x2> for I32x2 {
    type Output = I32x2;
    #[inline]
    fn mul(self, other: I32x2) -> I32x2 {
        (self.to_i32x4() * other.to_i32x4()).xy()
    }
}

impl Debug for I32x2 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}>", self[0], self[1])
    }
}

impl PartialEq for I32x2 {
    #[inline]
    fn eq(&self, other: &I32x2) -> bool {
        self.packed_eq(*other).is_all_ones()
    }
}

// Four 32-bit signed integers

#[derive(Clone, Copy)]
pub struct I32x4(pub __m128i);

impl I32x4 {
    // Constructors

    #[inline]
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> I32x4 {
        unsafe {
            let vector = [a, b, c, d];
            I32x4(x86_64::_mm_loadu_si128(vector.as_ptr() as *const __m128i))
        }
    }

    #[inline]
    pub fn splat(x: i32) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_set1_epi32(x)) }
    }

    // Extraction

    #[inline]
    pub fn xy(self) -> I32x2 {
        unsafe { I32x2(x86_64::_mm_cvtsi128_si64(self.0) as u64) }
    }

    #[inline]
    pub fn xw(self) -> I32x2 {
        unsafe { I32x2(x86_64::_mm_cvtsi128_si64(self.xwyz().0) as u64) }
    }

    #[inline]
    pub fn yx(self) -> I32x2 {
        unsafe { I32x2(x86_64::_mm_cvtsi128_si64(self.yxwz().0) as u64) }
    }

    #[inline]
    pub fn zy(self) -> I32x2 {
        unsafe { I32x2(x86_64::_mm_cvtsi128_si64(self.zyxw().0) as u64) }
    }

    #[inline]
    pub fn zw(self) -> I32x2 {
        unsafe { I32x2(x86_64::_mm_cvtsi128_si64(self.zwxy().0) as u64) }
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: I32x4) -> I32x4 {
        unsafe {
            let this = x86_64::_mm_castsi128_pd(self.0);
            let other = x86_64::_mm_castsi128_pd(other.0);
            let result = x86_64::_mm_unpacklo_pd(this, other);
            I32x4(x86_64::_mm_castpd_si128(result))
        }
    }

    // Conversions

    /// Converts these packed integers to floats.
    #[inline]
    pub fn to_f32x4(self) -> F32x4 {
        unsafe { F32x4(x86_64::_mm_cvtepi32_ps(self.0)) }
    }

    // Basic operations

    #[inline]
    pub fn min(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_min_epi32(self.0, other.0)) }
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: I32x4) -> U32x4 {
        unsafe { U32x4(x86_64::_mm_cmpeq_epi32(self.0, other.0)) }
    }

    // Comparisons

    #[inline]
    pub fn packed_gt(self, other: I32x4) -> U32x4 {
        unsafe { U32x4(x86_64::_mm_cmpgt_epi32(self.0, other.0)) }
    }

    #[inline]
    pub fn packed_le(self, other: I32x4) -> U32x4 {
        !self.packed_gt(other)
    }
}

impl Default for I32x4 {
    #[inline]
    fn default() -> I32x4 {
        unsafe { I32x4(x86_64::_mm_setzero_si128()) }
    }
}

impl Index<usize> for I32x4 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &i32 {
        unsafe { &mem::transmute::<&__m128i, &[i32; 4]>(&self.0)[index] }
    }
}

impl IndexMut<usize> for I32x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        unsafe { &mut mem::transmute::<&mut __m128i, &mut [i32; 4]>(&mut self.0)[index] }
    }
}

impl Add<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn add(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_add_epi32(self.0, other.0)) }
    }
}

impl Sub<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn sub(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_sub_epi32(self.0, other.0)) }
    }
}

impl Mul<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn mul(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_mullo_epi32(self.0, other.0)) }
    }
}

impl BitAnd<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn bitand(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_and_si128(self.0, other.0)) }
    }
}

impl BitOr<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn bitor(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_or_si128(self.0, other.0)) }
    }
}

impl Shr<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn shr(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(x86_64::_mm_srlv_epi32(self.0, other.0)) }
    }
}

impl Debug for I32x4 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}, {}, {}>", self[0], self[1], self[2], self[3])
    }
}

impl PartialEq for I32x4 {
    #[inline]
    fn eq(&self, other: &I32x4) -> bool {
        self.packed_eq(*other).is_all_ones()
    }
}

// Two 32-bit unsigned integers

#[derive(Clone, Copy)]
pub struct U32x2(pub u64);

impl U32x2 {
    #[inline]
    pub fn is_all_ones(self) -> bool {
        self.0 == !0
    }

    #[inline]
    pub fn is_all_zeroes(self) -> bool {
        self.0 == 0
    }
}

// Four 32-bit unsigned integers

#[derive(Clone, Copy)]
pub struct U32x4(pub __m128i);

impl U32x4 {
    // Constructors

    #[inline]
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> U32x4 {
        unsafe {
            let vector = [a, b, c, d];
            U32x4(x86_64::_mm_loadu_si128(vector.as_ptr() as *const __m128i))
        }
    }

    #[inline]
    pub fn splat(x: u32) -> U32x4 {
        unsafe { U32x4(x86_64::_mm_set1_epi32(x as i32)) }
    }

    // Basic operations

    #[inline]
    pub fn is_all_ones(self) -> bool {
        unsafe { x86_64::_mm_test_all_ones(self.0) != 0 }
    }

    #[inline]
    pub fn is_all_zeroes(self) -> bool {
        unsafe { x86_64::_mm_test_all_zeros(self.0, self.0) != 0 }
    }

    // Extraction

    #[inline]
    pub fn xy(self) -> U32x2 {
        unsafe { U32x2(x86_64::_mm_cvtsi128_si64(self.0) as u64) }
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: U32x4) -> U32x4 {
        unsafe { U32x4(x86_64::_mm_cmpeq_epi32(self.0, other.0)) }
    }
}

impl Debug for U32x4 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}, {}, {}>", self[0], self[1], self[2], self[3])
    }
}

impl Index<usize> for U32x4 {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &u32 {
        unsafe { &mem::transmute::<&__m128i, &[u32; 4]>(&self.0)[index] }
    }
}

impl PartialEq for U32x4 {
    #[inline]
    fn eq(&self, other: &U32x4) -> bool {
        self.packed_eq(*other).is_all_ones()
    }
}

impl Not for U32x4 {
    type Output = U32x4;
    #[inline]
    fn not(self) -> U32x4 {
        self ^ U32x4::splat(!0)
    }
}

impl BitXor<U32x4> for U32x4 {
    type Output = U32x4;
    #[inline]
    fn bitxor(self, other: U32x4) -> U32x4 {
        unsafe { U32x4(x86_64::_mm_xor_si128(self.0, other.0)) }
    }
}
