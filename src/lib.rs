pub mod raw;

pub struct LiquidFloatComplex {
    raw: raw::liquid_float_complex,
}

impl LiquidFloatComplex {
    pub fn new(re: f32, im: f32) -> Self {
        Self {
            raw: raw::liquid_float_complex { re, im },
        }
    }
}

pub struct DetectorCCCF {
    ptr: raw::detector_cccf,
}

impl DetectorCCCF {
    pub fn detector_cccf_correlate(
        &mut self,
        x: LiquidFloatComplex,
        tau_hat: &mut f32,
        dphi_hat: &mut f32,
        gamma: &mut f32,
    ) -> i32 {
        unsafe { raw::detector_cccf_correlate(self.ptr, x.raw, tau_hat, dphi_hat, gamma) }
    }
}

impl Drop for DetectorCCCF {
    fn drop(&mut self) {
        unsafe { raw::detector_cccf_destroy(self.ptr) }
    }
}

pub struct MSequence {
    ptr: raw::msequence,
}

impl MSequence {
    pub fn new_default(order: u32) -> Self {
        let ptr = unsafe { raw::msequence_create_default(order) };
        assert!(!ptr.is_null());
        Self { ptr }
    }

    pub fn advance(&mut self) -> u32 {
        unsafe { raw::msequence_advance(self.ptr) }
    }
}

impl Drop for MSequence {
    fn drop(&mut self) {
        unsafe {
            raw::msequence_destroy(self.ptr);
        }
    }
}
