use assert_cmd::Command;

#[test]
fn zero_factorial() {
    let expected = "1\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("0")
    .assert()
    .stdout(expected);
}

#[test]
fn one_factorial() {
    let expected = "1\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("1")
    .assert()
    .stdout(expected);
}

#[test]
fn nine_factorial() {
    let expected = "362880\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("9")
    .assert()
    .stdout(expected);
}

#[test]
fn ten_factorial() {
    let expected = "3628800\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("10")
    .assert()
    .stdout(expected);
}

#[test]
fn elleven_factorial() {
    let expected = "39916800\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("11")
    .assert()
    .stdout(expected);
}

#[test]
fn twelve_factorial() {
    let expected = "479001600\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("12")
    .assert()
    .stdout(expected);
}


#[test]
fn thirteen_factorial() {
    let expected = "6227020800\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("13")
    .assert()
    .stdout(expected);
}

#[test]
fn fourteen_factorial() {
    let expected = "87178291200\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("14")
    .assert()
    .stdout(expected);
}

#[test]
fn twenty_three_factorial() {
    let expected = "25852016738884976640000\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("23")
    .assert()
    .stdout(expected);
}

#[test]
fn one_hundred_factorial() {
    let expected = "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000\n";

    Command::cargo_bin("pfact")
    .unwrap()
    .arg("-n")
    .arg("100")
    .assert()
    .stdout(expected);
}