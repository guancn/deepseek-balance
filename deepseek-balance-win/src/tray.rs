use windows::{
    core::*,
    Win32::Graphics::Gdi::*,
    Win32::System::SystemInformation::GetSystemMetrics,
    Win32::UI::Shell::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Foundation::*,
};
use std::mem;

pub struct TrayIcon {
    hwnd: HWND,
    uid: u32,
}

impl TrayIcon {
    pub fn new(hwnd: HWND, uid: u32) -> Self {
        Self { hwnd, uid }
    }

    /// Add icon to system tray with initial text.
    pub fn add(&self, text: &str) -> Result<()> {
        let icon = render_text_to_icon(text)?;
        let mut nid = notify_data(self.hwnd, self.uid);
        nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        nid.uCallbackMessage = WM_APP + 1;
        nid.hIcon = icon;

        let tip: Vec<u16> = encode_wide("DeepSeek Balance");
        nid.szTip[..tip.len().min(127)].copy_from_slice(&tip[..tip.len().min(127)]);

        unsafe { Shell_NotifyIconW(NIM_ADD, &nid) }.ok()?;
        Ok(())
    }

    /// Update the tray icon with new text (e.g. "¥12.34").
    pub fn set_text(&self, text: &str) -> Result<()> {
        let icon = render_text_to_icon(text)?;
        let mut nid = notify_data(self.hwnd, self.uid);
        nid.uFlags = NIF_ICON;
        nid.hIcon = icon;
        unsafe { Shell_NotifyIconW(NIM_MODIFY, &nid) }.ok()?;
        Ok(())
    }

    /// Update tooltip.
    pub fn set_tooltip(&self, tip: &str) {
        let mut nid = notify_data(self.hwnd, self.uid);
        nid.uFlags = NIF_TIP;
        let wide: Vec<u16> = encode_wide(tip);
        nid.szTip[..wide.len().min(127)].copy_from_slice(&wide[..wide.len().min(127)]);
        unsafe { Shell_NotifyIconW(NIM_MODIFY, &nid) }.ok();
    }

    /// Show right-click context menu at current cursor position.
    pub fn show_menu(&self, items: &[(&str, u32)]) -> u32 {
        unsafe {
            let menu = CreatePopupMenu().unwrap_or(HMENU::default());
            for (_i, (label, id)) in items.iter().enumerate() {
                let wide: Vec<u16> = encode_wide(label);
                AppendMenuW(menu, MF_STRING, *id, PCWSTR(wide.as_ptr())).ok();
            }
            // Fix separator items (signalled by empty string)
            for (i, (label, _)) in items.iter().enumerate() {
                if label.is_empty() && i > 0 {
                    ModifyMenuW(menu, i as u32, MF_BYPOSITION | MF_SEPARATOR, 0, PCWSTR::null()).ok();
                }
            }

            SetForegroundWindow(self.hwnd);
            let mut pt = POINT::default();
            GetCursorPos(&mut pt).ok();
            let cmd = TrackPopupMenu(
                menu,
                TPM_RETURNCMD | TPM_RIGHTBUTTON,
                pt.x, pt.y,
                0,
                self.hwnd,
                None,
            );
            PostMessageW(self.hwnd, WM_NULL, WPARAM(0), LPARAM(0)).ok();
            DestroyMenu(menu).ok();
            cmd
        }
    }

    pub fn remove(&self) {
        let nid = notify_data(self.hwnd, self.uid);
        unsafe { Shell_NotifyIconW(NIM_DELETE, &nid) }.ok();
    }
}

fn notify_data(hwnd: HWND, uid: u32) -> NOTIFYICONDATAW {
    NOTIFYICONDATAW {
        cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: uid,
        ..Default::default()
    }
}

/// Render a short string (e.g. "¥12.34") to a system-tray-sized HICON.
fn render_text_to_icon(text: &str) -> Result<HICON> {
    unsafe {
        let cx = GetSystemMetrics(SM_CXSMICON);
        let cy = GetSystemMetrics(SM_CYSMICON);
        let hdc_screen = GetDC(None); // HWND::default() = None
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbm_color = CreateCompatibleBitmap(hdc_screen, cx, cy);
        let hbm_mask = CreateCompatibleBitmap(hdc_screen, cx, cy);

        // Detect dark mode and pick colors
        let (bg, fg) = detect_theme_colors();

        ReleaseDC(None, hdc_screen);

        // Draw color bitmap
        let old_bmp = SelectObject(hdc_mem, hbm_color);
        let bg_brush = CreateSolidBrush(COLORREF(bg.0 | (bg.1 as u32) << 8 | (bg.2 as u32) << 16));
        let bg_pen = CreatePen(PS_SOLID, 1, COLORREF(bg.0 | (bg.1 as u32) << 8 | (bg.2 as u32) << 16));
        let old_pen = SelectObject(hdc_mem, bg_pen);
        let rect = RECT { left: 0, top: 0, right: cx, bottom: cy };
        FillRect(hdc_mem, &rect, bg_brush);

        // Create font
        let font_height = if cy < 20 { cy - 4 } else { cy - 6 };
        let hfont = CreateFontW(
            font_height, 0, 0, 0, FW_NORMAL.0 as i32,
            0, 0, 0, DEFAULT_CHARSET as u32, OUT_DEFAULT_PRECIS as u32,
            CLIP_DEFAULT_PRECIS as u32, DEFAULT_QUALITY as u32,
            DEFAULT_PITCH as u32, w!("Consolas"),
        );
        SelectObject(hdc_mem, hfont);

        SetBkMode(hdc_mem, TRANSPARENT);
        SetTextColor(hdc_mem, COLORREF(fg.0 | (fg.1 as u32) << 8 | (fg.2 as u32) << 16));

        let mut text_rect = RECT { left: 1, top: 1, right: cx - 1, bottom: cy - 1 };
        let wide: Vec<u16> = encode_wide(text);
        DrawTextW(hdc_mem, &wide, &mut text_rect, DT_CENTER | DT_VCENTER | DT_SINGLELINE | DT_NOCLIP);

        SelectObject(hdc_mem, old_bmp);
        SelectObject(hdc_mem, old_pen);
        DeleteObject(bg_pen);
        DeleteObject(bg_brush);
        DeleteObject(hfont);

        // Draw mask bitmap (all white = fully opaque)
        let old_bmp2: HGDIOBJ = SelectObject(hdc_mem, hbm_mask);
        let white_brush = CreateSolidBrush(COLORREF(0x00FFFFFF));
        FillRect(hdc_mem, &rect, white_brush);
        SelectObject(hdc_mem, old_bmp2);
        DeleteObject(white_brush);

        DeleteDC(hdc_mem);

        let icon_info = ICONINFO {
            fIcon: true.into(),
            hbmColor: hbm_color,
            hbmMask: hbm_mask,
            ..Default::default()
        };

        let icon = CreateIconIndirect(&icon_info);
        DeleteObject(hbm_color);
        DeleteObject(hbm_mask);

        Ok(icon?)
    }
}

fn detect_theme_colors() -> ((u8, u8, u8), (u8, u8, u8)) {
    // Try to detect dark mode via registry
    let dark = is_dark_mode();
    if dark {
        ((32, 32, 32), (255, 255, 255))  // dark bg, white text
    } else {
        ((240, 240, 240), (0, 0, 0))     // light bg, black text
    }
}

fn is_dark_mode() -> bool {
    use windows::Win32::System::Registry::*;
    unsafe {
        let mut key = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            w!("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize"),
            0,
            KEY_READ,
            &mut key,
        ).is_ok() {
            let mut data: u32 = 0;
            let mut data_size: u32 = 4;
            if RegQueryValueExW(
                key,
                w!("AppsUseLightTheme"),
                None,
                None,
                Some(&mut data as *mut _ as *mut u8),
                Some(&mut data_size),
            ).is_ok() {
                RegCloseKey(key).ok();
                return data == 0; // 0 = dark mode
            }
            RegCloseKey(key).ok();
        }
    }
    false
}

fn encode_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
