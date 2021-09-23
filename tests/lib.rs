use assert_cmd::assert::Assert;
use assert_cmd::Command;

pub fn assert_create_task(text: &str) -> Result<Assert, Box<dyn std::error::Error>> {
    let assert = Command::cargo_bin("rusty_journal")?
        .args(["add", text])
        .assert()
        .success()
        .stdout(predicates::str::contains("created"));

    Ok(assert)
}

pub fn assert_update_task(id: &str, text: &str) -> Result<Assert, Box<dyn std::error::Error>> {
    let assert = Command::cargo_bin("rusty_journal")?
        .args(["update", id, text])
        .assert()
        .success()
        .stdout(predicates::str::contains("updated"));

    Ok(assert)
}

pub fn assert_complete_task(id: &str) -> Result<Assert, Box<dyn std::error::Error>> {
    let assert = Command::cargo_bin("rusty_journal")?
        .args(["done", id])
        .assert()
        .success()
        .stdout(predicates::str::contains("completed"));

    Ok(assert)
}

pub fn assert_delete_task(id: &str) -> Result<Assert, Box<dyn std::error::Error>> {
    let assert = Command::cargo_bin("rusty_journal")?
        .args(["delete", id])
        .assert()
        .success()
        .stdout(predicates::str::contains("deleted"));

    Ok(assert)
}

pub fn list_all() -> Result<Assert, Box<dyn std::error::Error>> {
    let assert = Command::cargo_bin("rusty_journal")?
        .arg("list-all")
        .assert()
        .success();

    Ok(assert)
}

pub fn list() -> Result<Assert, Box<dyn std::error::Error>> {
    let assert = Command::cargo_bin("rusty_journal")?
        .arg("list")
        .assert()
        .success();

    Ok(assert)
}

pub fn assert_reset() -> Result<Assert, Box<dyn std::error::Error>> {
    let assert = Command::cargo_bin("rusty_journal")?
        .arg("reset")
        .assert()
        .success();

    Ok(assert)
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let task_1 = "Hello, world!";
    let task_1_updated = "Hello, JR!";
    let task_2 = "Do that";

    assert_reset()?;

    assert_create_task(task_1)?;
    assert_create_task(task_2)?;

    list_all()
        .unwrap()
        .stdout(predicates::str::contains(task_1))
        .stdout(predicates::str::contains(task_2));

    assert_update_task("1", task_1_updated)?;

    list_all()?
        .stdout(predicates::str::diff(task_1))
        .stdout(predicates::str::contains(task_1_updated))
        .stdout(predicates::str::contains(task_2));

    assert_complete_task("1")?;

    list()?
        .stdout(predicates::str::diff(task_1))
        .stdout(predicates::str::diff(task_1_updated))
        .stdout(predicates::str::contains(task_2));

    assert_delete_task("1")?;

    list_all()?
        .stdout(predicates::str::diff(task_1))
        .stdout(predicates::str::diff(task_1_updated))
        .stdout(predicates::str::contains(task_2));

    Ok(())
}
