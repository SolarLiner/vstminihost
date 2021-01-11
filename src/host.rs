use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use vst::{
    host::{Host, PluginInstance, PluginLoadError, PluginLoader},
    plugin::Plugin,
};

#[derive(Default)]
struct VstHostImpl {
    plugin: Option<PluginInstance>,
}

impl VstHostImpl {
    fn load(&mut self, mut loader: PluginLoader<Self>) -> Result<(), PluginLoadError> {
        println!("Loading plugin instance");
        let mut instance = loader.instance()?;
        println!("Initializing plugin");
        instance.init();
        self.plugin.replace(instance);
        println!("Done");
        Ok(())
    }

    fn get_instance(&self) -> Option<&PluginInstance> {
        self.plugin.as_ref()
    }
}

#[derive(Default)]
pub struct VstHost(Arc<Mutex<VstHostImpl>>);

impl Host for VstHostImpl {}

impl VstHost {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn load<P: AsRef<Path>>(&self, path: P) -> Result<(), PluginLoadError> {
        let path = path.as_ref();

        let loader = PluginLoader::load(path, self.0.clone())?;
        self.0.lock().unwrap().load(loader)?;
        Ok(())
    }

    pub fn get_ref_map<T, F: FnOnce(&PluginInstance) -> T>(&self, func: F) -> Option<T> {
        self.0.lock().unwrap().get_instance().map(func)
    }
}
