use predicates::prelude::*;

// Integration test: run the binary in headless mode to execute the simulation path without a terminal UI.
#[test]
fn test_auto_ui_headless_runs_and_prints_summary() {
    // Use the recommended cargo helper to get a command for the compiled binary
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("blackjack");
    cmd.env("RATJACK_HEADLESS", "1")
        .arg("--auto-ui")
        .arg("3")
        .arg("2")
        .arg("--strategies")
        .arg("threshold:16,prob:0.35");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Resumen final después de"));
}
