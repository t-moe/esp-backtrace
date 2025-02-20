use crate::MAX_BACKTRACE_ADRESSES;
use core::arch::asm;

#[doc(hidden)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum ExceptionCause {
    /// Illegal Instruction
    IllegalInstruction = 0,
    /// System Call (Syscall Instruction)
    Syscall = 1,
    /// Instruction Fetch Error
    InstrFetchError = 2,
    /// Load Store Error
    LoadStoreError = 3,
    /// Level 1 Interrupt
    LevelOneInterrupt = 4,
    /// Stack Extension Assist (movsp Instruction) For Alloca
    Alloca = 5,
    /// Integer Divide By Zero
    DivideByZero = 6,
    /// Use Of Failed Speculative Access (Not Implemented)
    NextPCValueIllegal = 7,
    /// Privileged Instruction
    PrivilegedInstruction = 8,
    /// Unaligned Load Or Store
    UnalignedLoadOrStore = 9,
    /// Reserved
    ExternalRegisterPrivilegeError = 10,
    /// Reserved
    ExclusiveError = 11,
    /// Pif Data Error On Instruction Fetch (Rb-200x And Later)
    InstrDataError = 12,
    /// Pif Data Error On Load Or Store (Rb-200x And Later)
    LoadStoreDataError = 13,
    /// Pif Address Error On Instruction Fetch (Rb-200x And Later)
    InstrAddrError = 14,
    /// Pif Address Error On Load Or Store (Rb-200x And Later)
    LoadStoreAddrError = 15,
    /// Itlb Miss (No Itlb Entry Matches, Hw Refill Also Missed)
    ItlbMiss = 16,
    /// Itlb Multihit (Multiple Itlb Entries Match)
    ItlbMultiHit = 17,
    /// Ring Privilege Violation On Instruction Fetch
    InstrRing = 18,
    /// Size Restriction On Ifetch (Not Implemented)
    Reserved19 = 19,
    /// Cache Attribute Does Not Allow Instruction Fetch
    InstrProhibited = 20,
    /// Reserved
    Reserved21 = 21,
    /// Reserved
    Reserved22 = 22,
    /// Reserved
    Reserved23 = 23,
    /// Dtlb Miss (No Dtlb Entry Matches, Hw Refill Also Missed)
    DtlbMiss = 24,
    /// Dtlb Multihit (Multiple Dtlb Entries Match)
    DtlbMultiHit = 25,
    /// Ring Privilege Violation On Load Or Store
    LoadStoreRing = 26,
    /// Size Restriction On Load/Store (Not Implemented)
    Reserved27 = 27,
    /// Cache Attribute Does Not Allow Load
    LoadProhibited = 28,
    /// Cache Attribute Does Not Allow Store
    StoreProhibited = 29,
    /// Reserved
    Reserved30 = 30,
    /// Reserved
    Reserved31 = 31,
    /// Access To Coprocessor 0 When Disabled
    Cp0Disabled = 32,
    /// Access To Coprocessor 1 When Disabled
    Cp1Disabled = 33,
    /// Access To Coprocessor 2 When Disabled
    Cp2Disabled = 34,
    /// Access To Coprocessor 3 When Disabled
    Cp3Disabled = 35,
    /// Access To Coprocessor 4 When Disabled
    Cp4Disabled = 36,
    /// Access To Coprocessor 5 When Disabled
    Cp5Disabled = 37,
    /// Access To Coprocessor 6 When Disabled
    Cp6Disabled = 38,
    /// Access To Coprocessor 7 When Disabled
    Cp7Disabled = 39,

    None = 255,
}

#[doc(hidden)]
#[allow(missing_docs, non_snake_case)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Context {
    pub PC: u32,
    pub PS: u32,
    pub A0: u32,
    pub A1: u32,
    pub A2: u32,
    pub A3: u32,
    pub A4: u32,
    pub A5: u32,
    pub A6: u32,
    pub A7: u32,
    pub A8: u32,
    pub A9: u32,
    pub A10: u32,
    pub A11: u32,
    pub A12: u32,
    pub A13: u32,
    pub A14: u32,
    pub A15: u32,
    pub SAR: u32,
    pub EXCCAUSE: u32,
    pub EXCVADDR: u32,
    pub LBEG: u32,
    pub LEND: u32,
    pub LCOUNT: u32,
    pub THREADPTR: u32,
    pub SCOMPARE1: u32,
    pub BR: u32,
    pub ACCLO: u32,
    pub ACCHI: u32,
    pub M0: u32,
    pub M1: u32,
    pub M2: u32,
    pub M3: u32,
    pub F64R_LO: u32,
    pub F64R_HI: u32,
    pub F64S: u32,
    pub FCR: u32,
    pub FSR: u32,
    pub F0: u32,
    pub F1: u32,
    pub F2: u32,
    pub F3: u32,
    pub F4: u32,
    pub F5: u32,
    pub F6: u32,
    pub F7: u32,
    pub F8: u32,
    pub F9: u32,
    pub F10: u32,
    pub F11: u32,
    pub F12: u32,
    pub F13: u32,
    pub F14: u32,
    pub F15: u32,
}

impl core::fmt::Debug for Context {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            fmt,
            "Context
PC=0x{:08x}       PS=0x{:08x}
A0=0x{:08x}       A1=0x{:08x}       A2=0x{:08x}       A3=0x{:08x}       A4=0x{:08x}
A5=0x{:08x}       A6=0x{:08x}       A7=0x{:08x}       A8=0x{:08x}       A9=0x{:08x}
A10=0x{:08x}      A11=0x{:08x}      A12=0x{:08x}      A13=0x{:08x}      A14=0x{:08x}
A15=0x{:08x}
SAR={:08x}
EXCCAUSE=0x{:08x} EXCVADDR=0x{:08x}
LBEG=0x{:08x}     LEND=0x{:08x}     LCOUNT=0x{:08x}
THREADPTR=0x{:08x}
SCOMPARE1=0x{:08x}
BR=0x{:08x}
ACCLO=0x{:08x}    ACCHI=0x{:08x}
M0=0x{:08x}       M1=0x{:08x}       M2=0x{:08x}       M3=0x{:08x}
F64R_LO=0x{:08x}  F64R_HI=0x{:08x}  F64S=0x{:08x}
FCR=0x{:08x}      FSR=0x{:08x}
F0=0x{:08x}       F1=0x{:08x}       F2=0x{:08x}       F3=0x{:08x}       F4=0x{:08x}
F5=0x{:08x}       F6=0x{:08x}       F7=0x{:08x}       F8=0x{:08x}       F9=0x{:08x}
F10=0x{:08x}      F11=0x{:08x}      F12=0x{:08x}      F13=0x{:08x}      F14=0x{:08x}
F15=0x{:08x}
",
            self.PC,
            self.PS,
            self.A0,
            self.A1,
            self.A2,
            self.A3,
            self.A4,
            self.A5,
            self.A6,
            self.A7,
            self.A8,
            self.A9,
            self.A10,
            self.A11,
            self.A12,
            self.A13,
            self.A14,
            self.A15,
            self.SAR,
            self.EXCCAUSE,
            self.EXCVADDR,
            self.LBEG,
            self.LEND,
            self.LCOUNT,
            self.THREADPTR,
            self.SCOMPARE1,
            self.BR,
            self.ACCLO,
            self.ACCHI,
            self.M0,
            self.M1,
            self.M2,
            self.M3,
            self.F64R_LO,
            self.F64R_HI,
            self.F64S,
            self.FCR,
            self.FSR,
            self.F0,
            self.F1,
            self.F2,
            self.F3,
            self.F4,
            self.F5,
            self.F6,
            self.F7,
            self.F8,
            self.F9,
            self.F10,
            self.F11,
            self.F12,
            self.F13,
            self.F14,
            self.F15,
        )
    }
}

/// Get an array of backtrace addresses.
///
pub fn backtrace() -> [Option<usize>; MAX_BACKTRACE_ADRESSES] {
    let sp = unsafe {
        let mut _tmp: u32;
        asm!("mov {0}, a1", out(reg) _tmp);
        _tmp
    };

    backtrace_internal(sp, 1)
}

pub(crate) fn sanitize_address(address: u32) -> u32 {
    (address & 0x3fff_ffff) | 0x4000_0000
}

pub(crate) fn backtrace_internal(
    sp: u32,
    suppress: i32,
) -> [Option<usize>; MAX_BACKTRACE_ADRESSES] {
    let mut result = [None; 10];
    let mut index = 0;

    let mut fp = sp;
    let mut suppress = suppress;
    let mut old_address = 0;

    loop {
        unsafe {
            let address = sanitize_address((fp as *const u32).offset(-4).read_volatile()); // RA/PC
            fp = (fp as *const u32).offset(-3).read_volatile(); // next FP

            if old_address == address {
                break;
            }

            old_address = address;

            if address == 0 {
                break;
            }

            if !crate::is_valid_ram_address(fp) {
                break;
            }

            if fp == 0 {
                break;
            }

            if suppress == 0 {
                result[index] = Some(address as usize);
                index += 1;

                if index >= MAX_BACKTRACE_ADRESSES {
                    break;
                }
            } else {
                suppress -= 1;
            }
        }
    }

    result
}
