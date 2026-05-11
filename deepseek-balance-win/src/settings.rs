use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
};
const SETTINGS_CLASS: PCWSTR = w!("DeepSeekSettings");
const IDC_KEY_SECURE: u16 = 101;
const IDC_KEY_PLAIN: u16 = 102;
const IDC_SHOW: u16 = 103;
const IDC_INTERVAL: u16 = 104;
const IDC_SAVE: u16 = 105;
const IDC_KEY_LABEL: u16 = 106;

// Intervals in minutes
const INTERVALS: &[(&str, u32)] = &[
    ("1 minute", 1),
    ("5 minutes", 5),
    ("10 minutes", 10),
    ("30 minutes", 30),
    ("1 hour", 60),
];

/// Create and show the Settings window.  Returns new (api_key, interval_seconds) when saved.
pub fn show_settings(
    hinstance: HINSTANCE,
    current_key: &str,
    current_interval_sec: u32,
) -> Option<(String, u32)> {
    // One-time window class registration
    register_class(hinstance).ok()?;

    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_DLGMODALFRAME,
            SETTINGS_CLASS,
            w!("Settings"),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
            CW_USEDEFAULT, CW_USEDEFAULT, 380, 230,
            None, None, hinstance, None,
        )
    }.ok()?;

    // Store initial values as window properties
    let interval_sec: isize = current_interval_sec as isize;
    unsafe {
        SetPropW(hwnd, w!("INTERVAL"), Some(HANDLE(interval_sec)));
    }

    // Center on screen
    center_window(hwnd);

    // Populate controls
    unsafe {
        SetWindowTextW(GetDlgItem(hwnd, IDC_KEY_SECURE), &encode_wide(current_key));
        SetWindowTextW(GetDlgItem(hwnd, IDC_KEY_PLAIN), &encode_wide(current_key));

        let interval_min = current_interval_sec / 60;
        let cb = GetDlgItem(hwnd, IDC_INTERVAL);
        for (i, (_, mins)) in INTERVALS.iter().enumerate() {
            if *mins == interval_min {
                SendMessageW(cb, CB_SETCURSEL, WPARAM(i as _), LPARAM(0));
                break;
            }
        }
    }

    // Run local message loop until window closes
    let mut result: Option<(String, u32)> = None;

    unsafe {
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, &mut result as *mut _ as isize);
    }

    let mut msg = MSG::default();
    loop {
        let ret = unsafe { GetMessageW(&mut msg, None, 0, 0) };
        match ret {
            BOOL(0) | BOOL(-1) => break,
            _ => {}
        }
        unsafe {
            if !IsDialogMessageW(hwnd, &msg).as_bool() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    result
}

unsafe extern "system" fn settings_proc(
    hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let hinstance = (*(lparam.0 as *const CREATESTRUCTW)).hInstance;

            // Label
            CreateWindowExW(
                WS_EX_NONE, w!("STATIC"), w!("DeepSeek API Key:"),
                WS_CHILD | WS_VISIBLE,
                20, 185, 340, 18,
                hwnd, HMENU(IDC_KEY_LABEL as _), hinstance, None,
            );

            // Secure edit
            CreateWindowExW(
                WS_EX_CLIENTEDGE, w!("EDIT"), PCWSTR::null(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | ES_AUTOHSCROLL | ES_PASSWORD,
                20, 158, 340, 22,
                hwnd, HMENU(IDC_KEY_SECURE as _), hinstance, None,
            );

            // Plain edit (hidden)
            CreateWindowExW(
                WS_EX_CLIENTEDGE, w!("EDIT"), PCWSTR::null(),
                WS_CHILD | WS_TABSTOP | ES_AUTOHSCROLL,
                20, 158, 340, 22,
                hwnd, HMENU(IDC_KEY_PLAIN as _), hinstance, None,
            );

            // Show checkbox
            CreateWindowExW(
                WS_EX_NONE, w!("BUTTON"), w!("Show API Key"),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_AUTOCHECKBOX,
                20, 128, 140, 20,
                hwnd, HMENU(IDC_SHOW as _), hinstance, None,
            );

            // Interval label
            CreateWindowExW(
                WS_EX_NONE, w!("STATIC"), w!("Refresh Interval:"),
                WS_CHILD | WS_VISIBLE,
                20, 95, 340, 18,
                hwnd, None, hinstance, None,
            );

            // Combo box
            let cb = CreateWindowExW(
                WS_EX_NONE, w!("COMBOBOX"), PCWSTR::null(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | CBS_DROPDOWNLIST,
                20, 66, 140, 100,
                hwnd, HMENU(IDC_INTERVAL as _), hinstance, None,
            );
            for (title, _) in INTERVALS {
                SendMessageW(cb, CB_ADDSTRING, WPARAM(0), LPARAM(encode_wide(title).as_ptr() as isize));
            }

            // Save button
            CreateWindowExW(
                WS_EX_NONE, w!("BUTTON"), w!("Save"),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_PUSHBUTTON,
                280, 28, 80, 24,
                hwnd, HMENU(IDC_SAVE as _), hinstance, None,
            );

            // Set font for all children
            let hfont = CreateFontW(
                13, 0, 0, 0, FW_NORMAL.0 as i32,
                0, 0, 0, DEFAULT_CHARSET as u32, OUT_DEFAULT_PRECIS as u32,
                CLIP_DEFAULT_PRECIS as u32, DEFAULT_QUALITY as u32,
                DEFAULT_PITCH as u32, w!("Segoe UI"),
            );
            EnumChildWindows(hwnd, Some(set_child_font), LPARAM(hfont.0));

            LRESULT(0)
        }

        WM_COMMAND => {
            let id = (wparam.0 & 0xFFFF) as u16;
            match id {
                IDC_SHOW => {
                    let checked = SendMessageW(
                        GetDlgItem(hwnd, IDC_SHOW), BM_GETCHECK, WPARAM(0), LPARAM(0)
                    ).0 != 0;
                    ShowWindow(GetDlgItem(hwnd, IDC_KEY_SECURE), if checked { SW_HIDE } else { SW_SHOW });
                    ShowWindow(GetDlgItem(hwnd, IDC_KEY_PLAIN), if checked { SW_SHOW } else { SW_HIDE });
                    if checked {
                        let mut buf = [0u16; 256];
                        GetWindowTextW(GetDlgItem(hwnd, IDC_KEY_SECURE), &mut buf);
                        SetWindowTextW(GetDlgItem(hwnd, IDC_KEY_PLAIN), &buf);
                    } else {
                        let mut buf = [0u16; 256];
                        GetWindowTextW(GetDlgItem(hwnd, IDC_KEY_PLAIN), &mut buf);
                        SetWindowTextW(GetDlgItem(hwnd, IDC_KEY_SECURE), &buf);
                    }
                    LRESULT(0)
                }

                IDC_SAVE => {
                    let mut key_buf = [0u16; 256];
                    // Read from whichever is visible
                    let secure_hidden = !IsWindowVisible(GetDlgItem(hwnd, IDC_KEY_SECURE)).as_bool();
                    let src_id = if secure_hidden { IDC_KEY_PLAIN } else { IDC_KEY_SECURE };
                    GetWindowTextW(GetDlgItem(hwnd, src_id), &mut key_buf);
                    let key = String::from_utf16_lossy(&key_buf)
                        .trim_end_matches('\0')
                        .trim()
                        .to_string();

                    if key.is_empty() {
                        // Flash the field
                        FlashWindow(hwnd, true);
                        return LRESULT(0);
                    }

                    // Read interval
                    let cb = GetDlgItem(hwnd, IDC_INTERVAL);
                    let sel = SendMessageW(cb, CB_GETCURSEL, WPARAM(0), LPARAM(0)).0 as usize;
                    let interval_sec = if sel < INTERVALS.len() {
                        INTERVALS[sel].1 * 60
                    } else {
                        300
                    };

                    // Write result back through userdata
                    let result_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Option<(String, u32)>;
                    if !result_ptr.is_null() {
                        unsafe { *result_ptr = Some((key, interval_sec)); }
                    }

                    DestroyWindow(hwnd);
                    LRESULT(0)
                }

                _ => LRESULT(0),
            }
        }

        WM_CLOSE => {
            DestroyWindow(hwnd);
            LRESULT(0)
        }

        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }

        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe extern "system" fn set_child_font(hwnd: HWND, lparam: LPARAM) -> BOOL {
    SendMessageW(hwnd, WM_SETFONT, WPARAM(lparam.0 as usize), LPARAM(1));
    BOOL::from(true)
}

fn register_class(hinstance: HINSTANCE) -> Result<()> {
    // Check if already registered (ignore error if already exists)
    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(settings_proc),
        hInstance: hinstance.into(),
        lpszClassName: SETTINGS_CLASS,
        hbrBackground: GetSysColorBrush(COLOR_WINDOW),
        ..Default::default()
    };
    unsafe { RegisterClassW(&wc) };
    Ok(())
}

fn center_window(hwnd: HWND) {
    unsafe {
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).ok();
        let w = rect.right - rect.left;
        let h = rect.bottom - rect.top;
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        SetWindowPos(
            hwnd, None,
            (screen_w - w) / 2, (screen_h - h) / 2,
            0, 0,
            SWP_NOSIZE | SWP_NOZORDER,
        ).ok();
    }
}

fn encode_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
