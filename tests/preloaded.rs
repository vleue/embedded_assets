use std::path::Path;

use bevy_embedded_assets::EmbeddedAssetIo;

#[test]
fn check_preloaded_simple() {
    let embedded = EmbeddedAssetIo::preloaded();

    let path = "example_asset";

    let loaded = embedded.load_path_sync(&Path::new(path));
    assert!(loaded.is_ok());
    let raw_asset = loaded.unwrap();
    assert!(String::from_utf8(raw_asset.clone()).is_ok());
    assert_eq!(String::from_utf8(raw_asset).unwrap(), "hello");
}

#[test]
fn check_preloaded_special_chars() {
    let embedded = EmbeddedAssetIo::preloaded();

    let path = "açèt";

    let loaded = embedded.load_path_sync(&Path::new(path));
    assert!(loaded.is_ok());
    let raw_asset = loaded.unwrap();
    assert!(String::from_utf8(raw_asset.clone()).is_ok());
    assert_eq!(String::from_utf8(raw_asset).unwrap(), "with special chars");
}

#[test]
fn check_preloaded_subdir() {
    let embedded = EmbeddedAssetIo::preloaded();

    let path = "subdir/other_asset";

    let loaded = embedded.load_path_sync(&Path::new(path));
    assert!(loaded.is_ok());
    let raw_asset = loaded.unwrap();
    assert!(String::from_utf8(raw_asset.clone()).is_ok());
    assert_eq!(String::from_utf8(raw_asset).unwrap(), "in subdirectory");
}
