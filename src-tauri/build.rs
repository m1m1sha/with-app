use tauri_build::{try_build, Attributes, WindowsAttributes};

fn main() {
    try_build(
        Attributes::new().windows_attributes(
            WindowsAttributes::new().app_manifest(include_str!("manifest.xml")),
        ),
    )
    .expect("failed to run build script");
}
