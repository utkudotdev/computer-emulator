#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]

use crate::computer::Computer;
use crate::device::console::Console;
use crate::device::Device;
use common::architecture::PROGRAM_MEMORY_SIZE;
use device::connectable::splitter::Splitter;
use device::connectable::Connectable;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

mod computer;
mod device;

fn load_program_from_file(path: &Path) -> Result<[u8; PROGRAM_MEMORY_SIZE], io::Error> {
    let mut f = File::open(path)?;
    let mut buf = [0_u8; PROGRAM_MEMORY_SIZE];
    f.read_exact(&mut buf)?;
    Ok(buf)
}

fn main() {
    let program = load_program_from_file(Path::new("./programs/hello_world.out")).unwrap();
    let mut computer = Computer::with_program(program.map(|e| (e as u8).into()));

    let mut console = Console::new();

    let mut splitter = Splitter::<4, 4>::new();
    let computer_port1 = computer.get_port(0u8.into());
    computer_port1.connect_to(&splitter.as_low_end());

    let computer_port2 = computer.get_port(1u8.into());
    computer_port2.connect_to(&splitter.as_high_end());

    let ascii_port = console.ascii_port();
    ascii_port.connect_to(&splitter);

    let computer_pin = computer.get_pin(0u8.into());
    let write_pin = console.write_pin();
    write_pin.connect_to(computer_pin);

    run_simulation(
        vec![Box::new(computer), Box::new(splitter), Box::new(console)],
        None,
    );
}

fn run_simulation(mut devices: Vec<Box<dyn Device>>, ticks: Option<u32>) {
    let mut tick: u32 = 0;

    loop {
        if ticks.map_or(false, |t| tick >= t) {
            break;
        }

        for device in devices.iter_mut() {
            device.tick(tick)
        }
        tick += 1;
    }
}
