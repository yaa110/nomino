use assert_cmd::Command;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;

#[cfg(not(target_os = "windows"))]
#[test]
fn test_map() {
    let dir = tempfile::tempdir().unwrap();

    let inputs = vec![
        "Nomino (2020) S1.E1.1080p.mkv",
        "Nomino (2020) S1.E2.1080p.mkv",
        "Nomino (2020) S1.E3.1080p.mkv",
        "Nomino (2020) S1.E4.1080p.mkv",
        "Nomino (2020) S1.E5.1080p.mkv",
    ];

    let mut outputs = vec![
        "01.mkv",
        "02.mkv",
        "03.mkv",
        "04.mkv",
        "05.mkv",
        "map.json",
        "undo.json",
    ];

    let mut outputs_undo = inputs.clone();
    outputs_undo.push("map.json");
    outputs_undo.push("undo.json");

    let mut map = File::create(dir.path().join("map.json")).unwrap();
    map.write_all(
        r#"{
        "Nomino (2020) S1.E1.1080p.mkv": "01.mkv",
        "Nomino (2020) S1.E2.1080p.mkv": "02.mkv",
        "Nomino (2020) S1.E3.1080p.mkv": "03.mkv",
        "Nomino (2020) S1.E4.1080p.mkv": "04.mkv",
        "Nomino (2020) S1.E5.1080p.mkv": "05.mkv"
    }"#
        .as_bytes(),
    )
    .unwrap();
    map.sync_all().unwrap();

    for input in inputs {
        let _ = File::create(dir.path().join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "-d",
            dir.path().to_str().unwrap(),
            "-m",
            "map.json",
            "-g",
            "undo.json",
        ])
        .unwrap();

    let mut files: Vec<String> = read_dir(dir.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    files.sort();
    outputs.sort();

    assert!(cmd.status.success());
    assert_eq!(files.len(), outputs.len());
    assert!(outputs.iter().zip(files.iter()).all(|(a, b)| a == b));

    let cmd_undo = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-d", dir.path().to_str().unwrap(), "-m", "undo.json"])
        .unwrap();

    let mut files_undo: Vec<String> = read_dir(dir.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    files_undo.sort();
    outputs_undo.sort();

    assert!(cmd_undo.status.success());
    assert_eq!(files_undo.len(), outputs.len());
    assert!(outputs_undo
        .iter()
        .zip(files_undo.iter())
        .all(|(a, b)| a == b));

    dir.close().unwrap();
}
