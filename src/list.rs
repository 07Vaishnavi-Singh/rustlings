use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Borders, HighlightSpacing, Row, Table, TableState},
    Terminal,
};
use std::io;

use crate::{exercise::Exercise, state::State};

fn table<'a>(state: &State, exercises: &'a [Exercise]) -> Table<'a> {
    let header = Row::new(["Next", "State", "Name", "Path"]);

    let max_name_len = exercises
        .iter()
        .map(|exercise| exercise.name.len())
        .max()
        .unwrap_or(4) as u16;

    let widths = [
        Constraint::Length(4),
        Constraint::Length(7),
        Constraint::Length(max_name_len),
        Constraint::Fill(1),
    ];

    let rows = exercises
        .iter()
        .zip(&state.progress)
        .enumerate()
        .map(|(ind, (exercise, done))| {
            let exercise_state = if *done {
                "DONE".green()
            } else {
                "PENDING".yellow()
            };

            let next = if ind == state.next_exercise_ind {
                ">>>>".bold().red()
            } else {
                Span::default()
            };

            Row::new([
                next,
                exercise_state,
                Span::raw(&exercise.name),
                Span::raw(exercise.path.to_string_lossy()),
            ])
        })
        .collect::<Vec<_>>();

    Table::new(rows, widths)
        .header(header)
        .column_spacing(2)
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::new().bg(ratatui::style::Color::Rgb(50, 50, 50)))
        .highlight_symbol("🦀")
        .block(Block::default().borders(Borders::BOTTOM))
}

pub fn list(state: &State, exercises: &[Exercise]) -> Result<()> {
    let mut stdout = io::stdout().lock();

    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(&mut stdout))?;
    terminal.clear()?;

    let table = table(state, exercises);

    let last_ind = exercises.len() - 1;
    let mut selected = 0;
    let mut table_state = TableState::default().with_selected(Some(selected));

    'outer: loop {
        terminal.draw(|frame| {
            let area = frame.size();

            frame.render_stateful_widget(
                &table,
                Rect {
                    x: 0,
                    y: 0,
                    width: area.width,
                    height: area.height - 1,
                },
                &mut table_state,
            );

            // Help footer
            frame.render_widget(
                Span::raw("↓/j ↑/k home/g end/G │ Filter <d>one/<p>ending │ <r>eset │ <c>ontinue at │ <q>uit"),
                Rect {
                    x: 0,
                    y: area.height - 1,
                    width: area.width,
                    height: 1,
                },
            );
        })?;

        let key = loop {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    break key;
                }
                // Redraw
                Event::Resize(_, _) => continue 'outer,
                // Ignore
                Event::FocusGained | Event::FocusLost | Event::Mouse(_) | Event::Paste(_) => (),
            }
        };

        match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Down | KeyCode::Char('j') => {
                selected = selected.saturating_add(1).min(last_ind);
                table_state.select(Some(selected));
            }
            KeyCode::Up | KeyCode::Char('k') => {
                selected = selected.saturating_sub(1).max(0);
                table_state.select(Some(selected));
            }
            KeyCode::Home | KeyCode::Char('g') => {
                selected = 0;
                table_state.select(Some(selected));
            }
            KeyCode::End | KeyCode::Char('G') => {
                selected = last_ind;
                table_state.select(Some(selected));
            }
            _ => (),
        }
    }

    drop(terminal);
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
