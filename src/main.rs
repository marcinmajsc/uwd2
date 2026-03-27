#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::env;
#[cfg(target_os = "windows")]
use windows::Win32::System::Console::{AllocConsole, AttachConsole, ATTACH_PARENT_PROCESS};

use crate::cache_pdb::get_rva;
use crate::explorer_modinfo::get_guid;

mod cache_pdb;
mod constants;
mod explorer_modinfo;
mod fetch_pdb;
mod inject;
mod parse_pdb;

#[cfg(target_os = "windows")]
fn enable_log_console() {
    unsafe {
        if AttachConsole(ATTACH_PARENT_PROCESS).is_err() {
            // If no parent console exists (e.g. started from Explorer), create one.
            let _ = AllocConsole();
        }
    }
}

fn prog() -> String {
    // modified from https://stackoverflow.com/a/58113997/9044183
    env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}

fn help() {
    println!(
        include_str!("../help.txt"),
        env!("CARGO_PKG_VERSION"),
        prog()
    )
}

fn rva() -> u32 {
    let guid;
    unsafe {
        guid = get_guid();
    }
    let rva = get_rva(guid);
    println!("RVA is {rva:#x}");
    rva
}

fn inject() {
    unsafe {
        inject::inject(rva());
        inject::refresh();
    }
}
fn main() {
    let mut command: Option<String> = None;
    let mut background = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "background" | "--background" => background = true,
            _ => {
                if command.is_none() {
                    command = Some(arg);
                } else {
                    eprintln!(
                        "Too many arguments. Run `{} help` to see all commands.",
                        prog()
                    );
                    return;
                }
            }
        }
    }

    if !background {
        #[cfg(target_os = "windows")]
        enable_log_console();
    }

    match command.as_deref() {
        None => inject(),
        Some("inject") => inject(),
        Some("help") => help(),
        Some("about") => {
            println!(include_str!("../about.txt"), env!("CARGO_PKG_VERSION"))
        }
        Some(err) => {
            eprintln!(
                "Invalid argument `{err}`. Run `{} help` to see all commands.",
                prog()
            )
        }
    }
}
