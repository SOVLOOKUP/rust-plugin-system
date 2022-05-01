use dynamic_reload::{DynamicReload, Lib, PlatformName, Search, Symbol, UpdateState};
use my_interface::Plugin;
use std::{sync::Arc, thread, time::Duration};

struct Plugins {
    plugins: Vec<Box<dyn Plugin>>,
}

fn get_service(plugin: &Arc<Lib>) -> Box<dyn Plugin> {
    unsafe {
        plugin
            .lib
            .get::<Symbol<fn() -> Box<dyn Plugin>>>(b"new")
            .unwrap()()
    }
}

impl Plugins {
    fn load_plugin(&mut self, plugin: &Arc<Lib>) {
        let service = get_service(&plugin);
        service.load();
        self.plugins.push(service);
    }

    fn unload_plugin(&mut self, plugin: &Arc<Lib>) {
        let service = get_service(&plugin);
        let id = service.id();
        for i in (0..self.plugins.len()).rev() {
            let plug = &self.plugins[i];
            if plug.id() == id {
                plug.unload();
                self.plugins.swap_remove(i);
            }
        }
    }

    // called when a lib needs to be reloaded.
    fn reload_callback(&mut self, state: UpdateState, plugin: Option<&Arc<Lib>>) {
        match state {
            UpdateState::Before => Self::unload_plugin(self, plugin.unwrap()),
            UpdateState::After => Self::load_plugin(self, plugin.unwrap()),
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }
}

fn main() {
    let mut plugs = Plugins {
        plugins: Vec::new(),
    };
    let mut reload_handler = DynamicReload::new(
        Some(vec!["target/debug"]),
        Some("target/debug"),
        Search::Default,
        Duration::from_secs(2),
    );
    unsafe {
        match reload_handler.add_library("my_plugin", PlatformName::Yes) {
            Ok(lib) => {
                plugs.load_plugin(&lib);
            }
            Err(e) => {
                println!("Unable to load dynamic lib, err {:?}", e);
                return;
            }
        }

        loop {
            reload_handler.update(&Plugins::reload_callback, &mut plugs);

            // if plugs.plugins.len() > 0 {
            //     // In a real program you want to cache the symbol and not do it every time if your
            //     // application is performance critical
            //     let fun: Symbol<extern "C" fn() -> i32> =
            //         unsafe { plugs.plugins[0].lib.get(b"shared_fun\0").unwrap() };

            //     println!("Value {}", fun());
            // }

            // Wait for 0.5 sec
            thread::sleep(Duration::from_millis(500));
        }
    }
}
