use anyhow::Result;
use resvg::usvg::{Options, Transform, Tree};
use std::sync::Arc;
use tiny_skia::Pixmap;

static HP_BAR_TEMPLATE: &str = include_str!("../../assets/hp_bar.svg");
static FONT_DATA: &[u8] = include_bytes!("../../assets/font.ttf");

pub fn generate_hp_bar(current: i32, max: i32, name: &str) -> Result<Vec<u8>> {
    let percentage = (current as f32 / max as f32).clamp(0.0, 1.0);
    let bar_width = 560.0 * percentage;
    let bar_color = if percentage > 0.3 {
        "#00E5FF"
    } else {
        "#FF2E2E"
    };

    let svg_data = HP_BAR_TEMPLATE
        .replace("{{NAME}}", name)
        .replace("{{CURRENT_HP}}", &current.to_string())
        .replace("{{MAX_HP}}", &max.to_string())
        .replace("{{COLOR}}", bar_color)
        .replace("{{WIDTH}}", &bar_width.to_string());

    let mut font_db = fontdb::Database::new();
    font_db.load_font_data(FONT_DATA.to_vec());

    let opt = Options {
        fontdb: Arc::new(font_db),
        font_family: "Rajdhani".to_string(),
        ..Default::default()
    };

    let tree = Tree::from_str(&svg_data, &opt)?;

    let mut pixmap = Pixmap::new(600, 150).ok_or(anyhow::anyhow!("Buffer alloc failed"))?;

    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());

    Ok(pixmap.encode_png()?)
}
