use plugin_api::HostTrait;

#[stabby::stabby]
pub struct MyExecutorHost;

unsafe impl Send for MyExecutorHost {}
unsafe impl Sync for MyExecutorHost {}

impl HostTrait for MyExecutorHost {
  extern "C" fn call_from_plugin<'a>(&'a self, a: u64) -> stabby::future::DynFuture<'a, u64> {
    stabby::boxed::Box::new(async move { a + 1 }).into()
  }
}
