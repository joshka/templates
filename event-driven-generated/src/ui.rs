use indoc::indoc;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Modifier, Style, Stylize,
    },
    symbols,
    text::ToLine,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::App;

struct Theme {
    title: Style,
    primary: Style,
    secondary: Style,
}

static THEME: Theme = Theme {
    title: Style::new()
        .fg(BLUE.c600)
        .bg(SLATE.c200)
        .add_modifier(Modifier::BOLD),
    primary: Style::new().fg(SLATE.c200).bg(SLATE.c950),
    secondary: Style::new().fg(BLUE.c400).bg(SLATE.c900),
};

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    /// - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = "event-driven-generated"
            .to_line()
            .centered()
            .bold()
            .style(THEME.title);
        let block = Block::new()
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .style(THEME.primary)
            .border_style(THEME.title)
            .title(title);
        let inner = block.inner(area);
        block.render(area, buf);

        let [message_area, counter_area] = Layout::vertical(Constraint::from_lengths([4, 1]))
            .margin(1)
            .spacing(1)
            .areas(inner);

        let message = indoc! {"
            This is a Ratatui app built with the event-driven template.

            Press <Esc>, <Ctrl-C> or <q> to exit.
            Press <Left> / <Right> to increment / decrement the counter.
        "};
        Paragraph::new(message)
            .centered()
            .style(THEME.primary)
            .render(message_area, buf);

        let counter_text = format!("Counter: {}", self.counter);
        Paragraph::new(counter_text)
            .centered()
            .style(THEME.secondary)
            .render(counter_area, buf);
    }
}
