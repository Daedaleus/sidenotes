use druid::widget::*;
use druid::{Color, Command, Insets, Target, Widget};

use crate::models::*;
use crate::ui::commands;
use crate::ui::theme::{CARD_COLOR, STATUS_COLOR};
use crate::ui::widgets::ClickableArea;

pub fn todo_builder() -> impl Widget<Todo> {
    let with_state = Label::new(|todo: &Todo, _env: &_| todo.state.clone().unwrap_or_default())
        .with_text_color(Color::BLACK)
        .padding(2.0)
        .background(STATUS_COLOR)
        .rounded(2.0);
    let tags = List::new(tag_builder).with_spacing(4.).lens(Todo::tags);
    let row = Flex::row()
        .with_child(with_state)
        .with_spacer(4.)
        .with_child(tags);
    let with_state = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(todo_title_builder())
        .with_spacer(4.0)
        .with_child(row);
    let without_state = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(todo_title_builder())
        .with_spacer(4.0)
        .with_child(List::new(tag_builder).lens(Todo::tags));

    let state = Either::new(
        |todo: &Todo, _: &_| todo.state.is_some(),
        with_state,
        without_state,
    );

    state
        .padding(4.0)
        .background(CARD_COLOR)
        .rounded(2.0)
        .padding(Insets::uniform_xy(0., 2.))
        .expand_width()
        .controller(ClickableArea)
        .on_click(|event_ctx, todo: &mut Todo, _: &_| {
            event_ctx.submit_command(Command::new(
                commands::OPEN_TODO,
                todo.clone(),
                Target::Auto,
            ))
        })
}

fn tag_builder() -> impl Widget<String> {
    Label::new(|tag: &String, _env: &_| tag.clone())
        .with_text_color(Color::BLACK)
        .padding(2.0)
        .background(STATUS_COLOR)
        .rounded(2.0)
}

fn todo_title_builder() -> impl Widget<Todo> {
    Label::new(|item: &Todo, _env: &_| item.title.clone())
        .with_line_break_mode(LineBreaking::WordWrap)
}
