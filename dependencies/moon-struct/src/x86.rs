use crate::{RT_BIT_32, RT_BIT_64};

/// CPUID Feature information. eax=1
/** ECX Bit 0 - SSE3 - Supports SSE3 or not. */
pub const X86_CPUID_FEATURE_ECX_SSE3:u32 = RT_BIT_32!(0);
/** ECX Bit 1 - PCLMUL - PCLMULQDQ support (for AES-GCM). */
pub const X86_CPUID_FEATURE_ECX_PCLMUL:u32 = RT_BIT_32!(1);
/** ECX Bit 2 - DTES64 - DS Area 64-bit Layout. */
pub const X86_CPUID_FEATURE_ECX_DTES64:u32 = RT_BIT_32!(2);
/** ECX Bit 3 - MONITOR - Supports MONITOR/MWAIT. */
pub const X86_CPUID_FEATURE_ECX_MONITOR:u32 = RT_BIT_32!(3);
/** ECX Bit 4 - CPL-DS - CPL Qualified Debug Store. */
pub const X86_CPUID_FEATURE_ECX_CPLDS:u32 = RT_BIT_32!(4);
/** ECX Bit 5 - VMX - Virtual Machine Technology. */
pub const X86_CPUID_FEATURE_ECX_VMX:u32 = RT_BIT_32!(5);
/** ECX Bit 6 - SMX - Safer Mode Extensions. */
pub const X86_CPUID_FEATURE_ECX_SMX:u32 = RT_BIT_32!(6);
/** ECX Bit 7 - EST - Enh. SpeedStep Tech. */
pub const X86_CPUID_FEATURE_ECX_EST:u32 = RT_BIT_32!(7);
/** ECX Bit 8 - TM2 - Terminal Monitor 2. */
pub const X86_CPUID_FEATURE_ECX_TM2:u32 = RT_BIT_32!(8);
/** ECX Bit 9 - SSSE3 - Supplemental Streaming SIMD Extensions 3. */
pub const X86_CPUID_FEATURE_ECX_SSSE3:u32 = RT_BIT_32!(9);
/** ECX Bit 10 - CNTX-ID - L1 Context ID. */
pub const X86_CPUID_FEATURE_ECX_CNTXID:u32 = RT_BIT_32!(10);
/** ECX Bit 11 - SDBG - Sillicon debug interface (IA32_DEBUG_INTERFACE MSR).
 * See figure 3-6 and table 3-10, in intel Vol. 2A. from 2015-01-01. */
pub const X86_CPUID_FEATURE_ECX_SDBG:u32 = RT_BIT_32!(11);
/** ECX Bit 12 - FMA. */
pub const X86_CPUID_FEATURE_ECX_FMA:u32 = RT_BIT_32!(12);
/** ECX Bit 13 - CX16 - CMPXCHG16B. */
pub const X86_CPUID_FEATURE_ECX_CX16:u32 = RT_BIT_32!(13);
/** ECX Bit 14 - xTPR Update Control. Processor supports changing IA32_MISC_ENABLES[bit 23]. */
pub const X86_CPUID_FEATURE_ECX_TPRUPDATE:u32 = RT_BIT_32!(14);
/** ECX Bit 15 - PDCM - Perf/Debug Capability MSR. */
pub const X86_CPUID_FEATURE_ECX_PDCM:u32 = RT_BIT_32!(15);
/** ECX Bit 17 - PCID - Process-context identifiers. */
pub const X86_CPUID_FEATURE_ECX_PCID:u32 = RT_BIT_32!(17);
/** ECX Bit 18 - DCA - Direct Cache Access. */
pub const X86_CPUID_FEATURE_ECX_DCA:u32 = RT_BIT_32!(18);
/** ECX Bit 19 - SSE4_1 - Supports SSE4_1 or not. */
pub const X86_CPUID_FEATURE_ECX_SSE4_1:u32 = RT_BIT_32!(19);
/** ECX Bit 20 - SSE4_2 - Supports SSE4_2 or not. */
pub const X86_CPUID_FEATURE_ECX_SSE4_2:u32 = RT_BIT_32!(20);
/** ECX Bit 21 - x2APIC support. */
pub const X86_CPUID_FEATURE_ECX_X2APIC:u32 = RT_BIT_32!(21);
/** ECX Bit 22 - MOVBE instruction. */
pub const X86_CPUID_FEATURE_ECX_MOVBE:u32 = RT_BIT_32!(22);
/** ECX Bit 23 - POPCNT instruction. */
pub const X86_CPUID_FEATURE_ECX_POPCNT:u32 = RT_BIT_32!(23);
/** ECX Bir 24 - TSC-Deadline. */
pub const X86_CPUID_FEATURE_ECX_TSCDEADL:u32 = RT_BIT_32!(24);
/** ECX Bit 25 - AES instructions. */
pub const X86_CPUID_FEATURE_ECX_AES:u32 = RT_BIT_32!(25);
/** ECX Bit 26 - XSAVE instruction. */
pub const X86_CPUID_FEATURE_ECX_XSAVE:u32 = RT_BIT_32!(26);
/** ECX Bit 27 - Copy of CR4.OSXSAVE. */
pub const X86_CPUID_FEATURE_ECX_OSXSAVE:u32 = RT_BIT_32!(27);
/** ECX Bit 28 - AVX. */
pub const X86_CPUID_FEATURE_ECX_AVX:u32 = RT_BIT_32!(28);
/** ECX Bit 29 - F16C - Half-precision convert instruction support. */
pub const X86_CPUID_FEATURE_ECX_F16C:u32 = RT_BIT_32!(29);
/** ECX Bit 30 - RDRAND instruction. */
pub const X86_CPUID_FEATURE_ECX_RDRAND:u32 = RT_BIT_32!(30);
/** ECX Bit 31 - Hypervisor Present (software only). */
pub const X86_CPUID_FEATURE_ECX_HVP:u32 = RT_BIT_32!(31);

/// Cr0
/** Bit 0 - PE - Protection Enabled */
pub const X86_CR0_PE:u32 = RT_BIT_32!(0);
pub const X86_CR0_PROTECTION_ENABLE:u32 = RT_BIT_32!(0);
/** Bit 1 - MP - Monitor Coprocessor */
pub const X86_CR0_MP:u32 = RT_BIT_32!(1);
pub const X86_CR0_MONITOR_COPROCESSOR:u32 = RT_BIT_32!(1);
/** Bit 2 - EM - Emulation. */
pub const X86_CR0_EM:u32 = RT_BIT_32!(2);
pub const X86_CR0_EMULATE_FPU:u32 = RT_BIT_32!(2);
/** Bit 3 - TS - Task Switch. */
pub const X86_CR0_TS:u32 = RT_BIT_32!(3);
pub const X86_CR0_TASK_SWITCH:u32 = RT_BIT_32!(3);
/** Bit 4 - ET - Extension flag. (386, 'hardcoded' to 1 on 486+) */
pub const X86_CR0_ET:u32 = RT_BIT_32!(4);
pub const X86_CR0_EXTENSION_TYPE:u32 = RT_BIT_32!(4);
/** Bit 5 - NE - Numeric error (486+). */
pub const X86_CR0_NE:u32 = RT_BIT_32!(5);
pub const X86_CR0_NUMERIC_ERROR:u32 = RT_BIT_32!(5);
/** Bit 16 - WP - Write Protect (486+). */
pub const X86_CR0_WP:u32 = RT_BIT_32!(16);
pub const X86_CR0_WRITE_PROTECT:u32 = RT_BIT_32!(16);
/** Bit 18 - AM - Alignment Mask (486+). */
pub const X86_CR0_AM:u32 = RT_BIT_32!(18);
pub const X86_CR0_ALIGMENT_MASK:u32 = RT_BIT_32!(18);
/** Bit 29 - NW - Not Write-though (486+). */
pub const X86_CR0_NW:u32 = RT_BIT_32!(29);
pub const X86_CR0_NOT_WRITE_THROUGH:u32 = RT_BIT_32!(29);
/** Bit 30 - WP - Cache Disable (486+). */
pub const X86_CR0_CD:u32 = RT_BIT_32!(30);
pub const X86_CR0_CACHE_DISABLE:u32 = RT_BIT_32!(30);
/** Bit 31 - PG - Paging. */
pub const X86_CR0_PG:u32 = RT_BIT_32!(31);
pub const X86_CR0_PAGING:u32 = RT_BIT_32!(31);
pub const X86_CR0_BIT_PG:u32 = 31; /**< Bit number of X86_CR0_PG */


/// CR3
/** Bit 3 - PWT - Page-level Writes Transparent. */
pub const X86_CR3_PWT:u32 = RT_BIT_32!(3);
/** Bit 4 - PCD - Page-level Cache Disable. */
pub const X86_CR3_PCD:u32 = RT_BIT_32!(4);
/** Bits 12-31 - - Page directory page number. */
pub const X86_CR3_PAGE_MASK:u32 = 0xfffff000;
/** Bits  5-31 - - PAE Page directory page number. */
pub const X86_CR3_PAE_PAGE_MASK:u32 = 0xffffffe0;
/** Bits 12-51 - - AMD64 PML4 page number.
 * @note This is a maxed out mask, the actual acceptable CR3 value can
 *       be lower depending on the PhysAddrSize from CPUID Fn8000_0008. */
pub const X86_CR3_AMD64_PAGE_MASK:u64 = 0x000ffffffffff000;
/** Bits 12-51 - - Intel EPT PML4 page number (EPTP).
 * @note This is a maxed out mask, the actual acceptable CR3/EPTP value can
 *       be lower depending on the PhysAddrSize from CPUID Fn8000_0008. */
 pub const X86_CR3_EPT_PAGE_MASK:u64 = 0x000ffffffffff000;


/// Cr4
/** Bit 0 - VME - Virtual-8086 Mode Extensions. */
pub const X86_CR4_VME:u32 = RT_BIT_32!(0);
/** Bit 1 - PVI - Protected-Mode Virtual Interrupts. */
pub const X86_CR4_PVI:u32 = RT_BIT_32!(1);
/** Bit 2 - TSD - Time Stamp Disable. */
pub const X86_CR4_TSD:u32 = RT_BIT_32!(2);
/** Bit 3 - DE - Debugging Extensions. */
pub const X86_CR4_DE:u32 = RT_BIT_32!(3);
/** Bit 4 - PSE - Page Size Extension. */
pub const X86_CR4_PSE:u32 = RT_BIT_32!(4);
/** Bit 5 - PAE - Physical Address Extension. */
pub const X86_CR4_PAE:u32 = RT_BIT_32!(5);
/** Bit 6 - MCE - Machine-Check Enable. */
pub const X86_CR4_MCE:u32 = RT_BIT_32!(6);
/** Bit 7 - PGE - Page Global Enable. */
pub const X86_CR4_PGE:u32 = RT_BIT_32!(7);
/** Bit 8 - PCE - Performance-Monitoring Counter Enable. */
pub const X86_CR4_PCE:u32 = RT_BIT_32!(8);
/** Bit 9 - OSFXSR - Operating System Support for FXSAVE and FXRSTORE instructions. */
pub const X86_CR4_OSFXSR:u32 = RT_BIT_32!(9);
/** Bit 10 - OSXMMEEXCPT - Operating System Support for Unmasked SIMD Floating-Point Exceptions. */
pub const X86_CR4_OSXMMEEXCPT:u32 = RT_BIT_32!(10);
/** Bit 11 - UMIP - User-Mode Instruction Prevention. */
pub const X86_CR4_UMIP:u32 = RT_BIT_32!(11);
/** Bit 13 - VMXE - VMX mode is enabled. */
pub const X86_CR4_VMXE:u32 = RT_BIT_32!(13);
/** Bit 14 - SMXE - Safer Mode Extensions Enabled. */
pub const X86_CR4_SMXE:u32 = RT_BIT_32!(14);
/** Bit 16 - FSGSBASE - Read/write FSGSBASE instructions Enable. */
pub const X86_CR4_FSGSBASE:u32 = RT_BIT_32!(16);
/** Bit 17 - PCIDE - Process-Context Identifiers Enabled. */
pub const X86_CR4_PCIDE:u32 = RT_BIT_32!(17);
/** Bit 18 - OSXSAVE - Operating System Support for XSAVE and processor
 * extended states. */
pub const X86_CR4_OSXSAVE:u32 = RT_BIT_32!(18);
/** Bit 20 - SMEP - Supervisor-mode Execution Prevention enabled. */
pub const X86_CR4_SMEP:u32 = RT_BIT_32!(20);
/** Bit 21 - SMAP - Supervisor-mode Access Prevention enabled. */
pub const X86_CR4_SMAP:u32 = RT_BIT_32!(21);
/** Bit 22 - PKE - Protection Key Enable. */
pub const X86_CR4_PKE:u32 = RT_BIT_32!(22);
/** Bit 23 - CET - Control-flow Enhancement Technology enabled. */
pub const X86_CR4_CET:u32 = RT_BIT_32!(23);



/// Dr6
/** Bit 0 - B0 - Breakpoint 0 condition detected. */
pub const X86_DR6_B0:u32 = RT_BIT_32!(0);
/** Bit 1 - B1 - Breakpoint 1 condition detected. */
pub const X86_DR6_B1:u32 = RT_BIT_32!(1);
/** Bit 2 - B2 - Breakpoint 2 condition detected. */
pub const X86_DR6_B2:u32 = RT_BIT_32!(2);
/** Bit 3 - B3 - Breakpoint 3 condition detected. */
pub const X86_DR6_B3:u32 = RT_BIT_32!(3);
/** Mask of all the Bx bits. */
pub const X86_DR6_B_MASK:u64 = 0x0000000f;
/** Bit 13 - BD - Debug register access detected. Corresponds to the X86_DR7_GD bit. */
pub const X86_DR6_BD:u32 = RT_BIT_32!(13);
/** Bit 14 - BS - Single step */
pub const X86_DR6_BS:u32 = RT_BIT_32!(14);
/** Bit 15 - BT - Task switch. (TSS T bit.) */
pub const X86_DR6_BT:u32 = RT_BIT_32!(15);
/** Bit 16 - RTM - Cleared if debug exception inside RTM (@sa X86_DR7_RTM). */
pub const X86_DR6_RTM:u32 = RT_BIT_32!(16);
/** Value of DR6 after powerup/reset. */
pub const  X86_DR6_INIT_VAL:u64 = 0xffff0ff0;
/** Bits which must be 1s in DR6. */
pub const X86_DR6_RA1_MASK:u64 = 0xffff0ff0;
/** Bits which must be 1s in DR6, when RTM is supported. */
pub const X86_DR6_RA1_MASK_RTM:u64 = 0xfffe0ff0;
/** Bits which must be 0s in DR6. */
pub const X86_DR6_RAZ_MASK:u64 = RT_BIT_64!(12);
/** Bits which must be 0s on writes to DR6. */
pub const X86_DR6_MBZ_MASK:u64 = 0xffffffff00000000;



/// Dr7
/** Bit 0 - L0 - Local breakpoint enable. Cleared on task switch. */
pub const X86_DR7_L0:u32 = RT_BIT_32!(0);
/** Bit 1 - G0 - Global breakpoint enable. Not cleared on task switch. */
pub const X86_DR7_G0:u32 = RT_BIT_32!(1);
/** Bit 2 - L1 - Local breakpoint enable. Cleared on task switch. */
pub const X86_DR7_L1:u32 = RT_BIT_32!(2);
/** Bit 3 - G1 - Global breakpoint enable. Not cleared on task switch. */
pub const X86_DR7_G1:u32 = RT_BIT_32!(3);
/** Bit 4 - L2 - Local breakpoint enable. Cleared on task switch. */
pub const X86_DR7_L2:u32 = RT_BIT_32!(4);
/** Bit 5 - G2 - Global breakpoint enable. Not cleared on task switch. */
pub const X86_DR7_G2:u32 = RT_BIT_32!(5);
/** Bit 6 - L3 - Local breakpoint enable. Cleared on task switch. */
pub const X86_DR7_L3:u32 = RT_BIT_32!(6);
/** Bit 7 - G3 - Global breakpoint enable. Not cleared on task switch. */
pub const X86_DR7_G3:u32 = RT_BIT_32!(7);
/** Bit 8 - LE - Local breakpoint exact. (Not supported (read ignored) by P6 and later.) */
pub const X86_DR7_LE:u32 = RT_BIT_32!(8);
/** Bit 9 - GE - Global breakpoint exact. (Not supported (read ignored) by P6 and later.) */
pub const X86_DR7_GE:u32 = RT_BIT_32!(9);


