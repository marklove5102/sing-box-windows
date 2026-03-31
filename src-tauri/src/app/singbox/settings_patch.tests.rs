use super::*;
use crate::app::storage::state_model::AppConfig;
use serde_json::{json, Value};

fn assert_inbounds_do_not_contain_legacy_fields(config: &Value) {
    let inbounds = config
        .get("inbounds")
        .and_then(|v| v.as_array())
        .expect("inbounds 应存在");

    for inbound in inbounds {
        for legacy_field in [
            "sniff",
            "sniff_override_destination",
            "sniff_timeout",
            "domain_strategy",
            "udp_disable_domain_unmapping",
        ] {
            assert!(
                inbound.get(legacy_field).is_none(),
                "inbound 不应包含 legacy 字段 {}: {:?}",
                legacy_field,
                inbound
            );
        }
    }
}

#[test]
fn apply_app_settings_should_remove_legacy_inbound_fields_from_existing_config() {
    let mut config = json!({
        "dns": {
            "servers": [],
            "rules": []
        },
        "experimental": {
            "clash_api": {},
            "cache_file": {}
        },
        "inbounds": [
            {
                "type": "mixed",
                "tag": "mixed-in",
                "listen": "127.0.0.1",
                "listen_port": 7890,
                "sniff": true,
                "sniff_override_destination": true,
                "sniff_timeout": "1s",
                "domain_strategy": "prefer_ipv4",
                "udp_disable_domain_unmapping": true
            }
        ],
        "route": {
            "rule_set": [],
            "rules": [
                {
                    "action": "sniff"
                }
            ],
            "final": "direct",
            "auto_detect_interface": true
        }
    });

    apply_app_settings_to_config(&mut config, &AppConfig::default());

    assert_inbounds_do_not_contain_legacy_fields(&config);
}

#[test]
fn apply_app_settings_should_not_reintroduce_legacy_fields_when_tun_enabled() {
    let mut config = json!({
        "dns": {
            "servers": [],
            "rules": []
        },
        "experimental": {
            "clash_api": {},
            "cache_file": {}
        },
        "inbounds": [],
        "route": {
            "rule_set": [],
            "rules": [
                {
                    "action": "sniff"
                }
            ],
            "final": "direct",
            "auto_detect_interface": true
        }
    });
    let app_config = AppConfig {
        tun_enabled: true,
        tun_enable_ipv6: true,
        ..AppConfig::default()
    };

    apply_app_settings_to_config(&mut config, &app_config);

    let inbounds = config
        .get("inbounds")
        .and_then(|v| v.as_array())
        .expect("inbounds 应存在");
    assert_eq!(inbounds.len(), 2, "启用 TUN 时应生成 mixed + tun 两个入站");
    assert_inbounds_do_not_contain_legacy_fields(&config);
}

#[test]
fn apply_app_settings_should_insert_sniff_route_for_legacy_configs() {
    let mut config = json!({
        "dns": {
            "servers": [],
            "rules": []
        },
        "experimental": {
            "clash_api": {},
            "cache_file": {}
        },
        "inbounds": [
            {
                "type": "mixed",
                "tag": "mixed-in",
                "listen": "127.0.0.1",
                "listen_port": 7890,
                "sniff": true,
                "sniff_override_destination": true
            }
        ],
        "route": {
            "rule_set": [],
            "rules": [],
            "final": "direct",
            "auto_detect_interface": true
        }
    });

    apply_app_settings_to_config(&mut config, &AppConfig::default());

    let rules = config
        .get("route")
        .and_then(|v| v.get("rules"))
        .and_then(|v| v.as_array())
        .expect("route.rules 应存在");
    assert!(
        rules.iter()
            .any(|rule| rule.get("action").and_then(|v| v.as_str()) == Some("sniff")),
        "旧配置迁移后应补充 sniff 规则: {:?}",
        rules
    );
}
