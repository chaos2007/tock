#![feature(asm, concat_idents, const_fn)]
#![feature(exclusive_range_pattern)]
#![no_std]
#![crate_name = "k210"]
#![crate_type = "rlib"]

mod interrupts;

pub mod chip;
pub mod gpio;
pub mod uart;
