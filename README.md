# Experiment with Message Passing

Experiment with message passing, currently testing the difference
between Large and Small messages and box sending as is and also
cloning before sending.

## Run

Initial stab at creating a "servi server manager
```
wink@3900x 23-01-02T17:21:37.129Z:~/prgs/rust/myrepos/exper_msg_passing (enable-all-benches)
$ cargo run
   Compiling exper_msg_passing v0.2.0 (/home/wink/prgs/rust/myrepos/exper_msg_passing)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
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
server_thread:-
main: Waiting for server and client to stop
client_thread:-
main:-
```

## Benchmarks:

As expected if you clone a message and it's total data size is large
the performance drops linearly. Also when not cloning box and owned are
pretty similar with box usually faster. But, I'm guessing if all of
the message processing was done on one thread we would see a greater difference.
```
wink@3900x 23-01-02T16:56:25.491Z:~/prgs/rust/myrepos/exper_msg_passing (main)
$ taskset -c 0 cargo criterion
   Compiling exper_msg_passing v0.2.0 (/home/wink/prgs/rust/myrepos/exper_msg_passing)
    Finished bench [optimized] target(s) in 5.08s
one_thread/1000         time:   [37.350 µs 37.427 µs 37.514 µs]                             

echo/MsgOf/1            time:   [2.1992 µs 2.2033 µs 2.2080 µs]                          
echo/box MsgOf/1        time:   [2.2018 µs 2.2075 µs 2.2135 µs]                              
Benchmarking echo/MsgMf/1: Warming up for 3.0000 s^C
wink@3900x 23-01-02T16:58:30.633Z:~/prgs/rust/myrepos/exper_msg_passing (main)
$ taskset -c 0 cargo criterion
   Compiling exper_msg_passing v0.2.0 (/home/wink/prgs/rust/myrepos/exper_msg_passing)
    Finished bench [optimized] target(s) in 4.38s
one_thread/1000         time:   [37.609 µs 37.686 µs 37.771 µs]                             
                        change: [-0.8359% +0.0303% +0.7306%] (p = 0.95 > 0.05)
                        No change in performance detected.

echo/MsgOf/1            time:   [2.1975 µs 2.2031 µs 2.2102 µs]                          
                        change: [-1.5569% -0.9464% -0.3819%] (p = 0.00 < 0.05)
                        Change within noise threshold.
echo/box MsgOf/1        time:   [2.2066 µs 2.2122 µs 2.2193 µs]                              
                        change: [-1.0522% -0.5062% -0.0090%] (p = 0.06 > 0.05)
                        No change in performance detected.
echo/MsgMf/1            time:   [2.4235 µs 2.4305 µs 2.4386 µs]                          
echo/box MsgMf/1        time:   [2.2060 µs 2.2105 µs 2.2150 µs]                              
echo/MsgOf/4096         time:   [2.2313 µs 2.2385 µs 2.2470 µs]                             
echo/box MsgOf/4096     time:   [2.2214 µs 2.2279 µs 2.2358 µs]                                 
echo/MsgMf/4096         time:   [2.4414 µs 2.4448 µs 2.4487 µs]                             
echo/box MsgMf/4096     time:   [2.1819 µs 2.1860 µs 2.1904 µs]                                 
echo/MsgOf/65536        time:   [2.2048 µs 2.2109 µs 2.2179 µs]                              
echo/box MsgOf/65536    time:   [2.2119 µs 2.2203 µs 2.2304 µs]                                  
echo/MsgMf/65536        time:   [2.4663 µs 2.4886 µs 2.5087 µs]                              
echo/box MsgMf/65536    time:   [2.1924 µs 2.1971 µs 2.2022 µs]                                  
echo/MsgOf/131072       time:   [2.1950 µs 2.1992 µs 2.2041 µs]                               
echo/box MsgOf/131072   time:   [2.1849 µs 2.1910 µs 2.1988 µs]                                   
echo/MsgMf/131072       time:   [2.4167 µs 2.4206 µs 2.4249 µs]                               
echo/box MsgMf/131072   time:   [2.1937 µs 2.1980 µs 2.2028 µs]                                   
echo/MsgOf/262144       time:   [2.2044 µs 2.2128 µs 2.2225 µs]                               
echo/box MsgOf/262144   time:   [2.1865 µs 2.1894 µs 2.1924 µs]                                   
echo/MsgMf/262144       time:   [2.4291 µs 2.4335 µs 2.4385 µs]                               
echo/box MsgMf/262144   time:   [2.1983 µs 2.2049 µs 2.2114 µs]                                   
echo/MsgOf/524288       time:   [2.1927 µs 2.1952 µs 2.1979 µs]                               
echo/box MsgOf/524288   time:   [2.1974 µs 2.2055 µs 2.2151 µs]                                   
echo/MsgMf/524288       time:   [2.4181 µs 2.4246 µs 2.4314 µs]                               
echo/box MsgMf/524288   time:   [2.1966 µs 2.1996 µs 2.2029 µs]                                   
echo/MsgOf/1048576      time:   [2.1998 µs 2.2042 µs 2.2094 µs]                                
echo/box MsgOf/1048576  time:   [2.1891 µs 2.1940 µs 2.1993 µs]                                    
echo/MsgMf/1048576      time:   [2.4202 µs 2.4279 µs 2.4366 µs]                                
echo/box MsgMf/1048576  time:   [2.1973 µs 2.2015 µs 2.2062 µs]                                    

echo_clone/clone MsgOf/1                                                                             
                        time:   [2.2576 µs 2.2644 µs 2.2716 µs]
echo_clone/clone box MsgOf/1                                                                             
                        time:   [2.2584 µs 2.2634 µs 2.2701 µs]
echo_clone/clone MsgMf/1                                                                             
                        time:   [2.4673 µs 2.4734 µs 2.4805 µs]
echo_clone/clone box MsgMf/1                                                                             
                        time:   [2.2908 µs 2.2976 µs 2.3050 µs]
echo_clone/clone MsgOf/4096                                                                             
                        time:   [2.4434 µs 2.4515 µs 2.4615 µs]
echo_clone/clone box MsgOf/4096                                                                             
                        time:   [2.5013 µs 2.5065 µs 2.5121 µs]
echo_clone/clone MsgMf/4096                                                                             
                        time:   [2.7292 µs 2.7357 µs 2.7423 µs]
echo_clone/clone box MsgMf/4096                                                                             
                        time:   [2.5925 µs 2.6002 µs 2.6085 µs]
echo_clone/clone MsgOf/65536                                                                             
                        time:   [4.3877 µs 4.3956 µs 4.4050 µs]
echo_clone/clone box MsgOf/65536                                                                             
                        time:   [4.3703 µs 4.3744 µs 4.3789 µs]
echo_clone/clone MsgMf/65536                                                                             
                        time:   [4.6050 µs 4.6184 µs 4.6341 µs]
echo_clone/clone box MsgMf/65536                                                                             
                        time:   [4.4837 µs 4.4935 µs 4.5048 µs]
echo_clone/clone MsgOf/131072                                                                             
                        time:   [6.6088 µs 6.6220 µs 6.6364 µs]
echo_clone/clone box MsgOf/131072                                                                             
                        time:   [6.6231 µs 6.6375 µs 6.6533 µs]
echo_clone/clone MsgMf/131072                                                                             
                        time:   [6.8530 µs 6.8643 µs 6.8777 µs]
echo_clone/clone box MsgMf/131072                                                                             
                        time:   [6.6610 µs 6.6723 µs 6.6860 µs]
echo_clone/clone MsgOf/262144                                                                             
                        time:   [12.605 µs 12.671 µs 12.725 µs]
echo_clone/clone box MsgOf/262144                                                                             
                        time:   [12.904 µs 12.927 µs 12.953 µs]
echo_clone/clone MsgMf/262144                                                                             
                        time:   [12.830 µs 12.932 µs 13.017 µs]
echo_clone/clone box MsgMf/262144                                                                             
                        time:   [12.967 µs 12.986 µs 13.009 µs]
echo_clone/clone MsgOf/524288                                                                             
                        time:   [23.080 µs 23.213 µs 23.346 µs]
echo_clone/clone box MsgOf/524288                                                                             
                        time:   [23.218 µs 23.335 µs 23.444 µs]
echo_clone/clone MsgMf/524288                                                                             
                        time:   [23.913 µs 23.947 µs 23.981 µs]
echo_clone/clone box MsgMf/524288                                                                             
                        time:   [23.738 µs 23.784 µs 23.825 µs]
echo_clone/clone MsgOf/1048576                                                                             
                        time:   [43.030 µs 43.096 µs 43.175 µs]
echo_clone/clone box MsgOf/1048576                                                                             
                        time:   [43.936 µs 44.116 µs 44.276 µs]
echo_clone/clone MsgMf/1048576                                                                             
                        time:   [43.349 µs 43.466 µs 43.591 µs]
echo_clone/clone box MsgMf/1048576                                                                             
                        time:   [43.077 µs 43.172 µs 43.282 µs]
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
