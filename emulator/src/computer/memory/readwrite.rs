use crate::bits_to_index_length;
use common::bytes_to_store_bits;
use common::un::U;

pub struct ReadWriteMemory<const STORED_BITS: usize, const MEMORY_SIZE: usize>
where
    [(); bytes_to_store_bits!(STORED_BITS)]: Sized,
{
    memory: [U<STORED_BITS>; MEMORY_SIZE],
}

impl<const STORED_BITS: usize, const MEMORY_SIZE: usize> ReadWriteMemory<STORED_BITS, MEMORY_SIZE>
where
    [(); bytes_to_store_bits!(STORED_BITS)]: Sized,
    [(); bytes_to_store_bits!(bits_to_index_length!(MEMORY_SIZE))]: Sized, // Guarantee index type is representable
{
    pub fn new() -> Self {
        ReadWriteMemory {
            memory: [0u8.into(); MEMORY_SIZE],
        }
    }

    pub fn read(&self, location: U<{ bits_to_index_length!(MEMORY_SIZE) }>) -> U<STORED_BITS> {
        self.memory[u128::from(location) as usize]
    }

    pub fn write(
        &mut self,
        location: U<{ bits_to_index_length!(MEMORY_SIZE) }>,
        value: U<STORED_BITS>,
    ) {
        self.memory[u128::from(location) as usize] = value;
    }
}
