use assert_cmd::Command;

#[test]
fn sum_empty_file() {
    let expected = "0\n";

    Command::cargo_bin("psum")
    .unwrap()
    .arg("-f")
    .arg("./tests/empty.txt")
    .assert()
    .stdout(expected);
}

#[test]
fn sum_only_the_integer_1() {
    let expected = "1\n";

    Command::cargo_bin("psum")
    .unwrap()
    .arg("-f")
    .arg("./tests/integers1.txt")
    .assert()
    .stdout(expected);
}

#[test]
fn sum_1_to_10() {
    let expected = "55\n";

    Command::cargo_bin("psum")
    .unwrap()
    .arg("-f")
    .arg("./tests/integers2.txt")
    .assert()
    .stdout(expected);
}

#[test]
fn sum_1_to_100() {
    let expected = "5050\n";

    Command::cargo_bin("psum")
    .unwrap()
    .arg("-f")
    .arg("./tests/integers3.txt")
    .assert()
    .stdout(expected);
}

#[test]
fn sum_1_to_999() {
    let expected = "499500\n";

    Command::cargo_bin("psum")
    .unwrap()
    .arg("-f")
    .arg("./tests/integers4.txt")
    .assert()
    .stdout(expected);
}

#[test]
fn sum_rnd_integers() {
    let expected = "3297\n";

    Command::cargo_bin("psum")
    .unwrap()
    .arg("-f")
    .arg("./tests/rnd_integers.txt")
    .assert()
    .stdout(expected);
}

