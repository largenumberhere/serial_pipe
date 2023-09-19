# Serial Pipe
A very basic program for reading from serial and piping it to another process or file.

The following example opens serial port `COM4` (on windows) with baud `57600` and waits until the first byte is recieved and then exits with success when the port has been quiet for 2 **seconds** . 
All the output is written into `output.txt`.  
```
cargo run -- --port COM4 --baud 57600 --timeout 2000 > output.txt
```
<br>

### Features
- The program has a few handy options:
```
Usage: serial-pipe.exe [OPTIONS] --port <PORT> --baud <BAUD>

Options:
  -p, --port <PORT>        Serial port path
  -t, --timeout <TIMEOUT>  Silently timeout after n ms of no input? Only applies after the first byte has been received
  -b, --baud <BAUD>        The baud rate for the port
  -h, --help               Print help
  -V, --version            Print version
```
- It is made with the `serialport` crate so it should work fine on many platforms.


### Non-features: 
- Utf8 is the only supported serial encoding.

### Build it yourself
Dependencies:  
- `cargo` with min rust `1.56.1`
- Packages (**linux only**):
  [serialport-rs](https://github.com/serialport/serialport-rs) requires a few packages. For detailed explanation see the crate documentation. In summary, install:
  1. `pkg-config`, ` pkgconf-pkg-config`, or similar
  2. `libudev-dev`, `systemd-devel` or similar

Build the program with `cargo build --release`. 
The executable file will be stored under `target\release\`  as a file called `serial-pipe.exe` (windows) or just `serial-pipe` (linux).

### Binaries
Please express interest if you would like prebuilt binaries to become available