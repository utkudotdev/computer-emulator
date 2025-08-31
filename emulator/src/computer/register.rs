use common::bytes_to_store_bits;
use common::un::U;

pub struct Register<const N: usize>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    value: U<N>,
}

impl<const N: usize> Register<N>
where
    [(); bytes_to_store_bits!(N)]: Sized,
{
    pub fn new() -> Self {
        Register { value: 0u8.into() }
    }

    pub fn store(&mut self, value: U<N>) {
        self.value = value
    }

    pub fn load(&self) -> U<N> {
        self.value
    }

    pub fn increment(&mut self) {
        self.value = self.value + 1u8.into()
    }

    pub fn decrement(&mut self) {
        self.value = self.value - 1u8.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let r: Register<8> = Register::new();
        assert_eq!(r.value, 0u8.into())
    }

    #[test]
    fn store() {
        let mut r: Register<8> = Register::new();
        r.store(1u8.into());
        assert_eq!(r.value, 1u8.into())
    }

    #[test]
    fn load() {
        let mut r: Register<8> = Register::new();
        r.value = 10u8.into();
        assert_eq!(r.load(), 10u8.into())
    }

    #[test]
    fn increment() {
        let mut r: Register<8> = Register::new();
        r.value = 10u8.into();
        r.increment();
        assert_eq!(r.value, 11u8.into())
    }

    #[test]
    fn decrement() {
        let mut r: Register<8> = Register::new();
        r.value = 10u8.into();
        r.decrement();
        assert_eq!(r.value, 9u8.into())
    }
}
