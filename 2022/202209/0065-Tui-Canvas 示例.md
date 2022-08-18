# 0065-Tui-Canvas 示例

## 环境

- Time 2022-08-17
- Rust 1.63.0
- Tui 0.18.0

## 前言

### 说明

参考：<https://github.com/fdehau/tui-rs/blob/master/examples/canvas.rs>

### 目标

使用 `tui-rs` 显示 Canvas。

## 定义应用

```rust
struct App {
    x: f64,
    y: f64,
    ball: Rectangle,
    playground: layout::Rect,
    vx: f64,
    vy: f64,
    dir_x: bool,
    dir_y: bool,
}

impl App {
    fn new() -> App {
        App {
            x: 0.0,
            y: 0.0,
            ball: Rectangle {
                x: 10.0,
                y: 30.0,
                width: 10.0,
                height: 10.0,
                color: Color::Yellow,
            },
            playground: layout::Rect::new(10, 10, 100, 100),
            vx: 1.0,
            vy: 1.0,
            dir_x: true,
            dir_y: true,
        }
    }

    fn on_tick(&mut self) {
        if self.ball.x < self.playground.left() as f64
            || self.ball.x + self.ball.width > self.playground.right() as f64
        {
            self.dir_x = !self.dir_x;
        }
        if self.ball.y < self.playground.top() as f64
            || self.ball.y + self.ball.height > self.playground.bottom() as f64
        {
            self.dir_y = !self.dir_y;
        }

        if self.dir_x {
            self.ball.x += self.vx;
        } else {
            self.ball.x -= self.vx;
        }

        if self.dir_y {
            self.ball.y += self.vy;
        } else {
            self.ball.y -= self.vy
        }
    }
}
```

## ui

```rust
   let chunks = Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("世界地图"))
        .paint(|context| {
            context.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            context.print(
                app.x,
                -app.y,
                text::Span::styled("这里", Style::default().fg(Color::Yellow)),
            );
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    frame.render_widget(canvas, chunks[0]);

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Pong"))
        .paint(|context| {
            context.draw(&app.ball);
        })
        .x_bounds([10.0, 110.0])
        .y_bounds([10.0, 110.0]);
    frame.render_widget(canvas, chunks[1]);
```

## 效果展示

![Canvas][1]

## 总结

使用 `tui-rs` 渲染 Canvas。

[1]:images/tui-canvas.gif

## 附录

### 源码

```rust
use anyhow::{Context, Result};
use crossterm::{event, terminal, ExecutableCommand};
use layout::{Constraint, Layout};
use tui::backend::{Backend, CrosstermBackend};
use tui::style::{Color, Style};
use tui::{layout, text, widgets, Frame, Terminal};
use widgets::canvas::{Canvas, Map, MapResolution, Rectangle};
use widgets::{Block, Borders};

pub fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut backend = CrosstermBackend::new(std::io::stdout());
    backend
        .execute(terminal::EnterAlternateScreen)?
        .execute(terminal::Clear(terminal::ClearType::All))?
        .hide_cursor()?;
    let mut terminal = tui::Terminal::new(backend)?;
    run(&mut terminal)?;
    terminal::disable_raw_mode()?;
    terminal
        .backend_mut()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(terminal::LeaveAlternateScreen)?
        .show_cursor()
        .context("重置控制台失败")
}

struct App {
    x: f64,
    y: f64,
    ball: Rectangle,
    playground: layout::Rect,
    vx: f64,
    vy: f64,
    dir_x: bool,
    dir_y: bool,
}

impl App {
    fn new() -> App {
        App {
            x: 0.0,
            y: 0.0,
            ball: Rectangle {
                x: 10.0,
                y: 30.0,
                width: 10.0,
                height: 10.0,
                color: Color::Yellow,
            },
            playground: layout::Rect::new(10, 10, 100, 100),
            vx: 1.0,
            vy: 1.0,
            dir_x: true,
            dir_y: true,
        }
    }

    fn on_tick(&mut self) {
        if self.ball.x < self.playground.left() as f64
            || self.ball.x + self.ball.width > self.playground.right() as f64
        {
            self.dir_x = !self.dir_x;
        }
        if self.ball.y < self.playground.top() as f64
            || self.ball.y + self.ball.height > self.playground.bottom() as f64
        {
            self.dir_y = !self.dir_y;
        }

        if self.dir_x {
            self.ball.x += self.vx;
        } else {
            self.ball.x -= self.vx;
        }

        if self.dir_y {
            self.ball.y += self.vy;
        } else {
            self.ball.y -= self.vy
        }
    }
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let timeout = std::time::Duration::from_millis(500);
    let mut app = App::new();
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if event::poll(timeout)? {
            if let event::Event::Key(key) = event::read()? {
                use event::KeyCode::{self, Char, Esc};
                match key.code {
                    Char('q') | Char('Q') | Esc => return Ok(()),
                    KeyCode::Down => app.y += 1.0,
                    KeyCode::Up => app.y -= 1.0,
                    KeyCode::Right => app.x += 1.0,
                    KeyCode::Left => app.x -= 1.0,
                    _ => {}
                }
            }
        }
        app.on_tick();
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("世界地图"))
        .paint(|context| {
            context.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            context.print(
                app.x,
                -app.y,
                text::Span::styled("这里", Style::default().fg(Color::Yellow)),
            );
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    frame.render_widget(canvas, chunks[0]);

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Pong"))
        .paint(|context| {
            context.draw(&app.ball);
        })
        .x_bounds([10.0, 110.0])
        .y_bounds([10.0, 110.0]);
    frame.render_widget(canvas, chunks[1]);
}
```
