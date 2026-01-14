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

impl From<f32> for LiquidFloatComplex {
    fn from(value: f32) -> Self {
        Self::new(value, 0.0)
    }
}

pub struct DetectorCCCF {
    ptr: raw::detector_cccf,
}

impl DetectorCCCF {
    pub fn new(k: &mut [LiquidFloatComplex], p: f32, beta: f32) -> Self {
        let mut preample = k.iter().map(|c| c.raw).collect::<Vec<_>>();
        let n = preample.len() as u32;
        let ptr = unsafe { raw::detector_cccf_create(preample.as_mut_ptr(), n, p, beta) };
        assert!(!ptr.is_null());
        Self { ptr }
    }
    pub fn correlate(&mut self, x: LiquidFloatComplex) -> CorrelationResult {
        let mut tau_hat = 0.0;
        let mut dphi_hat = 0.0;
        let mut gamma = 0.0;
        let r = unsafe {
            raw::detector_cccf_correlate(self.ptr, x.raw, &mut tau_hat, &mut dphi_hat, &mut gamma)
        };

        CorrelationResult {
            detected: r != 0,
            tau_hat,
            dphi_hat,
            gamma,
        }
    }
}

pub struct CorrelationResult {
    pub detected: bool,
    pub tau_hat: f32,
    pub dphi_hat: f32,
    pub gamma: f32,
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
