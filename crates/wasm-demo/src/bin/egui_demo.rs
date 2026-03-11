#[path = "../../../egui-shadcn/examples/button/main.rs"]
mod egui_button_example;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> eframe::Result<()> {
    egui_button_example::main()
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
    egui_button_example::main();
}
