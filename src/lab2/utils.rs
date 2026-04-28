use color_eyre::Result;
use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea, PathElement},
    series::LineSeries,
    style::{BLACK, Color, IntoFont, RED, WHITE},
};

pub fn diff_x(f: fn(f64, f64) -> f64, x: f64, y: f64, eps: f64) -> f64 {
    (f(x + eps, y) - f(x, y)) / eps
}

pub fn diff_y(f: fn(f64, f64) -> f64, x: f64, y: f64, eps: f64) -> f64 {
    (f(x, y + eps) - f(x, y)) / eps
}

pub fn plot_equation(f: impl Fn(f64) -> f64, a: f64, b: f64, file_name: &str) -> Result<()> {
    let root = BitMapBackend::new(file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let steps = 1000;
    let step_size = (b - a) / steps as f64;

    let mut min_y: f64 = 1000.0;
    let mut max_y: f64 = -1000.0;

    let points: Vec<(f64, f64)> = (0..=steps)
        .filter_map(|i| {
            let x = a + (i as f64) * step_size;
            let y = f(x);

            if y.is_finite() {
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
                Some((x, y))
            } else {
                None
            }
        })
        .collect();

    let y_padding = (max_y - min_y) * 0.1;
    let (y_min_padded, y_max_padded) = if y_padding == 0.0 {
        (min_y - 1.0, max_y + 1.0) // Если функция константа
    } else {
        (min_y - y_padding, max_y + y_padding)
    };

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("График функции на отрезке [{}, {}]", a, b),
            ("sans-serif", 30).into_font(),
        )
        .margin(15)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(a..b, y_min_padded..y_max_padded)?;

    chart
        .configure_mesh()
        .x_desc("Ось X")
        .y_desc("Ось Y")
        .draw()?;

    chart
        .draw_series(LineSeries::new(points, &RED))?
        .label("f(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}