use plugin_api::{Host, Plugin, PluginError};

pub async fn init_plugin(host: Host) -> core::result::Result<Plugin, PluginError> {
  unsafe {
    let path = if cfg!(target_os = "linux") {
      "./target/release/libplugin_a.so"
    } else if cfg!(target_os = "windows") {
      "./target/release/plugin_a.dll"
    } else if cfg!(target_os = "macos") {
      "./target/release/libplugin_a.dylib"
    } else {
      ""
    };
    let lib = libloading::Library::new(path).unwrap_or_else(|e| {
      panic!(
        "{e}\n\nreaddir(./target/release)={:?}",
        std::fs::read_dir("./target/release")
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

    let result = init_plugin(host);

    result.into()
  }
}
