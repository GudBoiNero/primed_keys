/// OS Independent Interface
pub trait App {
    fn new() -> Self;
    fn initialized(&self) -> bool;
}

pub enum ThreadPriority {
    Main = 0,
    Command = 1,
}

impl Into<usize> for ThreadPriority {
    fn into(self) -> usize {
        self as usize
    }
}
