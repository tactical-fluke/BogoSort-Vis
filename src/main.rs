extern crate core;

use crate::egui::plot::Plot;
use crate::egui::Context;
use bogo_sort::bogo_sort::{is_sorted, randomize_order};
use eframe::egui::plot::Legend;
use eframe::{egui, Frame};
use egui::plot::BarChart;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let options = eframe::NativeOptions::default();
    let iv = randomize_order(&(0..7).map(|a| a as u32).collect());
    let list = Arc::new(Mutex::new(iv));
    let second_list = list.clone();

    let close_flag = Arc::new(Mutex::new(false));
    let thread_close_flag = close_flag.clone();

    let thread = thread::spawn(|| run_sort_in_background(list, thread_close_flag));

    eframe::run_native(
        "Bogo Sort",
        options,
        Box::new(|_cc| {
            Box::new(BogoSortApp {
                values: second_list,
            })
        }),
    );

    *close_flag.lock().unwrap() = true;
    thread.join().unwrap();
}

fn run_sort_in_background(values: Arc<Mutex<Vec<u32>>>, close_flag: Arc<Mutex<bool>>) {
    let start = std::time::Instant::now();
    loop {
        {
            if *close_flag.lock().unwrap() {
                return;
            }
            let mut vals = values.lock().unwrap();
            let new_val = randomize_order(&*vals);
            *vals = new_val;
            if is_sorted(&*vals) {
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
    let after = std::time::Instant::now();
    let duration = after - start;
    println!("duration: {} seconds", duration.as_secs());

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("./sound.mp3").unwrap());
    let source = rodio::Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    thread::sleep(Duration::from_secs(10));
}

struct BogoSortApp {
    values: Arc<Mutex<Vec<u32>>>,
}

impl eframe::App for BogoSortApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bogo Sort");
            let chart = BarChart::new(
                self.values
                    .lock()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(x, y)| egui::plot::Bar::new(x as f64, *y as f64).width(1.0))
                    .collect(),
            )
            .color(egui::Color32::LIGHT_BLUE)
            .name("Values");

            Plot::new("Normal Distribution Demo")
                .legend(Legend::default())
                .data_aspect(1.0)
                .allow_boxed_zoom(false)
                .show(ui, |plot_ui| plot_ui.bar_chart(chart))
                .response
        });
        ctx.request_repaint_after(Duration::from_millis(500));
    }
}
