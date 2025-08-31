pub mod connectable;
pub mod console;
mod store;

pub trait Device {
    fn tick(&mut self, tick: u32);
}
