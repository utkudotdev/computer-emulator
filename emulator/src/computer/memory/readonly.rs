use crate::bits_to_index_length;
use common::bytes_to_store_bits;
use common::un::U;

pub struct ReadOnlyMemory<const STORED_BITS: usize, const MEMORY_SIZE: usize>
where
    [(); bytes_to_store_bits!(STORED_BITS)]: Sized,
{
    memory: [U<STORED_BITS>; MEMORY_SIZE],
}

impl<const STORED_BITS: usize, const MEMORY_SIZE: usize> ReadOnlyMemory<STORED_BITS, MEMORY_SIZE>
where
    [(); bytes_to_store_bits!(STORED_BITS)]: Sized,
    [(); bytes_to_store_bits!(bits_to_index_length!(MEMORY_SIZE))]: Sized, // Guarantee index type is representable
{
    pub fn with_values(values: [U<STORED_BITS>; MEMORY_SIZE]) -> Self {
        ReadOnlyMemory { memory: values }
    }

    pub fn read(&self, location: U<{ bits_to_index_length!(MEMORY_SIZE) }>) -> U<STORED_BITS> {
        // this is a bit of a hack because we don't have usize
        self.memory[u128::from(location) as usize]
    }
}
