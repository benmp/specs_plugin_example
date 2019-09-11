#[no_mangle]
pub fn it_works(
    crossterm: &crossterm::Crossterm,
    character: &components::Character,
    position: &mut components::Position,
) {
    crossterm.cursor().goto(position.x, position.y);
    crossterm.terminal().write(position.x);

    position.x = position.x + 1;
    position.y = position.y + 1;
}
