pub trait ByteStore {
    type Iter<'a> : Iterator<Item=&'a u8>;
    // Returns None if unable to store the byte
    fn push(&mut self, byte : u8) -> Option<()>;
    fn iter<'a>(&'a mut self) -> Self::Iter<'a>;
    fn empty() -> Self;
}

impl ByteStore for Vec<u8> {

}