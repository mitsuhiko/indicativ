extern crate indicatif;

use std::thread;
use std::time::Duration;
use std::borrow::Cow;

use indicatif::{ProgressBar, MultiProgress, ProgressStyle};

fn main() {
    let mut m = MultiProgress::new();
    let mut sty = ProgressStyle::default();
    sty.tick_chars = "⠁⠁⠉⠙⠚⠒⠂⠂⠒⠲⠴⠤⠄⠄⠤⠠⠠⠤⠦⠖⠒⠐⠐⠒⠓⠋⠉⠈⠈ ".chars().collect();
    sty.bar_template = Cow::Borrowed("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}");
    sty.progress_chars = "##-".chars().collect();

    let pb = m.add(ProgressBar::new(128));
    pb.set_style(sty.clone());
    let _ = thread::spawn(move || {
        for i in 0..128 {
            pb.set_message(&format!("item #{}", i + 1));
            pb.inc(1);
            thread::sleep(Duration::from_millis(15));
        }
        pb.finish_with_message("done");
    });

    let pb = m.add(ProgressBar::new(128));
    pb.set_style(sty.clone());
    let _ = thread::spawn(move || {
        for _ in 0..3 {
            pb.set_position(0);
            for i in 0..128 {
                pb.set_message(&format!("item #{}", i + 1));
                pb.inc(1);
                thread::sleep(Duration::from_millis(8));
            }
        }
        pb.finish_with_message("done");
    });

    let pb = m.add(ProgressBar::new(1024));
    pb.set_style(sty.clone());
    let _ = thread::spawn(move || {
        for i in 0..1024 {
            pb.set_message(&format!("item #{}", i + 1));
            pb.inc(1);
            thread::sleep(Duration::from_millis(2));
        }
        pb.finish_with_message("done");
    });

    m.join_and_clear().unwrap();
}
