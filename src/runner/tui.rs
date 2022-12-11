use std::{fmt::Display, io::stderr, path::PathBuf, time::Duration};

use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, ListState},
    Terminal,
};

crate::error_wrapper! {
    Error {
        IoError(std::io::Error),
    }
}

fn handle_list_events(state: &mut ListState, max: usize) -> Result<Option<usize>, Error> {
    while crossterm::event::poll(Duration::from_secs(1))? {
        let key = match crossterm::event::read()? {
            Event::Key(key) => key,
            _ => continue,
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Up => {
                let mut selected = state.selected().unwrap_or(0);
                selected += 1;
                if selected >= max {
                    selected = max - 1;
                }
                state.select(Some(selected));
            }
            KeyCode::Down => {
                let mut selected = state.selected().unwrap_or(0);
                selected = selected.saturating_sub(1);
                state.select(Some(selected));
            }
            KeyCode::Enter => {
                return Ok(state.selected());
            }
            _ => {}
        }
    }
    Ok(None)
}

fn select<B, T, I>(title: &str, term: &mut Terminal<B>, iter: I) -> Result<T, Error>
where
    B: tui::backend::Backend,
    T: Display,
    I: IntoIterator<Item = T>,
{
    let mut vec = iter.into_iter().collect::<Vec<_>>();
    let mut i = 0;

    let list_items = vec
        .iter()
        .map(|t| {
            let text = format!("{}", t);
            ListItem::new(text)
        })
        .collect::<Vec<_>>();

    let mut list_state = ListState::default();
    list_state.select(Some(1));
    let list = List::new(list_items)
        .highlight_symbol(">>")
        .block(Block::default().title(title).border_type(BorderType::Plain));

    let value = loop {
        term.draw(|f| {
            let list = list.clone();
            let area = f.size();
            f.render_stateful_widget(list, area, &mut list_state);
        })?;

        println!("{}", i);
        i += 1;

        if let Some(i) = handle_list_events(&mut list_state, vec.len())? {
            break vec.drain(i..=i).next().unwrap();
        }
    };
    Ok(value)
}

pub fn select_file<B>(term: &mut Terminal<B>) -> Result<PathBuf, Error>
where
    B: Backend,
{
    let mut current_dir = std::env::current_dir()?;
    let backtrack = "..".to_string();

    loop {
        let dir = std::fs::read_dir(current_dir.clone())?;
        let names = dir.filter_map(|r| r.ok().map(|d| d.file_name()));
        let entries = names.map(|e| e.to_str().unwrap().to_owned());
        let entries = [backtrack.clone()].into_iter().chain(entries);
        let selected = select("Select input", term, entries)?;

        current_dir.push(selected);

        if current_dir.is_file() {
            break;
        }
    }

    Ok(current_dir)
}

pub fn run() -> Result<(), Error> {
    execute!(stderr(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr());
    let mut terminal = Terminal::new(backend)?;

    let year = select("Select year", &mut terminal, advent_of_code::get_years())?;
    let day = select("Select day", &mut terminal, year.days())?;
    let task = select("Select task", &mut terminal, day.tasks())?;

    let input = select_file(&mut terminal)?;
    let input = std::fs::read_to_string(input)?;

    task.run(input);

    execute!(stderr(), LeaveAlternateScreen)?;

    Ok(())
}
