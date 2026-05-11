use windows::{
    core::*,
    Win32::Security::Cryptography::*,
    Win32::System::Registry::*,
    Win32::Foundation::*,
};

const REG_PATH: &str = "Software\\DeepSeekBalance";
const RUN_PATH: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const APP_NAME: &str = "DeepSeekBalance";

/// Encrypt API key with DPAPI and store in HKCU registry.
pub fn save_key(key: &str) -> Result<()> {
    let data = encrypt(key.as_bytes())?;
    unsafe {
        let mut hkey = HKEY::default();
        RegCreateKeyExW(
            HKEY_CURRENT_USER,
            &encode_wide(REG_PATH),
            0,
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut hkey,
            None,
        ).ok()?;

        RegSetValueExW(
            hkey,
            w!("ApiKeyEncrypted"),
            0,
            REG_BINARY,
            Some(&data),
        ).ok()?;

        RegCloseKey(hkey).ok();
    }
    Ok(())
}

/// Load and decrypt API key from HKCU registry.
pub fn load_key() -> Option<String> {
    unsafe {
        let mut hkey = HKEY::default();
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            &encode_wide(REG_PATH),
            0,
            KEY_READ,
            &mut hkey,
        ).ok()?;

        let mut data: Vec<u8> = vec![0u8; 4096];
        let mut data_size: u32 = data.len() as u32;
        RegQueryValueExW(
            hkey,
            w!("ApiKeyEncrypted"),
            None,
            None,
            Some(data.as_mut_ptr()),
            Some(&mut data_size),
        ).ok()?;
        RegCloseKey(hkey).ok();

        data.truncate(data_size as usize);
        let decrypted = decrypt(&data).ok()?;
        String::from_utf8(decrypted).ok()
    }
}

/// Delete stored API key.
pub fn delete_key() {
    unsafe {
        let mut hkey = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            &encode_wide(REG_PATH),
            0,
            KEY_SET_VALUE,
            &mut hkey,
        ).is_ok() {
            RegDeleteValueW(hkey, w!("ApiKeyEncrypted")).ok();
            RegCloseKey(hkey).ok();
        }
    }
}

/// Enable or disable auto-start via registry Run key.
pub fn set_autostart(enabled: bool) -> Result<()> {
    unsafe {
        let mut hkey = HKEY::default();
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            &encode_wide(RUN_PATH),
            0,
            KEY_SET_VALUE,
            &mut hkey,
        ).ok()?;

        if enabled {
            let exe_path = std::env::current_exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            RegSetValueExW(
                hkey,
                &encode_wide(APP_NAME),
                0,
                REG_SZ,
                Some(exe_path.as_bytes()),
            ).ok()?;
        } else {
            RegDeleteValueW(hkey, &encode_wide(APP_NAME)).ok();
        }
        RegCloseKey(hkey).ok();
    }
    Ok(())
}

// ── DPAPI helpers ─────────────────────────────────────────────────

fn encrypt(data: &[u8]) -> Result<Vec<u8>> {
    unsafe {
        let input = CRYPTOAPI_BLOB {
            cbData: data.len() as u32,
            pbData: data.as_ptr() as *mut u8,
        };
        let mut output = CRYPTOAPI_BLOB::default();
        CryptProtectData(
            &input,
            w!("DeepSeekBalance"),
            None,
            None,
            None,
            0,
            &mut output,
        ).ok()?;

        let result = std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec();
        LocalFree(Some(HLOCAL(output.pbData as isize)));
        Ok(result)
    }
}

fn decrypt(data: &[u8]) -> Result<Vec<u8>> {
    unsafe {
        let input = CRYPTOAPI_BLOB {
            cbData: data.len() as u32,
            pbData: data.as_ptr() as *mut u8,
        };
        let mut output = CRYPTOAPI_BLOB::default();
        CryptUnprotectData(
            &input,
            None,
            None,
            None,
            None,
            0,
            &mut output,
        ).ok()?;

        let result = std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec();
        LocalFree(Some(HLOCAL(output.pbData as isize)));
        Ok(result)
    }
}

fn encode_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
