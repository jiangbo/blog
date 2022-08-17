# 0067-Tui-panic 处理

## 环境

- Time 2022-08-17
- Rust 1.63.0
- Tui 0.18.0

## 前言

### 说明

参考：<https://github.com/fdehau/tui-rs/blob/master/examples/panic.rs>

### 目标

使用 `tui-rs` 定义一个 panic hook。

## 定义组件

```rust
#[derive(Default)]
struct App {
    hook_enabled: bool,
}

impl App {
    fn chain_hook(&mut self) {
        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            reset_terminal().unwrap();
            original_hook(panic);
        }));

        self.hook_enabled = true;
    }
}
```

## reset_terminal

```rust
fn reset_terminal() -> Result<()> {
    terminal::disable_raw_mode()?;
    std::io::stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}
```

## ui

```rust
fn ui<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let text = vec![
        if app.hook_enabled {
            Spans::from("HOOK IS CURRENTLY **ENABLED**")
        } else {
            Spans::from("HOOK IS CURRENTLY **DISABLED**")
        },
        Spans::from(""),
        Spans::from("press `p` to panic"),
        Spans::from("press `e` to enable the terminal-resetting panic hook"),
        Spans::from("press any other key to quit without panic"),
        Spans::from(""),
        Spans::from("when you panic without the chained hook,"),
        Spans::from("you will likely have to reset your terminal afterwards"),
        Spans::from("with the `reset` command"),
        Spans::from(""),
        Spans::from("with the chained panic hook enabled,"),
        Spans::from("you should see the panic report as you would without tui"),
        Spans::from(""),
        Spans::from("try first without the panic handler to see the difference"),
    ];

    let b = Block::default()
        .title("Panic Handler Demo")
        .borders(Borders::ALL);

    let p = Paragraph::new(text).block(b).alignment(Alignment::Center);

    frame.render_widget(p, frame.size());
}
```

## 总结

使用 `tui-rs` 创建 panic hook，即使在 panic 的时候，也可以重置终端。

## 附录

### 源码

```rust
use anyhow::Result;
use crossterm::{event, terminal, ExecutableCommand};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::Alignment;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph};
use tui::{Frame, Terminal};

pub fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut backend = CrosstermBackend::new(std::io::stdout());
    backend
        .execute(terminal::EnterAlternateScreen)?
        .execute(terminal::Clear(terminal::ClearType::All))?
        .hide_cursor()?;
    let mut terminal = tui::Terminal::new(backend)?;
    run(&mut terminal)?;
    reset_terminal()
}

fn reset_terminal() -> Result<()> {
    terminal::disable_raw_mode()?;
    std::io::stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}

#[derive(Default)]
struct App {
    hook_enabled: bool,
}

impl App {
    fn chain_hook(&mut self) {
        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            reset_terminal().unwrap();
            original_hook(panic);
        }));

        self.hook_enabled = true;
    }
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let timeout = std::time::Duration::from_millis(500);
    let mut app = App::default();
    loop {
        terminal.draw(|frame| ui(frame, &app))?;

        if event::poll(timeout)? {
            if let event::Event::Key(key) = event::read()? {
                use event::KeyCode::{Char, Esc};
                match key.code {
                    Char('q') | Char('Q') | Esc => return Ok(()),
                    Char('p') => panic!("intentional demo panic"),
                    Char('e') => app.chain_hook(),
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let text = vec![
        if app.hook_enabled {
            Spans::from("HOOK IS CURRENTLY **ENABLED**")
        } else {
            Spans::from("HOOK IS CURRENTLY **DISABLED**")
        },
        Spans::from(""),
        Spans::from("press `p` to panic"),
        Spans::from("press `e` to enable the terminal-resetting panic hook"),
        Spans::from("press any other key to quit without panic"),
        Spans::from(""),
        Spans::from("when you panic without the chained hook,"),
        Spans::from("you will likely have to reset your terminal afterwards"),
        Spans::from("with the `reset` command"),
        Spans::from(""),
        Spans::from("with the chained panic hook enabled,"),
        Spans::from("you should see the panic report as you would without tui"),
        Spans::from(""),
        Spans::from("try first without the panic handler to see the difference"),
    ];

    let b = Block::default()
        .title("Panic Handler Demo")
        .borders(Borders::ALL);

    let p = Paragraph::new(text).block(b).alignment(Alignment::Center);

    frame.render_widget(p, frame.size());
}
```
