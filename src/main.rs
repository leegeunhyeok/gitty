use embedded_graphics::pixelcolor::Rgb565;
use esp_idf_hal::{
    delay::{Ets, FreeRtos},
    gpio::{Gpio0, PinDriver},
    prelude::*,
    spi::{config::Config, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
};
use mipidsi::{
    interface::SpiInterface,
    models::{Model, GC9107},
    options::{ColorOrder, Orientation},
    Builder,
};
use mousefood::{
    fonts,
    prelude::*,
    ratatui::{self, layout::*, symbols::*, widgets::*},
};

// Pin mapping
// LCD_BL   = 10
// LCD_CD   = 6
// LCD_CS   = 5
// LCD_CLK  = 3
// LCD_MOSI = 2
// LCD_RES  = 1

const DISPLAY_WIDTH: u16 = 128;
const DISPLAY_HEIGHT: u16 = 128;

static mut BUFFER: [u8; 512] = [0; 512];

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let dc = PinDriver::output(peripherals.pins.gpio6).unwrap();
    let mut rst = PinDriver::output(peripherals.pins.gpio1).unwrap();
    let mut bl = PinDriver::output(peripherals.pins.gpio10).unwrap();

    rst.set_high().unwrap();

    let spi_driver = SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio3, // CLK
        peripherals.pins.gpio2, // MOSI
        None::<Gpio0>,
        &SpiDriverConfig::new(),
    )
    .unwrap();

    let dd = SpiDeviceDriver::new(
        spi_driver,
        Some(peripherals.pins.gpio5), // CS
        &Config::new()
            .baudrate(10.MHz().into())
            .data_mode(embedded_hal::spi::MODE_0),
    )
    .unwrap();

    let di = unsafe { SpiInterface::new(dd, dc, &mut BUFFER) };

    let (bw, bh) = GC9107::FRAMEBUFFER_SIZE;

    // Initialize display
    let mut display = Builder::new(GC9107, di)
        .reset_pin(rst)
        .color_order(ColorOrder::Bgr)
        .display_offset(bw - DISPLAY_WIDTH, bh - DISPLAY_HEIGHT)
        .display_size(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .orientation(Orientation::default())
        .init(&mut Ets)
        .unwrap();

    // Clear display initially and turn on the backlight
    display.clear(Rgb565::BLACK).unwrap();
    bl.set_low().unwrap(); // BL: Active low
    log::info!("Display initialized");

    let backend = EmbeddedBackend::new(
        &mut display,
        EmbeddedBackendConfig {
            font_regular: fonts::MONO_6X13,
            font_bold: fonts::MONO_6X13_BOLD,
            ..Default::default()
        },
    );
    let mut terminal = Terminal::new(backend).unwrap();

    App::default().run(&mut terminal).unwrap();
}

struct App {
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { exit: false }
    }
}

impl App {
    pub fn run<B: Backend>(&mut self, terminal: &mut ratatui::Terminal<B>) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            log::info!("Updated");
            FreeRtos::delay_ms(500);
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_set(border::THICK)
            .border_style(Style::new().fg(Color::DarkGray));

        let content_area = block.inner(area);
        let layout = Layout::vertical([
            Constraint::Max(3),
            Constraint::Length(0),
            Constraint::Max(3),
        ])
        .flex(Flex::SpaceBetween)
        .direction(Direction::Vertical)
        .split(content_area);

        block.render(area, buf);

        Paragraph::new(rainbow_text("Hello, world!"))
            .wrap(Wrap { trim: true })
            .centered()
            .render(layout[0], buf);

        Paragraph::new("Rust Ratatui on Embedded device! (ESP32S3)")
            .wrap(Wrap { trim: true })
            .centered()
            .render(layout[2], buf);
    }
}

fn rainbow_colors() -> Vec<Color> {
    vec![
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
    ]
}

fn rainbow_text(text: &str) -> Line<'_> {
    let colors = rainbow_colors();
    let mut spans = vec![];
    let mut skipped = 0;

    for (i, ch) in text.chars().enumerate() {
        if ch == ' ' {
            skipped += 1;
        }
        let color = colors[(i - skipped) % colors.len()];
        spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
    }

    Line::from(spans)
}
