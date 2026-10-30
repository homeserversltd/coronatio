mod indicators {
    use super::*;
    include!("indicators/registry/index.rs");
    include!("indicators/stats-cache/index.rs");
    include!("indicators/core-events/index.rs");
    include!("indicators/tailscale/index.rs");
    include!("indicators/internet/index.rs");
    include!("indicators/openvpn/index.rs");
    include!("indicators/services/index.rs");
    include!("indicators/power-meter/index.rs");

    include!("indicators/notifications/index.rs");
}

pub(crate) use indicators::{
    caduceus_stats_sample, collect_indicator_topic, collect_indicator_topic_for_request,
    completed_current_indicator_projections, read_scoped_admin_stats_sample, render_indicator_modal_registry,
    render_indicator_strip, CaduceusStatsReadback, IndicatorTopicSample,
};
