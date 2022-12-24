# Experiment with Message Passing

Experiment with message passing, currently testing the difference
between Large and Small messages and box sending as is and also
cloning before sending.

## Benchmarks:

As expected if you clone a message and it's total data size is large
the performance drops linearly. Also when not cloning box and owned are
pretty similar with box usually faster. But, I'm guessing if all of
the message processing was done on one thread we would see a greater difference.
```
wink@3900x 22-12-24T00:22:22.340Z:~/prgs/rust/myrepos/exper_msg_passing (main)
$ cargo criterion
   Compiling autocfg v1.1.0
   Compiling libc v0.2.139
  ...
   Compiling criterion v0.4.0
   Compiling exper_msg_passing v0.2.0 (/home/wink/prgs/rust/myrepos/exper_msg_passing)
    Finished bench [optimized] target(s) in 24.13s
echo/MsgOf/1            time:   [8.1461 µs 8.2460 µs 8.3590 µs]                          
echo/box MsgOf/1        time:   [8.1858 µs 8.2807 µs 8.3852 µs]                              
echo/MsgMf/1            time:   [8.7879 µs 9.0272 µs 9.2534 µs]                          
echo/box MsgMf/1        time:   [7.8949 µs 8.0446 µs 8.1840 µs]                              
echo/MsgOf/4096         time:   [7.5911 µs 7.7979 µs 7.9922 µs]                             
echo/box MsgOf/4096     time:   [8.0560 µs 8.1692 µs 8.3035 µs]                                 
echo/MsgMf/4096         time:   [9.1950 µs 9.3794 µs 9.5573 µs]                             
echo/box MsgMf/4096     time:   [8.2317 µs 8.3703 µs 8.5098 µs]                                 
echo/MsgOf/65536        time:   [7.4485 µs 7.6323 µs 7.8407 µs]                              
echo/box MsgOf/65536    time:   [6.3541 µs 6.3928 µs 6.4288 µs]                                  
echo/MsgMf/65536        time:   [8.0988 µs 8.3553 µs 8.6047 µs]                              
echo/box MsgMf/65536    time:   [8.1676 µs 8.2627 µs 8.3685 µs]                                  
echo/MsgOf/131072       time:   [7.6926 µs 7.8525 µs 7.9931 µs]                               
echo/box MsgOf/131072   time:   [6.4535 µs 6.4822 µs 6.5095 µs]                                   
echo/MsgMf/131072       time:   [9.1338 µs 9.3160 µs 9.4608 µs]                               
echo/box MsgMf/131072   time:   [7.8516 µs 7.9669 µs 8.0898 µs]                                   
echo/MsgOf/262144       time:   [6.4178 µs 6.4591 µs 6.5097 µs]                               
echo/box MsgOf/262144   time:   [6.5108 µs 6.7483 µs 6.9972 µs]                                   
echo/MsgMf/262144       time:   [9.4498 µs 9.5558 µs 9.6608 µs]                               
echo/box MsgMf/262144   time:   [7.0588 µs 7.2696 µs 7.4690 µs]                                   
echo/MsgOf/524288       time:   [8.0551 µs 8.1791 µs 8.3101 µs]                               
echo/box MsgOf/524288   time:   [8.1683 µs 8.2827 µs 8.3977 µs]                                   
echo/MsgMf/524288       time:   [9.4473 µs 9.5531 µs 9.6631 µs]                               
echo/box MsgMf/524288   time:   [7.6598 µs 7.9523 µs 8.2114 µs]                                   
echo/MsgOf/1048576      time:   [8.1982 µs 8.2880 µs 8.3879 µs]                                
echo/box MsgOf/1048576  time:   [6.5289 µs 6.5761 µs 6.6312 µs]                                    
echo/MsgMf/1048576      time:   [9.2883 µs 9.3888 µs 9.4928 µs]                                
echo/box MsgMf/1048576  time:   [6.6545 µs 6.6961 µs 6.7436 µs]                                    

echo_clone/clone MsgOf/1                                                                             
                        time:   [8.3359 µs 8.4530 µs 8.5761 µs]
echo_clone/clone box MsgOf/1                                                                             
                        time:   [7.5969 µs 7.8100 µs 8.0535 µs]
echo_clone/clone MsgMf/1                                                                             
                        time:   [7.7043 µs 7.8175 µs 7.9226 µs]
echo_clone/clone box MsgMf/1                                                                             
                        time:   [8.7628 µs 8.8741 µs 8.9852 µs]
echo_clone/clone MsgOf/4096                                                                             
                        time:   [11.035 µs 11.102 µs 11.176 µs]
echo_clone/clone box MsgOf/4096                                                                             
                        time:   [11.367 µs 11.431 µs 11.500 µs]
echo_clone/clone MsgMf/4096                                                                             
                        time:   [11.648 µs 11.715 µs 11.785 µs]
echo_clone/clone box MsgMf/4096                                                                             
                        time:   [11.755 µs 11.813 µs 11.873 µs]
echo_clone/clone MsgOf/65536                                                                             
                        time:   [26.187 µs 26.336 µs 26.459 µs]
echo_clone/clone box MsgOf/65536                                                                             
                        time:   [27.777 µs 27.841 µs 27.904 µs]
echo_clone/clone MsgMf/65536                                                                             
                        time:   [27.122 µs 27.659 µs 27.935 µs]
echo_clone/clone box MsgMf/65536                                                                             
                        time:   [30.249 µs 31.032 µs 31.481 µs]
echo_clone/clone MsgOf/131072                                                                             
                        time:   [46.700 µs 46.923 µs 47.080 µs]
echo_clone/clone box MsgOf/131072                                                                             
                        time:   [48.709 µs 50.282 µs 51.449 µs]
echo_clone/clone MsgMf/131072                                                                             
                        time:   [51.895 µs 52.127 µs 52.310 µs]
echo_clone/clone box MsgMf/131072                                                                             
                        time:   [45.975 µs 46.986 µs 47.668 µs]
echo_clone/clone MsgOf/262144                                                                            
                        time:   [84.767 µs 86.184 µs 86.913 µs]
echo_clone/clone box MsgOf/262144                                                                            
                        time:   [93.574 µs 93.981 µs 94.283 µs]
echo_clone/clone MsgMf/262144                                                                            
                        time:   [87.655 µs 91.689 µs 94.212 µs]
echo_clone/clone box MsgMf/262144                                                                            
                        time:   [85.064 µs 86.056 µs 86.673 µs]
echo_clone/clone MsgOf/524288                                                                            
                        time:   [155.17 µs 160.10 µs 162.68 µs]
echo_clone/clone box MsgOf/524288                                                                            
                        time:   [153.60 µs 160.05 µs 163.23 µs]
echo_clone/clone MsgMf/524288                                                                            
                        time:   [162.64 µs 163.50 µs 164.15 µs]
echo_clone/clone box MsgMf/524288                                                                            
                        time:   [160.29 µs 163.41 µs 165.26 µs]
echo_clone/clone MsgOf/1048576                                                                            
                        time:   [333.33 µs 336.63 µs 338.83 µs]
echo_clone/clone box MsgOf/1048576                                                                            
                        time:   [321.49 µs 330.22 µs 336.20 µs]
echo_clone/clone MsgMf/1048576                                                                            
                        time:   [330.69 µs 333.31 µs 335.01 µs]
echo_clone/clone box MsgMf/1048576                                                                            
                        time:   [315.20 µs 323.77 µs 330.13 µs]
```

Actaully what's amazing the difference between `cargo criterion` and
`taskset -c 0-1 cargo criterion` which means that when both threads
are on the same CPU there is a 70%+ performance improvement:
```
wink@3900x 22-12-24T00:35:26.376Z:~/prgs/rust/myrepos/exper_msg_passing (main)
$ taskset -c 0 cargo criterion
    Finished bench [optimized] target(s) in 0.03s
echo/MsgOf/1            time:   [2.2424 µs 2.2482 µs 2.2551 µs]                          
                        change: [-71.530% -70.839% -70.096%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgOf/1        time:   [2.2271 µs 2.2321 µs 2.2378 µs]                              
                        change: [-73.660% -73.356% -73.067%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgMf/1            time:   [2.4361 µs 2.4426 µs 2.4500 µs]                          
                        change: [-74.916% -74.638% -74.360%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgMf/1        time:   [2.2209 µs 2.2262 µs 2.2320 µs]                              
                        change: [-72.189% -71.761% -71.268%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgOf/4096         time:   [2.2418 µs 2.2474 µs 2.2540 µs]                             
                        change: [-69.562% -68.906% -68.261%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgOf/4096     time:   [2.2218 µs 2.2272 µs 2.2336 µs]                                 
                        change: [-72.618% -72.289% -71.934%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgMf/4096         time:   [2.4551 µs 2.4617 µs 2.4684 µs]                             
                        change: [-73.898% -73.518% -73.110%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgMf/4096     time:   [2.2149 µs 2.2201 µs 2.2260 µs]                                 
                        change: [-73.442% -73.110% -72.788%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgOf/65536        time:   [2.2374 µs 2.2423 µs 2.2479 µs]                              
                        change: [-72.285% -71.835% -71.366%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgOf/65536    time:   [2.2109 µs 2.2165 µs 2.2231 µs]                                  
                        change: [-66.066% -65.659% -65.301%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgMf/65536        time:   [2.4375 µs 2.4429 µs 2.4490 µs]                              
                        change: [-69.925% -69.368% -68.817%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgMf/65536    time:   [2.2084 µs 2.2145 µs 2.2211 µs]                                  
                        change: [-73.189% -72.968% -72.748%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgOf/131072       time:   [2.2419 µs 2.2482 µs 2.2545 µs]                               
                        change: [-71.070% -70.301% -69.478%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgOf/131072   time:   [2.2249 µs 2.2307 µs 2.2368 µs]                                   
                        change: [-66.924% -66.365% -65.905%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgMf/131072       time:   [2.4641 µs 2.4700 µs 2.4765 µs]                               
                        change: [-73.752% -73.392% -72.932%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgMf/131072   time:   [2.2380 µs 2.2422 µs 2.2468 µs]                                   
                        change: [-72.198% -71.881% -71.581%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgOf/262144       time:   [2.2423 µs 2.2481 µs 2.2547 µs]                               
                        change: [-65.654% -65.335% -65.076%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgOf/262144   time:   [2.2151 µs 2.2226 µs 2.2312 µs]                                   
                        change: [-67.093% -66.482% -65.884%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgMf/262144       time:   [2.4231 µs 2.4292 µs 2.4356 µs]                               
                        change: [-74.592% -74.351% -74.107%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgMf/262144   time:   [2.1985 µs 2.2033 µs 2.2091 µs]                                   
                        change: [-69.770% -69.104% -68.455%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgOf/524288       time:   [2.2327 µs 2.2367 µs 2.2408 µs]                               
                        change: [-73.142% -72.790% -72.441%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgOf/524288   time:   [2.1981 µs 2.2043 µs 2.2114 µs]                                   
                        change: [-73.605% -73.321% -73.056%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgMf/524288       time:   [2.4300 µs 2.4359 µs 2.4431 µs]                               
                        change: [-74.647% -74.339% -74.069%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgMf/524288   time:   [2.2071 µs 2.2149 µs 2.2235 µs]                                   
                        change: [-73.121% -72.723% -72.258%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgOf/1048576      time:   [2.2312 µs 2.2362 µs 2.2420 µs]                                
                        change: [-73.117% -72.856% -72.616%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgOf/1048576  time:   [2.2049 µs 2.2106 µs 2.2174 µs]                                    
                        change: [-66.716% -66.416% -66.164%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/MsgMf/1048576      time:   [2.4300 µs 2.4365 µs 2.4437 µs]                                
                        change: [-74.051% -73.813% -73.533%] (p = 0.00 < 0.05)
                        Performance has improved.
echo/box MsgMf/1048576  time:   [2.2055 µs 2.2120 µs 2.2195 µs]                                    
                        change: [-69.213% -68.488% -67.794%] (p = 0.00 < 0.05)
                        Performance has improved.

echo_clone/clone MsgOf/1                                                                             
                        time:   [2.2724 µs 2.2814 µs 2.2923 µs]
                        change: [-73.101% -72.740% -72.414%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgOf/1                                                                             
                        time:   [2.2735 µs 2.2802 µs 2.2875 µs]
                        change: [-73.079% -72.570% -72.033%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgMf/1                                                                             
                        time:   [2.4888 µs 2.4962 µs 2.5045 µs]
                        change: [-70.572% -69.803% -69.038%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgMf/1                                                                             
                        time:   [2.3157 µs 2.3211 µs 2.3269 µs]
                        change: [-73.974% -73.732% -73.504%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgOf/4096                                                                             
                        time:   [2.5038 µs 2.5099 µs 2.5166 µs]
                        change: [-77.425% -77.231% -77.063%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgOf/4096                                                                             
                        time:   [2.5272 µs 2.5349 µs 2.5437 µs]
                        change: [-78.314% -77.981% -77.746%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgMf/4096                                                                             
                        time:   [2.7340 µs 2.7443 µs 2.7559 µs]
                        change: [-76.860% -76.685% -76.520%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgMf/4096                                                                             
                        time:   [2.5893 µs 2.5966 µs 2.6043 µs]
                        change: [-78.660% -78.384% -78.166%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgOf/65536                                                                             
                        time:   [4.5254 µs 4.5326 µs 4.5409 µs]
                        change: [-83.141% -82.990% -82.860%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgOf/65536                                                                             
                        time:   [4.5175 µs 4.5243 µs 4.5325 µs]
                        change: [-83.799% -83.712% -83.642%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgMf/65536                                                                             
                        time:   [4.7686 µs 4.7803 µs 4.7930 µs]
                        change: [-82.959% -82.796% -82.528%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgMf/65536                                                                             
                        time:   [4.5804 µs 4.5889 µs 4.5988 µs]
                        change: [-85.382% -85.270% -85.113%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgOf/131072                                                                             
                        time:   [6.6146 µs 6.6233 µs 6.6327 µs]
                        change: [-85.978% -85.914% -85.855%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgOf/131072                                                                             
                        time:   [6.6104 µs 6.6227 µs 6.6370 µs]
                        change: [-87.194% -87.024% -86.797%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgMf/131072                                                                             
                        time:   [7.0351 µs 7.0477 µs 7.0618 µs]
                        change: [-86.531% -86.456% -86.353%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgMf/131072                                                                             
                        time:   [6.8013 µs 6.8124 µs 6.8251 µs]
                        change: [-85.721% -85.549% -85.296%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgOf/262144                                                                             
                        time:   [12.781 µs 12.805 µs 12.830 µs]
                        change: [-85.311% -85.050% -84.702%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgOf/262144                                                                             
                        time:   [12.831 µs 12.856 µs 12.882 µs]
                        change: [-86.426% -86.354% -86.273%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgMf/262144                                                                             
                        time:   [13.012 µs 13.045 µs 13.077 µs]
                        change: [-86.211% -86.033% -85.782%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgMf/262144                                                                             
                        time:   [12.217 µs 12.265 µs 12.326 µs]
                        change: [-85.963% -85.775% -85.484%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgOf/524288                                                                             
                        time:   [22.729 µs 22.755 µs 22.787 µs]
                        change: [-85.923% -85.794% -85.599%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgOf/524288                                                                             
                        time:   [22.754 µs 22.776 µs 22.799 µs]
                        change: [-86.024% -85.864% -85.613%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgMf/524288                                                                             
                        time:   [23.005 µs 23.031 µs 23.062 µs]
                        change: [-86.001% -85.946% -85.893%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgMf/524288                                                                             
                        time:   [23.447 µs 23.575 µs 23.684 µs]
                        change: [-86.368% -86.205% -86.006%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgOf/1048576                                                                             
                        time:   [43.693 µs 43.878 µs 44.064 µs]
                        change: [-87.005% -86.929% -86.836%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgOf/1048576                                                                             
                        time:   [43.572 µs 43.754 µs 43.941 µs]
                        change: [-86.911% -86.752% -86.539%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone MsgMf/1048576                                                                             
                        time:   [43.682 µs 43.719 µs 43.761 µs]
                        change: [-86.872% -86.790% -86.696%] (p = 0.00 < 0.05)
                        Performance has improved.
echo_clone/clone box MsgMf/1048576                                                                             
                        time:   [43.554 µs 43.663 µs 43.798 µs]
                        change: [-86.826% -86.660% -86.425%] (p = 0.00 < 0.05)
                        Performance has improved.
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
