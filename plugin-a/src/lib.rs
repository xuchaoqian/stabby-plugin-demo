use plugin_api::{Host, HostTraitDyn, Plugin, PluginError, PluginTrait};

#[stabby::stabby]
struct MyPlugin {
  host: Host,
}

unsafe impl Send for MyPlugin {}
unsafe impl Sync for MyPlugin {}

impl PluginTrait for MyPlugin {
  extern "C" fn call_from_host<'a>(&'a self, a: u64) -> stabby::future::DynFuture<'a, u64> {
    stabby::boxed::Box::new(async move { self.host.call_from_plugin(a).await }).into()
  }
}

#[stabby::export]
extern "C" fn init_plugin(host: Host) -> stabby::result::Result<Plugin, PluginError> {
  stabby::result::Result::Ok(stabby::boxed::Box::new(MyPlugin { host }).into())
}

const _: () = {
  assert!(MyPlugin::has_optimal_layout());
  stabby::abi::assert_stable::<Plugin>();
  stabby::abi::assert_stable::<PluginError>();
  stabby::abi::assert_stable::<stabby::result::Result<Plugin, PluginError>>();
};
