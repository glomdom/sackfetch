use colored::Colorize;
use sysinfo::{Disks, System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let system_info = vec![
        format!(
            "{} {}",
            "OS \u{eb70}".bright_green(),
            System::name().unwrap_or_default()
        ),
        format!(
            "{} {}",
            "Kernel Version \u{eb70}".bright_yellow(),
            System::kernel_version().unwrap_or_default()
        ),
        format!(
            "{} {} | [{}]",
            "Memory \u{eb70}".bright_purple(),
            get_memory(&mut sys),
            memory_usage_bar(&mut sys, 30)
        ),
        format!(
            "{} {}",
            "Number of CPUs \u{eb70}".bright_blue(),
            sys.cpus().len()
        ),
        format!(
            "{} {}",
            "Uptime \u{eb70}".bright_cyan(),
            format_uptime(System::uptime())
        ),
        format!(
            "{} {}",
            "Host \u{eb70}".bright_red(),
            System::host_name().unwrap_or_default()
        ),
        format!("{} {}", "Disk Usage \u{eb70}".yellow(), get_disk_usage()),
    ];

    for item in system_info {
        println!("{}", item);
    }
}

fn get_disk_usage() -> String {
    if let Some(disk) = Disks::new_with_refreshed_list().first() {
        let total = disk.total_space() / (1024 * 1024 * 1024);
        let available = disk.available_space() / (1024 * 1024 * 1024);
        format!("{}GB / {}GB", total - available, total)
    } else {
        "No disk data available".to_string()
    }
}

fn format_uptime(seconds: u64) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    let hours = hours % 24;
    let minutes = minutes % 60;

    format!("{}d {}h {}m", days, hours, minutes)
}

fn get_memory(sys: &mut System) -> String {
    format!(
        "{}MB / {}MB",
        sys.used_memory() / (1024 * 1024),
        sys.total_memory() / (1024 * 1024)
    )
}

fn memory_usage_bar(sys: &mut System, bar_length: usize) -> String {
    let current_memory = sys.used_memory() / (1024 * 1024);
    let total_memory = sys.total_memory() / (1024 * 1024);
    let usage_percentage = current_memory as f64 / total_memory as f64;
    let filled_length = (usage_percentage * bar_length as f64).round() as usize;

    let mut bar = String::new();

    for i in 0..bar_length {
        let pos_percentage = i as f64 / bar_length as f64;

        let part = if pos_percentage < 0.1 {
            "█".green()
        } else if pos_percentage < 0.2 {
            "█".bright_green()
        } else if pos_percentage < 0.3 {
            "█".bright_yellow()
        } else if pos_percentage < 0.4 {
            "█".yellow()
        } else if pos_percentage < 0.5 {
            "█".bright_magenta()
        } else if pos_percentage < 0.6 {
            "█".magenta()
        } else if pos_percentage < 0.7 {
            "█".bright_red()
        } else if pos_percentage < 0.8 {
            "█".red()
        } else if pos_percentage < 0.9 {
            "█".bright_black()
        } else {
            "█".black()
        };

        if i < filled_length {
            bar.push_str(&part.to_string());
        } else {
            bar.push_str(" ");
        }
    }

    bar
}
