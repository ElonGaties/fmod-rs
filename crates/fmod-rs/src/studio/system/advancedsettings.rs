use std::{ffi::c_char, ptr};
use fmod::{studio::System, Result, handle::Resource};
use fmod_studio_sys::{FMOD_Studio_System_GetAdvancedSettings, FMOD_Studio_System_SetAdvancedSettings, FMOD_STUDIO_ADVANCEDSETTINGS};

impl System {
    pub fn set_advanced_settings(&self, mut advanced_settings: AdvancedSettings) -> Result<()> {
        ffi!(FMOD_Studio_System_SetAdvancedSettings(self.as_raw(), advanced_settings.as_raw_mut()))?;
        Ok(())
    }

    pub fn get_advanced_settings(&self) -> Result<AdvancedSettings> {
        let mut advanced_settings = AdvancedSettings::default();
        ffi!(FMOD_Studio_System_GetAdvancedSettings(self.as_raw(), advanced_settings.as_raw_mut()))?;
        Ok(advanced_settings)
    }
}

fmod_struct! {
    #![fmod_no_pod]
    /// Studio Advanced Settings
    pub struct AdvancedSettings = FMOD_STUDIO_ADVANCEDSETTINGS {
        pub cbsize: i32,
        pub command_queue_size: u32,
        pub handle_initial_size: u32,
        pub studio_update_period: i32,
        pub idle_sample_data_pool_size: i32,
        pub streaming_schedule_delay: u32,
        #[default(ptr::null_mut())]
        pub encryption_key: *mut c_char,
    }
}