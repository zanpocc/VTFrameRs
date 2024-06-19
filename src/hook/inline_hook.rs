use alloc::vec::Vec;
use moon_driver_utils::{memory::utils::{wpoff, wpon}, rwlock::ReadWriteLock};
use moon_log::info;
use wdk_sys::{ntddk::memcpy, ACCESS_MASK, NTSTATUS, PCLIENT_ID, PHANDLE64, POBJECT_ATTRIBUTES};

use crate::slib::distorm35::DistormAsmLength;

lazy_static!{
    pub static ref HOOK_LIST:ReadWriteLock<Option<HookFunction>> = ReadWriteLock::new(Some(HookFunction::new()));
}


pub type NtOpenProcessFn = unsafe extern "system" fn(
    ProcessHandle: PHANDLE64,
    DesiredAccess: ACCESS_MASK,
    ObjectAttributes: POBJECT_ATTRIBUTES,
    ClientId: PCLIENT_ID,
) -> NTSTATUS;

pub struct HookFunction {
    pub nt_open_process: Option<InlineHook>,
}

impl HookFunction {
    pub fn new() -> Self{
        Self { 
            nt_open_process: Option::None 
        }
    }
}

impl Drop for HookFunction {
    fn drop(&mut self) {
        info!("HookFunction Drop");
    }
}

#[repr(C)]
#[repr(packed)]
pub struct JumpThunk{
    push_op: u8, // 0x68
    address_low: u32,
    mpv_op: u32, // 0x42444c7
    address_high: u32,
    ret_op: u8, // 0xc3
}

impl Default for JumpThunk{
    fn default() -> Self {
        Self { push_op: 0x68u8, address_low: 0, mpv_op: 0x42444c7, address_high: 0, ret_op: 0xc3 }
    }
}

impl JumpThunk {
    pub fn as_ptr(&self) -> *const JumpThunk{
        self as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut JumpThunk{
        self as *mut _
    }
}

pub struct InlineHook {
    pub buffer: Vec<u8>,
    pub patch_header: *mut u8, // origin function header
    pub patch_size: u32,
    pub old_func_header: *mut u8, // use on recover
    pub new_ori_func_header: *mut u8, // origin function header and jmp to origin function
    pub new_func_point: *mut u8,
    pub ori_func_point: *mut u8,
    pub hooked: bool,
}

unsafe impl Sync for InlineHook {}
unsafe impl Send for InlineHook {}

impl Drop for InlineHook {
    fn drop(&mut self) {
        self.unhook();
        info!("InlineHook Drop");
    }
}

fn init_jump_thunk(target_address: u64) -> JumpThunk {
    let mut r = JumpThunk::default();
    r.address_low = target_address as u32;
    r.address_high = (target_address >> 32) as _;
    r
}

impl InlineHook {
    pub fn inline_hook(rip: *mut u8,new_func: *mut u8) -> Result<InlineHook, &'static str> {
        let jmp_thunk_size = core::mem::size_of::<JumpThunk>() as u32;
        let patch_size = unsafe { DistormAsmLength(rip as _,jmp_thunk_size as _) };
        if patch_size < jmp_thunk_size as _ {
            return Err("Error to DistormAsmLength");
        }
    
         // alloc all
         let total_size = (patch_size * 3 + jmp_thunk_size) as usize;
         let mut buffer = alloc::vec![0u8; total_size];
     
         // split
         let patch_header = buffer.as_mut_ptr();
         let old_func_header = (patch_header as u64 + patch_size as u64) as *mut u8;
         let new_ori_func_header = (old_func_header as u64 + patch_size as u64) as *mut u8;
         let jmp_back = (new_ori_func_header as u64 + patch_size as u64) as *mut u8;
        
        let mut jmp_back_thunk = init_jump_thunk(rip as u64 + patch_size as u64);
        let mut patch_jmp_thunk = init_jump_thunk(new_func as u64);
    
        unsafe { 
            memcpy(patch_header as _, patch_jmp_thunk.as_mut_ptr() as _, jmp_thunk_size as _);
            memcpy(old_func_header as _, rip as _, patch_size as _);
            // todo: 根据指令判断是否需要修复一些相对地址，比如lea rax,[0x123456]
            memcpy(new_ori_func_header as _, rip as _, patch_size as _);
            memcpy(jmp_back as _, jmp_back_thunk.as_mut_ptr() as _, jmp_thunk_size as _);
        }
    
        let r = InlineHook {
            buffer: buffer,
            patch_header: patch_header,
            patch_size: patch_size,
            old_func_header: old_func_header,
            new_ori_func_header: new_ori_func_header,
            new_func_point: new_func,
            ori_func_point: rip,
            hooked: false,
        };
    
        Ok(r)
    }

    pub fn hook(&mut self) {
        unsafe {
            let irql = wpoff();
            memcpy(self.ori_func_point as _,self.patch_header as _, self.patch_size as _);
            wpon(irql);
        };
            
        self.hooked = true;
    }

    pub fn unhook(&mut self) {
        if !self.hooked {
            return;
        }

        unsafe {
            let irql = wpoff();
            memcpy(self.ori_func_point as _,self.old_func_header as _, self.patch_size as _);
            wpon(irql);
        };
            
        self.hooked = false;
    }
}

