use ovmf_prebuilt::ovmf_pure_efi;
use std::env;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex}; // Import for OVMF path

lazy_static::lazy_static! {
    static ref KEY_PRESSED: Mutex<Option<char>> = Mutex::new(None);
}
fn main() {
    // Read environment variables set in the build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    // Choose whether to start the UEFI or BIOS image
    let uefi = false; // Set this based on your logic

    let mut cmd = Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={}", uefi_path));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={}", bios_path));
    }

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error starting QEMU: {}", e);
            exit(1);
        }
    };

    match child.wait() {
        Ok(status) => println!("QEMU exited with: {}", status),
        Err(e) => eprintln!("Error waiting for QEMU: {}", e),
    }
}

#[macro_export]
macro_rules! input_str {
    () => {{
        match input_str() {
            Some(value) => value,
            None => "".to_owned(),
        }
    }};
}

pub fn input_str() -> Option<String> {
    let mut input = String::new();
    loop {
        if let Some(character) = *KEY_PRESSED.lock().unwrap() {
            if character == '\u{000D}' {
                // Enter key pressed, exit the loop
                print!("{}", input); // Print the input string
                break;
            } else {
                input.push(character);
            }
        }
        // Add a delay or yield here to avoid excessive CPU usage
    }
    Some(input)
}
