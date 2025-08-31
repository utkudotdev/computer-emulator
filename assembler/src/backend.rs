use std::collections::HashMap;

use common::{
    architecture::PC_BITS,
    instruction::{encode_instruction, Instruction},
    un::U,
};

use crate::{
    lexer::{InstructionKind, LabelPropertyKind, Register},
    parser::Node,
};

// TODO: add better error information
#[derive(Debug)]
pub enum CompilationErrorKind {
    UnexpectedNodeKind,
    BadInstructionArguments,
    BadLabelReference,
    DuplicateLabel,
}

#[derive(Debug)]
pub struct CompilationError {
    kind: CompilationErrorKind,
    block_index: usize,
}

pub fn compile<'a>(block: &Vec<Node<'a>>) -> Result<Vec<u8>, Vec<CompilationError>> {
    let mut result = Vec::new();
    let mut errors = Vec::new();

    let label_map = build_label_map(block)?;

    for (block_index, node) in block.iter().enumerate() {
        match node {
            Node::Instruction { kind, arguments } => {
                match compile_instruction(*kind, arguments, block_index, &label_map) {
                    Ok(inst) => result.push(inst),
                    Err(err) => errors.push(err),
                }
            }
            Node::Label { .. } => {} // labels are fine at top level but no need to emit anything
            _ => {
                errors.push(CompilationError {
                    kind: CompilationErrorKind::UnexpectedNodeKind,
                    block_index,
                });
            }
        }
    }

    println!("{:?}", result);

    if errors.is_empty() {
        Ok(result.into_iter().map(encode_instruction).collect())
    } else {
        Err(errors)
    }
}

fn build_label_map<'a>(
    nodes: &Vec<Node<'a>>,
) -> Result<HashMap<&'a str, (u32, u32)>, Vec<CompilationError>> {
    let mut map = HashMap::new();
    let mut errors = vec![];
    let mut instruction_count = 0;

    let instructions_per_page = 2u32.pow(PC_BITS as u32);

    for (block_index, node) in nodes.iter().enumerate() {
        match node {
            Node::Label { name } => {
                match map.try_insert(
                    *name,
                    (
                        instruction_count % instructions_per_page,
                        instruction_count / instructions_per_page,
                    ),
                ) {
                    Ok(_) => {}
                    Err(_) => errors.push(CompilationError {
                        kind: CompilationErrorKind::DuplicateLabel,
                        block_index: block_index,
                    }),
                }
            }
            Node::Instruction { .. } => instruction_count += 1,
            _ => errors.push(CompilationError {
                kind: CompilationErrorKind::UnexpectedNodeKind,
                block_index,
            }),
        }
    }

    if errors.is_empty() {
        Ok(map)
    } else {
        Err(errors)
    }
}

// TODO: immediates are not validated at all, just truncated
fn compile_instruction<'a>(
    kind: InstructionKind,
    arguments: &Vec<Node>,
    block_index: usize,
    label_map: &HashMap<&'a str, (u32, u32)>,
) -> Result<Instruction, CompilationError> {
    macro_rules! bad_args {
        () => {
            Err(CompilationError {
                kind: CompilationErrorKind::BadInstructionArguments,
                block_index,
            })
        };
    }

    match kind {
        InstructionKind::NOP => match arguments[..] {
            [] => Ok(Instruction::NOP),
            _ => bad_args!(),
        },
        InstructionKind::STR => match arguments[..] {
            [Node::RegisterLiteral { register }] => Ok(Instruction::STR {
                register_id: register_to_id(register),
            }),
            _ => bad_args!(),
        },
        InstructionKind::LOD => match arguments[..] {
            [Node::RegisterLiteral { register }] => Ok(Instruction::LOD {
                register_id: register_to_id(register),
            }),
            _ => bad_args!(),
        },
        InstructionKind::LDI => match arguments[..] {
            [Node::RegisterLiteral { register }, Node::NumberLiteral { value }] => {
                Ok(Instruction::LDI {
                    register_id: register_to_id(register),
                    immediate: U::from(value),
                })
            }
            _ => bad_args!(),
        },
        InstructionKind::INC => todo!(),
        InstructionKind::DEC => todo!(),
        InstructionKind::MOV => match arguments[..] {
            [Node::RegisterLiteral { register: r1 }, Node::RegisterLiteral { register: r2 }] => {
                Ok(Instruction::MOV {
                    register_from_id: register_to_id(r2),
                    register_to_id: register_to_id(r1),
                })
            }
            _ => bad_args!(),
        },
        InstructionKind::INP => todo!(),
        InstructionKind::OUT => match arguments[..] {
            [Node::NumberLiteral { value: port }] => Ok(Instruction::OUT {
                port_id: U::from(port),
            }),
            _ => bad_args!(),
        },
        InstructionKind::SEP => match arguments[..] {
            [Node::NumberLiteral { value: pin }] => Ok(Instruction::SEP {
                pin_id: U::from(pin),
            }),
            _ => bad_args!(),
        },
        InstructionKind::RSP => match arguments[..] {
            [Node::NumberLiteral { value: pin }] => Ok(Instruction::RSP {
                pin_id: U::from(pin),
            }),
            _ => bad_args!(),
        },
        InstructionKind::ADD => todo!(),
        InstructionKind::SUB => todo!(),
        InstructionKind::BOR => todo!(),
        InstructionKind::AND => todo!(),
        InstructionKind::CMP => todo!(),
        InstructionKind::GRT => todo!(),
        InstructionKind::LES => todo!(),
        InstructionKind::BRN => match arguments[..] {
            [Node::LabelReference {
                label_name,
                reference_kind,
            }] => {
                // I'll support doing strange things like using the page number because why not
                let branch_address = match reference_kind {
                    LabelPropertyKind::Address => label_map.get(label_name).map(|t| t.0),
                    LabelPropertyKind::Page => label_map.get(label_name).map(|t| t.1),
                };
                match branch_address {
                    Some(addr) => Ok(Instruction::BRN {
                        immediate: U::from(addr),
                    }),
                    None => Err(CompilationError {
                        kind: CompilationErrorKind::BadLabelReference,
                        block_index,
                    }),
                }
            }
            _ => bad_args!(),
        },
        InstructionKind::SSJ => match arguments[..] {
            [] => Ok(Instruction::SSJ),
            _ => bad_args!(),
        },
        InstructionKind::RSJ => match arguments[..] {
            [] => Ok(Instruction::RSJ),
            _ => bad_args!(),
        },
        InstructionKind::RET => match arguments[..] {
            [] => Ok(Instruction::RET),
            _ => bad_args!(),
        },
        InstructionKind::SSF => match arguments[..] {
            [] => Ok(Instruction::SSF),
            _ => bad_args!(),
        },
        InstructionKind::RSF => match arguments[..] {
            [] => Ok(Instruction::RSF),
            _ => bad_args!(),
        },
    }
}

fn register_to_id(register: Register) -> U<2> {
    match register {
        Register::A => U::from(0u32),
        Register::X => U::from(1u32),
        Register::Y => U::from(2u32),
        Register::Z => U::from(3u32),
    }
}
