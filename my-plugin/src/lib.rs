use my_interface::Plugin;

#[no_mangle]
pub fn new() -> Box<dyn Plugin> {
    Box::new(PluginSayHello { id: "3a90790e".to_string()})
}

pub struct PluginSayHello {
    id: String,
}

impl Plugin for PluginSayHello {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn load(&self) {
        println!("[{}] Created instance!", self.id);
    }

    fn unload(&self) {
        println!("[{}] Unload instance!", self.id);
    }
}
