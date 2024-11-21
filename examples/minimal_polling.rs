use log::{error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use transforms::types::{Duration, Quaternion, Registry, Timestamp, Transform, Vector3};

/// Dummy transform generator
fn generate_transform(t: Timestamp) -> Transform {
    let x = t.as_seconds().unwrap().sin();
    let y = t.as_seconds().unwrap().cos();
    let z = 0.;

    Transform {
        translation: Vector3 { x, y, z },
        rotation: Quaternion {
            w: 1.,
            x: 0.,
            y: 0.,
            z: 0.,
        },
        parent: "a".into(),
        child: "b".into(),
        timestamp: t,
    }
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let ttl = std::time::Duration::from_secs(10);
    let registry = Arc::new(Mutex::new(Registry::new(ttl.into())));

    let registry_writer = registry.clone();
    let writer = tokio::spawn(async move {
        loop {
            let time = Timestamp::now();
            let t = generate_transform(time);
            let mut r = registry_writer.lock().await;
            if let Err(e) = r.add_transform(t.clone()) {
                error!("Error adding transform: {:?}", e);
            }
            drop(r);
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    let registry_reader = registry.clone();
    let reader = tokio::spawn(async move {
        loop {
            let time = (Timestamp::now() - Duration::try_from(1.0).unwrap()).unwrap();
            let mut r = registry_reader.lock().await;
            let result = r.get_transform("a", "b", time);
            match result {
                Ok(tf) => info!("Found transform: {:?}", tf),
                Err(e) => error!("Transform not found: {:?}", e),
            }
            drop(r);
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    let _ = tokio::join!(writer, reader);
}
