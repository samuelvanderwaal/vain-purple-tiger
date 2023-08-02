use assert_cmd::Command;

#[test]
fn all_params_succeeds() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args([
        "-n",
        "test",
        "--cpus",
        "4",
        "-o",
        "/tmp/tmp_key",
        "words",
        "--color",
        "blue",
    ])
    .assert()
    .success();
}

#[test]
fn cmd_lists_succeeds() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("lists").assert().success();
}

#[test]
fn cmd_words_succeeds() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("words")
        .arg("--color")
        .arg("blue")
        .assert()
        .success();
}

#[test]
fn cmd_letter_succeeds() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("letter").arg("a").assert().success();
}

#[test]
fn cmd_regex_succeeds() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("regex")
        .arg("[a-z]+-[a-z]+-[a-z]+")
        .assert()
        .success();
}

#[test]
fn many_runs() {
    for _ in 0..250 {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("words")
            .arg("--animal")
            .arg("corgi")
            .assert()
            .success();
    }

    for _ in 0..250 {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("words")
            .arg("--color")
            .arg("red")
            .assert()
            .success();
    }
}
