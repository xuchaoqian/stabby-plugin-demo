use plugin_api::{
  ExecutorHost, ExecutorPluginDynMut, Host, InputDataset, OutputDataset, Plugin, PluginError,
  Settings,
};

#[stabby::stabby]
struct MyExecutorHost;

impl MyExecutorHost {
  async fn get_from_store(&self) -> InputDataset {
    let mut rows = stabby::vec::Vec::new();
    rows.push(1.0);
    rows.push(2.0);
    rows.push(3.0);
    InputDataset { rows }
  }
}

impl ExecutorHost for MyExecutorHost {
  extern "C" fn get_input_dataset<'a>(
    &'a self, n: u32, limit: u32,
  ) -> stabby::future::DynFuture<'a, InputDataset> {
    stabby::boxed::Box::new(async move {
      log::info!("get_input_dataset: n={}, limit={}", n, limit);
      self.get_from_store().await
    })
    .into()
  }
}

unsafe impl Send for MyExecutorHost {}
unsafe impl Sync for MyExecutorHost {}

async fn init_plugin() -> core::result::Result<Plugin, PluginError> {
  unsafe {
    let path = if cfg!(target_os = "linux") {
      "./target/debug/libplugin_a.so"
    } else if cfg!(target_os = "windows") {
      "./target/debug/plugin_a.dll"
    } else if cfg!(target_os = "macos") {
      "./target/debug/libplugin_a.dylib"
    } else {
      ""
    };
    let lib = libloading::Library::new(path).unwrap_or_else(|e| {
      panic!(
        "{e}\n\nreaddir(./target/debug)={:?}",
        std::fs::read_dir("./target/debug")
          .map(|d| d.map(|f| f.unwrap().file_name()).collect::<Vec<_>>())
      )
    });

    use stabby::libloading::StabbyLibrary;
    let init_plugin: stabby::libloading::Symbol<
      extern "C" fn(Host) -> stabby::result::Result<Plugin, PluginError>,
    > = lib
      .get_stabbied::<extern "C" fn(Host) -> stabby::result::Result<Plugin, PluginError>>(
        b"init_plugin",
      )
      .unwrap();
    let result = init_plugin(stabby::sync::Arc::new(MyExecutorHost).into());

    result.into()
  }
}

#[tokio::main]
async fn main() {
  log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

  match init_plugin().await {
    Ok(mut plugin) => {
      let dataset: OutputDataset = plugin.execute(Settings { max_rows: 100 }).await;
      log::info!("got output dataset: {:?}", dataset);
    }
    Err(err) => {
      log::error!("error occured: err: {:?}", err);
    }
  }
}
