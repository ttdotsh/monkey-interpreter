pub trait Is {
    fn is(&self, token: &Self) -> bool;
}
