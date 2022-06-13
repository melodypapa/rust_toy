# 1. Hello world Project

## 1.1. Create the project with Cargo

```bash
$cargo new hello
     Created binary (application) `hello` package
```

## 1.2. Compile the source code and run it in one command

```bash
$cargo run
```

## 1.3. Run the test

```bash
$cargo test
```

# 2. Program Exit Values

we can inspect the bash variable $? to see the exit status of the most recent command

```bash
$ true
$ echo $?
0
```

## 2.1. std::process::exit function

```rust
fn main() {
     std::process::exit(0);
}
```

Run the program and check the exit value:
```bash
$cargo run 
echo $?
```

Test is with test file
```rust
#[test]
fn test_true(){
     let mut cmd = Command::cargo_bin("true").unwrap();
     cmd.assert().success();
}
```

## Failed

```rust
std::process::exit(1);
```

or 

```rust
std::process::abort();
```