// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::Model;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.global::<TagListLayoutLogic>().on_calculate_positions(move |width, tags, widths| {
        println!("{:?}", widths);
        const PADDING: f32 = 8.;
        const CHAR_WIDTH_FACTOR: f32 = 10.;
        let mut ret_widths = vec![];
        let mut ret_xs = vec![];
        let mut ret_ys = vec![];
        let mut x = 0.;
        let mut y = 0.;
        for (i, tag) in tags.iter().enumerate() {
            let w = tag.len() as f32 * CHAR_WIDTH_FACTOR + (PADDING * 2.);
            if x + w >= width {
                x = 0.;
                y += 24. + PADDING;
            }
            ret_widths.push(w);
            ret_xs.push(x); 
            ret_ys.push(y);
            x += w + PADDING;
        }
        TagListLayoutSpec {
            xs: ret_xs.as_slice().into(),
            ys: ret_ys.as_slice().into(),
            widths: ret_widths.as_slice().into(),
            height: y + 24. + PADDING,
        }
    });

    ui.run()?;

    Ok(())
}
