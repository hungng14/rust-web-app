use regex::Regex;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub struct CheckTypeResult {
    pub ok: bool,
    pub kind: String
}

pub fn is_image_file(path: &String) -> CheckTypeResult {
    let re = Regex::new(r"/jpeg|jpg|png|gif|ico|svg/").unwrap();
    let tail = Path::new(&path).extension().unwrap();
    match tail.to_str() {
        Some(t) => {
            let _t = t.to_lowercase();
            if re.is_match(&_t) {
                let mut kind = String::from("image/jpeg");
                if &_t == "png" {
                    kind = String::from("image/png");
                } else if &_t == "gif" {
                    kind = String::from("image/gif");
                }else if &_t == "ico" {
                    kind = String::from("image/x-icon");
                }else if &_t == "svg" {
                    kind = String::from("image/svg+xml");
                }
                return CheckTypeResult {ok: true, kind};

            } else {
                CheckTypeResult {ok: false, kind: String::from("unknow")}
            }
        },
        None => CheckTypeResult {ok: false, kind: String::from("unknow")}
    }
}

pub fn is_js_file(path: &String) -> CheckTypeResult {
    let tail = Path::new(&path).extension().unwrap();
    match tail.to_str() {
        Some(t) => {
            let _t = t.to_lowercase();
            if _t == "js" {
                return CheckTypeResult {ok: true, kind: String::from("text/javascript")};

            } else {
                CheckTypeResult {ok: false, kind: String::from("unknow")}
            }
        },
        None => CheckTypeResult {ok: false, kind: String::from("unknow")}
    }
}

pub fn is_css_file(path: &String) -> CheckTypeResult {
    let tail = Path::new(&path).extension().unwrap();
    match tail.to_str() {
        Some(t) => {
            let _t = t.to_lowercase();
            if _t == "css" {
                return CheckTypeResult {ok: true, kind: String::from("text/css")};

            } else {
                CheckTypeResult {ok: false, kind: String::from("unknow")}
            }
        },
        None => CheckTypeResult {ok: false, kind: String::from("unknow")}
    }
}

pub fn get_content_type(path: String) -> String {
    let check = is_image_file(&path);
    if check.ok {
        return check.kind;
    }
    let check = is_css_file(&path);
    if check.ok {
        return check.kind;
    }

    let check = is_js_file(&path);
    if check.ok {
        return check.kind;
    }
    String::from("")
}

pub fn get_file_content(path: String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_img_content(path: String) -> std::io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}