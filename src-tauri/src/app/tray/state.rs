use super::model::{TrayProxyMode, TrayRuntimeStateInput};

#[derive(Debug, Clone)]
pub struct TrayRuntimeState {
    pub kernel_running: bool,
    pub system_proxy_enabled: bool,
    pub tun_enabled: bool,
    pub active_subscription_name: Option<String>,
    pub locale: String,
    pub window_visible: bool,
    pub last_visible_route: String,
}

impl Default for TrayRuntimeState {
    fn default() -> Self {
        Self {
            kernel_running: false,
            system_proxy_enabled: false,
            tun_enabled: false,
            active_subscription_name: None,
            locale: "en-US".to_string(),
            window_visible: true,
            last_visible_route: "/".to_string(),
        }
    }
}

impl TrayRuntimeState {
    pub fn apply_sync_payload(&mut self, payload: TrayRuntimeStateInput) -> bool {
        let mut changed = false;

        if self.kernel_running != payload.kernel_running {
            self.kernel_running = payload.kernel_running;
            changed = true;
        }

        if self.system_proxy_enabled != payload.system_proxy_enabled {
            self.system_proxy_enabled = payload.system_proxy_enabled;
            changed = true;
        }

        if self.tun_enabled != payload.tun_enabled {
            self.tun_enabled = payload.tun_enabled;
            changed = true;
        }

        let next_name = payload
            .active_subscription_name
            .map(|name| name.trim().to_string())
            .filter(|name| !name.is_empty());
        if self.active_subscription_name != next_name {
            self.active_subscription_name = next_name;
            changed = true;
        }

        let next_locale = normalize_locale(&payload.locale);
        if self.locale != next_locale {
            self.locale = next_locale;
            changed = true;
        }

        if self.window_visible != payload.window_visible {
            self.window_visible = payload.window_visible;
            changed = true;
        }

        changed
    }

    pub fn set_last_visible_route(&mut self, path: &str) -> bool {
        let normalized = normalize_route(path);
        if normalized == self.last_visible_route {
            return false;
        }
        self.last_visible_route = normalized;
        true
    }

    pub fn set_window_visible(&mut self, visible: bool) -> bool {
        if self.window_visible == visible {
            return false;
        }
        self.window_visible = visible;
        true
    }

    pub fn display_mode(&self) -> TrayProxyMode {
        if self.tun_enabled {
            TrayProxyMode::Tun
        } else if self.system_proxy_enabled {
            TrayProxyMode::System
        } else {
            TrayProxyMode::Manual
        }
    }
}

fn normalize_locale(locale: &str) -> String {
    let trimmed = locale.trim();
    if trimmed.is_empty() {
        "en-US".to_string()
    } else {
        trimmed.to_string()
    }
}

fn normalize_route(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() || trimmed == "/blank" {
        "/".to_string()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{}", trimmed)
    }
}
