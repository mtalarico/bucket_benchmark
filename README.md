# bucket_benchmark
A small project to benchmark the bucket pattern insertion rate vs just inserting documents

Currently, runs very sychronously even though it is async, and there is room to expand it to be multi-threaded, but it currently runs on one...

# Instructions
[install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
1. clone the project
2. `cd bucket_benchmark`
3. to compile `cargo build`, to run `cargo run`
