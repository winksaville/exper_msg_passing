# Experiment with Message Passing

Experiment with message passing, currently testing the difference
between Large and Small messages and box sending as is and also
cloning before sending.

Now using crossbeam_channel instead std::sync::mpsc::channel, the
main reasong for this is so eventually `select` or `select!` can be
used and it will be unnecessary to have a "SuperProtocol" type with
all messages in one type. Instead each "service/actor" will have different
message types.

The minimum changes needed to use crossbeam_channel (cbc) was to create channels
using `unbouned()` rather than `channel()`. I'm a little surpised that
the performance was reduced and was expecting it to be improved as one of
the supposed advantages of cbc is that it faster.

## Run

Initial stab at creating a service server manager
```
wink@3900x 23-01-02T17:51:56.388Z:~/prgs/rust/myrepos/exper_msg_passing (use-crossbeam_channel)
$ cargo run
   Compiling crossbeam-utils v0.8.14
   Compiling cfg-if v1.0.0
   Compiling crossbeam-channel v0.5.6
   Compiling exper_msg_passing v0.2.0 (/home/wink/prgs/rust/myrepos/exper_msg_passing)
    Finished dev [unoptimized + debuginfo] target(s) in 0.99s
     Running `target/debug/exper_msg_passing`
main:+
client_thread:+
client_thread:  registered with client_service_manager
client_thread:  Sent client_tx to main
client_thread:  Invoke run
main:  client_tx=Sender { .. }
server_thread:+
server_thread:  registered with server_service_manager
server_thread:  Sent server_tx to main
server_thread:  Invoke run
main:  server_tx=Sender { .. }
main: server is Done
main: server is Done
main: Stopping server and client
main: Waiting for server and client to stop
server_thread:-
client_thread:-
main:-
```

## Benchmarks:

As expected if you clone a message and it's total data size is large
the performance drops linearly. Also when not cloning box and owned are
pretty similar with box usually faster. But, I'm guessing if all of
the message processing was done on one thread we would see a greater difference.

As mentioned above, it is surprising that performance is reduced using cbc.
```
wink@3900x 23-01-02T17:30:34.174Z:~/prgs/rust/myrepos/exper_msg_passing (use-crossbeam_channel)
$ taskset -c 0 cargo criterion
   Compiling exper_msg_passing v0.2.0 (/home/wink/prgs/rust/myrepos/exper_msg_passing)
    Finished bench [optimized] target(s) in 6.42s
one_thread/1000         time:   [54.572 ??s 54.602 ??s 54.638 ??s]                            
                        change: [+46.444% +46.785% +47.162%] (p = 0.00 < 0.05)
                        Performance has regressed.

echo/MsgOf/1            time:   [5.4832 ??s 5.5043 ??s 5.5320 ??s]                          
                        change: [+169.34% +182.07% +195.99%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgOf/1        time:   [5.4735 ??s 5.4933 ??s 5.5188 ??s]                              
                        change: [+166.78% +179.83% +195.84%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgMf/1            time:   [5.6200 ??s 5.6411 ??s 5.6687 ??s]                          
                        change: [+148.00% +160.33% +173.67%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgMf/1        time:   [5.4752 ??s 5.4976 ??s 5.5264 ??s]                              
                        change: [+164.21% +178.76% +195.04%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgOf/4096         time:   [5.5004 ??s 5.5212 ??s 5.5480 ??s]                             
                        change: [+160.50% +173.85% +189.96%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgOf/4096     time:   [5.4864 ??s 5.5053 ??s 5.5297 ??s]                                 
                        change: [+162.80% +176.15% +192.23%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgMf/4096         time:   [5.6358 ??s 5.6565 ??s 5.6836 ??s]                             
                        change: [+146.60% +159.11% +172.40%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgMf/4096     time:   [5.4696 ??s 5.4899 ??s 5.5165 ??s]                                 
                        change: [+166.45% +181.00% +196.96%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgOf/65536        time:   [5.4985 ??s 5.5206 ??s 5.5489 ??s]                              
                        change: [+165.22% +178.84% +194.15%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgOf/65536    time:   [5.4746 ??s 5.4951 ??s 5.5218 ??s]                                  
                        change: [+165.44% +177.74% +192.86%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgMf/65536        time:   [5.6359 ??s 5.6571 ??s 5.6845 ??s]                              
                        change: [+146.49% +158.12% +173.12%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgMf/65536    time:   [5.4770 ??s 5.4972 ??s 5.5234 ??s]                                  
                        change: [+165.56% +180.36% +195.70%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgOf/131072       time:   [5.4858 ??s 5.5070 ??s 5.5341 ??s]                               
                        change: [+167.68% +181.09% +198.53%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgOf/131072   time:   [5.4860 ??s 5.5077 ??s 5.5350 ??s]                                   
                        change: [+167.71% +182.61% +199.67%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgMf/131072       time:   [5.6407 ??s 5.6646 ??s 5.6955 ??s]                               
                        change: [+148.12% +160.82% +174.91%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgMf/131072   time:   [5.4760 ??s 5.4969 ??s 5.5241 ??s]                                   
                        change: [+166.25% +180.23% +194.78%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgOf/262144       time:   [5.4948 ??s 5.5166 ??s 5.5451 ??s]                               
                        change: [+167.94% +181.11% +199.57%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgOf/262144   time:   [5.4721 ??s 5.4948 ??s 5.5240 ??s]                                   
                        change: [+169.85% +181.76% +197.42%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgMf/262144       time:   [5.6319 ??s 5.6535 ??s 5.6811 ??s]                               
                        change: [+147.33% +159.37% +172.58%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgMf/262144   time:   [5.4789 ??s 5.5000 ??s 5.5274 ??s]                                   
                        change: [+167.45% +179.96% +195.56%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgOf/524288       time:   [5.4890 ??s 5.5113 ??s 5.5401 ??s]                               
                        change: [+166.61% +179.38% +194.82%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgOf/524288   time:   [5.4710 ??s 5.4908 ??s 5.5164 ??s]                                   
                        change: [+165.57% +177.17% +194.40%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgMf/524288       time:   [5.6423 ??s 5.6655 ??s 5.6955 ??s]                               
                        change: [+148.22% +161.00% +173.25%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgMf/524288   time:   [5.4874 ??s 5.5102 ??s 5.5385 ??s]                                   
                        change: [+164.95% +177.15% +191.86%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgOf/1048576      time:   [5.4824 ??s 5.5031 ??s 5.5300 ??s]                                
                        change: [+164.42% +177.59% +192.72%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgOf/1048576  time:   [5.4681 ??s 5.4883 ??s 5.5148 ??s]                                    
                        change: [+164.52% +178.53% +192.07%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/MsgMf/1048576      time:   [5.6261 ??s 5.6480 ??s 5.6765 ??s]                                
                        change: [+146.44% +159.56% +174.60%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo/box MsgMf/1048576  time:   [5.4644 ??s 5.4854 ??s 5.5123 ??s]                                    
                        change: [+164.10% +176.77% +190.28%] (p = 0.00 < 0.05)
                        Performance has regressed.

echo_clone/clone MsgOf/1                                                                             
                        time:   [5.5310 ??s 5.5519 ??s 5.5788 ??s]
                        change: [+159.73% +174.35% +191.07%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgOf/1                                                                             
                        time:   [5.5492 ??s 5.5690 ??s 5.5946 ??s]
                        change: [+162.04% +175.37% +190.64%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgMf/1                                                                             
                        time:   [5.6925 ??s 5.7146 ??s 5.7431 ??s]
                        change: [+146.28% +158.10% +172.09%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgMf/1                                                                             
                        time:   [5.5808 ??s 5.6010 ??s 5.6272 ??s]
                        change: [+160.48% +174.16% +189.88%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgOf/4096                                                                             
                        time:   [5.6807 ??s 5.7047 ??s 5.7356 ??s]
                        change: [+146.81% +160.30% +174.53%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgOf/4096                                                                             
                        time:   [5.6786 ??s 5.7016 ??s 5.7311 ??s]
                        change: [+142.46% +153.98% +167.20%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgMf/4096                                                                             
                        time:   [5.8515 ??s 5.8752 ??s 5.9056 ??s]
                        change: [+129.30% +140.04% +152.46%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgMf/4096                                                                             
                        time:   [5.7574 ??s 5.7782 ??s 5.8054 ??s]
                        change: [+138.00% +148.70% +161.66%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgOf/65536                                                                             
                        time:   [7.7030 ??s 7.7350 ??s 7.7754 ??s]
                        change: [+86.607% +94.393% +103.20%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgOf/65536                                                                             
                        time:   [7.7057 ??s 7.7363 ??s 7.7746 ??s]
                        change: [+88.193% +95.919% +105.28%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgMf/65536                                                                             
                        time:   [7.8780 ??s 7.9104 ??s 7.9519 ??s]
                        change: [+83.449% +90.088% +98.611%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgMf/65536                                                                             
                        time:   [7.7439 ??s 7.7717 ??s 7.8077 ??s]
                        change: [+84.157% +92.025% +100.51%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgOf/131072                                                                             
                        time:   [9.7985 ??s 9.8361 ??s 9.8840 ??s]
                        change: [+57.946% +63.784% +69.356%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgOf/131072                                                                             
                        time:   [9.7439 ??s 9.7804 ??s 9.8270 ??s]
                        change: [+56.810% +62.528% +69.453%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgMf/131072                                                                             
                        time:   [9.9745 ??s 10.011 ??s 10.058 ??s]
                        change: [+55.182% +60.589% +66.421%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgMf/131072                                                                             
                        time:   [9.7697 ??s 9.8148 ??s 9.8697 ??s]
                        change: [+56.120% +61.927% +67.607%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgOf/262144                                                                             
                        time:   [15.289 ??s 15.344 ??s 15.415 ??s]
                        change: [+30.767% +34.191% +38.438%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgOf/262144                                                                             
                        time:   [15.280 ??s 15.339 ??s 15.416 ??s]
                        change: [+26.255% +29.390% +33.007%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgMf/262144                                                                             
                        time:   [16.264 ??s 16.333 ??s 16.422 ??s]
                        change: [+36.501% +40.101% +44.197%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgMf/262144                                                                             
                        time:   [15.989 ??s 16.056 ??s 16.139 ??s]
                        change: [+30.569% +34.271% +38.241%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgOf/524288                                                                             
                        time:   [27.402 ??s 27.607 ??s 27.844 ??s]
                        change: [+23.331% +25.429% +27.916%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgOf/524288                                                                             
                        time:   [27.542 ??s 27.738 ??s 27.957 ??s]
                        change: [+24.002% +26.254% +28.311%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgMf/524288                                                                             
                        time:   [27.948 ??s 28.078 ??s 28.229 ??s]
                        change: [+22.381% +24.417% +26.383%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgMf/524288                                                                             
                        time:   [27.750 ??s 27.873 ??s 28.022 ??s]
                        change: [+22.996% +24.985% +26.946%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgOf/1048576                                                                             
                        time:   [48.589 ??s 48.830 ??s 49.130 ??s]
                        change: [+15.766% +17.086% +18.523%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgOf/1048576                                                                             
                        time:   [48.557 ??s 48.771 ??s 49.039 ??s]
                        change: [+14.102% +15.492% +17.077%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone MsgMf/1048576                                                                             
                        time:   [48.478 ??s 48.690 ??s 48.958 ??s]
                        change: [+14.268% +15.457% +16.727%] (p = 0.00 < 0.05)
                        Performance has regressed.
echo_clone/clone box MsgMf/1048576                                                                             
                        time:   [48.405 ??s 48.636 ??s 48.930 ??s]
                        change: [+15.418% +16.964% +18.534%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
