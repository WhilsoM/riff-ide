trait EditorPlugin {
    fn on_load(&self);
    fn on_key_press(&self, key: char);
    fn name(&self) -> String;
}
