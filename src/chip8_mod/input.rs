pub trait InputReciever {
    fn check_key(&mut self, key : u8) -> bool;
    fn check_any_key(&mut self) -> Option<u8>;

    fn check_should_die(&mut self) -> bool;
}
