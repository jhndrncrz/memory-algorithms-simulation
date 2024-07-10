use std::io;
use std::io::prelude::*;
use std::process::Command;

// *********************************************************
// Displays
// *********************************************************
pub fn display_header() {
    const NAME: &'static str = "CRUZ, JOHN ADRIAN B.";
    const TERM: &'static str = "Midterm";
    const MODULE_NUMBER: i8 = 3;
    const ACTIVITY_NUMBER: i8 = 3;
    const ACTIVITY_TITLE: &'static str = "Dynamic Partition - First Fit";
  
    terminal_clear_screen(0);

    display_title(&format!(
      "{}L-M{}: ACT{} {}\n",
      TERM.as_bytes()[0] as char,
      MODULE_NUMBER,
      ACTIVITY_NUMBER,
      ACTIVITY_TITLE
    ));
    display_subtitle(&format!("{}\n", NAME));
    display("───────────────────────────────────────────────────────────\n");
}

pub fn display_footer() {
    display("───────────────────────────────────────────────────────────\n");
}

pub fn display_table(headers: &Vec<String>, table: &Vec<Vec<String>>) {
    let mut max_columns: Vec<usize> = vec![0; table.len()];

    for i in 0..table.len() {
        for data in &table[i] {
            if data.len() > max_columns[i] {
                max_columns[i] = data.len();
            }
        }

        if headers[i].len() > max_columns[i] {
            max_columns[i] = headers[i].len();
        }
    }

    for i in 0..table.len() {
        io::stdout().write("┼".as_bytes()).unwrap();
        for _ in 0..max_columns[i] + 2 {
            io::stdout().write("─".as_bytes()).unwrap();
        }
    }
    io::stdout().write("┼\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    for i in 0..table[0].len() {
        io::stdout().write("│".as_bytes()).unwrap();

        if i == 0 {
            for j in 0..table.len() {
                io::stdout()
                    .write(
                        text_apply_style(
                            &format!(" {: >width$} ", headers[j], width = max_columns[j]),
                            "bg-white",
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                io::stdout().write("│".as_bytes()).unwrap();
            }
            io::stdout().write("\n".as_bytes()).unwrap();
            io::stdout().flush().unwrap();

            for k in 0..table.len() {
                io::stdout().write("┼".as_bytes()).unwrap();
                for _ in 0..max_columns[k] + 2 {
                    io::stdout().write("─".as_bytes()).unwrap();
                }
            }
            io::stdout().write("┼\n".as_bytes()).unwrap();
            io::stdout().write("│".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }

        for j in 0..table.len() {
            io::stdout()
                .write(
                    if table[j][i].contains("[") && table[j][i].contains("]") {
                        text_apply_style(
                            &format!(" {: >width$} ", table[j][i], width = max_columns[j]),
                            "fg-red",
                        )
                    }
                    else if table[j][i].contains("FREE") {
                        text_apply_style(
                            &format!(" {: >width$} ", table[j][i], width = max_columns[j]),
                            "fg-yellow italic",
                        )
                    } 
                    else {
                        format!(" {: >width$} ", table[j][i], width = max_columns[j])
                    }
                    .as_bytes(),
                )
                .unwrap();
            io::stdout().write("│".as_bytes()).unwrap();
        }
        io::stdout().write("\n".as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
    for i in 0..table.len() {
        io::stdout().write("┼".as_bytes()).unwrap();
        for _ in 0..max_columns[i] + 2 {
            io::stdout().write("─".as_bytes()).unwrap();
        }
    }
    io::stdout().write("┼\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_bar(
    parts_count: usize,
    parts_label: &Vec<String>,
    parts_size_filled: &Vec<f64>,
    parts_size: &Vec<f64>,
) {
    let mut titles: Vec<String> = vec![];
    let mut values: Vec<f64> = vec![];

    for part_index in 0..parts_count {
        if parts_size_filled[part_index] == 0.0 {
            titles.push(parts_label[part_index].clone());
        } else {
            titles.push(format!(
                "{}",
                parts_label[part_index]
            ));
        }
        values.push(parts_size_filled[part_index]);
        values.push(parts_size[part_index]);
    }

    let mut parts: Vec<usize> = vec![];
    let mut current_part: usize = 0;
    let total_values: f64 = values.iter().sum::<f64>();

    for value in &values {
        let part: f64 = value / total_values * 100.0;
        parts.push(part.round() as usize);
    }

    for _ in 0..parts[0] {
        io::stdout().write("█".as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }

    for part_index in (1..parts.len()).step_by(2) {
        for _ in 0..parts[part_index] {
            io::stdout()
                .write(
                    &text_apply_style(
                        "█",
                        match current_part % 6 {
                            0 => "fg-red",
                            1 => "fg-green",
                            2 => "fg-yellow",
                            3 => "fg-blue",
                            4 => "fg-magenta",
                            5 => "fg-cyan",
                            _ => "",
                        },
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        for _ in 0..parts[part_index + 1] {
            io::stdout()
                .write(
                    &text_apply_style(
                        "🮀",
                        match current_part % 6 {
                            0 => "fg-red",
                            1 => "fg-green",
                            2 => "fg-yellow",
                            3 => "fg-blue",
                            4 => "fg-magenta",
                            5 => "fg-cyan",
                            _ => "",
                        },
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        io::stdout().flush().unwrap();
        current_part += 1;
    }
    io::stdout().write("\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    io::stdout()
        .write(
            &text_apply_style(
                &format!("{}: {} M / {} M", titles[0], values[0], values[0]),
                "fg-white",
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().write("\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    current_part = 0;
    for value_index in (1..values.len()).step_by(2) {
        terminal_clear_line(0);
        terminal_cursor_right(parts[0..value_index].iter().sum());
        io::stdout()
            .write(
                &text_apply_style(
                    &format!(
                        "{}: {} M / {} M",
                        titles[(value_index - 1) / 2 + 1],
                        values[value_index],
                        values[value_index] + values[value_index + 1]
                    ),
                    match current_part % 6 {
                        0 => "fg-red",
                        1 => "fg-green",
                        2 => "fg-yellow",
                        3 => "fg-blue",
                        4 => "fg-magenta",
                        5 => "fg-cyan",
                        _ => "",
                    },
                )
                .as_bytes(),
            )
            .unwrap();
        if current_part != values.len() {
            io::stdout().write("\n".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        current_part += 1;
    }
}

pub fn display(string: &str) {
    if string.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout().write(string.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_pause() {
    if cfg!(windows) {
        Command::new("cmd").args(["/c", "pause"]).status().unwrap();
    } else if cfg!(unix) {
        Command::new("bash")
            .args([
                "-c",
                &format!(
                    "read -n 1 -s -r -p \"{}\"",
                    text_apply_style(&"Press any key to continue. . .", "italic")
                ),
            ])
            .status()
            .unwrap();
    }
    terminal_clear_line(0);
}

pub fn display_title(string: &str) {
    if string.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(text_apply_style(string, "bold").as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_subtitle(string: &str) {
    if string.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(text_apply_style(string, "italic").as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_divider(n: usize) {
    for _ in 0..n {
        io::stdout().write("\n".as_bytes()).unwrap();
    }
    io::stdout().flush().unwrap();
}

pub fn display_prompt(message: &str, descriptor: &str) -> String {
    terminal_clear_line(0);
    io::stdout()
        .write(format!("{} {}: ", message, text_apply_style(descriptor, "italic")).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input;
}

pub fn display_confirm(message: &str, descriptor: &str) -> char {
    terminal_clear_line(0);
    io::stdout()
        .write(
            format!(
                "{} {}? ([Y]es/[N]o): ",
                message,
                text_apply_style(descriptor, "italic")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input.to_uppercase().chars().next().unwrap();
}

pub fn display_labelled(label: &str, value: &str) {
    if value.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(format!("{}: {}", text_apply_style(label, "italic"), value).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_log(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style(" LOG ", "bg-white"),
                text_apply_style(message, "italic")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_success(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style(" SUCCESS ", "bold bg-green"),
                text_apply_style(message, "italic fg-green")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_error(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style(" ERROR ", "bold bg-red"),
                text_apply_style(message, "italic fg-red")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_info(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style(" INFO ", "bold bg-yellow"),
                text_apply_style(message, "italic fg-yellow")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();
}

// *********************************************************
// Terminal control
// *********************************************************
pub fn terminal_clear_screen(n: i8) {
    match n {
        -1 => {
            io::stdout().write("\x1B[1J\x1B[1;1H".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        1 => {
            io::stdout().write("\x1B[0J\x1B[1;1H".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        _ => {
            io::stdout().write("\x1B[2J\x1B[1;1H".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
    }
}

pub fn terminal_clear_line(n: i8) {
    match n {
        -1 => {
            io::stdout().write("\x1B[1K\x1B[1G".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        1 => {
            io::stdout().write("\x1B[0K\x1B[1G".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        _ => {
            io::stdout().write("\x1B[2K\x1B[1G".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
    }
}

pub fn terminal_cursor_up(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}A", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_down(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}B", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_right(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}C", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_left(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}D", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_next_line(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}E", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_previous_line(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}F", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_clear_previous_lines(n: usize) {
    terminal_clear_line(0);
    for _ in 1..n {
        terminal_cursor_previous_line(1);
        terminal_clear_line(0);
    }
}

pub fn terminal_clear_next_lines(n: usize) {
    terminal_clear_line(0);
    for _ in 1..n {
        terminal_cursor_next_line(1);
        terminal_clear_line(0);
    }
}

// *********************************************************
// Text style
// *********************************************************
pub fn text_apply_style(string: &str, styles: &str) -> String {
    let mut ansi_style_codes: String = String::new();

    for style in styles.split(" ") {
        ansi_style_codes.push_str(match style {
            "bold" => "\x1B[1m",
            "italic" => "\x1B[3m",
            "underline" => "\x1B[4m",
            "fg-black" => "\x1B[30m",
            "fg-red" => "\x1B[31m",
            "fg-green" => "\x1B[32m",
            "fg-yellow" => "\x1B[33m",
            "fg-blue" => "\x1B[34m",
            "fg-magenta" => "\x1B[35m",
            "fg-cyan" => "\x1B[36m",
            "fg-white" => "\x1B[37m",
            "bg-red" => "\x1B[41m",
            "bg-green" => "\x1B[42m",
            "bg-yellow" => "\x1B[43m",
            "bg-blue" => "\x1B[44m",
            "bg-magenta" => "\x1B[45m",
            "bg-cyan" => "\x1B[46m",
            "bg-white" => "\x1B[47m",
            _ => "",
        });
    }

    return format!("{}{}\x1B[0m", ansi_style_codes, string);
}
