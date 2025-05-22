/*
user.email=hatfield.69@wright.edu
user.name=ReeseHatfield
core.editor=/usr/bin/vim
*/

use std::process::Command;

pub fn get_config(key: &str) -> Option<String> {
    let handle = Command::new("git")
        .arg("config")
        .arg("list")
        .output()
        .ok()?;

    let conf_out = String::from_utf8(handle.stdout).ok()?;

    conf_out.split("\n").find_map(|line| {
        // restrict to only the first 2, shouldnt matter but good convention
        let mut parts = line.splitn(2, '=');
        // the ? will still do the early return from closure
        let cur_key = parts.next()?;
        let value = parts.next()?;
        if cur_key == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}
