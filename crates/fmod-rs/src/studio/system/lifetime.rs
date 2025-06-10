use std::{hint::unreachable_unchecked, ptr};
use parking_lot::RwLockUpgradableReadGuard;
use fmod::{debug, handle::{GLOBAL_STUDIO_SYSTEM_STATE, Resource}, studio::System, Handle, error::Result, Error, InitFlags, core};
use fmod_core_sys::{FMOD_System_Create, FMOD_OK, FMOD_RESULT, FMOD_VERSION};
use fmod_studio_sys::{FMOD_Studio_System_Create, FMOD_Studio_System_GetCoreSystem, FMOD_Studio_System_Initialize, FMOD_Studio_System_Release, FMOD_STUDIO_INITFLAGS, FMOD_STUDIO_INIT_ALLOW_MISSING_PLUGINS, FMOD_STUDIO_INIT_DEFERRED_CALLBACKS, FMOD_STUDIO_INIT_LIVEUPDATE, FMOD_STUDIO_INIT_LOAD_FROM_UPDATE, FMOD_STUDIO_INIT_MEMORY_TRACKING, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_INIT_SYNCHRONOUS_UPDATE, FMOD_STUDIO_SYSTEM};
impl System {
    pub fn new() -> Result<(Handle<'static, Self>, Handle<'static, core::System>)> {
        // guard against creating multiple systems
        let system_exists = GLOBAL_STUDIO_SYSTEM_STATE.upgradable_read();
        if *system_exists != 0 {
            whoops!("Only one FMOD studio system may be created safely. \
                Read the docs on `System::new_unchecked` if you actually mean to create more than one system. \
                Note: constructing a studio system automatically creates a core system for you!");
            yeet!(Error::Initialized);
        }

        // guard against racing other free API calls
        let mut system_count = RwLockUpgradableReadGuard::upgrade(system_exists);

        // actual creation
        unsafe { Self::new_inner(&mut system_count) }
    }

    pub unsafe fn new_unchecked() -> Result<(Handle<'static, Self>, Handle<'static, core::System>)> {
        cfg_match! {
            (debug_assertions) => {
                let Some(mut system_count) = GLOBAL_STUDIO_SYSTEM_STATE.try_write() else {
                    // NB: will assert_unsafe_precondition!(false)
                    unreachable_unchecked()
                };
            },
            _ => {
                let mut system_count = unsafe {
                    &mut *GLOBAL_STUDIO_SYSTEM_STATE.data_ptr()
                };
            }
        }
        Self::new_inner(&mut system_count)
    }

    unsafe fn new_inner(system_count: &mut usize) -> Result<(Handle<'static, Self>, Handle<'static, core::System>)> {
        debug::initialize_default(); // setup debug logging

        let mut studio_raw = ptr::null_mut();
        ffi!(FMOD_Studio_System_Create(&mut studio_raw, FMOD_VERSION))?;
        *system_count += 1;

        let mut core_raw = ptr::null_mut();
        ffi!(FMOD_Studio_System_GetCoreSystem(studio_raw, &mut core_raw))?;

        let core_system = core::System::from_raw_ptr(core_raw)?;

        Ok((Handle::new(studio_raw), core_system))
    }

    pub fn init(&self, max_channels: i32, studio_flags: StudioInitFlags, flags: InitFlags) -> Result {
        unsafe { self.init_ex(max_channels, studio_flags, flags, ptr::null()) }
    }

    pub unsafe fn init_ex(
        &self,
        max_channels: i32,
        studio_flags: StudioInitFlags,
        flags: InitFlags,
        extra_driver_data: *const (),
    ) -> Result {
        let flags = InitFlags::into_raw(flags);
        let studio_flags = StudioInitFlags::into_raw(studio_flags);
        ffi!(FMOD_Studio_System_Initialize(self.as_raw(), max_channels, studio_flags, flags, extra_driver_data as *mut _))?;
        Ok(())
    }

    raw! {
        /// Closes and frees this object and its resources.
        ///
        /// This will internally call [`System::close`], so calling
        /// [`System::close`] before this function is not necessary.
        pub unsafe fn raw_release(raw: *mut FMOD_STUDIO_SYSTEM) -> FMOD_RESULT {
            let mut system_count = GLOBAL_STUDIO_SYSTEM_STATE.write();
            let result = FMOD_Studio_System_Release(raw);
            if result == FMOD_OK {
                *system_count -= 1;
                FMOD_OK
            } else {
                result
            }
        }
    }
}

fmod_flags! {
    /// Configuration flags used when initializing the System object.
    pub struct StudioInitFlags: FMOD_STUDIO_INITFLAGS {
        #[default]
        /// Load banks even if they reference plug-ins that have not been loaded.
        Normal                 = FMOD_STUDIO_INIT_NORMAL,
        /// Enable live update.
        LiveUpdate             = FMOD_STUDIO_INIT_LIVEUPDATE,
        /// Load banks even if they reference plug-ins that have not been loaded.
        AllowMissingPlugins    = FMOD_STUDIO_INIT_ALLOW_MISSING_PLUGINS,
        /// Disable asynchronous processing and perform all processing on the calling thread instead.
        SynchronousUpdate      = FMOD_STUDIO_INIT_SYNCHRONOUS_UPDATE,
        /// Defer timeline callbacks until the main update. See [Studio::EventInstance::setCallback] for more information.
        DeferedCallbacks       = FMOD_STUDIO_INIT_DEFERRED_CALLBACKS,
        /// No additional threads are created for bank and resource loading. Loading is driven from [Studio::System::update].
        LoadFromUpdate         = FMOD_STUDIO_INIT_LOAD_FROM_UPDATE,
        /// Enables detailed memory usage statistics. Increases memory footprint and impacts performance. See [Studio::Bus::getMemoryUsage] and [Studio::EventInstance::getMemoryUsage] for more information. Implies FMOD_INIT_MEMORY_TRACKING.
        MemoryTracking         = FMOD_STUDIO_INIT_MEMORY_TRACKING,
    }
}