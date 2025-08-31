use crate::device::connectable::Connectable;
use crate::device::store::Store;
use crate::device::Device;
use common::bytes_to_store_bits;
use common::un::U;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Splitter<const N: usize, const M: usize>
where
    [(); bytes_to_store_bits!(N)]: Sized,
    [(); bytes_to_store_bits!(M)]: Sized,
    [(); bytes_to_store_bits!({ N + M })]: Sized,
{
    combined: Rc<RefCell<Store<U<{ N + M }>>>>,
    low_end: Rc<RefCell<Store<U<N>>>>,
    high_end: Rc<RefCell<Store<U<M>>>>,
}

impl<const N: usize, const M: usize> Splitter<N, M>
where
    [(); bytes_to_store_bits!(N)]: Sized,
    [(); bytes_to_store_bits!(M)]: Sized,
    [(); bytes_to_store_bits!({ N + M })]: Sized,
{
    pub fn new() -> Self {
        Splitter {
            combined: Rc::new(RefCell::new(Store::new(0u8.into()))),
            low_end: Rc::new(RefCell::new(Store::new(0u8.into()))),
            high_end: Rc::new(RefCell::new(Store::new(0u8.into()))),
        }
    }

    pub fn as_low_end(&mut self) -> LowEnd<N, M> {
        LowEnd(self)
    }

    pub fn as_high_end(&mut self) -> HighEnd<N, M> {
        HighEnd(self)
    }
}

impl<const N: usize, const M: usize> Device for Splitter<N, M>
where
    [(); bytes_to_store_bits!(N)]: Sized,
    [(); bytes_to_store_bits!(M)]: Sized,
    [(); bytes_to_store_bits!({ N + M })]: Sized,
{
    fn tick(&mut self, tick: u32) {
        let mut combined = self.combined.borrow_mut();
        let mut low_end = self.low_end.borrow_mut();
        let mut high_end = self.high_end.borrow_mut();

        if combined.get_store_tick() > low_end.get_store_tick() {
            low_end.set(combined.get().change_bits(), tick);
        } else {
            let bottom_zeroed = combined.get() & (U::<{ N + M }>::max() << N);
            combined.set(bottom_zeroed | low_end.get().change_bits(), tick);
        }

        if combined.get_store_tick() > high_end.get_store_tick() {
            high_end.set((combined.get() >> N).change_bits(), tick);
        } else {
            let top_zeroed = combined.get() & (U::<{ N + M }>::max() >> M);
            let high_end_shifted = high_end.get().change_bits() << N;
            combined.set(top_zeroed | high_end_shifted, tick);
        }
    }
}

impl<const N: usize, const M: usize> Connectable<{ N + M }> for Splitter<N, M>
where
    [(); bytes_to_store_bits!(N)]: Sized,
    [(); bytes_to_store_bits!(M)]: Sized,
    [(); bytes_to_store_bits!({ N + M })]: Sized,
{
    fn get_value_ref(&self) -> Rc<RefCell<Store<U<{ N + M }>>>> {
        self.combined.clone()
    }

    fn connect_to(&mut self, other: &dyn Connectable<{ N + M }>) {
        self.combined = other.get_value_ref();
    }
}

pub struct LowEnd<'a, const N: usize, const M: usize>(&'a mut Splitter<N, M>)
where
    [(); bytes_to_store_bits!(N)]: Sized,
    [(); bytes_to_store_bits!(M)]: Sized,
    [(); bytes_to_store_bits!({ N + M })]: Sized;

impl<const N: usize, const M: usize> Connectable<N> for LowEnd<'_, N, M>
where
    [(); bytes_to_store_bits!(N)]: Sized,
    [(); bytes_to_store_bits!(M)]: Sized,
    [(); bytes_to_store_bits!({ N + M })]: Sized,
{
    fn get_value_ref(&self) -> Rc<RefCell<Store<U<N>>>> {
        self.0.low_end.clone()
    }

    fn connect_to(&mut self, other: &dyn Connectable<N>) {
        self.0.low_end = other.get_value_ref();
    }
}

pub struct HighEnd<'a, const N: usize, const M: usize>(&'a mut Splitter<N, M>)
where
    [(); bytes_to_store_bits!(N)]: Sized,
    [(); bytes_to_store_bits!(M)]: Sized,
    [(); bytes_to_store_bits!({ N + M })]: Sized;

impl<const N: usize, const M: usize> Connectable<M> for HighEnd<'_, N, M>
where
    [(); bytes_to_store_bits!(N)]: Sized,
    [(); bytes_to_store_bits!(M)]: Sized,
    [(); bytes_to_store_bits!({ N + M })]: Sized,
{
    fn get_value_ref(&self) -> Rc<RefCell<Store<U<M>>>> {
        self.0.high_end.clone()
    }

    fn connect_to(&mut self, other: &dyn Connectable<M>) {
        self.0.high_end = other.get_value_ref();
    }
}
