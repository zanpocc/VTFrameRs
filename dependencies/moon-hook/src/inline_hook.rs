use core::{arch::asm, ffi::c_void};

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};

use iced_x86::{
    Code, Decoder, DecoderOptions, FlowControl, Instruction, OpKind,
};
use wdk::println;
use wdk_sys::ntddk::memcpy;

pub struct InlineHook {
    patch_header: Vec<u8>, // origin function header
    old_func_header: Vec<u8>,
    ori_func_header: Vec<u8>, // origin function header and jmp to origin function
    new_func_point: *mut c_void,
    ori_func_point: *mut c_void,
}

pub fn inline_hook(rip: *mut c_void,new_func: *mut c_void) -> Result<Box<InlineHook>, &'static str> {
    unsafe{
        asm!{
            "int 3"
        }
    }

    let mut code_array = Box::new([0u8;30]);
    let raw_ptr = Box::into_raw(code_array);

    unsafe { memcpy(raw_ptr as _, rip, 30); }

    unsafe {
        code_array =  Box::from_raw(raw_ptr); 
    }
    
    let example_code = code_array.to_vec();

    let mut decoder = Decoder::with_ip(
        64,
        &example_code,
        rip as _,
        DecoderOptions::NONE,
    );

    // In 64-bit mode, we need 12 bytes to jump to any address:
    //      mov rax,imm64   // 10
    //      jmp rax         // 2
    // We overwrite rax because it's probably not used by the called function.
    // In 32-bit mode, a normal JMP is just 5 bytes
    let required_bytes = 10 + 2;
    let mut total_bytes = 0;
    let mut orig_instructions: Vec<Instruction> = Vec::new();
    for instr in &mut decoder {
        orig_instructions.push(instr);
        total_bytes += instr.len() as u32;
        if instr.is_invalid() {
            return Err("Found garbage");
        }
        if total_bytes >= required_bytes {
            break;
        }

        match instr.flow_control() {
            FlowControl::Next => {}

            FlowControl::UnconditionalBranch => {
                if instr.op0_kind() == OpKind::NearBranch64 {
                    let _target = instr.near_branch_target();
                    // todo,maybe need
                }
            }

            FlowControl::IndirectBranch
            | FlowControl::ConditionalBranch
            | FlowControl::Return
            | FlowControl::Call
            | FlowControl::IndirectCall
            | FlowControl::Interrupt
            | FlowControl::XbeginXabortXend
            | FlowControl::Exception => { 
                // todo,maybe need
            }
        }
    }

    if total_bytes < required_bytes {
        return Err("Not enough bytes!");
    }
    
    if orig_instructions.is_empty(){
        return Err("Empty Origin Instructions");
    }

    // Create a JMP instruction that branches to the original code, except those instructions
    // that we'll re-encode. We don't need to do it if it already ends in 'ret'
    let (jmp_back_addr, add) = {
        let last_instr = orig_instructions.last().unwrap();
        if last_instr.flow_control() != FlowControl::Return {
            (last_instr.next_ip(), true)
        } else {
            (last_instr.next_ip(), false)
        }
    };

    if add {
        match Instruction::with_branch(Code::Jmp_rel32_64, jmp_back_addr){
            Ok(new_branch)=>{
                orig_instructions.push(new_branch);
            }
            Err(_) =>{
                return Err("New JmpBack Branch Error");
            }
        }
    }

    // todo: maybe need
    // // Relocate the code to some new location. It can fix short/near branches and
    // // convert them to short/near/long forms if needed. This also works even if it's a
    // // jrcxz/loop/loopcc instruction which only have short forms.
    // //
    // // It can currently only fix RIP relative operands if the new location is within 2GB
    // // of the target data location.
    // //
    // // Note that a block is not the same thing as a basic block. A block can contain any
    // // number of instructions, including any number of branch instructions. One block
    // // should be enough unless you must relocate different blocks to different locations.
    // let relocated_base_address = rip as u64 + 0x20_0000;
    // let block = InstructionBlock::new(&orig_instructions, relocated_base_address);
    // // This method can also encode more than one block but that's rarely needed, see above comment.
    // let result = match BlockEncoder::encode(decoder.bitness(), block, BlockEncoderOptions::NONE) {
    //     Err(err) => panic!("{}", err),
    //     Ok(result) => result,
    // };
    // let new_code = result.code_buffer;

    // // Patch the original code. Pretend that we use some OS API to write to memory...
    // // We could use the BlockEncoder/Encoder for this but it's easy to do yourself too.
    // // This is 'mov rax,imm64; jmp rax'
    // let mut example_code = example_code.to_vec();
    // example_code[0] = 0x48; // \ 'MOV RAX,imm64'
    // example_code[1] = 0xB8; // /
    // let mut v = new_func as u64;
    // for p in &mut example_code[2..10] {
    //     *p = v as u8;
    //     v >>= 8;
    // }
    // example_code[10] = 0xFF; // \ JMP RAX
    // example_code[11] = 0xE0; // /

    // // Disassemble it
    // println!("Original + patched code:");
    // disassemble(&example_code, rip as _);

    // // Disassemble the moved code
    // println!("Moved code:");
    // disassemble(&new_code, relocated_base_address);

    let r = Box::new(InlineHook {
        patch_header: example_code.clone(),
        old_func_header: example_code.clone(),
        ori_func_header: example_code.clone(),
        new_func_point: new_func,
        ori_func_point: rip,
    });
    
	Ok(r)
}