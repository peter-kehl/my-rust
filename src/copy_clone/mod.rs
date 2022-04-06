mod tests;

#[derive(Clone, Copy, Debug)]
pub struct DeriveCopy<'a> {
    i: i32,
    s: &'a str,
}

#[derive(Clone)]
pub struct ContainsString {
    pub s: String, // it gets cloned as a part of ContainsString::clone
}

pub struct WithMutRef<'a> {
    ii: &'a [i32],
    st: &'a mut str,
    string: &'a mut String,
}

pub struct WithPrivateField {
    i: i32,
}

impl WithPrivateField {
    pub fn new() -> Self {
        WithPrivateField { i: 0 }
    }
}
