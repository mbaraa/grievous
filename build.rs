extern crate cmake;
use cmake::Config;
use serde_json::Value;
use std::fs::{read_to_string, File};
use std::io::Write;

fn main() -> std::io::Result<()> {
    build_cmake();
    load_scales()?;
    Ok(())
}

fn build_cmake() {
    let dst = Config::new("libalsa").build();

    println!("cargo:rerun-if-changed=libalsa/alsa.c");
    println!("cargo:rerun-if-changed=libalsa/alsa.h");
    println!("cargo:rerun-if-changed=libalsa/CMakeLists.txt");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=alsa");
    println!("cargo:rustc-link-lib=dylib=asound");
}

fn load_scales() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=.env");

    // read le json.
    let scales_raw = read_to_string("./scales.json")?;
    let scales = serde_json::from_str::<Value>(&scales_raw)?;

    // write le rust file.
    let dest_path = "./src/music/scales.rs";
    let mut f = File::create(&dest_path).unwrap();

    // mental illness.
    f.write_all(b"// This file is automatically generated by build.rs\n\n")?;
    f.write_all(
        b"use super::Scale;
use std::collections::HashMap;

pub fn get_scales() -> Vec<Scale> {
    vec![
",
    )?;

    match scales {
        Value::Array(scales) => {
            scales
                .iter()
                .map(|scale| -> std::io::Result<()> {
                    let charset = match scale["charset"].clone() {
                        Value::Object(obj) => obj
                            .iter()
                            .map(|cs| {
                                (
                                    cs.0.as_bytes()[0] as char,
                                    match cs.1 {
                                        Value::Number(n) => n.as_f64().unwrap() as f32,
                                        _ => 440.0f32,
                                    },
                                )
                            })
                            .collect::<Vec<(char, f32)>>(),
                        _ => {
                            vec![('a', 1.0f32)]
                        }
                    };

                    f.write_all(
                        format!(
                            "\t\tScale::new({}.to_string(), HashMap::from([",
                            scale["name"],
                        )
                        .as_bytes(),
                    )?;

                    charset
                        .iter()
                        .map(|cs| f.write_all(format!("('{}', {:.2}), ", cs.0, cs.1).as_bytes()))
                        .collect::<std::io::Result<()>>()?;

                    f.write_all(b"])),\n")?;
                    Ok(())
                })
                .collect::<std::io::Result<()>>()?;
        }
        _ => {
            panic!("invalid scales.json");
        }
    }

    f.write_all(
        b"\t]
}",
    )?;

    Ok(())
}
