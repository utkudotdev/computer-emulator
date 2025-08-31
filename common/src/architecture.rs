pub const WORKING_BITS: usize = 4;
pub const PC_BITS: usize = 6;
pub const PA_BITS: usize = 4;
pub const INSTRUCTION_BITS: usize = 8;
pub const PORT_BITS: usize = 4;

pub const NUM_REGISTERS: usize = 4;
pub const NUM_PORTS: usize = 4;
pub const NUM_PINS: usize = 4;

pub const REGISTER_INDEX_BITS: usize = NUM_REGISTERS.ilog2() as usize;
pub const PORT_INDEX_BITS: usize = NUM_PORTS.ilog2() as usize;
pub const PIN_INDEX_BITS: usize = NUM_PINS.ilog2() as usize;

pub const PROGRAM_MEMORY_SIZE: usize = 2usize.pow(PC_BITS as u32) * 2usize.pow(PA_BITS as u32);
pub const WORKING_MEMORY_SIZE: usize = 2usize.pow(2 * WORKING_BITS as u32); // two registers used to index
