pub trait Message {
    fn into_vec_u8(self) -> Vec<u8>;
    fn from_vec_u8() -> Self;
}