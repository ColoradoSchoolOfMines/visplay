mod window;
pub use self::window::*;
use self::window::Window;

use std::ptr::{null, null_mut};
use libc::{c_int};

use std::mem::replace;
use std::rc::Rc;

use x11::xlib::*;
pub use x11::xlib::Xlib;

use std::sync::Mutex;

lazy_static!{
    static ref GLOBAL_DATA: Mutex<Option<GlobalData>> = Mutex::new(None);
}

#[derive(Debug, Clone, Copy)]
struct GlobalData {
    wm_detected: bool,
}

fn get_global() -> Option<GlobalData> {
    *GLOBAL_DATA.lock().expect("Could not lock global data")
}

fn map_global<F: FnOnce(&mut GlobalData) -> R, R>(f: F) -> R {
    let ref mut glob = *GLOBAL_DATA.lock().expect("Could not lock global data");
    if let &mut Some(ref mut glob) = glob { f(glob) }
    else { panic!("GlobalData not available"); }
}

fn map_global_safe<F: FnOnce(&mut GlobalData) -> R, R>(f: F) -> Option<R> {
    if let Ok(mut glob) = GLOBAL_DATA.lock() {
        if let Some(ref mut glob) = *glob {
            return Some(f(glob));
        } else {
            error!{"Failed to inform system, RootStructure not set up"};
        }
    } else {
        error!{"Failed to inform system, could not lock global data"};
    }

    None
}

pub struct RootStructure {
    display: *mut Display,
    root: u64,
    xlib: Rc<Xlib>
}

#[derive(Debug)]
pub enum InitError {
    OpenNull,
    RootStructureAlreadySetup,
    WMAlreadyPresent,
}

fn on_error(event: &XErrorEvent) {
    error!("XLib error: {}", event.error_code);
}

unsafe extern "C" fn error_handler(_: *mut Display, event: *mut XErrorEvent) -> c_int {
    on_error(&*event);
    return 0;
}

unsafe extern "C" fn detect_wm(_: *mut Display, event: *mut XErrorEvent) -> c_int {
    on_error(&*event);
    if (*event).error_code == BadAccess {
        error!("Existing WM detected");
        map_global_safe(|dat| dat.wm_detected = true);
    }
    return 0;
}

impl RootStructure {
    pub fn setup(xlib: Rc<Xlib>) -> Result<RootStructure, InitError> {
        use self::InitError::*;

        info!("Starting setup...");

        {
            if let Some(_) = get_global() {
                return Err(RootStructureAlreadySetup);
            }

            trace!("Initializing global data");
            let mut global = GLOBAL_DATA.lock().unwrap();
            replace(&mut *global, Some(GlobalData {
                wm_detected: false,
            }));
        }

        unsafe {
            debug!("Opening display...");
            let display = (xlib.XOpenDisplay)(null());
            if display == null_mut() {
                error!("Could not open dipslay");
                return Err(OpenNull);
            }

            let screen = (xlib.XDefaultScreenOfDisplay)(display);
            let root = (xlib.XRootWindowOfScreen)(screen);

            let out = RootStructure {
                display: display,
                root: root as u64,
                xlib: xlib,
            };

            out.initilize()?;

            info!("Setup finished");
            return Ok(out);
        }
    }

    pub fn poll_event(&mut self) -> XEvent {
        unsafe {
            let mut event = XEvent { pad: [0; 24] };
            let event_id = (self.xlib.XNextEvent)(self.display, &mut event);
            debug!("Event polled: {}", event_id);
            event
        }
    }

    pub fn teardown(self) {
        info!("Starting teardown...");

        unsafe { 
            debug!("Closing display...");
            (self.xlib.XCloseDisplay)(self.display); 
            (self.xlib.XSync)(self.display, false as i32);
        }

        trace!("Deleting global data");
        let mut global = GLOBAL_DATA.lock().unwrap();
        replace(&mut *global, None);

        info!("Teardown finished");
    }

    fn initilize(&self) -> Result<(), InitError> {
        use self::InitError::*;

        unsafe {
            (self.xlib.XSetErrorHandler)(Some(detect_wm));
            debug!("Selecting substructure redirection...");
            (self.xlib.XSelectInput)(self.display, self.root, SubstructureRedirectMask | SubstructureNotifyMask);
            (self.xlib.XSync)(self.display, false as i32);

            if map_global(|g| g.wm_detected) {
                return Err(WMAlreadyPresent);
            }

            (self.xlib.XSetErrorHandler)(Some(error_handler));
        }

        Ok(())
    }

    pub fn get_root(&self) -> Window {
        Window::from_id(self.root)
    }
}