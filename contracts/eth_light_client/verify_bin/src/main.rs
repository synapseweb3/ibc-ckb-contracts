#![no_std]
#![no_main]
#![feature(asm_sym)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

macro_rules! debug {
    ($fmt:literal $(,$args:expr)* $(,)?) => {
        #[cfg(feature = "debugging")]
        ckb_std::syscalls::debug(alloc::format!($fmt $(,$args)*));
    };
}

mod entry;
mod error;

use ckb_std::default_alloc;
use core::arch::asm;

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry(argc: u64, argv: *const *const u8) -> i8 {
    match entry::main(argc, argv) {
        Ok(_) => 0,
        Err(err) => err.into(),
    }
}
