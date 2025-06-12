use std::ptr;
use cstr8::CStr8;
use fmod::{Result, studio::{EventDescription, System}, handle::Resource, Handle, Guid};
use fmod_studio_sys::{FMOD_Studio_System_GetEvent, FMOD_Studio_System_GetEventByID};

impl System {
    pub fn get_event(&self, name_or_path: &CStr8) -> Result<Handle<'_, EventDescription>> {
        let mut event_desc = ptr::null_mut();
        ffi!(FMOD_Studio_System_GetEvent(self.as_raw(), name_or_path.as_ptr() as _, &mut event_desc))?;
        Ok( unsafe { Handle::new(event_desc) } )
    }

    pub fn get_event_by_id(&self, id: &Guid) -> Result<Handle<'_, EventDescription>> {
        let mut event_desc = ptr::null_mut();
        ffi!(FMOD_Studio_System_GetEventByID(self.as_raw(), id.as_raw(), &mut event_desc))?;
        Ok( unsafe { Handle::new(event_desc) } )
    }
}