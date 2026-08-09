mod indicators {
    use super::*;
    include!("indicators/registry/index.rs");
    include!("indicators/core-events/index.rs");
    include!("indicators/tailscale/index.rs");
    include!("indicators/internet/index.rs");
    include!("indicators/openvpn/index.rs");
    include!("indicators/services/index.rs");
    include!("indicators/power-meter/index.rs");
    include!("indicators/source-currency/index.rs");
}

use indicators::{collect_indicator_topic, render_indicator_modal_registry, render_indicator_strip};
