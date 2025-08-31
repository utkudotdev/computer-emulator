mod alu;
mod memory;
mod register;

use crate::computer::alu::ArithmeticLogicUnit;
use crate::computer::memory::readonly::ReadOnlyMemory;
use crate::computer::memory::readwrite::ReadWriteMemory;
use crate::computer::register::Register;
use crate::device::connectable::device_pin::DevicePin;
use crate::device::connectable::device_port::DevicePort;
use crate::device::Device;
use common::architecture::*;
use common::instruction::{decode_instruction, Instruction};
use common::un::U;
use std::borrow::BorrowMut;

pub struct Computer {
    alu: ArithmeticLogicUnit,

    x_register: Register<WORKING_BITS>,
    y_register: Register<WORKING_BITS>,
    z_register: Register<WORKING_BITS>,

    program_counter: Register<PC_BITS>,
    page_address: Register<PA_BITS>,
    page_buffer: U<PA_BITS>,
    subroutine_jump_flag: bool,
    subroutine_buffer: Register<PC_BITS>,

    status_flag: bool,

    ports: [DevicePort<PORT_BITS>; NUM_PORTS],
    pins: [DevicePin; NUM_PINS],

    program_memory: ReadOnlyMemory<INSTRUCTION_BITS, PROGRAM_MEMORY_SIZE>,
    working_memory: ReadWriteMemory<WORKING_BITS, WORKING_MEMORY_SIZE>,
}

impl Device for Computer {
    fn tick(&mut self, tick: u32) {
        let inst_bits = self.fetch();
        self.program_counter.increment();
        let inst = self.decode(inst_bits);
        self.execute(inst);

        for port in self.ports.iter_mut() {
            port.tick(tick);
        }

        for pin in self.pins.iter_mut() {
            pin.tick(tick);
        }
    }
}

impl Computer {
    pub fn with_program(program: [U<INSTRUCTION_BITS>; PROGRAM_MEMORY_SIZE]) -> Self {
        Computer {
            alu: ArithmeticLogicUnit::new(),
            x_register: Register::new(),
            y_register: Register::new(),
            z_register: Register::new(),
            program_counter: Register::new(),
            page_address: Register::new(),
            page_buffer: 0u8.into(),
            subroutine_jump_flag: false,
            subroutine_buffer: Register::new(),
            status_flag: false,
            ports: [
                DevicePort::new(),
                DevicePort::new(),
                DevicePort::new(),
                DevicePort::new(),
            ],
            pins: [
                DevicePin::new(),
                DevicePin::new(),
                DevicePin::new(),
                DevicePin::new(),
            ],
            program_memory: ReadOnlyMemory::with_values(program),
            working_memory: ReadWriteMemory::new(),
        }
    }

    fn fetch(&self) -> U<INSTRUCTION_BITS> {
        let pc = self.program_counter.load();
        let pa = self.page_address.load();
        let addr = (pa.change_bits() << PC_BITS) | pc.change_bits();
        self.program_memory.read(addr)
    }

    fn decode(&self, instruction: U<INSTRUCTION_BITS>) -> Instruction {
        decode_instruction(instruction)
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NOP => (),
            Instruction::STR { register_id } => {
                let addr = self.decode_xy();
                let register = self.get_register(register_id);
                let value = register.load();
                self.working_memory.write(addr, value)
            }
            Instruction::LOD { register_id } => {
                let addr = self.decode_xy();
                let value = self.working_memory.read(addr);
                let register = self.get_register(register_id);
                register.store(value)
            }
            Instruction::LDI {
                register_id,
                immediate,
            } => {
                let register = self.get_register(register_id);
                register.store(immediate)
            }
            Instruction::INC { register_id } => {
                let register = self.get_register(register_id);
                register.increment()
            }
            Instruction::DEC { register_id } => {
                let register = self.get_register(register_id);
                register.decrement()
            }
            Instruction::MOV {
                register_to_id,
                register_from_id,
            } => {
                let register_from = self.get_register(register_from_id);
                let value = register_from.load();

                let register_to = self.get_register(register_to_id);
                register_to.store(value)
            }
            Instruction::INP { port_id } => {
                let value = self.get_port(port_id).read();
                self.z_register.store(value)
            }
            Instruction::OUT { port_id } => {
                let val = self.z_register.load();
                let port = self.get_port(port_id);
                port.write(val);
            }
            Instruction::SEP { pin_id } => self.get_pin(pin_id).write(1u8.into()),
            Instruction::RSP { pin_id } => self.get_pin(pin_id).write(0u8.into()),
            Instruction::ADD { register_id } => {
                let value = self.get_register(register_id).load();
                self.alu.add(value)
            }
            Instruction::SUB { register_id } => {
                let value = self.get_register(register_id).load();
                self.alu.sub(value)
            }
            Instruction::BOR { register_id } => {
                let value = self.get_register(register_id).load();
                self.alu.bor(value)
            }
            Instruction::AND { register_id } => {
                let value = self.get_register(register_id).load();
                self.alu.and(value)
            }
            Instruction::CMP { register_id } => {
                let value = self.get_register(register_id).load();
                self.status_flag = self.alu.cmp(value)
            }
            Instruction::GRT { register_id } => {
                let value = self.get_register(register_id).load();
                self.status_flag = self.alu.grt(value)
            }
            Instruction::LES { register_id } => {
                let value = self.get_register(register_id).load();
                self.status_flag = self.alu.les(value)
            }
            Instruction::BRN { immediate } => {
                if !self.status_flag {
                    return;
                }

                if self.subroutine_jump_flag {
                    // pc has actually already moved so no need to +1
                    self.subroutine_buffer.store(self.program_counter.load());
                } else {
                    self.page_address.store(self.page_buffer);
                }

                self.program_counter.store(immediate)
            }
            Instruction::LPB { immediate } => self.page_buffer = immediate,
            Instruction::SSF => self.status_flag = true,
            Instruction::RSF => self.status_flag = false,
            Instruction::SSJ => self.subroutine_jump_flag = true,
            Instruction::RSJ => self.subroutine_jump_flag = false,
            Instruction::RET => self.program_counter.store(self.subroutine_buffer.load()),
            _ => panic!("Instruction not implemented."),
        }
    }

    fn get_register(&mut self, id: U<REGISTER_INDEX_BITS>) -> &mut Register<WORKING_BITS> {
        let id_u8: u8 = id.into();
        match id_u8 {
            0 => self.alu.accumulator_mut(),
            1 => self.x_register.borrow_mut(),
            2 => self.y_register.borrow_mut(),
            3 => self.z_register.borrow_mut(),
            _ => panic!("Max register id exceeded."),
        }
    }

    pub fn get_port(&mut self, id: U<PORT_INDEX_BITS>) -> &mut DevicePort<PORT_BITS> {
        let id_u8: u8 = id.into();
        &mut self.ports[id_u8 as usize]
    }

    pub fn get_pin(&mut self, id: U<PIN_INDEX_BITS>) -> &mut DevicePin {
        let id_u8: u8 = id.into();
        &mut self.pins[id_u8 as usize]
    }

    fn decode_xy(&self) -> U<{ 2 * WORKING_BITS }> {
        let x = self.x_register.load().change_bits();
        let y = self.y_register.load().change_bits();

        let x_shifted = x << WORKING_BITS;
        x_shifted | y
    }
}
