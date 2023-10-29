use assert_cmd::Command;
use std::fs::read_dir;
use std::fs::File;

#[cfg(not(target_os = "windows"))]
#[test]
fn test_sort() {
    let dir = tempfile::tempdir().unwrap();

    let inputs = vec![
        "Nomino (2020) S1.E1.1080p.mkv",
        "Nomino (2020) S1.E2.1080p.mkv",
        "Nomino (2020) S1.E3.1080p.mkv",
        "Nomino (2020) S1.E4.1080p.mkv",
        "Nomino (2020) S1.E5.1080p.mkv",
    ];

    let mut outputs = vec!["001.mkv", "002.mkv", "003.mkv", "004.mkv", "005.mkv"];

    for input in inputs {
        let _ = File::create(dir.path().join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-d", dir.path().to_str().unwrap(), "-s", "asc", "{:3}.mkv"])
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

    dir.close().unwrap();
}
