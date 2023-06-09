use std::env::consts::{ARCH, OS};
use std::env::current_dir;
use std::fs::{create_dir_all, File, remove_file};
use std::io::{Write};
use std::process::Command;
use rust_embed::RustEmbed;
use serde_json::Value;

pub mod install;
pub mod launcher;
pub mod network;
pub mod util;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct Asset;

// 御坂美琴生日快乐！
pub struct VersionSource {
    pub version_id: String,
    pub version_url: String,
    pub version_type: String,
    pub release_time: String
}

// 龟则检查函数
fn check_rule(rules: &Vec<Value>) -> bool {
    let mut allow = false;
    for rule in rules {
        // TODO: 跨平台以及更多的规则
        // https://doc.rust-lang.org/std/env/consts/constant.OS.html
        let allow_ = rule["action"].as_str().unwrap() == "allow";
        if rule.get("os").is_some() {
            // 系统名称
            if rule["os"].get("name").is_some() {
                let os = OS;
                let os_ = rule["os"]["name"].as_str().unwrap();

                if os == "windows" && os_ == "windows" {
                    allow |= allow_;
                } else if os == "linux" && os_ == "linux" {
                    allow |= allow_;
                } else if os == "macos" && os_ == "osx" {
                    allow |= allow_;
                } else {
                    allow |= false;
                }
            }
            if rule["os"].get("arch").is_some() {
                if ARCH.to_lowercase().find(&rule["os"]["arch"].as_str().unwrap().to_lowercase()).is_some() {
                    allow |= allow_;
                } else {
                    allow |= false;
                }
            }
        } else {
            allow |= allow_;
        }
    }
    return allow;
}

// 爪哇检查函数
pub fn check_java(java: &str, temp_dir: &str) -> Result<Value, String> {
    let temp_dir = current_dir().unwrap().join(temp_dir);
    create_dir_all(temp_dir.clone()).expect("Could not create .temp directory!");
    let temp_file_path = temp_dir.join("VersionChecker.class");
    let mut temp_file = File::create(temp_file_path.clone()).expect("Could not create temp file!");

    let class = Asset::get("java/VersionChecker.class").unwrap().data;
    temp_file.write(&*class.into_owned()).expect("Could not write to temp file!");

    let proc = Command::new(java)
        .current_dir(temp_dir)
        .arg("VersionChecker")
        .output()
        .expect("Could not execute Version Checker!");
    let stdout = std::str::from_utf8(&*proc.stdout).unwrap();
    let res: Value = serde_json::from_str(&*stdout.replace("\\", "\\\\")).unwrap();

    remove_file(temp_file_path.as_path()).expect("Could not delete temp file!");
    Ok(res)
}

pub fn merge_json(json: Value, inherit: Value) -> Result<Value, String> {
    let mut out = inherit.clone();
    for key in json.as_object().unwrap().keys() {
        match key.as_str() {
            "_comment_" => {
                out[key] = json[key].clone();
            }
            "id" => {
                out[key] = json[key].clone();
            }
            // "logging" => {
            //     out[key] = json[key].clone();
            // }
            "mainClass" => {
                out[key] = json[key].clone();
            }
            "releaseTime" => {
                out[key] = json[key].clone();
            }
            "time" => {
                out[key] = json[key].clone();
            }
            "type" => {
                out[key] = json[key].clone();
            }
            "arguments" => {
                if json["arguments"].get("jvm").is_some() {
                    let mut arr = inherit["arguments"]["jvm"].as_array().unwrap().clone();
                    for j in json["arguments"]["jvm"].as_array().unwrap() {
                        arr.push(j.clone());
                    }
                    out["arguments"]["jvm"] = Value::Array(arr);
                }
                if json["arguments"].get("game").is_some() {
                    let mut arr = inherit["arguments"]["game"].as_array().unwrap().clone();
                    for j in json["arguments"]["game"].as_array().unwrap() {
                        arr.push(j.clone());
                    }
                    out["arguments"]["game"] = Value::Array(arr);
                }
            }
            "libraries" => {
                let mut arr = inherit["libraries"].as_array().unwrap().clone();
                for j in json["libraries"].as_array().unwrap() {
                    arr.push(j.clone());
                }
                out["libraries"] = Value::Array(arr);
            }
            _ => {}
        }
    }
    Ok(out)
}

#[allow(dead_code)]
pub fn name_to_url(name: String) -> String {
    let split: Vec<&str> = name.split(":").collect();
    let package = split[0];
    let name = split[1];
    let version = split[2];
    return format!("{}/{}/{}/{}-{}.jar", package, name, version, name, version);
}

#[allow(dead_code)]
pub fn name_to_path(name: String) -> String {
    let split: Vec<&str> = name.split(":").collect();
    let package = split[0];
    let name = split[1];
    let version = split[2];
    return format!("{}/{}/{}/{}-{}.jar", String::from(package).replace(".", "/"), name, version, name, version);
}