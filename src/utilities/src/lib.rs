use wasm_bindgen::prelude::*;

pub trait IntoVecU8 {
    fn into_vec_u8(self) -> Vec<u8>;
}

impl IntoVecU8 for String {
    fn into_vec_u8(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl IntoVecU8 for Vec<u8> {
    fn into_vec_u8(self) -> Vec<u8> {
        self
    }
}

impl IntoVecU8 for &Vec<u8> {
    fn into_vec_u8(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl<'a> IntoVecU8 for &'a str {
    fn into_vec_u8(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl<'a> IntoVecU8 for &'a [u8] {
    fn into_vec_u8(self) -> Vec<u8> {
        self.to_vec()
    }
}
