use fmod::studio::EventDescription;
use fmod_core_sys::{FMOD_OK, FMOD_RESULT};
use fmod_studio_sys::FMOD_STUDIO_EVENTDESCRIPTION;

impl EventDescription {
    raw! {
        /// Unloading/releasing Event Description object docu
        pub unsafe fn raw_release(this: *mut FMOD_STUDIO_EVENTDESCRIPTION) -> FMOD_RESULT {
            // unimplemented!();
            FMOD_OK
        }
    }
}