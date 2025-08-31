pub struct Store<T> {
    value: T,
    store_tick: u32,
}

impl<T: Clone> Store<T> {
    pub fn new(inner: T) -> Self {
        Store {
            value: inner,
            store_tick: 0,
        }
    }

    pub fn get(&self) -> T {
        self.value.clone()
    }

    pub fn get_store_tick(&self) -> u32 {
        self.store_tick
    }

    pub fn set(&mut self, value: T, tick: u32) {
        self.value = value;
        self.store_tick = tick;
    }
}
