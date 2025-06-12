use fmod::studio::Bus;
use fmod_core_sys::{FMOD_OK, FMOD_RESULT};
use fmod_studio_sys::FMOD_STUDIO_BUS;

impl Bus {
    raw! {
        /// Unloading/releasing Bus object docu
        pub unsafe fn raw_release(this: *mut FMOD_STUDIO_BUS) -> FMOD_RESULT {
            // unimplemented!();
            FMOD_OK
        }
    }
}