use fmod::{studio::System, Attributes3d, Vector, Result, handle::Resource};
use fmod_studio_sys::{FMOD_Studio_System_GetListenerAttributes, FMOD_Studio_System_GetListenerWeight, FMOD_Studio_System_GetNumListeners, FMOD_Studio_System_SetListenerAttributes, FMOD_Studio_System_SetListenerWeight, FMOD_Studio_System_SetNumListeners};

impl System {
    pub fn set_listener_attributes(&self, listener: u32, attributes: &Attributes3d, attenuation_position: &Vector) -> Result<()> {
        ffi!(FMOD_Studio_System_SetListenerAttributes(self.as_raw(), listener as i32, attributes.as_raw(), attenuation_position.as_raw()))?;
        Ok(())
    }

    pub fn get_listener_attributes(&self, listener: u32) -> Result<(Attributes3d, Vector)> {
        let mut attributes = Attributes3d::default();
        let mut attenuation_position = Vector::default();
        ffi!(FMOD_Studio_System_GetListenerAttributes(self.as_raw(), listener as i32, attributes.as_raw_mut(), attenuation_position.as_raw_mut()))?;
        Ok((attributes, attenuation_position))
    }

    pub fn set_listener_weight(&self, listener: u32, weight: f32) -> Result<()> {
        ffi!(FMOD_Studio_System_SetListenerWeight(self.as_raw(), listener as i32, weight))?;
        Ok(())
    }

    pub fn get_listener_weight(&self, listener: u32) -> Result<f32> {
        let mut weight = 0.0;
        ffi!(FMOD_Studio_System_GetListenerWeight(self.as_raw(), listener as i32, &mut weight))?;
        Ok(weight)
    }

    pub fn set_num_listeners(&self, listeners: u32) -> Result<()> {
        ffi!(FMOD_Studio_System_SetNumListeners(self.as_raw(), listeners as i32))?;
        Ok(())
    }
    
    pub fn get_num_listeners(&self, listener: u32) -> Result<u32> {
        let mut num = 0;
        ffi!(FMOD_Studio_System_GetNumListeners(self.as_raw(), &mut num))?;
        Ok(num as u32)
    }
}