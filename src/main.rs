use ovmf_prebuilt::ovmf_pure_efi;
use std::env;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref KEY_PRESSED: Mutex<Option<char>> = Mutex::new(None);
}
fn main() {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    let uefi = false;

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
