fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("./../assets/icons/icon_256x256.ico");
        res.compile().unwrap();
    }
}
