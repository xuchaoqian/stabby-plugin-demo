// This is a struct that tells Criterion.rs to use the "futures" crate's current-thread executor

use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use plugin_api::{Plugin, PluginTraitDyn};
use rand::{Rng, SeedableRng};
use stabby_plugin_demo::{host::MyExecutorHost, utils::init_plugin};
use tokio::runtime::Runtime;

pub fn call_stabby_plugin(c: &mut Criterion) {
  let rt = Runtime::new().unwrap();
  let plugin: Arc<Plugin> =
    Arc::new(rt.block_on(async {
      init_plugin(stabby::sync::Arc::new(MyExecutorHost).into()).await.unwrap()
    }));
  let rng = rand::rngs::StdRng::seed_from_u64(0);

  c.bench_function("call stabby plugin", |b| {
    b.to_async(&rt).iter(|| async {
      let a = rng.clone().gen::<u64>();
      let _ = plugin.call_from_host(black_box(a)).await;
    });
  });
}

async fn call_from_plugin(a: u64) -> u64 {
  a + 1
}

async fn call_from_host(a: u64) -> u64 {
  call_from_plugin(a).await
}

pub fn call_async_function(c: &mut Criterion) {
  let rt = Runtime::new().unwrap();
  let rng = rand::rngs::StdRng::seed_from_u64(0);

  c.bench_function("call async function", |b| {
    b.to_async(&rt).iter(|| async {
      let a = rng.clone().gen::<u64>();
      let _ = call_from_host(black_box(a)).await;
    });
  });
}

criterion_group!(benches, call_stabby_plugin, call_async_function);
criterion_main!(benches);
