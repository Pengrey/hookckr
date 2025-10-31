use hooked::find_hooks_in_dll;
use loaded::get_loaded_modules;
use logger::{error, info, sub, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match get_loaded_modules() {
        Ok(modules) => {
            info("Listing Loaded Modules...");
            for module in modules {
                sub(&format!("{:<50} at 0x{:x}", module.name, module.base));
            }
        }
        Err(e) => error(&format!("Error getting loaded modules: {}", e)),
    }

    let dll_to_check = "ntdll.dll";
    info(&format!("Checking for hooks in {}...", dll_to_check));
    match find_hooks_in_dll(dll_to_check) {
        Ok(hooks) => {
            if hooks.is_empty() {
                warn(&format!("No hooks found in {}.", dll_to_check));
            } else {
                for hook in hooks {
                    sub(&format!(
                        "{:<50} -> 0x{:x}",
                        hook.name, hook.target_address
                    ));
                }
            }
        }
        Err(e) => error(&format!("Error finding hooks: {}", e)),
    }

    Ok(())
}
