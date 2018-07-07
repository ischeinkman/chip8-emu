pub trait InputReciever {
    fn check_key(&mut self, key : u8) -> bool;
    fn wait_for_key(&mut self) -> u8;
}