# Experiment with Message Passing

Experiment with message passing, currently testing the difference
between Large and Small messages and box, sending as is and cloning before sending.

## Benchmarks:

At the moment cloning is horrible if the size of the data to be cloned is large.
If <= 256 times are the same, if 64K it's slower by 2x, and if 1M its slower by 10x.

TODO: Add runs


Box is faster when messages are by about 10%. but otherwise fairly similar, we're talking in the
2.8 vs 3.1us. This is using `std:{sync::mpsc::{channel, Receiver, Sender}` on nightly.


TODO: Add runs

Actaully what's amazing the difference between `cargo bench` and `taskset -c 0-1 cargo bench`.
I saw 2x slower for both echo and echo_box. And 6x slower for both echo_box and echo_box_clone.

```
wink@3900x 22-12-22T04:36:41.846Z:~/prgs/rust/myrepos/exper_msg_passing (main)
$ cargo bench
   Compiling exper_msg_passing v0.2.0 (/home/wink/prgs/rust/myrepos/exper_msg_passing)
    Finished bench [optimized] target(s) in 3.45s
     Running unittests src/lib.rs (target/release/deps/exper_msg_passing-76343adda99ab7f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crit.rs (target/release/deps/crit-e9c0c5ad7045ebfe)
crit_echo_clone         time:   [327.76 µs 336.65 µs 343.43 µs]
                        change: [+560.03% +577.70% +591.58%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild

crit_echo               time:   [5.6873 µs 5.7527 µs 5.8532 µs]
                        change: [+98.909% +100.99% +103.27%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low severe
  2 (2.00%) high severe

crit_echo_box           time:   [5.5739 µs 5.6008 µs 5.6271 µs]
                        change: [+98.096% +99.446% +100.97%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe

crit_echo_box_clone     time:   [319.20 µs 323.05 µs 326.27 µs]
                        change: [+566.74% +574.13% +580.94%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low severe

wink@3900x 22-12-22T04:37:46.904Z:~/prgs/rust/myrepos/exper_msg_passing (main)
$ taskset -c 0-1 cargo bench
    Finished bench [optimized] target(s) in 0.03s
     Running unittests src/lib.rs (target/release/deps/exper_msg_passing-76343adda99ab7f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crit.rs (target/release/deps/crit-e9c0c5ad7045ebfe)
crit_echo_clone         time:   [50.160 µs 50.424 µs 50.743 µs]
                        change: [-85.391% -85.114% -84.743%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

crit_echo               time:   [2.8435 µs 2.8532 µs 2.8648 µs]
                        change: [-50.473% -49.907% -49.291%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

crit_echo_box           time:   [2.8336 µs 2.8456 µs 2.8589 µs]
                        change: [-50.081% -49.710% -49.358%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

crit_echo_box_clone     time:   [49.217 µs 49.420 µs 49.652 µs]
                        change: [-85.074% -84.919% -84.749%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
