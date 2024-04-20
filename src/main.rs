slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = App::new()?;
    let _uihandle = ui.as_weak();
    ui.run()
}
