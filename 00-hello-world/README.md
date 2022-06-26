# 00-hello-world

```bash
cargo new hello-world
# Created binary (application) `hello-world` package
cd hello-world
cargo run
#Compiling hello-world v0.1.0 (/workspace/rust-training/00-hello-world/hello-world)
# Finished dev [unoptimized + debuginfo] target(s) in 0.22s
#  Running `target/debug/hello-world`
#Hello, world!
ls -l target/debug/
# Run it again
./target/debug/hello-world

# You can try
cargo clean
# it will remove the `debug` directory
```

