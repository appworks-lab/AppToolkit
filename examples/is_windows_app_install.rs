extern crate winreg;

use std::collections::HashSet;

use winreg::RegKey;
use winreg::enums::*;
use winreg::HKEY;

fn main() {
    let mut display_names_set:HashSet<String>  = HashSet::new();

    let paths: Vec<(HKEY, &str)> = vec![
        (HKEY_CURRENT_USER, "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
        (HKEY_LOCAL_MACHINE, "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
        (HKEY_LOCAL_MACHINE, "Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
    ];

    for path in paths {
        let display_names = get_app_display_names(path);
        for display_name in display_names {
            if !display_names_set.contains(&display_name) {
                display_names_set.insert(display_name);
            }
        }
    }
    println!("display_names_set: {:?}", display_names_set);
}

fn get_app_display_names(path: (HKEY, &str)) -> Vec<String> {
    let (hkey, path) = path;
    let mut display_names: Vec<String> = Vec::new();

    let hkcu = RegKey::predef(hkey);
    let uninstall = hkcu.open_subkey_with_flags(path, KEY_READ).expect("failed to open uninstall key");

    for key_result in uninstall.enum_keys().map(|x| x.unwrap()) {
        let key: RegKey = uninstall.open_subkey_with_flags(&key_result, KEY_READ).unwrap();

        if let Ok(display_name) = key.get_value::<String, _>("DisplayName") {
            display_names.push(display_name);
        }
    }
    display_names
}