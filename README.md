# ION - ECU Diagnostics Library

PC Application that is a UDS Client that can interact with Gridania UDS server to perform vehicle's diagnostic tasks
Comply with these standards:  
 - UDS: ISO 14229-1  
 - PCAN-ISO-TP: ISO 15765-2 

## Compile from source

### Dependency

Pcan basic library (3rd party)

#### Windows

Download pcan library (PCANBasic.dll and PCANBasic.lib) from [Peakcan](https://www.peak-system.com/PCAN-Basic.239.0.html?&L=1)  

#### Linux

Build pcan library from [source](https://github.com/Ion-Mobility/PeakCanLib).  
Note: you need to build the `libpcanbasic.so` and build the pcan kernel module `pcan.ko`

#### MacOS

Pcan basic does not support on macOS

### Build soure code

To build all  
`cargo build`

To build command-line program.  
`cargo build --bin ion-diagnostic`  

Option: pass `-release` to build release binary

## To run test

`cargo test`