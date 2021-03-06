// SPDX-FileCopyrightText: 2020 Sean Cross <sean@xobs.io>
// SPDX-License-Identifier: Apache-2.0

use riscv::register::{satp, sie, sstatus};
use xous_kernel::PID;

pub mod exception;
pub mod irq;
pub mod mem;
pub mod process;
pub mod rand;
pub mod syscall;

pub use process::Thread;

pub fn current_pid() -> PID {
    PID::new(satp::read().asid() as _).unwrap()
}

pub fn init() {
    unsafe {
        sstatus::set_sie();
        sie::set_ssoft();
        sie::set_sext();
    }
    rand::init();
}

/// Put the core to sleep until an interrupt hits. Returns `true`
/// to indicate the kernel should not exit.
pub fn idle() -> bool {
    unsafe { riscv::asm::wfi() };
    true
}
