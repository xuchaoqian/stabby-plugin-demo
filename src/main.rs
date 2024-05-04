mod host;
mod utils;

use host::MyExecutorHost;
use plugin_api::PluginTraitDyn;
use utils::init_plugin;

#[tokio::main]
async fn main() {
  log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

  match init_plugin(stabby::sync::Arc::new(MyExecutorHost).into()).await {
    Ok(plugin) => {
      let result = plugin.call_from_host(100).await;
      log::info!("got result: {:?}", result);
    }
    Err(err) => {
      log::error!("error occured: err: {:?}", err);
    }
  }
}
