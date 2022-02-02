use assert_cmd::Command;

#[test]
fn work(){
    assert!(true);
}

#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}