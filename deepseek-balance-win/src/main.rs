#![windows_subsystem = "windows"]

mod balance;
mod tray;
mod storage;
mod settings;

use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Foundation::*,
};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

const WM_TRAY: u32 = WM_APP + 1;
const WM_BALANCE_UPDATE: u32 = WM_USER + 100;
const WM_BALANCE_ERROR: u32 = WM_USER + 101;

const MENU_REFRESH: u32 = 1001;
const MENU_SETTINGS: u32 = 1002;
const MENU_QUIT: u32 = 1003;

const TRAY_UID: u32 = 1;

// Shared state
static BALANCE_TEXT: Mutex<Option<String>> = Mutex::new(None);
static API_KEY: Mutex<Option<String>> = Mutex::new(None);
static REFRESH_INTERVAL: Mutex<u32> = Mutex::new(300);
static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() -> Result<()> {
    let hinstance = unsafe { GetModuleHandleW(None)? };

    // Load saved API key
    if let Some(key) = storage::load_key() {
        *API_KEY.lock().unwrap() = Some(key);
    }

    // Load saved interval (stored as DWORD in registry)
    *REFRESH_INTERVAL.lock().unwrap() = load_interval();

    // Register main window class
    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
        hInstance: hinstance.into(),
        lpszClassName: w!("DeepSeekBalanceWnd"),
        hbrBackground: GetSysColorBrush(COLOR_WINDOW),
        ..Default::default()
    };
    unsafe { RegisterClassW(&wc) };

    // Create hidden message-only window
    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_NONE,
            w!("DeepSeekBalanceWnd"),
            w!(""),
            0, CW_USEDEFAULT, CW_USEDEFAULT, 0, 0,
            HWND_MESSAGE, None, hinstance, None,
        )
    }?;

    // Create tray icon
    let tray = tray::TrayIcon::new(hwnd, TRAY_UID);
    let has_key = API_KEY.lock().unwrap().is_some();

    if has_key {
        tray.add("...")?;
        tray.set_tooltip("DeepSeek Balance — Loading...");
        // Start polling
        spawn_polling_thread(hwnd);
    } else {
        tray.add("🔑")?;
        tray.set_tooltip("DeepSeek Balance — API Key not set");
        // Open settings on first launch
        PostMessageW(hwnd, WM_COMMAND, WPARAM(MENU_SETTINGS as _), LPARAM(0)).ok();
    }

    // Check for updates timer
    unsafe { SetTimer(hwnd, 1, 500, None) };

    // Message loop
    let mut msg = MSG::default();
    loop {
        let ret = unsafe { GetMessageW(&mut msg, None, 0, 0) };
        match ret {
            BOOL(0) | BOOL(-1) => break,
            _ => {}
        }
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    // Cleanup
    tray.remove();
    RUNNING.store(false, Ordering::SeqCst);
    Ok(())
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAY => {
            let event = lparam.0 as u32;
            match event {
                win if win == WM_RBUTTONUP || win == WM_CONTEXTMENU => {
                    // Build context menu
                    let balance_line = BALANCE_TEXT
                        .lock().unwrap()
                        .clone()
                        .unwrap_or_else(|| "Balance: ...".to_string());

                    let menu_items = {
                        let mut items = vec![
                            (balance_line.as_str(), 0),
                            ("", 0),                     // separator
                            ("Refresh Now", MENU_REFRESH),
                            ("Settings...", MENU_SETTINGS),
                            ("", 0),                     // separator
                            ("Quit", MENU_QUIT),
                        ];
                        items
                    };

                    let tray = tray::TrayIcon::new(hwnd, TRAY_UID);
                    let cmd = tray.show_menu(&menu_items);

                    match cmd {
                        MENU_REFRESH => {
                            PostMessageW(hwnd, WM_COMMAND, WPARAM(MENU_REFRESH as _), LPARAM(0));
                        }
                        MENU_SETTINGS => {
                            PostMessageW(hwnd, WM_COMMAND, WPARAM(MENU_SETTINGS as _), LPARAM(0));
                        }
                        MENU_QUIT => {
                            PostMessageW(hwnd, WM_COMMAND, WPARAM(MENU_QUIT as _), LPARAM(0));
                        }
                        _ => {}
                    }
                }

                WM_LBUTTONUP => {
                    // Quick refresh on left click
                    let tray = tray::TrayIcon::new(hwnd, TRAY_UID);
                    let tip = BALANCE_TEXT.lock().unwrap().clone()
                        .unwrap_or_else(|| "DeepSeek Balance".to_string());
                    tray.set_tooltip(&tip);
                }

                _ => {}
            }
            LRESULT(0)
        }

        WM_COMMAND => {
            let id = wparam.0 as u32;
            match id {
                MENU_REFRESH => {
                    let tray = tray::TrayIcon::new(hwnd, TRAY_UID);
                    tray.set_text("...");
                    tray.set_tooltip("Refreshing...");

                    if let Some(ref key) = *API_KEY.lock().unwrap() {
                        let key = key.clone();
                        thread::spawn(move || {
                            match balance::fetch_balance(&key) {
                                Ok((amount, currency)) => {
                                    let symbol = if currency == "USD" { "$" } else { "¥" };
                                    let text = format!("{}{:.2}", symbol, amount);
                                    *BALANCE_TEXT.lock().unwrap() = Some(format!("Balance: {}{:.2}", symbol, amount));
                                    PostMessageW(hwnd, WM_BALANCE_UPDATE, WPARAM(0), LPARAM(0)).ok();
                                }
                                Err(e) => {
                                    PostMessageW(hwnd, WM_BALANCE_ERROR, WPARAM(0), LPARAM(0)).ok();
                                }
                            }
                        });
                    }
                }

                MENU_SETTINGS => {
                    let hinstance = GetWindowLongPtrW(hwnd, GWLP_HINSTANCE);
                    let hinstance = HINSTANCE(hinstance as _);

                    let current_key = API_KEY.lock().unwrap().clone().unwrap_or_default();
                    let current_interval = *REFRESH_INTERVAL.lock().unwrap();

                    if let Some((key, interval_sec)) = settings::show_settings(
                        hinstance, &current_key, current_interval,
                    ) {
                        storage::save_key(&key).ok();
                        *API_KEY.lock().unwrap() = Some(key.clone());
                        *REFRESH_INTERVAL.lock().unwrap() = interval_sec;

                        // Save interval to registry
                        save_interval(interval_sec).ok();

                        // Enable auto-start
                        storage::set_autostart(true).ok();

                        // Restart polling
                        RUNNING.store(false, Ordering::SeqCst);
                        thread::sleep(Duration::from_millis(200));
                        RUNNING.store(true, Ordering::SeqCst);

                        let tray = tray::TrayIcon::new(hwnd, TRAY_UID);
                        tray.set_text("...");
                        spawn_polling_thread(hwnd);
                    }
                }

                MENU_QUIT => {
                    DestroyWindow(hwnd);
                }

                _ => {}
            }
            LRESULT(0)
        }

        WM_TIMER => {
            // Check for pending balance updates
            if let Some(ref text) = *BALANCE_TEXT.lock().unwrap() {
                let tray = tray::TrayIcon::new(hwnd, TRAY_UID);
                // Extract just the number portion for the tray icon
                // text is "Balance: ¥12.34" — extract "¥12.34"
                let display = if let Some(pos) = text.find(": ") {
                    &text[pos + 2..]
                } else {
                    text.as_str()
                };
                tray.set_text(display).ok();
                tray.set_tooltip(text).ok();
            }
            LRESULT(0)
        }

        WM_BALANCE_UPDATE => {
            // This is posted after a balance fetch — the text is already in BALANCE_TEXT
            // The WM_TIMER will pick it up
            LRESULT(0)
        }

        WM_BALANCE_ERROR => {
            let tray = tray::TrayIcon::new(hwnd, TRAY_UID);
            tray.set_text("⚠️").ok();
            tray.set_tooltip("Error fetching balance").ok();
            LRESULT(0)
        }

        WM_DESTROY => {
            RUNNING.store(false, Ordering::SeqCst);
            PostQuitMessage(0);
            LRESULT(0)
        }

        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn spawn_polling_thread(hwnd: HWND) {
    let hwnd2 = hwnd;
    let key = API_KEY.lock().unwrap().clone().unwrap_or_default();
    let interval = *REFRESH_INTERVAL.lock().unwrap();

    thread::spawn(move || {
        RUNNING.store(true, Ordering::SeqCst);

        // Initial fetch
        do_fetch(hwnd2, &key);

        // Periodic fetch
        while RUNNING.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(interval as u64));
            if !RUNNING.load(Ordering::SeqCst) {
                break;
            }
            do_fetch(hwnd2, &key);
        }
    });
}

fn do_fetch(hwnd: HWND, key: &str) {
    match balance::fetch_balance(key) {
        Ok((amount, currency)) => {
            let symbol = if currency == "USD" { "$" } else { "¥" };
            let display = format!("{}{:.2}", symbol, amount);
            let tooltip = format!("DeepSeek Balance: {}{:.2}", symbol, amount);
            *BALANCE_TEXT.lock().unwrap() = Some(tooltip);
            // Force immediate update
            PostMessageW(hwnd, WM_TIMER, WPARAM(0), LPARAM(0)).ok();
        }
        Err(e) => {
            *BALANCE_TEXT.lock().unwrap() = Some(format!("Error: {}", e));
            PostMessageW(hwnd, WM_BALANCE_ERROR, WPARAM(0), LPARAM(0)).ok();
        }
    }
}

fn load_interval() -> u32 {
    use windows::Win32::System::Registry::*;
    unsafe {
        let mut hkey = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            w!("Software\\DeepSeekBalance"),
            0, KEY_READ, &mut hkey,
        ).is_ok() {
            let mut data: u32 = 0;
            let mut data_size: u32 = 4;
            if RegQueryValueExW(hkey, w!("RefreshIntervalSec"), None, None,
                Some(&mut data as *mut _ as *mut u8), Some(&mut data_size),
            ).is_ok() {
                RegCloseKey(hkey).ok();
                if data >= 60 && data <= 3600 { return data; }
            }
            RegCloseKey(hkey).ok();
        }
    }
    300 // default 5 min
}

fn save_interval(interval: u32) -> Result<()> {
    use windows::Win32::System::Registry::*;
    unsafe {
        let mut hkey = HKEY::default();
        RegCreateKeyExW(
            HKEY_CURRENT_USER,
            w!("Software\\DeepSeekBalance"),
            0, None, REG_OPTION_NON_VOLATILE, KEY_WRITE, None, &mut hkey, None,
        ).ok()?;
        let data = interval.to_le_bytes();
        RegSetValueExW(hkey, w!("RefreshIntervalSec"), 0, REG_DWORD, Some(&data)).ok()?;
        RegCloseKey(hkey).ok();
    }
    Ok(())
}
