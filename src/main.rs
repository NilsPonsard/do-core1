use clap::Parser;
use do_core::instruction::{Instruction, OpCode};
use do_core::{Error, MAX_REGISTER_INDEX};

#[derive(Parser)]
#[clap(version, author)]
struct DoCoreOpts {
    /// DO Core instruction
    #[clap(short, long)]
    insn: String,
}

fn add(op0: u32, op1: u32) -> Result<u32, Error> {
    op0.checked_add(op1)
        .ok_or(Error::AdditionOverflow(op0, op1))
}

fn xor(op0: u32, op1: u32) -> u32 {
    op0 ^ op1
}

fn dump_cpu_state(preamble: &str, registers: &[u32; MAX_REGISTER_INDEX as usize + 1]) {
    println!("do-core1: {}:", preamble);
    for (index, register) in registers.iter().enumerate() {
        println!("\tR{}: {:#x?}", index, *register);
    }
}

fn main() -> Result<(), Error> {
    let opts: DoCoreOpts = DoCoreOpts::parse();
    let insn = u32::from_str_radix(opts.insn.trim_start_matches("0x"), 16).unwrap();
    let mut registers = [0u32; MAX_REGISTER_INDEX as usize + 1];
    // Arbitrary initial registers value.
    // Registers will eventually be initialized through memory loads.
    for (index, register) in registers.iter_mut().enumerate() {
        *register = index as u32 * 0x10;
    }

    dump_cpu_state("Initial CPU state", &registers);

    let decoded_instruction = Instruction::disassemble(insn)?;
    println!(
        "do-core-1: instruction decoded into {:?}",
        decoded_instruction
    );
    let op0 = decoded_instruction.op0 as usize;
    let op1 = decoded_instruction.op1 as usize;

    match decoded_instruction.opcode {
        OpCode::ADD => registers[op0] = add(registers[op0], registers[op1])?,
        OpCode::XOR => registers[op0] = xor(registers[op0], registers[op1]),

        _ => panic!("Unknown opcode {:?}", decoded_instruction.opcode),
    }

    dump_cpu_state("Final CPU state", &registers);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Error, Instruction, OpCode};

    #[test]
    fn test_instruction_disassemble_add_r1_r3() -> Result<(), Error> {
        let insn_bytes: u32 = 0x1842;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 1);
        assert_eq!(insn.op1, 3);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r9_r1() -> Result<(), Error> {
        let insn_bytes: u32 = 0x5002;
        assert!(Instruction::disassemble(insn_bytes).is_err());

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r0_r10() -> Result<(), Error> {
        let insn_bytes: u32 = 0x20a;
        assert!(Instruction::disassemble(insn_bytes).is_err());

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_add_r7_r2() -> Result<(), Error> {
        let insn_bytes: u32 = 0x11c2;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::ADD);
        assert_eq!(insn.op0, 7);
        assert_eq!(insn.op1, 2);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_ldw_r0_r1() -> Result<(), Error> {
        let insn_bytes: u32 = 0x0800;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::LDW);
        assert_eq!(insn.op0, 0);
        assert_eq!(insn.op1, 1);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_xor_r2_r3() -> Result<(), Error> {
        let insn_bytes: u32 = 0x1883;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::XOR);
        assert_eq!(insn.op0, 2);
        assert_eq!(insn.op1, 3);

        Ok(())
    }

    #[test]
    fn test_instruction_disassemble_stw_r5_r0() -> Result<(), Error> {
        let insn_bytes: u32 = 0x0141;
        let insn = Instruction::disassemble(insn_bytes)?;

        assert_eq!(insn.opcode, OpCode::STW);
        assert_eq!(insn.op0, 5);
        assert_eq!(insn.op1, 0);

        Ok(())
    }
}
