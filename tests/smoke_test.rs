use liquid_dsp_bindgen;

#[test]
pub fn smoke_test() {
    // Just ensure that the library can be linked and a simple function can be called.
    unsafe {
        let maj = liquid_dsp_bindgen::raw::LIQUID_VERSION_MAJOR;
        let min = liquid_dsp_bindgen::raw::LIQUID_VERSION_MINOR;
        let rev = liquid_dsp_bindgen::raw::LIQUID_VERSION_PATCH;

        dbg!(maj, min, rev);
    }
}
