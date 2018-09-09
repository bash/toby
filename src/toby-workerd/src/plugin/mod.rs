use libloading as lib;
use toby_plugin::job::Hook as JobHook;
use toby_plugin::Registry;

type RegistrarFn = fn(&mut dyn Registry);
const PLUGIN_REGISTRAR_SYMBOL: &[u8] = b"plugin_registrar";

#[derive(Default)]
pub(crate) struct RegistryImpl {
    pub(crate) job_hooks: Vec<Box<dyn JobHook>>,
}

impl Registry for RegistryImpl {
    fn register_job_hook(&mut self, hook: Box<dyn JobHook>) {
        self.job_hooks.push(hook);
    }
}

fn library_path(name: &str) -> String {
    format!(
        "{}{}{}",
        std::env::consts::DLL_PREFIX,
        name,
        std::env::consts::DLL_SUFFIX
    )
}

pub(crate) fn load_plugins(plugins: &[&str]) -> RegistryImpl {
    let mut registry = RegistryImpl::default();

    for plugin in plugins {
        let lib = lib::Library::new(library_path(plugin)).unwrap();

        let registrar_fn: lib::Symbol<'_, RegistrarFn> =
            unsafe { lib.get(PLUGIN_REGISTRAR_SYMBOL).unwrap() };

        registrar_fn(&mut registry);
    }

    registry
}
