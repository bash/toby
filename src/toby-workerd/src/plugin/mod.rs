use libloading as lib;
use toby_plugin::Registry;

type RegistrarFn = fn(&mut Registry);

const PLUGIN_REGISTRAR_SYMBOL: &[u8] = b"plugin_registrar";

fn library_path(name: &str) -> String {
    format!(
        "{}{}{}",
        std::env::consts::DLL_PREFIX,
        name,
        std::env::consts::DLL_SUFFIX
    )
}

pub(crate) fn load_plugins(plugins: &[&str]) -> Registry {
    let mut registry = Registry::new();

    for plugin in plugins {
        let lib = lib::Library::new(library_path(plugin)).unwrap();

        let registrar_fn: lib::Symbol<'_, RegistrarFn> =
            unsafe { lib.get(PLUGIN_REGISTRAR_SYMBOL).unwrap() };

        registrar_fn(&mut registry);
    }

    registry
}
