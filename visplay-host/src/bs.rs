use std::env;
use std::rc::Rc;
use std::ops::Deref;
use std::process::Command;
use zombie;
use wtftw_core::config::Config;
use wtftw_core::window_manager::WindowManager;
use wtftw_core::window_system::*;
use wtftw_xlib::XlibWindowSystem;

pub fn main() {
    // Create a default config.generaluration
    let mut config = Config::initialize();
    // Initialize window system. Use xlib here for now
    debug!("initialize window system");
    let xlib = XlibWindowSystem::new();
    let window_system : Rc<WindowSystem> = Rc::new(xlib);
    // Create the actual window manager
    debug!("create window manager");
    let mut window_manager = WindowManager::new(window_system.deref(), &config.general);

    // If available, compile the config.general file at ~/.wtftw/config.general.rs
    // and call the config.generalure method
    config.compile_and_call(&mut window_manager, window_system.deref());
    window_manager = WindowManager::new(window_system.deref(), &config.general);

    // Output some initial information
    info!("WTFTW - Window Tiling For The Win");
    info!("Starting wtftw on {} screen(s)", window_system.get_screen_infos().len());

    // Output information about displays
    for (i, &Rectangle(x, y, w, h)) in window_system.get_screen_infos().iter().enumerate() {
        debug!("Display {}: {}x{} ({}, {})", i, w, h, x, y);
    }

    debug!("Size of keyhandlers after config.generaluration: {}", config.internal.key_handlers.len());

    let enter_key = KeyCommand::new(window_system.get_keycode_from_string("Return"), NONEMASK);
    let x_key = KeyCommand::new(window_system.get_keycode_from_string("X"), NONEMASK);
    window_system.grab_keys(vec![enter_key, x_key]);

    for (command, _) in config.internal.key_handlers.iter() {
        window_system.grab_keys(vec!(command.clone()));
    }

    for (&command, _) in config.internal.mouse_handlers.iter() {
        window_system.grab_button(command);
    }

    window_manager = (*config.internal.startup_hook)(window_manager, window_system.clone(), &config);

    // Enter the event loop and just listen for events
    while window_manager.running {
        let event = window_system.clone().get_event();
        match event {
            WindowSystemEvent::ClientMessageEvent(_, _, _, _) => {
            },
            WindowSystemEvent::PropertyMessageEvent(process, window, atom) => {
                if process {
                    window_manager = window_system.process_message(&window_manager, &config.general, window, atom);
                }
            },
            // The X11/Wayland configuration changed, so we need to readjust the
            // screen configurations.
            WindowSystemEvent::ConfigurationNotification(window) => {
                if window_system.get_root() == window {
                    debug!("screen configuration changed. rescreen");
                    window_manager = window_manager.rescreen(window_system.deref());
                }
            },
            // A window asked to be reconfigured (i.e. resized, border change, etc.)
            WindowSystemEvent::ConfigurationRequest(window, window_changes, mask) => {
                let floating = window_manager.workspaces.floating.iter().any(|(&x, _)| x == window) ||
                    !window_manager.workspaces.contains(window);
                window_system.configure_window(window, window_changes, mask, floating);
                window_manager = window_manager.windows(window_system.deref(), &config.general, &|x| x.clone());
            },
            // A new window was created, so we need to manage
            // it unless it is already managed by us.
            WindowSystemEvent::WindowCreated(window) => {
                if window_manager.is_window_managed(window) || window_system.overrides_redirect(window) {
                    continue;
                }

                window_manager = window_manager.manage(window_system.deref(), window, &config.general)
                                               .windows(window_system.deref(), &config.general,
                                                        &|x| (config.internal.manage_hook)(x.clone(),
                                                        window_system.clone(), window));
            },
            WindowSystemEvent::WindowUnmapped(window, synthetic) => {
                if synthetic && window_manager.is_window_managed(window) {
                    window_manager = if synthetic || !window_manager.is_waiting_unmap(window) {
                        window_manager.unmanage(window_system.deref(), window, &config.general)
                    } else {
                        window_manager.update_unmap(window)
                    };
                }
                zombie::collect_zombies();
            },
            WindowSystemEvent::WindowDestroyed(window) => {
                if window_manager.is_window_managed(window) {
                    window_manager = window_manager.unmanage(window_system.deref(), window, &config.general)
                        .remove_from_unmap(window);
                }
            },
            WindowSystemEvent::Enter(window) => {
                if window_manager.is_window_managed(window) {
                    window_manager = window_manager.focus(window, window_system.deref(), &config.general);
                }
            },
            WindowSystemEvent::KeyPressed(_, key) => {
                if key == enter_key { Command::new("urxvt").spawn().unwrap(); }
                else if key == x_key { window_manager = window_manager.kill_window(window_system.as_ref()) }
            },
            _ => ()
        };

        if let Some(ref mut loghook) = config.internal.loghook {
            loghook(window_manager.clone(), window_system.clone());
        }

        window_system.update_server_state(&window_manager);
    }
}