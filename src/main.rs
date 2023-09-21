

use std::io::{Error, Read, Write};

use std::process::exit;
use std::time::Duration;
use serialport::SerialPort;

use clap::{Parser, command, arg};

type IOErrorKind = std::io::ErrorKind;
type SerialErrorKind = serialport::ErrorKind;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Serial port path
    #[arg(short,long)]
    port: String,

    ///Silently timeout after n ms of no input? Only applies after the first byte has been received
    #[arg(short,long)]
    timeout: Option<u64>,

    ///The baud rate for the port
    #[arg(short,long)]
    baud: u32
}

fn main() {
    let args = Args::parse();

    let serial = serialport::new(&args.port, args.baud);
    let mut serial = match serial.open() {
        Ok(v) => {v}
        Err(e) => {
            match e.kind {
                SerialErrorKind::NoDevice => {
                    eprintln!("No device was found for port '{}'",&args.port);
                    exit(-1);
                }
                SerialErrorKind::InvalidInput => {
                    eprintln!("'{}' is not valid input", &args.port);
                }
                SerialErrorKind::Unknown => {
                    eprintln!("Unknown error opening port");
                }
                SerialErrorKind::Io(ioe) => {
                    eprintln!("IoError encountered when opening port {:?}", ioe);
                }
            }

            exit(-1);
        }
    };


    read(&mut serial, args.timeout);


}

fn read(serial: &mut Box<dyn SerialPort>, timeout: Option<u64>) {

    // Stops reads from blocking eternally.
    // Timeouts are ignored, this is the most efficient way to avoid polling with breaks and instead relying on the os' updates with occasional breaks
    serial.set_timeout(Duration::from_millis(1)).expect("TODO: panic message");

    let mut reader =utf8_read::Reader::new(serial);
    let mut last_time_time = std::time::Instant::now();
    let mut received_any = false;
    let mut stdout_handle = std::io::stdout().lock();
    loop{

        // While the program is running, keep trying to read from the serial port
        match reader.next_char() {
            Err(e) => {
                match e {
                    utf8_read::Error::IoError(ioe) => {
                        match ioe.kind() {
                            IOErrorKind::TimedOut => {
                                //Ignore read timeouts
                            }
                            _ =>{
                                eprintln!("{:?}", ioe);
                            }
                        }

                    }

                    _ => {
                        eprintln!("{:?}", e);
                    }
                }
            }

            Ok(v) => {
                last_time_time = std::time::Instant::now();
                received_any = true;
                stdout_handle.write_fmt(format_args!("{}", v)).unwrap();
            }
        };

        if let Some(ms) = timeout{
            let diff = std::time::Instant::now().duration_since(last_time_time);
            if diff > Duration::from_millis(ms) && received_any {
                    break;
            }
        }
    }

    stdout_handle.flush().unwrap();
}
