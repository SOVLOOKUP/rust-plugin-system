pub trait Plugin {
    // 插件ID
    fn id(&self) -> &str;
    // 插件加载
    fn load(&self);
    // 插件卸载
    fn unload(&self);
}
