use std::ptr;
use cstr8::CStr8;
use fmod::{studio::{System, Bus}, Result, handle::Resource, Guid, Handle};
use fmod_studio_sys::{FMOD_Studio_System_GetBus, FMOD_Studio_System_GetBusByID};

impl System {
    pub fn get_bus(&self, path: &CStr8) -> Result<Handle<'_, Bus>> {
        let mut bus = ptr::null_mut();
        ffi!(FMOD_Studio_System_GetBus(self.as_raw(), path.as_ptr() as _, &mut bus))?;
        Ok( unsafe { Handle::new(bus) } )
    }

    pub fn get_bus_by_id(&self, id: &Guid) -> Result<Handle<'_, Bus>> {
        let mut bus = ptr::null_mut();
        ffi!(FMOD_Studio_System_GetBusByID(self.as_raw(), id.as_raw(), &mut bus))?;
        Ok( unsafe { Handle::new(bus) } )
    }
}