use std::io::{Read, Write};
use std::process::exit;
use std::time::Duration;
use serialport::SerialPort;

use clap::{Parser, command, arg};

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
            eprint!("Failed to open serial port '{}' Perhaps it's being used by another application? {:?}", &args.port , e);
            exit(-1);
        }
    };

    read(&mut serial, args.timeout);


}

fn read(serial: &mut Box<dyn SerialPort>, timeout: Option<u64>) {

    let mut buff= [0; 16];
    let mut last_time_time = std::time::Instant::now();
    let mut received_any = false;
    loop{

        let availible = serial.bytes_to_read().expect("Failed to read from serial port. Perhaps it has been disconnected?");
        if availible > 0{
            received_any = true;
            if timeout.is_some() {
                last_time_time = std::time::Instant::now();
            }
            let end = if availible > 16 {
                16usize
            } else{
                availible as usize
            };
            let result = serial.read_exact(&mut buff[0..end]);
            let read_count = match result {
                Err(e) => {
                    eprintln!("{:?}", e);
                    0
                },
                Ok(_) => {
                    end
                }
            };

            let str = std::str::from_utf8(&buff[0..read_count]);
            match str {
                Err(e) => {
                    eprintln!("Unexpected non-utf-8 bytes received {:?}.\n[{:?}]", e, buff);
                }
                Ok(v) => {
                    let mut stdout_handle = std::io::stdout().lock();

                    // Write to console
                    stdout_handle.write_fmt(format_args!("{}", v))
                        .expect("Failed to write to stdout!!!");

                    // Make it immediately visible
                    stdout_handle.flush()
                        .expect("Failed to flush stdout!");
                }
            }
        }

        if let Some(ms) = timeout{
            let diff = std::time::Instant::now().duration_since(last_time_time);
            if diff > Duration::from_millis(ms) && received_any {
                    break;
            }
        }



    }

}
