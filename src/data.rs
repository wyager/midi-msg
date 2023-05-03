pub trait Store {
    type T;
    type Iter<'a> : Iterator<Item=&'a Self::T> where Self : 'a;
    // Returns None if unable to store the byte
    fn push(&mut self, t : Self::T) -> Option<()>;
    fn iter<'a>(&'a mut self) -> Self::Iter<'a>;
    fn empty() -> Self;
}

impl Store for Vec<u8> {
    type T = u8;
    type Iter<'a> = std::slice::Iter<'a,u8>;

    fn push(&mut self, t : Self::T) -> Option<()> {
        self.push(t);
        Some(())
    }

    fn iter<'a>(&'a mut self) -> Self::Iter<'a> {
        Vec::iter(self)
    }

    fn empty() -> Self {
        Vec::new()
    }

}