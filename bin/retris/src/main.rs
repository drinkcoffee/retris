mod state;
use state::get_colour;

use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand, 
    terminal, cursor, style::{self, Stylize}
};

use std::time::Duration;

use crossterm::{
    cursor::position,
    event::{poll, read, /*DisableMouseCapture, EnableMouseCapture, */Event, KeyCode},
    /* execute, */
    terminal::{disable_raw_mode, enable_raw_mode},
};


fn main() -> io::Result<()> {
  let mut stdout = io::stdout();

  stdout.execute(terminal::Clear(terminal::ClearType::All))?;

  for y in 0..40 {
    for x in 0..150 {
      if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        // in this loop we are more efficient by not flushing the buffer.
        stdout
          .queue(cursor::MoveTo(x,y))?
          .queue(style::PrintStyledContent( "█".magenta()))?;
      }
    }
  }
  stdout.flush()?;

    enable_raw_mode()?;

    loop {

        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
                let pos = position();
                let colour = get_colour(5, 7);
                stdout
                .queue(cursor::MoveTo(5,7))?
                .queue(style::PrintStyledContent( "X".red()))?;
                stdout.flush()?;
      
            }

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            // Timeout expired, no event for 1s
            println!(".\r");
        }
    }



    disable_raw_mode()?;

  Ok(())
}





// fn main() -> crossterm::Result<()> {
//     let _clean_up = CleanUp;
//     terminal::enable_raw_mode()?;
//     /* modify */
//     let editor = Editor::new();
//     while editor.run()? {}
//     /* end */
//     Ok(())
// }



// fn main() {
// }


// use std::io;
// use std::sync::mpsc;
// use std::sync::mpsc::Receiver;
// use std::sync::mpsc::TryRecvError;
// use std::{thread, time};

// fn main() {
//     println!("Rust Tetris");
//     let stdin_channel = spawn_stdin_channel();
//     loop {
//         match stdin_channel.try_recv() {
//             Ok(key) => println!("Received: {}", key),
//             Err(TryRecvError::Empty) => println!("Channel empty"),
//             Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
//         }
//         sleep(1000);
//     }
// }

// // Copied from :https://stackoverflow.com/questions/30012995/how-can-i-read-non-blocking-from-stdin#:~:text=To%20read%20from%20a%20blocking,produced%20any%20data%20so%20far.
// fn spawn_stdin_channel() -> Receiver<String> {
//     let (tx, rx) = mpsc::channel::<String>();
//     thread::spawn(move || loop {
//         let mut buffer = String::new();
//         io::stdin().read_line(&mut buffer).unwrap();
//         tx.send(buffer).unwrap();
//     });
//     rx
// }

// fn sleep(millis: u64) {
//     let duration = time::Duration::from_millis(millis);
//     thread::sleep(duration);
// }