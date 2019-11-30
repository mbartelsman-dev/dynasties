use std::env;
use std::path::Path;
use std::fs;
use std::io::prelude::*;
use indoc::indoc;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let name_default_conf = indoc!(r#"
        [given_names]
        list = ["Mike", "Pete"]
        "#);

    fs::write(
        format!("{}{}", out_dir, "/names.toml"),
        name_default_conf
    ).expect("Unable to write data");
}
