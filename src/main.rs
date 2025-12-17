#![windows_subsystem = "windows"]

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::{thread, time, ffi::OsStr, os::windows::ffi::OsStrExt};
use tray_item::{TrayItem, IconSource};
use windows::{
    core::PCWSTR,
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowTextW},
};

fn to_pcwstr(text: &str) -> Vec<u16> {
    OsStr::new(text).encode_wide().chain(Some(0)).collect()
}

fn get_sumatra_title() -> Option<String> {
    unsafe {
        let class_name = to_pcwstr("SUMATRA_PDF_FRAME");
        let hwnd: HWND = FindWindowW(PCWSTR(class_name.as_ptr()), PCWSTR::null());

        if hwnd.0 == 0 { return None; }

        let mut buffer = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buffer);

        if len > 0 {
            return Some(String::from_utf16_lossy(&buffer[..len as usize]));
        }
        None
    }
}

fn monitor_loop() {
    let client_id = "1332844037194911775";
    
    loop {
        let mut client = match DiscordIpcClient::new(client_id) {
            Ok(c) => c,
            Err(_) => {
                thread::sleep(time::Duration::from_secs(10));
                continue;
            }
        };

        if client.connect().is_ok() {
            loop {
                if let Some(full_title) = get_sumatra_title() {
                    let filename = full_title.replace(" - SumatraPDF", "");
                    
                    let payload = activity::Activity::new()
                        .state("Reading Document")
                        .details(&filename)
                        .assets(activity::Assets::new()
                            .large_image("sumatra-512")
                            .large_text("SumatraPDF"));

                    if client.set_activity(payload).is_err() {
                        break; 
                    }
                } else {
                    let _ = client.clear_activity();
                }
                thread::sleep(time::Duration::from_secs(5));
            }
        }
        
        thread::sleep(time::Duration::from_secs(10));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut tray = TrayItem::new("Camel - SumatraRPC", IconSource::Resource("icon-name"))?;

    tray.add_label("Camel is running...")?;
    
    tray.add_menu_item("Exit", || {
        std::process::exit(0);
    })?;

    thread::spawn(move || {
        monitor_loop();
    });

    let mut message = std::mem::MaybeUninit::<windows::Win32::UI::WindowsAndMessaging::MSG>::uninit();
    unsafe {
        use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, TranslateMessage, DispatchMessageW};
        while GetMessageW(message.as_mut_ptr(), HWND(0), 0, 0).as_bool() {
            TranslateMessage(message.as_ptr());
            DispatchMessageW(message.as_ptr());
        }
    }

    Ok(())
}