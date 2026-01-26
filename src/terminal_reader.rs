use crossterm::execute;
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};
use crossterm::event::{KeyEvent,KeyEventKind,read,Event,KeyModifiers,KeyCode};
use crossterm::terminal::{EnterAlternateScreen,LeaveAlternateScreen};
use crossterm::cursor::{MoveTo,SetCursorStyle};
use std::io::{Result,Write};
use std::process::exit;

pub fn terminal_reader () -> Result<String>{
    let mut buffer = String::new();
    execute!(std::io::stdout(), EnterAlternateScreen)?;
    execute!(std::io::stdout(), MoveTo(0,0))?;
    execute!(std::io::stdout(), SetCursorStyle::BlinkingBar)?;
    enable_raw_mode()?;
    // print!(">");
    loop {
        std::io::stdout().flush()?;

        let event:Event  = read()?;

        match event {
            // ctrl + C 
            Event::Key(KeyEvent{
                code: KeyCode::Char('c'),
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) if modifiers.contains(KeyModifiers::CONTROL) => {
                disable_raw_mode()?;
                println!("^C");
                execute!(std::io::stdout(), LeaveAlternateScreen)?;
                exit(0);
            },

            Event::Key(KeyEvent{
                code: KeyCode::Char('s'),
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) if modifiers.contains(KeyModifiers::CONTROL) => {
                    disable_raw_mode()?;
                    execute!(std::io::stdout(), LeaveAlternateScreen)?;
                    return Ok(buffer);
            },

            // shifted characters (shift + a-z) and regular characters (a-z, 0-9, symbols)
            Event::Key(KeyEvent{
                code: KeyCode::Char(c),
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    buffer.push(c.to_ascii_uppercase());
                    print!("{}",c.to_ascii_uppercase());
                } else {
                    buffer.push(c);
                    print!("{}",c);
                }
            }

            // enter key 
            Event::Key(KeyEvent { 
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => {
                buffer.push('\n');
                print!("\n");
            }

            // backspace key
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if buffer.pop().is_some() {
                    print!("\x08 \x08");
                } else {
                    continue;
                }
            }

            // left key
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                ..
            }) => {
                print!("\x1b[D");
            }

            // right key
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                ..
            }) => {
                print!("\x1b[C");
            }

            //up key
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                ..
            }) => {
                print!("\x1b[A");
            }   

            //down key
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                ..
            }) => {
                print!("\x1b[B");
            }
            
            _ => {},
        }
    } 
}