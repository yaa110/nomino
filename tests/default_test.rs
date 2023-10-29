use assert_cmd::Command;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::File;
use std::path::MAIN_SEPARATOR;

#[cfg(not(target_os = "windows"))]
#[test]
fn test_default() {
    let dir = tempfile::tempdir().unwrap();

    let inputs = vec![
        "Nomino (2020) S1.E1.1080p.mkv",
        "Nomino (2020) S1.E2.1080p.mkv",
        "Nomino (2020) S1.E3.1080p.mkv",
        "Nomino (2020) S1.E4.1080p.mkv",
        "Nomino (2020) S1.E5.1080p.mkv",
    ];

    let mut outputs = vec!["01.mkv", "02.mkv", "03.mkv", "04.mkv", "05.mkv"];

    for input in inputs {
        let _ = File::create(dir.path().join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "-d",
            dir.path().to_str().unwrap(),
            r".*E(\d+).*",
            "{:2}.mkv",
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

    dir.close().unwrap();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_default_not_overwrite() {
    let dir = tempfile::tempdir().unwrap();

    let inputs = vec![
        "Nomino (2020) S1.E1.1080p.mkv",
        "Nomino (2020) S1.E2.1080p.mkv",
        "Nomino (2020) S1.E3.1080p.mkv",
        "Nomino (2020) S1.E4.1080p.mkv",
        "Nomino (2020) S1.E5.1080p.mkv",
    ];

    let mut outputs = vec!["1.mkv", "_1.mkv", "__1.mkv", "___1.mkv", "____1.mkv"];

    for input in inputs {
        let _ = File::create(dir.path().join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-d", dir.path().to_str().unwrap(), r".*E(\d+).*", "1.mkv"])
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

#[cfg(not(target_os = "windows"))]
#[test]
fn test_default_overwrite() {
    let dir = tempfile::tempdir().unwrap();

    let inputs = vec![
        "Nomino (2020) S1.E1.1080p.mkv",
        "Nomino (2020) S1.E2.1080p.mkv",
        "Nomino (2020) S1.E3.1080p.mkv",
        "Nomino (2020) S1.E4.1080p.mkv",
        "Nomino (2020) S1.E5.1080p.mkv",
    ];

    let mut outputs = vec!["1.mkv"];

    for input in inputs {
        let _ = File::create(dir.path().join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "-d",
            dir.path().to_str().unwrap(),
            "-w",
            r".*E(\d+).*",
            "1.mkv",
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

    dir.close().unwrap();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_default_subdir() {
    let dir = tempfile::tempdir().unwrap();

    create_dir_all(dir.path().join("s1")).unwrap();
    create_dir_all(dir.path().join("s2")).unwrap();
    create_dir_all(dir.path().join("a")).unwrap();

    let inputs = vec![
        ("s1", "Nomino (2020) S1.E1.1080p.mkv"),
        ("s1", "Nomino (2020) S1.E2.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E3.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E4.1080p.mkv"),
        ("a", "Nomino (2020) S1.E5.1080p.mkv"),
    ];

    let mut outputs_01 = vec!["01.mkv", "02.mkv"];
    let mut outputs_02 = vec!["03.mkv", "04.mkv"];

    for (d, input) in inputs {
        let _ = File::create(dir.path().join(d).join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "-d",
            dir.path().to_str().unwrap(),
            "-k",
            format!(r"s(\d+){}.*E(\d+).*", MAIN_SEPARATOR).as_str(),
            format!("{{:2}}{}{{:2}}.mkv", MAIN_SEPARATOR).as_str(),
        ])
        .unwrap();

    let mut files_01: Vec<String> = read_dir(dir.path().join("01"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    let mut files_02: Vec<String> = read_dir(dir.path().join("02"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    files_01.sort();
    files_02.sort();
    outputs_01.sort();
    outputs_02.sort();

    assert!(cmd.status.success());
    assert_eq!(files_01.len(), outputs_01.len());
    assert!(outputs_01.iter().zip(files_01.iter()).all(|(a, b)| a == b));
    assert_eq!(files_02.len(), outputs_02.len());
    assert!(outputs_02.iter().zip(files_02.iter()).all(|(a, b)| a == b));

    dir.close().unwrap();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_default_subdir_depth() {
    let dir = tempfile::tempdir().unwrap();

    create_dir_all(dir.path().join("s1")).unwrap();
    create_dir_all(dir.path().join("s2")).unwrap();
    create_dir_all(dir.path().join("a")).unwrap();

    let inputs = vec![
        ("s1", "Nomino (2020) S1.E1.1080p.mkv"),
        ("s1", "Nomino (2020) S1.E2.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E3.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E4.1080p.mkv"),
        ("a", "Nomino (2020) S1.E5.1080p.mkv"),
    ];

    let mut outputs_01 = vec!["01.mkv", "02.mkv"];
    let mut outputs_02 = vec!["03.mkv", "04.mkv"];

    for (d, input) in inputs {
        let _ = File::create(dir.path().join(d).join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "--depth",
            "2",
            "-d",
            dir.path().to_str().unwrap(),
            "-k",
            format!(r"s(\d+){}.*E(\d+).*", MAIN_SEPARATOR).as_str(),
            format!("{{:2}}{}{{:2}}.mkv", MAIN_SEPARATOR).as_str(),
        ])
        .unwrap();

    let mut files_01: Vec<String> = read_dir(dir.path().join("01"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    let mut files_02: Vec<String> = read_dir(dir.path().join("02"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    files_01.sort();
    files_02.sort();
    outputs_01.sort();
    outputs_02.sort();

    assert!(cmd.status.success());
    assert_eq!(files_01.len(), outputs_01.len());
    assert!(outputs_01.iter().zip(files_01.iter()).all(|(a, b)| a == b));
    assert_eq!(files_02.len(), outputs_02.len());
    assert!(outputs_02.iter().zip(files_02.iter()).all(|(a, b)| a == b));

    dir.close().unwrap();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_default_subdir_max_depth() {
    let dir = tempfile::tempdir().unwrap();

    create_dir_all(dir.path().join("s1")).unwrap();
    create_dir_all(dir.path().join("s2")).unwrap();
    create_dir_all(dir.path().join("a")).unwrap();

    let inputs = vec![
        ("s1", "Nomino (2020) S1.E1.1080p.mkv"),
        ("s1", "Nomino (2020) S1.E2.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E3.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E4.1080p.mkv"),
        ("a", "Nomino (2020) S1.E5.1080p.mkv"),
    ];

    let mut outputs_01 = vec!["01.mkv", "02.mkv"];
    let mut outputs_02 = vec!["03.mkv", "04.mkv"];

    for (d, input) in inputs {
        let _ = File::create(dir.path().join(d).join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "--depth",
            "3",
            "--max-depth",
            "2",
            "-d",
            dir.path().to_str().unwrap(),
            "-k",
            format!(r"s(\d+){}.*E(\d+).*", MAIN_SEPARATOR).as_str(),
            format!("{{:2}}{}{{:2}}.mkv", MAIN_SEPARATOR).as_str(),
        ])
        .unwrap();

    let mut files_01: Vec<String> = read_dir(dir.path().join("01"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    let mut files_02: Vec<String> = read_dir(dir.path().join("02"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    files_01.sort();
    files_02.sort();
    outputs_01.sort();
    outputs_02.sort();

    assert!(cmd.status.success());
    assert_eq!(files_01.len(), outputs_01.len());
    assert!(outputs_01.iter().zip(files_01.iter()).all(|(a, b)| a == b));
    assert_eq!(files_02.len(), outputs_02.len());
    assert!(outputs_02.iter().zip(files_02.iter()).all(|(a, b)| a == b));

    dir.close().unwrap();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_default_subdir_not_overwrite() {
    let dir = tempfile::tempdir().unwrap();

    create_dir_all(dir.path().join("s1")).unwrap();
    create_dir_all(dir.path().join("s2")).unwrap();
    create_dir_all(dir.path().join("a")).unwrap();

    let inputs = vec![
        ("s1", "Nomino (2020) S1.E1.1080p.mkv"),
        ("s1", "Nomino (2020) S1.E2.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E3.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E4.1080p.mkv"),
        ("a", "Nomino (2020) S1.E5.1080p.mkv"),
    ];

    let mut outputs_01 = vec!["_1.mkv", "1.mkv"];
    let mut outputs_02 = vec!["_1.mkv", "1.mkv"];

    for (d, input) in inputs {
        let _ = File::create(dir.path().join(d).join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "-d",
            dir.path().to_str().unwrap(),
            "-k",
            format!(r"s(\d+){}.*E(\d+).*", MAIN_SEPARATOR).as_str(),
            format!("{{:2}}{}1.mkv", MAIN_SEPARATOR).as_str(),
        ])
        .unwrap();

    let mut files_01: Vec<String> = read_dir(dir.path().join("01"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    let mut files_02: Vec<String> = read_dir(dir.path().join("02"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    files_01.sort();
    files_02.sort();
    outputs_01.sort();
    outputs_02.sort();

    assert!(cmd.status.success());
    assert_eq!(files_01.len(), outputs_01.len());
    assert!(outputs_01.iter().zip(files_01.iter()).all(|(a, b)| a == b));
    assert_eq!(files_02.len(), outputs_02.len());
    assert!(outputs_02.iter().zip(files_02.iter()).all(|(a, b)| a == b));

    dir.close().unwrap();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_default_subdir_overwrite() {
    let dir = tempfile::tempdir().unwrap();

    create_dir_all(dir.path().join("s1")).unwrap();
    create_dir_all(dir.path().join("s2")).unwrap();
    create_dir_all(dir.path().join("a")).unwrap();

    let inputs = vec![
        ("s1", "Nomino (2020) S1.E1.1080p.mkv"),
        ("s1", "Nomino (2020) S1.E2.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E3.1080p.mkv"),
        ("s2", "Nomino (2020) S1.E4.1080p.mkv"),
        ("a", "Nomino (2020) S1.E5.1080p.mkv"),
    ];

    let mut outputs_01 = vec!["1.mkv"];
    let mut outputs_02 = vec!["1.mkv"];

    for (d, input) in inputs {
        let _ = File::create(dir.path().join(d).join(input)).unwrap();
    }

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "-d",
            dir.path().to_str().unwrap(),
            "-k",
            "-w",
            format!(r"s(\d+){}.*E(\d+).*", MAIN_SEPARATOR).as_str(),
            format!("{{:2}}{}1.mkv", MAIN_SEPARATOR).as_str(),
        ])
        .unwrap();

    let mut files_01: Vec<String> = read_dir(dir.path().join("01"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    let mut files_02: Vec<String> = read_dir(dir.path().join("02"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    files_01.sort();
    files_02.sort();
    outputs_01.sort();
    outputs_02.sort();

    assert!(cmd.status.success());
    assert_eq!(files_01.len(), outputs_01.len());
    assert!(outputs_01.iter().zip(files_01.iter()).all(|(a, b)| a == b));
    assert_eq!(files_02.len(), outputs_02.len());
    assert!(outputs_02.iter().zip(files_02.iter()).all(|(a, b)| a == b));

    dir.close().unwrap();
}
