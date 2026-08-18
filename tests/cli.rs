use std::path::PathBuf;
use std::process::Command;

fn exe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_oddsradar"))
}
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures").join(name)
}

#[test]
fn doctor_secret() {
    let out = Command::new(exe())
        .args(["doctor", "--config", fixture("config.secret.json").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!out.status.success());
    let s = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(!s.contains("PLANT-SECRET-DO-NOT-LOG"));
}

#[test]
fn wide_alerts_tight_does_not() {
    let out = Command::new(exe())
        .args([
            "compare",
            "--config",
            fixture("config.ok.json").to_str().unwrap(),
            "--map",
            fixture("markets.csv").to_str().unwrap(),
            "--quotes",
            fixture("quotes_wide.json").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"kind\":\"spread\"") || stdout.contains("\"kind\": \"spread\""));
    assert!(stdout.contains("btc-100k"));
    assert!(stdout.contains("fed-cut"));

    let tight = Command::new(exe())
        .args([
            "compare",
            "--config",
            fixture("config.ok.json").to_str().unwrap(),
            "--map",
            fixture("markets.csv").to_str().unwrap(),
            "--quotes",
            fixture("quotes_tight.json").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(tight.status.success());
    let t = String::from_utf8_lossy(&tight.stdout);
    assert!(!t.contains("\"kind\":\"spread\"") && !t.contains("\"kind\": \"spread\""));
}
