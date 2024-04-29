use plugin_api::{
  ExecutorHostDyn, ExecutorPlugin, Host, InputDataset, OutputDataset, Plugin, PluginError, Settings,
};

#[stabby::stabby]
struct MyPlugin(Host);

unsafe impl Send for MyPlugin {}
unsafe impl Sync for MyPlugin {}

impl ExecutorPlugin for MyPlugin {
  extern "C" fn execute<'a>(
    &'a mut self, settings: Settings,
  ) -> stabby::future::DynFuture<'a, OutputDataset> {
    stabby::boxed::Box::new(async move {
      log::info!("execute: settings: {:?}", settings);
      let input_dataset = self.0.get_input_dataset(10, 10).await;
      log::info!("got input dataset: {:?}", input_dataset);
      self.calculate(input_dataset).await
    })
    .into()
  }
}

impl MyPlugin {
  async fn calculate(&mut self, input_dataset: InputDataset) -> OutputDataset {
    let mut rows = stabby::vec::Vec::new();
    for row in input_dataset.rows.iter() {
      rows.push(row * 2.0);
    }
    OutputDataset { rows }
  }
}

#[stabby::export]
pub extern "C" fn init_plugin(host: Host) -> stabby::result::Result<Plugin, PluginError> {
  log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

  stabby::result::Result::Ok(stabby::boxed::Box::new(MyPlugin(host)).into())
}
