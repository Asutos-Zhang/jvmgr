use std::collections::HashMap;

use directories::BaseDirs;
use directories::UserDirs;

#[test]
fn test_some() {
    println!("tests test");
}

#[test]
fn test_directories_base() {
    if let Some(base_dirs) = BaseDirs::new() {
        println!("config {:?}", base_dirs.config_dir());
        println!("cache {:?}", base_dirs.cache_dir());
        println!("data {:?}", base_dirs.data_dir());
        println!("preference_dir {:?}", base_dirs.preference_dir());
    }
}

#[test]
fn test_directories_user() {
    if let Some(user_dirs) = UserDirs::new() {
        println!("home : {:?}", user_dirs.home_dir());
    }
}

#[test]
fn test_nil_2() {
    let mut map = HashMap::<String, String>::new();

    let key = "key".to_string();

    match map.get_mut(&key) {
        Some(value) => Some(value.to_owned()),
        None => map.insert(key, "".to_string()),
    };
}

#[test]
fn test_nil_1() {
    let mut data = vec!['a', 'b', 'c', 'd'];
    let slice = &mut data[..];

    p(slice);

    data.push('d');
    data.push('e');
    data.push('f');
}

fn p(a: &mut [char]) {
    println!("{:?}", a);
}



