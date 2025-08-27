use std::fs;
use std::env;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use anyhow::Result;
use serde_json::Value;
use tungstenite::connect;
use serde_json::json;

fn main() -> Result<()> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().unwrap();

    // find candidate .exe
    let mut candidates = vec![];
    for entry in fs::read_dir(exe_dir)? {
        let entry = entry?;
        if entry.path().extension().map(|e| e == "exe").unwrap_or(false) {
            if entry.path() != exe_path {
                candidates.push(entry.path());
            }
        }
    }

    // not first run: find original
    let self_name = exe_path.file_stem().unwrap().to_str().unwrap();
    let orig_backup = exe_dir.join(format!("{}_original.exe", self_name));

    if !candidates.is_empty() && !orig_backup.exists() {
        // first run: rename original exe -> *_original.exe
        let original = &candidates[0];
        let orig_name = original.file_stem().unwrap().to_str().unwrap();
        let orig_backup = exe_dir.join(format!("{}_original.exe", orig_name));

        println!("[Bootstrap] First run detected, renaming...");
        fs::rename(original, &orig_backup)?;

        // rename self -> <orig_name>.exe
        let new_self = exe_dir.join(format!("{}.exe", orig_name));
        fs::rename(&exe_path, &new_self)?;

        println!("[Bootstrap] Installed. Please relaunch via Steam.");
        return Ok(()); // stop here so user restarts
    }

    // launch game with debug port
    println!("[Bootstrap] Launching original game...");
    let mut child = Command::new(&orig_backup)
        .arg("--remote-debugging-port=9222")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // wait for DevTools
    let targets: Vec<Value> = loop {
        if let Ok(resp) = ureq::get("http://127.0.0.1:9222/json").call() {
            if let Ok(json) = resp.into_json::<Vec<Value>>() {
                if !json.is_empty() {
                    break json;
                }
            }
        }
        thread::sleep(Duration::from_secs(1));
    };

    let ws_url = targets[0]["webSocketDebuggerUrl"].as_str().unwrap().to_string();
    let (mut socket, _) = connect(ws_url)?;
    println!("[Bootstrap] Connected to debugger");

    // load script
    let script = include_str!("ChoiceScriptSavePlugin.js");

    let payload = json!({
        "id": 1,
        "method": "Runtime.evaluate",
        "params": {
            "expression": script,
        }
    });

    thread::sleep(Duration::from_secs(1));
    socket.send(tungstenite::Message::Text(payload.to_string()))?;
    println!("[Bootstrap] Script injected");

    let status = child.wait()?;
    println!("[Bootstrap] Game exited with {:?}", status);

    Ok(())
}
