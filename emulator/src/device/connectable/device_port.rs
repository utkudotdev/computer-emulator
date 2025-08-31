use crate::device::connectable::Connectable;
use crate::device::store::Store;
use crate::device::Device;
use common::bytes_to_store_bits;
use common::un::U;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DevicePort<const N: usize>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    store: Rc<RefCell<Store<U<N>>>>,
    write_val: U<N>,
    writing: bool,
}

impl<const N: usize> Device for DevicePort<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    fn tick(&mut self, tick: u32) {
        if self.writing {
            self.store.borrow_mut().set(self.write_val, tick);
        }
    }
}

impl<const N: usize> DevicePort<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    pub fn new() -> Self {
        DevicePort {
            store: Rc::new(RefCell::new(Store::new(0u8.into()))),
            write_val: 0u8.into(),
            writing: false,
        }
    }

    pub fn read(&mut self) -> U<N> {
        self.writing = false;
        self.store.borrow().get()
    }

    pub fn write(&mut self, value: U<N>) {
        self.write_val = value;
        self.writing = true;
    }
}

impl<const N: usize> Connectable<N> for DevicePort<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    fn get_value_ref(&self) -> Rc<RefCell<Store<U<N>>>> {
        self.store.clone()
    }

    fn connect_to(&mut self, other: &dyn Connectable<N>) {
        self.store = other.get_value_ref();
    }
}
