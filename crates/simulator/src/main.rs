use mousefood::prelude::*;
use ui::App;

fn main() -> Result<(), std::io::Error> {
    let mut display = display::implements::get_display();
    let backend = display::implements::to_backend(&mut display);
    let terminal = Terminal::new(backend)?;
    let app = App::default();

    app.run(terminal)?;

    Ok(())
}

mod display;
mod profile;
mod ui;
