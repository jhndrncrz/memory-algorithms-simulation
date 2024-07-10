use std::io;
use std::io::prelude::*;
use std::process::Command;

pub fn application_start() {
    if cfg!(windows) {
        Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    } else if cfg!(unix) {
        Command::new("clear").status().unwrap();
    }

    terminal_clear_screen(0);
}

pub fn application_close() {
    display_pause();
    terminal_clear_screen(0);
}

pub fn display_table(headers: Vec<String>, table: Vec<&Vec<String>>) {
    let mut max_columns: Vec<usize> = vec![0; table.len()];

    for i in 0..table.len() {
        for data in table[i] {
            if data.len() > max_columns[i] {
                max_columns[i] = data.len();
            }
        }

        if headers[i].len() > max_columns[i] {
            max_columns[i] = headers[i].len();
        }
    }

    for i in 0..table.len() {
        io::stdout().write("+".as_bytes()).unwrap();
        for _ in 0..max_columns[i] + 2 {
            io::stdout().write("-".as_bytes()).unwrap();
        }
    }
    io::stdout().write("+\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
  
    for i in 0..table[0].len() {
        io::stdout().write("|".as_bytes()).unwrap();

        if i == 0 {
            for j in 0..table.len() {
                io::stdout().write(text_apply_style(&format!(" {: >width$} ", headers[j], width = max_columns[j]), "bg-white").as_bytes()).unwrap();   
                io::stdout().write("|".as_bytes()).unwrap();
            }
            io::stdout().write("\n".as_bytes()).unwrap();
            io::stdout().flush().unwrap();

            for k in 0..table.len() {
              io::stdout().write("+".as_bytes()).unwrap();
              for _ in 0..max_columns[k] + 2 {
                  io::stdout().write("-".as_bytes()).unwrap();
              }
            }
            io::stdout().write("+\n".as_bytes()).unwrap();
            io::stdout().write("|".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        
        for j in 0..table.len() {
            io::stdout().write(format!(" {: >width$} |", table[j][i], width = max_columns[j]).as_bytes()).unwrap();
        }
        io::stdout().write("\n".as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
    for i in 0..table.len() {
      io::stdout().write("+".as_bytes()).unwrap();
      for _ in 0..max_columns[i] + 2 {
          io::stdout().write("-".as_bytes()).unwrap();
      }
    }
    io::stdout().write("+\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_bar(titles: &Vec<&str>, values: &Vec<f64>) {
    let mut partitions: Vec<usize> = vec![];
    let mut current_partition = 0;
    let total_values: f64 = values.iter().sum();

    for value in values {
        let partition: f64 = value / total_values * 50.0;
        partitions.push(partition.round() as usize);
    }

    for partition in &partitions {
        for _ in 0..*partition {
            io::stdout()
                .write(
                    &text_apply_style(
                        "█",
                        match current_partition {
                            0 => "fg-blue",
                            1 => "fg-magenta",
                            2 => "fg-cyan",
                            _ => "",
                        },
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        io::stdout().flush().unwrap();
        current_partition += 1;
    }
    io::stdout().write("\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
    
    current_partition = 0;
    for _ in values {
        terminal_clear_line(0);
        if current_partition > 0 {
              terminal_cursor_right(partitions[0..current_partition].iter().sum());
        }
        io::stdout()
            .write(
                &text_apply_style(
                    &format!(
                        "{}: {} K",
                        titles[current_partition], values[current_partition]
                    ),
                    match current_partition {
                        0 => "fg-blue",
                        1 => "fg-magenta",
                        2 => "fg-cyan",
                        _ => "",
                    },
                )
                .as_bytes(),
            )
            .unwrap();
        if current_partition != partitions.len() {
              io::stdout().write("\n".as_bytes()).unwrap();
              io::stdout().flush().unwrap();
        }
        current_partition += 1;
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

pub fn display_info(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style("INFO", "bold"),
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

pub fn display_warning(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style(" WARNING ", "bold bg-yellow"),
                text_apply_style(message, "italic fg-yellow")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();
}

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
