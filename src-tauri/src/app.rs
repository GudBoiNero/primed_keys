// OS Independent Interface
pub trait App {
    fn new() -> Self;
    fn initialized(&self) -> bool;
}
