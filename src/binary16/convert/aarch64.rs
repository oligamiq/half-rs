use core::{
    arch::{
        aarch64::{float32x4_t, uint16x4_t},
        asm,
    },
    mem::MaybeUninit,
    ptr,
};

// TODO: Assembly conversions can go direct to f64 too, saving a cast

#[target_feature(enable = "neon")]
#[inline]
pub(super) unsafe fn f16_to_f32_neon(i: u16) -> f32 {
    let result: f32;
    unsafe {
        asm!(
            "fcvt {0:s}, {1:h}",
        out(vreg) result,
        in(vreg) i,
        options(pure, nomem, nostack));
    }
    result
}

#[target_feature(enable = "neon")]
#[inline]
pub(super) unsafe fn f32_to_f16_neon(f: f32) -> u16 {
    let result: u16;
    unsafe {
        asm!(
            "fcvt {0:h}, {1:s}",
        out(vreg) result,
        in(vreg) f,
        options(pure, nomem, nostack));
    }
    result
}

#[target_feature(enable = "neon")]
#[inline]
pub(super) unsafe fn f16x4_to_f32x4_neon(v: &[u16; 4]) -> [f32; 4] {
    let mut vec = MaybeUninit::<uint16x4_t>::uninit();
    ptr::copy_nonoverlapping(v.as_ptr(), vec.as_mut_ptr().cast(), 4);
    let result: float32x4_t;
    unsafe {
        asm!(
            "fcvtl {0:v}.4s, {1:v}.4h",
        in(vreg) vec.assume_init(),
        out(vreg) result,
        options(pure, nomem, nostack));
    }
    *(&result as *const float32x4_t).cast()
}

#[target_feature(enable = "neon")]
#[inline]
pub(super) unsafe fn f32x4_to_f16x4_neon(v: &[f32; 4]) -> [u16; 4] {
    let mut vec = MaybeUninit::<float32x4_t>::uninit();
    ptr::copy_nonoverlapping(v.as_ptr(), vec.as_mut_ptr().cast(), 4);
    let result: uint16x4_t;
    unsafe {
        asm!(
            "fcvtn {0:v}.4h, {1:v}.4s",
        in(vreg) vec.assume_init(),
        out(vreg) result,
        options(pure, nomem, nostack));
    }
    *(&result as *const uint16x4_t).cast()
}

#[target_feature(enable = "neon")]
#[inline]
pub(super) unsafe fn f16x4_to_f64x4_neon(v: &[u16; 4]) -> [f64; 4] {
    let array = f16x4_to_f32x4_neon(v);
    // Let compiler vectorize this regular cast for now.
    // TODO: investigate doing SIMD cast
    [
        array[0] as f64,
        array[1] as f64,
        array[2] as f64,
        array[3] as f64,
    ]
}

#[target_feature(enable = "neon")]
#[inline]
pub(super) unsafe fn f64x4_to_f16x4_neon(v: &[f64; 4]) -> [u16; 4] {
    // Let compiler vectorize this regular cast for now.
    // TODO: investigate doing SIMD cast
    let v = [v[0] as f32, v[1] as f32, v[2] as f32, v[3] as f32];
    f32x4_to_f16x4_neon(&v)
}
