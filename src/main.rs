// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::Model;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.global::<FlowLayoutLogic>().on_calculate_positions(move |width, num_elements, padding, spacing, preferred_widths, preferred_heights| {
        let mut ret_widths = vec![];
        let mut ret_heights = vec![];
        let mut ret_xs = vec![];
        let mut ret_ys = vec![];
        let mut x = 0.;
        let mut y = 0.;
        for i in 0..num_elements {
            let w = padding + preferred_widths.row_data(i as usize).unwrap_or(1.0) + padding;
            let h = padding + preferred_heights.row_data(i as usize).unwrap_or(1.0) + padding;
            if x + w >= width {
                x = 0.;
                y += h + spacing;
            }
            ret_widths.push(w);
            ret_heights.push(h);
            ret_xs.push(x); 
            ret_ys.push(y);
            x += w + spacing;
        }
        FlowLayoutResults {
            xs: ret_xs.as_slice().into(),
            ys: ret_ys.as_slice().into(),
            widths: ret_widths.as_slice().into(),
            heights: ret_heights.as_slice().into(),
            height: y + ret_heights[0] + spacing,
        }
    });

    ui.run()?;

    Ok(())
}
