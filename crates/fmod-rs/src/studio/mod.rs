#![doc = include_str!("README.md")]

use fmod::*;

fmod_class! {
    /// The main system object for FMOD Studio.
    ///
    /// Initializing the studio system object also initializes the core System object.
    class System = FMOD_STUDIO_SYSTEM;

    mod lifetime, update, banks, listeners, buses;
}

fmod_class! {
    /// Banks made in FMOD Studio contain the metadata and audio sample data required for runtime mixing and playback.
    /// Audio sample data may be packed into the same bank as the event metadata which references it, or it may be packed into separate banks.
    class Bank = FMOD_STUDIO_BANK;

    mod loading;
}

fmod_class! {
    /// Bus info docu
    class Bus = FMOD_STUDIO_BUS;

    mod general;
}