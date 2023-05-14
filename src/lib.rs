use context::{Context, FinishedFrame};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent,
        KeyboardEnhancementFlags,
    },
    execute,
    style::Attribute,
    terminal::{
        disable_raw_mode, enable_raw_mode, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    QueueableCommand,
};

use math_util::VecI2;
use screen::ScreenCellIterator;
use std::{
    io::{self, Stdout, Write},
    time::{Duration, Instant},
};
use style::Style;

pub mod containers;
pub mod context;
pub mod id;
pub mod input;
pub mod math_util;
pub mod memory;
pub mod response;
pub mod screen;
pub mod style;
pub mod symbols;
pub mod ui;

pub trait App {
    fn update(&mut self, ctx: &Context);
}

pub fn start_app(app: impl App) -> Result<(), io::Error> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let mut stdout = io::stdout();

        // restore terminal
        disable_raw_mode().unwrap();
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture).unwrap();
        execute!(stdout, EnableLineWrap).unwrap();
        execute!(stdout, crossterm::cursor::Show).unwrap();

        hook(info);
    }));

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    execute!(stdout, DisableLineWrap)?;
    execute!(stdout, crossterm::cursor::Hide)?;
    // execute!(
    //     stdout,
    //     crossterm::event::PushKeyboardEnhancementFlags(
    //         crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
    //     )
    // )?;
    execute!(stdout, crossterm::event::EnableFocusChange)?;
    // let app = App::new(driverstation, log);
    let res = run_app(stdout, app, std::time::Duration::from_millis(40));

    let mut stdout = io::stdout();
    // restore terminal
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    execute!(stdout, EnableLineWrap)?;
    execute!(stdout, crossterm::cursor::Show)?;
    // execute!(stdout, crossterm::event::PopKeyboardEnhancementFlags)?;
    execute!(stdout, crossterm::event::DisableFocusChange)?;

    res
}

fn run_app(mut stdout: Stdout, mut app: impl App, tick_rate: Duration) -> io::Result<()> {
    let mut last_frame;
    let (x, y) = crossterm::terminal::size()?;

    let mut ctx = Context::new(VecI2::new(x, y));

    let mut data: Vec<u8> = Vec::new();

    loop {
        let mut lock = ctx.inner().write().unwrap();
        lock.start_frame();
        drop(lock);

        app.update(&ctx);

        let mut lock = ctx.inner().write().unwrap();
        let frame_report = lock.get_finished_frame();
        let written = output_to_terminal(&mut stdout, &mut data, frame_report)?;

        lock.finish_frame(written);
        drop(lock);

        last_frame = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_frame.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                let event = event::read()?;

                if let Event::Key(key) = event {
                    if let event::KeyEvent {
                        code: KeyCode::Esc, ..
                    } = key
                    {
                        return Ok(());
                    }
                }

                match event {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        ..
                    }) => {
                        data.queue(crossterm::terminal::Clear(
                            crossterm::terminal::ClearType::All,
                        ))?;
                    }
                    _ => {}
                }

                if ctx.handle_event(event) {
                    break;
                } else {
                    continue;
                }
            }
            break;
        }
    }
}

fn output_to_terminal(
    stdout: &mut Stdout,
    data: &mut Vec<u8>,
    frame_report: FinishedFrame<'_>,
) -> std::io::Result<usize> {
    let FinishedFrame {
        resized,
        mut current_frame,
        mut last_frame,
    } = frame_report;

    if resized {
        data.queue(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;
    }

    let mut last_fg = None;
    let mut last_bg = None;
    let mut last_attr = None;
    let mut last_position = None;

    let mut update_prev = true;
    let mut update_now = true;
    let mut curr_prev = None;
    let mut curr_new = None;
    loop {
        if update_now {
            update_now = false;
            curr_new = current_frame.next();
        }
        if update_prev {
            update_prev = false;
            curr_prev = last_frame.next();
        }

        let (text, style, pos) = match (curr_new, curr_prev) {
            (Some(curr), Some(prev)) => {
                // thats very convienient
                if curr == prev {
                    update_prev = true;
                    update_now = true;
                    continue;
                }

                if curr.2 == prev.2 {
                    update_prev = true;
                    update_now = true;
                    // same position different text/style
                    curr
                } else if curr.2.y == prev.2.y {
                    if curr.2.x > prev.2.x {
                        update_prev = true;
                        (" ", Style::default(), prev.2)
                    } else if curr.2.x < prev.2.x {
                        update_now = true;
                        curr
                    } else {
                        panic!()
                    }
                } else if curr.2.y < prev.2.y {
                    update_now = true;
                    curr
                } else if curr.2.y > prev.2.y {
                    update_prev = true;
                    (" ", Style::default(), prev.2)
                } else {
                    panic!()
                }
            }
            (None, None) => break,
            (None, Some((_bruh, _style, pos))) => {
                update_prev = true;
                // idk if a single space will do cells with a larger character width might do somethin funky
                (" ", Style::default(), pos)
            }
            (Some(to_draw), None) => {
                update_now = true;
                to_draw
            }
        };

        if last_position == Some(pos) {
            let mut next = pos;
            next.x += unicode_width::UnicodeWidthStr::width(text) as u16;
            last_position = Some(next)
        } else {
            if let Some(old) = last_position {
                if old.x == pos.x {
                    data.queue(crossterm::cursor::MoveToRow(pos.y))?;
                } else if old.y == pos.y {
                    data.queue(crossterm::cursor::MoveToColumn(pos.x))?;
                } else {
                    data.queue(crossterm::cursor::MoveTo(pos.x, pos.y))?;
                }
            } else {
                data.queue(crossterm::cursor::MoveTo(pos.x, pos.y))?;
            }
            let mut next = pos;
            next.x += unicode_width::UnicodeWidthStr::width(text) as u16;
            last_position = Some(next);
        }

        //todo make this better
        if last_attr != Some(style.attributes) {
            let mut attr = style.attributes;
            attr.set(Attribute::Reset);
            data.queue(crossterm::style::SetAttributes(attr))?;
            last_attr = Some(style.attributes);
            data.queue(crossterm::style::SetForegroundColor(style.fg))?;
            last_fg = Some(style.fg);
            data.queue(crossterm::style::SetBackgroundColor(style.bg))?;
            last_bg = Some(style.bg);
        }

        if last_fg != Some(style.fg) {
            data.queue(crossterm::style::SetForegroundColor(style.fg))?;
            last_fg = Some(style.fg);
        }
        if last_bg != Some(style.bg) {
            data.queue(crossterm::style::SetBackgroundColor(style.bg))?;
            last_bg = Some(style.bg);
        }

        data.queue(crossterm::style::Print(text))?;
    }

    stdout.write_all(data)?;
    stdout.flush()?;
    let len = data.len();
    data.clear();

    Ok(len)
}
