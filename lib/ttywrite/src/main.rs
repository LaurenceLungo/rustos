mod parsers;

use serial;
use structopt;
use structopt_derive::StructOpt;
use xmodem::{Xmodem, Progress};

use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader, Write, Read, stdin};

    let opt = Opt::from_args();
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    // FIXME: Implement the `ttywrite` utility.
    let mut tty_settings = port.read_settings().unwrap();
    tty_settings.set_baud_rate(opt.baud_rate).expect("failed to set baud rate");
    tty_settings.set_char_size(opt.char_width);
    tty_settings.set_flow_control(opt.flow_control);
    tty_settings.set_stop_bits(opt.stop_bits);
    port.write_settings(&tty_settings).expect("failed to write tty settings");
    
    let timeout = Duration::new(opt.timeout,0);
    port.set_timeout(timeout).expect("port failed to set time out");
    let mut buffer = String::new();
    match opt.input {
        Some(input_source) => {
            let f = File::open(input_source).expect("failed to open file");
            let mut reader = BufReader::new(f);
            reader.read_to_string(&mut buffer).expect("failed to read line");
        },
        None => {
            stdin().read_to_string(&mut buffer).expect("stdin failed to read");
        },
    }
    let input = buffer.as_bytes();
    if opt.raw {
        port.write(input).expect("port failed to write");
    } else {
        Xmodem::transmit_with_progress(input, port, progress_fn).expect("failed to transmit with xmodem");
    }
    
}
