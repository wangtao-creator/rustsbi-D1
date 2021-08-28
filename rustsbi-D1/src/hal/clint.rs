pub mod mtimecmp{
    use crate::{hal::{pac_encoding::{CLINT_BASE, MTIMECMPL}, write_reg}};
    // pub fn read() -> u64{
    //     unsafe { read_reg::<u64>(CLINT_BASE, MTIMECMPL) }
    // }
    pub fn write(word:u64) {
        unsafe { write_reg(CLINT_BASE, MTIMECMPL, word)}
    }
}
pub mod msip{
    use crate::{hal::{pac_encoding::{CLINT_BASE, MSIP0}, write_reg}};

    pub fn set_ipi(_word:usize){
        unsafe { write_reg(CLINT_BASE, MSIP0, 1u64)}
    }
    pub fn clear_ipi(_word:usize) {
        unsafe { write_reg(CLINT_BASE, MSIP0, 0)}
    }
}