use fmod::studio::Bank;
use fmod_core_sys::FMOD_RESULT;
use fmod_studio_sys::{FMOD_Studio_Bank_Unload, FMOD_STUDIO_BANK};

impl Bank {
    raw! {
        /// Unloads a bank object.
        ///
        /// This will destroy all objects created from the bank,
        /// unload all sample data inside the bank,
        /// and invalidate all API handles referring to the bank.
        ///
        /// If the bank was loaded from user-managed memory,
        /// e.g. by Studio::System::loadBankMemory with the FMOD_STUDIO_LOAD_MEMORY_POINT mode,
        /// then the memory must not be freed until the unload has completed.
        ///
        /// Poll the loading state using Studio::Bank::getLoadingState or use
        /// the FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD system callback to determine when it is safe to free the memory.
        pub unsafe fn raw_release(this: *mut FMOD_STUDIO_BANK) -> FMOD_RESULT {
            FMOD_Studio_Bank_Unload(this)
        }
    }
}