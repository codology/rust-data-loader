use prometheus::{Encoder, TextEncoder, Registry, IntCounter};
use warp::Filter;

lazy_static::lazy_static! {
    pub static ref METRIC_REQUESTS: IntCounter = IntCounter::new("processed_files", "Number of processed files").unwrap();
    pub static ref REGISTRY: Registry = Registry::new_custom(Some("my_app".to_string()), None).unwrap();
}

pub fn init_metrics() {
    REGISTRY.register(Box::new(METRIC_REQUESTS.clone())).unwrap();

    tokio::spawn(async move {
        let metrics_route = warp::path("metrics")
            .map(|| {
                let encoder = TextEncoder::new();
                let mut buffer = vec![];
                encoder.encode(&REGISTRY.gather(), &mut buffer).unwrap();
                String::from_utf8(buffer).unwrap()
            });
        warp::serve(metrics_route).run(([0, 0, 0, 0], 9091)).await;
    });
}
