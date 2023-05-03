pub trait Store {
    type T;
    type Iter<'a> : Iterator<Item=&'a Self::T>;
    // Returns None if unable to store the byte
    fn push(&mut self, t : Self::T) -> Option<()>;
    fn iter<'a>(&'a mut self) -> Self::Iter<'a>;
    fn empty() -> Self;
}

impl Store for Vec<u8> {
    type T = u8;

}