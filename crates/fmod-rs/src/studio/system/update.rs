use fmod::{studio::System, handle::Resource, Result};
use fmod_studio_sys::{FMOD_Studio_System_FlushCommands, FMOD_Studio_System_FlushSampleLoading, FMOD_Studio_System_Update};

impl System {
    pub fn update(&self) -> Result {
        ffi!(FMOD_Studio_System_Update(self.as_raw()))?;
        Ok(())
    }

    pub fn flush_commands(&self) -> Result {
        ffi!(FMOD_Studio_System_FlushCommands(self.as_raw()))?;
        Ok(())
    }

    pub fn flush_sample_loading(&self) -> Result {
        ffi!(FMOD_Studio_System_FlushSampleLoading(self.as_raw()))?;
        Ok(())
    }
}