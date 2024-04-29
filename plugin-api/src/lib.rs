#[stabby::stabby]
#[derive(Debug, Default)]
pub struct Settings {
  pub max_rows: u32,
}
unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

#[stabby::stabby]
#[derive(Debug, Default)]
pub struct InputDataset {
  pub rows: stabby::vec::Vec<f64>,
}
unsafe impl Send for InputDataset {}
unsafe impl Sync for InputDataset {}

#[stabby::stabby]
#[derive(Debug, Default)]
pub struct OutputDataset {
  pub rows: stabby::vec::Vec<f64>,
}
unsafe impl Send for OutputDataset {}
unsafe impl Sync for OutputDataset {}

#[stabby::stabby(checked)]
pub trait ExecutorHost {
  extern "C" fn get_input_dataset<'a>(
    &'a self, n: u32, limit: u32,
  ) -> stabby::future::DynFuture<'a, InputDataset>;
}

#[stabby::stabby(checked)]
pub trait ExecutorPlugin {
  extern "C" fn execute<'a>(
    &'a mut self, settings: Settings,
  ) -> stabby::future::DynFuture<'a, OutputDataset>;
}

pub type Host = stabby::dynptr!(stabby::sync::Arc<dyn ExecutorHost>);
pub type Plugin = stabby::dynptr!(stabby::boxed::Box<dyn ExecutorPlugin>);

#[stabby::stabby]
#[repr(C)]
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginError {
  #[error("unknown plugin error")]
  Unknown,
}
