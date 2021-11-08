use std::{thread, fs, ptr};
use crate::extension;
use std::ffi::CStr;
use crate::state::State;
use std::process::Command;
use std::path::Path;
use directories::BaseDirs;

///
/// **MHV6.RS**
///
/// This file is meant for the GUI. I will provide examples so you can understand the insight.
/// Make sure to change any instances of "extension..." to your own mods name.
///
/// The top of the function renders last, while the bottom of the function renders first.
///
/// To find all the functions, refer to EXTENSION.RS.
/// I will update this repo whenever Absolute updates his MHV6 extensions repository.
///

pub fn mhv6_init() {
    while !extension::is_ready() { thread::sleep_ms(100); }
    let ext = extension::initialise_ext(b"Pitch Shifter\0".as_ptr());

    //This renders last.

    extension::add_button(ext, "Render\0".as_ptr(), button_cb);

    let textbox = extension::add_textbox(ext, textbox_cb);
    extension::set_textbox_placeholder(textbox, "Change (cents)\0".as_ptr());

    let id_textbox = extension::add_textbox(ext, id_textbox_cb);
    extension::set_textbox_placeholder(id_textbox, "Song ID\0".as_ptr());

    let version_textbox = extension::add_textbox(ext, version_textbox_cb);
    extension::set_textbox_placeholder(version_textbox, "Version: 0.1.0\0".as_ptr());

    //This renders first.

    extension::commit_ext(ext);
}

extern "stdcall" fn button_cb(ext: *mut ()) {
    let mut state = State::get();

    //sox input.mp3 output.mp3 pitch -10
    if let Some(base_dirs) = BaseDirs::new() {
        let path = base_dirs.cache_dir().to_str().unwrap().to_owned() + &*"\\GeometryDash\\".to_owned();
        fs::create_dir((format!("{}{}", path, "backup\\")).to_owned());

        if Path::new(format!("{}backup\\{}.mp3", path, state.id).as_str()).exists() {
            println!("true");
            Command::new("sox/sox.exe")
                .args(&[
                    format!("{}backup\\{}.mp3", &path, state.id).as_str(),
                    format!("{}{}.mp3", &path, state.id).as_str(),
                    "pitch",
                    &state.pitch_change.to_string(),
                ])
                .output()
                .expect("failed to execute process");
        } else {
            println!("false");
            fs::copy(format!("{}{}.mp3", &path, state.id), format!("{}backup\\{}.mp3", &path, state.id));
            println!("{}", format!("{}{}.mp3", &path, state.id));
            println!("{}", format!("{}backup\\{}.mp3", &path, state.id));
            Command::new("sox/sox.exe")
                .args(&[
                    format!("{}backup\\{}.mp3", &path, state.id).as_str(),
                    format!("{}{}.mp3", &path, state.id).as_str(),
                    "pitch",
                    &state.pitch_change.to_string(),
                ])
                .output()
                .expect("failed to execute process");
        }
    }

    println!("end");
}

extern "stdcall" fn id_textbox_cb(ext: *mut ()) {
    let mut state = State::get();
    unsafe {
        if let Ok(x) = CStr::from_ptr(extension::get_textbox_text(ext) as *const i8)
            .to_str()
            .unwrap()
            .parse::<String>()
        {
            state.id = x;
        }
    }
}

extern "stdcall" fn textbox_cb(ext: *mut ()) {
    let mut state = State::get();
    unsafe {
        if let Ok(x) = CStr::from_ptr(extension::get_textbox_text(ext) as *const i8)
            .to_str()
            .unwrap()
            .parse::<i32>()
        {
            state.pitch_change = x;
        }
    }
}

extern "stdcall" fn version_textbox_cb(ext: *mut ()) {
    extension::set_textbox_text(ext, "\0".as_ptr());
}