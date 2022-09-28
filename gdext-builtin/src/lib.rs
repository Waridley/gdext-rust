#![macro_use]

pub mod macros;
mod types;
mod util;
pub use types::*;

pub use glam;

use gdext_sys as sys;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
pub enum InitLevel {
    Core = sys::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_CORE,
    Servers = sys::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_SERVERS,
    Scene = sys::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_SCENE,
    Editor = sys::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_EDITOR,
}

impl InitLevel {
    #[doc(hidden)]
    pub fn from_sys(level: gdext_sys::GDNativeInitializationLevel) -> Self {
        match level {
            sys::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_CORE => Self::Core,
            sys::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_SERVERS => Self::Servers,
            sys::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_SCENE => Self::Scene,
            sys::GDNativeInitializationLevel_GDNATIVE_INITIALIZATION_EDITOR => Self::Editor,
            _ => Self::Scene,
        }
    }
    #[doc(hidden)]
    pub fn to_sys(self) -> sys::GDNativeInitializationLevel {
        self as _
    }
}

impl Default for InitLevel {
    fn default() -> Self {
        Self::Scene
    }
}

const LEVELS: usize = sys::GDNativeInitializationLevel_GDNATIVE_MAX_INITIALIZATION_LEVEL as usize;

pub struct InitOptions {
    init_levels: [Option<Box<dyn FnMut() + 'static>>; LEVELS],
    deinit_levels: [Option<Box<dyn FnMut() + 'static>>; LEVELS],
    lowest_level: InitLevel,
}

impl InitOptions {
    pub fn new() -> Self {
        Self {
            init_levels: Default::default(),
            deinit_levels: Default::default(),
            lowest_level: InitLevel::Scene,
        }
    }

    pub fn register_init_function(&mut self, level: InitLevel, f: impl FnMut() + 'static) {
        if self.init_levels[level as usize].is_some() {
            gdext_print_warning!("Replacing init function for {level:?}")
        }
        self.init_levels[level as usize] = Some(Box::new(f));
        self.lowest_level = self.lowest_level.min(level);
    }

    pub fn register_deinit_function(&mut self, level: InitLevel, f: impl FnMut() + 'static) {
        if self.deinit_levels[level as usize].is_some() {
            gdext_print_warning!("Replacing deinit function for {level:?}")
        }
        self.deinit_levels[level as usize] = Some(Box::new(f));
        self.lowest_level = self.lowest_level.min(level);
    }

    pub fn lowest_init_level(&self) -> InitLevel {
        self.lowest_level
    }

    pub fn run_init_function(&mut self, level: InitLevel) {
        if let Some(f) = self.init_levels[level as usize].as_mut() {
            f();
        }
    }

    pub fn run_deinit_function(&mut self, level: InitLevel) {
        if let Some(f) = self.deinit_levels[level as usize].as_mut() {
            f();
        }
    }
}

impl Default for InitOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)]
pub static mut INIT_OPTIONS: Option<InitOptions> = None;

#[macro_export]
macro_rules! gdext_init {
    ($name:ident, $f:expr) => {
        #[no_mangle]
        unsafe extern "C" fn $name(
            interface: *const ::gdext_sys::GDNativeInterface,
            library: ::gdext_sys::GDNativeExtensionClassLibraryPtr,
            init: *mut ::gdext_sys::GDNativeInitialization,
        ) -> bool {
            let mut result = true;

            ::gdext_sys::set_interface(interface);
            ::gdext_sys::set_library(library);

            let mut init_options = $crate::InitOptions::new();

            ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| ($f)(&mut init_options)))
                .unwrap_or_else(|_| result = false);

            *init = ::gdext_sys::GDNativeInitialization {
                minimum_initialization_level: init_options.lowest_init_level().to_sys(),
                userdata: std::ptr::null_mut(),
                initialize: Some(initialise),
                deinitialize: Some(deinitialise),
            };

            $crate::INIT_OPTIONS = Some(init_options);

            result
        }

        unsafe extern "C" fn initialise(
            _userdata: *mut std::ffi::c_void,
            init_level: ::gdext_sys::GDNativeInitializationLevel,
        ) {
            let init_options = $crate::INIT_OPTIONS.as_mut().unwrap();
            init_options.run_init_function($crate::InitLevel::from_sys(init_level));
        }

        unsafe extern "C" fn deinitialise(
            _userdata: *mut std::ffi::c_void,
            init_level: ::gdext_sys::GDNativeInitializationLevel,
        ) {
            let init_options = $crate::INIT_OPTIONS.as_mut().unwrap();
            init_options.run_deinit_function($crate::InitLevel::from_sys(init_level));
        }
    };
}
