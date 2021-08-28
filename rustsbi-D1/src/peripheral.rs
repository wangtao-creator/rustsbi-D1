use crate::hal;

use rustsbi::println;
use crate::hal::{Serial,msip, pac_encoding::UART0_BASE};
use riscv::register::mip;

pub fn init_peripheral() {


    rustsbi::legacy_stdio::init_legacy_stdio_embedded_hal(Serial::new(UART0_BASE));
    rustsbi::init_timer(Timer);
    rustsbi::init_reset(Reset);
    rustsbi::init_ipi(Ipi);
}

struct Ipi;

impl rustsbi::Ipi for Ipi {
    fn max_hart_id(&self) -> usize {
        1
    }
    fn send_ipi_many(&mut self, hart_mask: rustsbi::HartMask) -> rustsbi::SbiRet {
        for i in 0..=self.max_hart_id() {
            if hart_mask.has_bit(i) {
                msip::set_ipi(i);
                msip::clear_ipi(i);
            }
        }
        rustsbi::SbiRet::ok(0)
    }
}

struct Timer;

impl rustsbi::Timer for Timer {
    fn set_timer(&mut self, stime_value: u64) {
        // This function must clear the pending timer interrupt bit as well.
        use hal::clint::mtimecmp;
        mtimecmp::write(stime_value);
        unsafe { mip::clear_mtimer() };
    }
}

pub struct Reset;

impl rustsbi::Reset for Reset {
    fn system_reset(&self, reset_type: usize, reset_reason: usize) -> rustsbi::SbiRet {
        println!("[rustsbi] reset triggered! todo: shutdown all harts on D1; program halt. Type: {}, reason: {}", reset_type, reset_reason);
        loop {}
    }
}
