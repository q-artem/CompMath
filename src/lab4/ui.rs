use crate::lab4::functions::{MAX_POINTS, MIN_POINTS, Point, variant_function};
use crate::lab4::models::ApproximationResult;
use crate::lab4::solver::solve_lsm;
use eframe::egui;
use egui_plot::{HLine, Legend, Line, Plot, PlotPoints, Points, VLine};
use std::fs;

pub fn run_ui() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Лабораторная работа №4 - Аппроксимация функций"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Lab4 GUI",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            for text_style in style.text_styles.values_mut() {
                text_style.size *= 1.3;
            }
            cc.egui_ctx.set_style(style);
            Box::new(Lab4App::new())
        }),
    );
}

struct Lab4App {
    points: Vec<Point>,
    results: Vec<ApproximationResult>,
    best_idx: Option<usize>,

    status_msg: String,
    x_input: String,
    y_input: String,
    io_message: String,

    // UI state for scaling
    reset_plot: bool,
}

impl Lab4App {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            results: Vec::new(),
            best_idx: None,
            status_msg: "Ожидание ввода данных...".into(),
            x_input: String::new(),
            y_input: String::new(),
            io_message: "Готово к работе".into(),
            reset_plot: true,
        }
    }

    fn calculate(&mut self) {
        if self.points.len() < MIN_POINTS {
            self.status_msg = format!("Ошибка: Требуется минимум {} точек!", MIN_POINTS);
            return;
        }
        self.results = solve_lsm(&self.points);
        if !self.results.is_empty() {
            let mut best = 0;
            let mut min_eps = self.results[0].epsilon;
            for (i, res) in self.results.iter().enumerate() {
                if res.epsilon < min_eps {
                    min_eps = res.epsilon;
                    best = i;
                }
            }
            self.best_idx = Some(best);
            self.status_msg = "Вычисления успешно завершены.".into();
        }
    }

    fn load_variant(&mut self) {
        self.points.clear();
        let mut x = 0.0;
        while x <= 2.01 && self.points.len() < MAX_POINTS {
            self.points.push(Point {
                x,
                y: variant_function(x),
            });
            x += 0.2;
        }
        self.calculate();
        self.io_message = "Вариант №6 успешно загружен.".into();
        self.reset_plot = true;
    }

    fn load_from_file(&mut self) {
        if let Ok(content) = fs::read_to_string("lab4_input.txt") {
            let mut new_points = Vec::new();
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let (Ok(x), Ok(y)) = (
                        parts[0].replace(',', ".").parse::<f64>(),
                        parts[1].replace(',', ".").parse::<f64>(),
                    ) {
                        new_points.push(Point { x, y });
                    }
                }
                if new_points.len() >= MAX_POINTS {
                    break;
                }
            }
            if new_points.len() >= MIN_POINTS {
                self.points = new_points;
                self.calculate();
                self.io_message = format!("Загружено {} точек из файла.", self.points.len());
                self.reset_plot = true;
            } else {
                self.io_message =
                    format!("Ошибка: В файле должно быть минимум {} точек!", MIN_POINTS);
            }
        } else {
            self.io_message = "Ошибка: Не удалось открыть lab4_input.txt".into();
        }
    }

    fn save_to_file(&mut self) {
        if self.results.is_empty() {
            self.io_message = "Ошибка: Нет данных для сохранения!".into();
            return;
        }

        let mut report = String::new();
        report.push_str("ОТЧЕТ ОБ АППРОКСИМАЦИИ ФУНКЦИЙ (МНК)\n");
        report.push_str("=========================================\n\n");

        report.push_str("ИСХОДНЫЕ ТОЧКИ:\n");
        report.push_str("  x\t\t  y\n");
        for p in &self.points {
            report.push_str(&format!("{:.4}\t\t{:.4}\n", p.x, p.y));
        }
        report.push_str("\n-----------------------------------------\n");

        if let Some(best) = self.best_idx {
            let res = &self.results[best];
            report.push_str(&format!("НАИЛУЧШАЯ МОДЕЛЬ: {}\n", res.model_type));
            report.push_str(&format!("Уравнение: {}\n", res.formula()));
            report.push_str(&format!("СКО (RMSD): {:.6}\n", res.epsilon));
            report.push_str(&format!("Коэф. детерминации R²: {:.6}\n\n", res.r_squared));
        }

        report.push_str("СРАВНИТЕЛЬНАЯ ТАБЛИЦА ВСЕХ МОДЕЛЕЙ:\n");
        report.push_str(&format!(
            "{:<45} {:<10} {:<10} {:<10}\n",
            "Тип функции", "S", "RMSD", "R^2"
        ));
        for res in &self.results {
            report.push_str(&format!(
                "{:<45} {:<10.4} {:<10.4} {:<10.4}\n",
                res.model_type.to_string(),
                res.s,
                res.epsilon,
                res.r_squared
            ));
        }

        match fs::write("lab4_output.txt", report) {
            Ok(_) => self.io_message = "Результаты сохранены в lab4_output.txt".into(),
            Err(e) => self.io_message = format!("Ошибка сохранения: {}", e),
        }
    }
}

impl eframe::App for Lab4App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            std::process::exit(0);
        }
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.heading("Управление данными");
                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.set_width(380.0);
                    if ui.button("Загрузить вариант №6").clicked() {
                        self.load_variant();
                    }
                    if ui.button("Загрузить из lab4_input.txt").clicked() {
                        self.load_from_file();
                    }
                    if ui.button("Сохранить результаты в файл").clicked() {
                        self.save_to_file();
                    }
                    if ui.button("Очистить все точки").clicked() {
                        self.points.clear();
                        self.results.clear();
                        self.best_idx = None;
                        self.io_message = "Данные очищены.".into();
                        self.reset_plot = true;
                    }
                    if ui.button("Сбросить масштаб графика").clicked() {
                        self.reset_plot = true;
                    }
                });

                ui.add_space(20.0);
                ui.heading("Добавить точку");
                ui.group(|ui| {
                    ui.set_width(380.0);
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(egui::TextEdit::singleline(&mut self.x_input).desired_width(60.0));
                        ui.label("Y:");
                        ui.add(egui::TextEdit::singleline(&mut self.y_input).desired_width(60.0));
                    });

                    let can_add = self.points.len() < MAX_POINTS;
                    if ui
                        .add_enabled(can_add, egui::Button::new("Добавить точку"))
                        .clicked()
                    {
                        if let (Ok(x), Ok(y)) = (
                            self.x_input.replace(',', ".").parse::<f64>(),
                            self.y_input.replace(',', ".").parse::<f64>(),
                        ) {
                            self.points.push(Point { x, y });
                            self.io_message = format!("Добавлена точка ({:.2}, {:.2})", x, y);
                            self.x_input.clear();
                            self.y_input.clear();
                            if self.points.len() >= MIN_POINTS {
                                self.calculate();
                            }
                            self.reset_plot = true;
                        } else {
                            self.io_message = "Ошибка: Некорректные координаты!".into();
                        }
                    }
                    if !can_add {
                        ui.label(
                            egui::RichText::new(format!("Достигнут лимит ({} точек)", MAX_POINTS))
                                .color(egui::Color32::KHAKI),
                        );
                    }
                });

                ui.add_space(20.0);
                ui.label(egui::RichText::new(&self.io_message).color(egui::Color32::LIGHT_GREEN));
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                ui.heading("Результаты анализа");
                ui.label(&self.status_msg);

                if let Some(best) = self.best_idx {
                    let res = &self.results[best];
                    ui.add_space(10.0);
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        format!("Лучшая модель: {}", res.model_type),
                    );
                    ui.label(format!("Формула: {}", res.formula()));
                    ui.label(format!("RMSD (ε): {:.6}", res.epsilon));
                    ui.label(format!("R²: {:.6}", res.r_squared));
                    if let Some(r) = res.r_pearson {
                        ui.label(format!("r (Pearson): {:.6}", r));
                    }
                } else {
                    ui.label(format!(
                        "Добавьте от {} до {} точек для анализа.",
                        MIN_POINTS, MAX_POINTS
                    ));
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("lsm_plot")
                .view_aspect(2.0)
                .legend(Legend::default())
                .allow_zoom(true)
                .allow_drag(true)
                .allow_scroll(true);

            plot.show(ui, |plot_ui| {
                if self.reset_plot {
                    if !self.points.is_empty() {
                        let min_x = self
                            .points
                            .iter()
                            .map(|p| p.x)
                            .fold(f64::INFINITY, f64::min);
                        let max_x = self
                            .points
                            .iter()
                            .map(|p| p.x)
                            .fold(f64::NEG_INFINITY, f64::max);
                        let min_y = self
                            .points
                            .iter()
                            .map(|p| p.y)
                            .fold(f64::INFINITY, f64::min);
                        let max_y = self
                            .points
                            .iter()
                            .map(|p| p.y)
                            .fold(f64::NEG_INFINITY, f64::max);

                        let pad_x = (max_x - min_x).abs().max(1.0) * 0.2;
                        let pad_y = (max_y - min_y).abs().max(1.0) * 0.2;

                        plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                            [min_x - pad_x, min_y - pad_y],
                            [max_x + pad_x, max_y + pad_y],
                        ));
                    } else {
                        plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                            [-1.0, -1.0],
                            [3.0, 3.0],
                        ));
                    }
                    self.reset_plot = false;
                }

                plot_ui.hline(HLine::new(0.0).color(egui::Color32::GRAY).width(1.0));
                plot_ui.vline(VLine::new(0.0).color(egui::Color32::GRAY).width(1.0));

                // Исходные точки
                let points_data: Vec<[f64; 2]> = self.points.iter().map(|p| [p.x, p.y]).collect();
                plot_ui.points(
                    Points::new(PlotPoints::from(points_data))
                        .color(egui::Color32::RED)
                        .radius(6.0)
                        .name("Исходные данные"),
                );

                let colors = [
                    egui::Color32::LIGHT_BLUE,
                    egui::Color32::from_rgb(0, 200, 0), // Green
                    egui::Color32::GOLD,
                    egui::Color32::LIGHT_GRAY,
                    egui::Color32::from_rgb(200, 0, 200), // Purple
                    egui::Color32::from_rgb(200, 100, 0), // Orange
                ];

                if !self.results.is_empty() {
                    let bounds = plot_ui.plot_bounds();
                    let min_x = bounds.min()[0];
                    let max_x = bounds.max()[0];

                    for (i, res) in self.results.iter().enumerate() {
                        let is_best = self.best_idx == Some(i);
                        let color = if is_best {
                            egui::Color32::YELLOW
                        } else {
                            colors[i % colors.len()]
                        };
                        let width = if is_best { 3.0 } else { 1.5 };

                        let curve_points: PlotPoints = (0..500)
                            .map(|j| {
                                let x = min_x + (max_x - min_x) * (j as f64 / 499.0);
                                let y = res.predict(x);
                                [x, y]
                            })
                            .filter(|p| p[1].is_finite() && p[1].abs() < 1e6)
                            .collect();

                        plot_ui.line(
                            Line::new(curve_points)
                                .color(color)
                                .width(width)
                                .name(res.model_type.to_string()),
                        );
                    }
                }
            });
        });
    }
}
