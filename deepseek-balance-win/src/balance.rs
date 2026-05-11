use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BalanceResponse {
    pub is_available: bool,
    pub balance_infos: Vec<BalanceInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct BalanceInfo {
    pub currency: String,
    pub total_balance: String,
    pub granted_balance: String,
    pub topped_up_balance: String,
}

/// Fetch DeepSeek balance. Returns (total_balance_f64, currency).
pub fn fetch_balance(api_key: &str) -> Result<(f64, String), String> {
    let resp: BalanceResponse = ureq::get("https://api.deepseek.com/user/balance")
        .set("Authorization", &format!("Bearer {}", api_key))
        .set("Cache-Control", "no-cache")
        .call()
        .map_err(|e| format!("Network: {}", e))?
        .into_json()
        .map_err(|e| format!("JSON: {}", e))?;

    let info = resp
        .balance_infos
        .iter()
        .find(|i| i.currency == "CNY")
        .or_else(|| resp.balance_infos.first())
        .ok_or("Empty balance")?;

    let total: f64 = info.total_balance.parse().map_err(|_| "Bad number")?;
    Ok((total, info.currency.clone()))
}
