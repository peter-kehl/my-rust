use core::ptr::null;

//#[derive(Clone, Copy, Debug)]
struct SelfPointing {
    self_ptr: *const SelfPointing,
}

impl SelfPointing {
    #[inline] // inline doesn't help - the result is still copied
    pub fn new() -> Self {
        let mut result = Self { self_ptr: null() };
        result.self_ptr = &result;
        result
    }
}

#[test]
#[ignore]
fn should_not_copy() {
    let self_pointed = SelfPointing::new();
    assert_eq!(&self_pointed as *const SelfPointing, self_pointed.self_ptr);
}
