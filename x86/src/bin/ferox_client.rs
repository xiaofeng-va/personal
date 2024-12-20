// To debug rustyline:
// RUST_LOG=rustyline=debug cargo run --example example 2> debug.log
//
use std::{
    borrow::Cow::{self, Borrowed, Owned}, io::Read, sync::mpsc, thread, time::Duration
};

use clap::Parser;
use ferox::{
    debug, info, proto::{data::FeroxProto, parse::parse_proto}
};
use rustyline::{
    completion::FilenameCompleter,
    highlight::{CmdKind, Highlighter, MatchingBracketHighlighter},
    hint::HistoryHinter,
    history::FileHistory,
    validate::MatchingBracketValidator,
    Cmd, CompletionType, Config, EditMode, Editor, KeyEvent,
};
use rustyline_derive::{Completer, Helper, Hinter, Validator};
use serde::de;
use serialport::SerialPort;
use x86::{
    errors::{CmdLineError, CmdResult},
    find_serial_device,
};

#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(long)]
    #[arg(required_unless_present("probe"))]
    port: Option<String>,

    #[arg(long)]
    #[arg(required_unless_present("port"))]
    probe: Option<String>,
}

fn main() -> rustyline::Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    println!("cli = {:?}", cli);

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();
    let h = MyHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter::new(),
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let port_name = match (cli.port, cli.probe) {
        (Some(port), _) => port,
        (_, Some(probe)) => find_serial_device(probe.as_str())
            .expect(format!("Not found port with probe {}", probe).as_str()),
        _ => None.expect("No port or probe in your arguments"),
    };
    let mut port = serialport::new(port_name, 115_200)
        .timeout(Duration::from_secs(5))
        .open()
        .expect("Failed to open port");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(60));
        tx.send(()).unwrap();
    });

    let mut count = 1;
    loop {
        match handle_single_command(&mut rl, &mut port, count) {
            Ok(_) => {}
            Err(CmdLineError::Quit) => {
                break;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        count += 1;
        if let Ok(_) = rx.try_recv() {
            while let Ok(_) = rx.try_recv() {
            }
            info!("Rx: Saving history");
            rl.append_history("history.txt")?;
        }
    }
    rl.append_history("history.txt")?;
    Ok(())
}

fn handle_single_command(
    rl: &mut Editor<MyHelper, FileHistory>,
    port: &mut Box<dyn SerialPort>,
    count: u32,
) -> CmdResult<()> {
    let prompt = format!("{count}> ");
    rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{prompt}\x1b[0m");

    let line = rl
        .readline(&prompt)
        .map_err(|e| CmdLineError::ReadlineError(e))?;
    rl.add_history_entry(line.as_str())
        .map_err(|e| CmdLineError::AddHistoryError(e))?;
    let msg = parse_proto(&line).map_err(|e| CmdLineError::ParseProtoError(e))?;

    if msg == FeroxProto::Quit {
        return Err(CmdLineError::Quit);
    }
    info!("Sending message: {:?}", msg);
    write_message(port, &msg)?;

    let resp = read_message(port)?;
    info!("Got response: {:?}", resp);

    Ok(())
}

fn write_message(port: &mut Box<dyn SerialPort>, msg: &FeroxProto) -> CmdResult<()> {
    let data =
        postcard::to_vec::<FeroxProto, 8>(&msg).map_err(|e| CmdLineError::PostcardError(e))?;
    // Write size (u16) first
    let size = data.len() as u8;
    let size_bytes = size.to_le_bytes();
    info!("Sending data: {:?}, {:?}", size_bytes, data);
    port.write(&size_bytes)
        .map_err(CmdLineError::SerialPortError)?;
    // Write actual data
    port.write(&data).map_err(CmdLineError::SerialPortError)?;
    port.flush().map_err(CmdLineError::SerialPortError)?;

    Ok(())
}

fn read_message(port: &mut Box<dyn SerialPort>) -> CmdResult<FeroxProto> {
    // Read size first
    let mut size_buf = [0u8; 1];
    port.read_exact(&mut size_buf)
        .map_err(CmdLineError::SerialPortError)?;
    let size = u8::from_le_bytes(size_buf) as usize;
    info!("Read size: {}", size);
    // Read data
    let mut data = vec![0u8; size as usize];
    port.read_exact(&mut data)
        .map_err(CmdLineError::SerialPortError)?;
    info!("Read data: {:?}", data);
    let resp = postcard::from_bytes::<FeroxProto>(&data).map_err(CmdLineError::PostcardError)?;
    info!("Parsed response: {:?}", resp);

    Ok(resp)
}
