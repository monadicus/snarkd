use tui::{
    backend::Backend,
    layout::Margin,
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Tabs},
    Frame,
};

use super::state::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    f.render_widget(
        Tabs::new(vec![Spans::from("Peers"), Spans::from("Blocks")])
            .block(Block::default().borders(Borders::ALL).title("Peers")),
        f.size(),
    );
    let items: Vec<ListItem> = app
        .data
        .peers
        .clone()
        .iter()
        .map(|(k, _v)| {
            ListItem::new(vec![Spans::from(Span::styled(
                k.to_string(),
                Style::default(),
            ))])
        })
        .collect();
    f.render_widget(
        List::new(items),
        f.size().inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
    )
}
