#[stabby::stabby(checked)]
pub trait HostTrait {
  extern "C" fn call_from_plugin<'a>(&'a self, a: u64) -> stabby::future::DynFuture<'a, u64>;
}

#[stabby::stabby(checked)]
pub trait PluginTrait {
  extern "C" fn call_from_host<'a>(&'a self, a: u64) -> stabby::future::DynFuture<'a, u64>;
}

pub type Host = stabby::dynptr!(stabby::sync::Arc<dyn HostTrait>);
pub type Plugin = stabby::dynptr!(stabby::boxed::Box<dyn PluginTrait>);

#[stabby::stabby]
#[repr(C)]
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginError {
  #[error("unknown plugin error")]
  Unknown,
}
