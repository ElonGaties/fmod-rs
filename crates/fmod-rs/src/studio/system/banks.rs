use std::{ptr, ffi::c_void};
use fmod::{Result, studio::{System, Bank}, Handle};
use fmod_studio_sys::FMOD_Studio_System_LoadBankCustom;

impl System {
    pub fn load_bank_custom(&self) -> Result<Handle<'_, Bank>> {
        let mut bank = ptr::null_mut();
        ffi!(FMOD_Studio_System_LoadBankCustom(self.as_raw(), /* *const FMOD_STUDIO_BANK_INFO */, /* u32 */, &mut bank))?;
        Ok(unsafe { Handle::new(bank) })
    }
}

pub struct BankInfo {
    pub size: i32,
    pub user_data: *mut c_void,
    pub user_data_length: i32,
}