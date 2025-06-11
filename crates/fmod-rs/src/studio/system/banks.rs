use std::ptr;
use cstr8::CStr8;
use fmod::{Result, studio::{System, Bank}, Handle, handle::Resource, Error, Dsp, Guid};
use fmod_studio_sys::{FMOD_Studio_System_GetBank, FMOD_Studio_System_GetBankByID, FMOD_Studio_System_GetBankCount, FMOD_Studio_System_GetBankList, FMOD_Studio_System_LoadBankCustom, FMOD_Studio_System_LoadBankFile, FMOD_Studio_System_LoadBankMemory, FMOD_Studio_System_UnloadAll, FMOD_STUDIO_BANK, FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES, FMOD_STUDIO_LOAD_BANK_FLAGS, FMOD_STUDIO_LOAD_BANK_NONBLOCKING, FMOD_STUDIO_LOAD_BANK_NORMAL, FMOD_STUDIO_LOAD_BANK_UNENCRYPTED, FMOD_STUDIO_LOAD_MEMORY, FMOD_STUDIO_LOAD_MEMORY_FORCEINT, FMOD_STUDIO_LOAD_MEMORY_MODE, FMOD_STUDIO_LOAD_MEMORY_POINT};

impl System {
    // pub fn load_bank_custom(&self) -> Result<Handle<'_, Bank>> {
    //     let mut bank = ptr::null_mut();
    //     ffi!(FMOD_Studio_System_LoadBankCustom(self.as_raw(), /* *const FMOD_STUDIO_BANK_INFO */, /* u32 */, &mut bank))?;
    //     Ok(unsafe { Handle::new(bank) })
    // }

    pub fn load_bank_file(&self, path: &CStr8, flags: LoadBankFlags) -> Result<Handle<'_, Bank>> {
        let mut bank = ptr::null_mut();
        ffi!(FMOD_Studio_System_LoadBankFile(self.as_raw(), path.as_ptr() as _, flags.into_raw(), &mut bank))?;
        Ok( unsafe { Handle::new(bank) } )
    }

    pub fn load_bank_memory(&self, buffer: *const i8, length: u32, flags: LoadBankFlags, mode: LoadMemoryMode) -> Result<Handle<'_, Bank>> {
        let mut bank = ptr::null_mut();
        ffi!(FMOD_Studio_System_LoadBankMemory(self.as_raw(), buffer, length as i32, mode.into_raw(), flags.into_raw(), &mut bank))?;
        Ok( unsafe { Handle::new(bank) } )
    }

    pub fn unload_all(&self) -> Result<()> {
        ffi!(FMOD_Studio_System_UnloadAll(self.as_raw()))?;
        Ok(())
    }

    pub fn get_bank(&self, path: &CStr8) -> Result<Handle<'_, Bank>> {
        let mut bank = ptr::null_mut();
        ffi!(FMOD_Studio_System_GetBank(self.as_raw(), path.as_ptr() as _, &mut bank))?;
        Ok( unsafe { Handle::new(bank) } )
    }

    pub fn get_bank_by_id(&self, guid: Guid) -> Result<Handle<'_, Bank>> {
        let mut bank = ptr::null_mut();
        ffi!(FMOD_Studio_System_GetBankByID(self.as_raw(), guid.as_raw(), &mut bank))?;
        Ok( unsafe { Handle::new(bank) } )
    }

    pub fn get_bank_count(&self) -> Result<u32> {
        let mut count: i32 = 0;
        ffi!(FMOD_Studio_System_GetBankCount(self.as_raw(), &mut count))?;
        Ok(count as u32)
    }

    pub fn get_bank_list_with_cap(&self, capacity: u32) -> Result<Vec<Handle<'_, Bank>>> {
        let mut raw_array = ptr::null_mut();
        let mut count: i32 = 0;

        ffi!(FMOD_Studio_System_GetBankList(self.as_raw(), &mut raw_array, capacity as i32, &mut count))?;

        let mut array = Vec::with_capacity(capacity as usize);
        for i in 0..count {
            array.push(unsafe { Handle::new(raw_array.offset(i as isize)) });
        }

        Ok(array)
    }

    pub fn get_bank_list(&self) -> Result<Vec<Handle<'_, Bank>>> {
        let count = self.get_bank_count()?;
        self.get_bank_list_with_cap(count)
    }
}

fmod_flags! {
    pub struct LoadBankFlags: FMOD_STUDIO_LOAD_BANK_FLAGS {
        #[default]
        Normal            = FMOD_STUDIO_LOAD_BANK_NORMAL,
        NonBlocking       = FMOD_STUDIO_LOAD_BANK_NONBLOCKING,
        DecompressSamples = FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES,
        Unencrypted       = FMOD_STUDIO_LOAD_BANK_UNENCRYPTED,
    }
}

fmod_enum! {
    #[derive(Default)]
    pub enum LoadMemoryMode: FMOD_STUDIO_LOAD_MEMORY_MODE
    where const { self <= FMOD_STUDIO_LOAD_MEMORY_POINT }
    {
        #[default]
        Memory      = FMOD_STUDIO_LOAD_MEMORY,
        MemoryPoint = FMOD_STUDIO_LOAD_MEMORY_POINT,
        // ForceInt    = FMOD_STUDIO_LOAD_MEMORY_FORCEINT,
    }
}

// pub struct BankInfo {
//     pub size: i32,
//     pub user_data: *mut c_void,
//     pub user_data_length: i32,
// }