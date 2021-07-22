use assert_cmd::Command;

#[test]
fn test_wrong_journal_file() {
    let assert = Command::cargo_bin("rusty_journal")
        .unwrap()
        .arg("-j")
        .arg("./unexistising/path/to/file")
        .arg("list")
        .assert();
    assert
        .failure()
        .stderr(predicates::str::contains("No such file or directory"));
}

#[test]
fn run_add_done_list_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_file = tempfile::Builder::new().tempfile()?;

    // Add a first task
    let result = Command::cargo_bin("rusty_journal")?
        .arg("-j")
        .arg(tmp_file.path())
        .args(["add", "Hello, world!"])
        .ok();
    assert!(result.is_ok());

    // Add a second task
    let result = Command::cargo_bin("rusty_journal")?
        .arg("-j")
        .arg(tmp_file.path())
        .args(["add", "Buy milk"])
        .ok();
    assert!(result.is_ok());

    // List and check if we have our 2 tasks
    let assert = Command::cargo_bin("rusty_journal")?
        .arg("-j")
        .arg(tmp_file.path())
        .arg("list")
        .assert();
    assert
        .success()
        .stdout(predicates::str::contains("Hello, world!"))
        .stdout(predicates::str::contains("Buy milk"));

    // Remove the first task
    let result = Command::cargo_bin("rusty_journal")?
        .arg("-j")
        .arg(tmp_file.path())
        .args(["done", "1"])
        .ok();
    assert!(result.is_ok());

    // List and check if we have 1 task and the first removed
    let assert = Command::cargo_bin("rusty_journal")?
        .arg("-j")
        .arg(tmp_file.path())
        .arg("list")
        .assert();
    assert
        .success()
        .stdout(predicates::str::is_match("1. Buy milk")?);

    // Remove the last task
    let result = Command::cargo_bin("rusty_journal")?
        .arg("-j")
        .arg(tmp_file.path())
        .args(["done", "1"])
        .ok();
    assert!(result.is_ok());

    // List will be empty and display error message
    let assert = Command::cargo_bin("rusty_journal")?
        .arg("-j")
        .arg(tmp_file.path())
        .arg("list")
        .assert();
    assert
        .success()
        .stdout(predicates::str::is_match("Task list is empty!")?);

    Ok(())
}
