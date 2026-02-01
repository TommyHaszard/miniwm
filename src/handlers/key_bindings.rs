use smithay::input::keyboard::{Keysym, ModifiersState};

use crate::MiniWm;

impl MiniWm {
    pub fn handle_keybinding(&mut self, keysym: Keysym, modifiers: &ModifiersState) -> bool {
        // Check for Super/Logo key (Mod4 in i3 terms)
        if modifiers.logo {
            match keysym {
                Keysym::Return => {
                    // Spawn terminal
                    std::process::Command::new("alacritty").spawn().ok();
                    return true;
                }
                Keysym::d => {
                    // Spawn launcher
                    std::process::Command::new("tofi-drun")
                        .arg("--drun-launch=true")
                        .spawn()
                        .ok();
                    return true;
                }
                Keysym::w => {
                    // Spawn launcher
                    std::process::Command::new("firefox")
                        .spawn()
                        .ok();
                    return true;
                }
                Keysym::q if modifiers.shift => {
                    // Close focused window (implement this later)
                    // self.close_focused_window();
                    return true;
                }
                _ => {}
            }
        }

        false // Keybinding not handled
    }
}
