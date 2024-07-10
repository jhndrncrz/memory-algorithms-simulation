// https://onlinegdb.com/lNNzIRM8M

use activity::*;

fn main() {
    const NAME: &'static str = "CRUZ, JOHN ADRIAN B.";
    const TERM: &'static str = "Preliminary";
    const MODULE_NUMBER: i8 = 3;
    const ACTIVITY_NUMBER: i8 = 5;
    const ACTIVITY_TITLE: &'static str = "Single-User Contiguous Scheme";

    let memory_capacity: f64;
    let memory_os: f64;
    let mut memory_occupied: f64;
    let mut memory_unoccupied: f64;
    let mut memory_is_currently_allocated: bool = false;

    let number_of_jobs: usize;
    let mut number_of_failed_jobs: usize = 0;

    let mut job_names: Vec<String> = vec![];
    let mut job_values: Vec<f64> = vec![];
    let mut job_successfully_allocated: Vec<bool> = vec![];
    let mut job_failed_allocations_count: usize = 0;

    let mut is_last_input_valid: bool = true;

    application_start();

    display_title(&format!(
        "{}L-M{}: ACT{} {}\n",
        TERM.as_bytes()[0] as char,
        MODULE_NUMBER,
        ACTIVITY_NUMBER,
        ACTIVITY_TITLE
    ));
    display_subtitle(&format!("{}\n", NAME));

    display_divider(2);
    display("-------------------------------------------------\n");

    memory_capacity = input_memory_capacity(&mut is_last_input_valid);
    memory_os = input_memory_os(memory_capacity, &mut is_last_input_valid);
    number_of_jobs = input_number_of_jobs(&mut is_last_input_valid);

    display_divider(1);

    input_jobs(
        number_of_jobs,
        &mut job_names,
        &mut job_values,
        &mut job_successfully_allocated,
        &mut is_last_input_valid,
    );

    display_divider(1);

    display_title("List of Jobs\n");
    display_job_table(&job_names, &job_values);

    display_divider(2);

    display_title("Allocation Log\n");
    display_info("Allocating memory for OS...\n");
    display_success("Memory for OS allocated successfully.\n");
    display_bar(
        &vec!["OS", "Unused Space"],
        &vec![memory_os, memory_capacity - memory_os],
    );

    display_divider(2);

    display_pause();

    for job_index in 0..number_of_jobs {
        let job_name = &job_names[job_index];
        let job_value = job_values[job_index];

        display_info(&format!("Allocating memory for {}...\n", job_name));
        if memory_capacity - memory_os > job_value {
            memory_occupied = job_value;
            memory_unoccupied = memory_capacity - memory_os - job_value;
            display_success(&format!("Memory for {} allocated successfully.\n", job_name));
            display_bar(
                &vec!["OS", &job_name, "Unused Space"],
                &vec![memory_os, memory_occupied, memory_unoccupied],
            );
            memory_is_currently_allocated = true;
            job_successfully_allocated[job_index] = true;
        } else {
            display_warning(&format!(
                "Memory size of {} does not fit into available memory.\n",
                job_name
            ));
            display_error(&format!("Failed to allocate memory for {}.\n", job_name));
            number_of_failed_jobs += 1;
            job_successfully_allocated[job_index] = false;
        }

        if memory_is_currently_allocated {
            loop {
                terminal_clear_line(0);
                let confirm_input: char = display_confirm("Deallocate", &job_name);

                match confirm_input {
                    'Y' => {
                        display_info(&format!("Deallocating memory for {}...\n", job_name));
                        display_success(&format!("Memory for {} deallocated successfully.\n", job_name));

                        memory_is_currently_allocated = false;
                        is_last_input_valid = true;
                        break;
                    }
                    'N' => {
                        display_info(&format!("Waiting for {} to deallocate...\n", job_name));

                        terminal_cursor_previous_line(2);

                        is_last_input_valid = false;
                        continue;
                    }
                    _ => {
                        display_error("Please enter either Y or N.\n");

                        terminal_cursor_previous_line(2);

                        is_last_input_valid = false;
                        continue;
                    }
                }
            }
        }
        display_divider(2);
        display_pause();
    }

    display_title("Summary\n");
    display_labelled("Number of Jobs", &format!("{}\n", number_of_jobs));
    display_labelled(
        "Successful Jobs",
        &format!("{}\n", number_of_jobs - number_of_failed_jobs),
    );
    display_labelled("Unsuccessful Jobs", &format!("{}\n", number_of_failed_jobs));
    display_subtitle("Failed Jobs: \n");

    for job_index in 0..number_of_jobs {
        if !job_successfully_allocated[job_index] {
            job_failed_allocations_count += 1;
            display(&format!(
                "[{}] {}\n",
                job_failed_allocations_count, job_names[job_index]
            ));
        }
    }

    display("-------------------------------------------------\n");

    application_close();
}

fn input_memory_capacity(is_last_input_valid: &mut bool) -> f64 {
    loop {
        terminal_clear_line(0);
        let memory_capacity_input: String = display_prompt("Enter", "memory capacity (in K)");

        match memory_capacity_input.trim().parse::<u64>() {
            Ok(memory_capacity_value) => {
                if memory_capacity_value == 0 {
                    display_warning("Please enter a valid positive integer value.\n");
                    display_error(&format!("Memory capacity cannot be zero.\n"));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }

                if !*is_last_input_valid {
                    terminal_clear_next_lines(2);
                    terminal_cursor_previous_line(1);
                }

                *is_last_input_valid = true;

                return memory_capacity_value as f64;
            }
            Err(error) => {
                display_warning("Please enter a valid positive integer value.\n");
                display_error(&format!("Failed to parse number: {}.\n", error));

                terminal_cursor_previous_line(3);

                *is_last_input_valid = false;
                continue;
            }
        }
    }
}

fn input_memory_os(memory_capacity: f64, is_last_input_valid: &mut bool) -> f64 {
    loop {
        terminal_clear_line(0);
        let memory_os_input: String = display_prompt("Enter", "memory size of OS (in K)");

        match memory_os_input.trim().parse::<f64>() {
            Ok(memory_os_value) => {
                if memory_os_value <= 0.0 {
                    display_warning("Please enter a valid positive numeric value.\n");
                    display_error(&format!(
                        "Memory size of OS cannot be less than or equal to zero.\n"
                    ));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }
                if memory_os_value > memory_capacity {
                    display_warning("Please enter a valid positive numeric value.\n");
                    display_error(&format!(
                        "Memory size of OS cannot be greater than the memory capacity.\n"
                    ));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }

                if !*is_last_input_valid {
                    terminal_clear_next_lines(2);
                    terminal_cursor_previous_line(1);
                }

                *is_last_input_valid = true;
                return memory_os_value;
            }
            Err(error) => {
                display_warning("Please enter a valid positive numeric value.\n");
                display_error(&format!("Failed to parse number: {}.\n", error));

                terminal_cursor_previous_line(3);

                *is_last_input_valid = false;
                continue;
            }
        }
    }
}

fn input_number_of_jobs(is_last_input_valid: &mut bool) -> usize {
    loop {
        terminal_clear_line(0);
        let number_of_jobs_input: String = display_prompt("Enter", "number of jobs");

        match number_of_jobs_input.trim().parse::<usize>() {
            Ok(number_of_jobs_value) => {
                if number_of_jobs_value == 0 {
                    display_warning("Please enter a valid positive integer value.\n");
                    display_error(&format!("Number of jobs cannot be zero.\n"));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }

                if !*is_last_input_valid {
                    terminal_clear_next_lines(2);
                    terminal_cursor_previous_line(1);
                }

                *is_last_input_valid = true;
                return number_of_jobs_value;
            }
            Err(error) => {
                display_warning("Please enter a valid positive integer value.\n");
                display_error(&format!("Failed to parse number: {}.\n", error));

                terminal_cursor_previous_line(3);

                *is_last_input_valid = false;
                continue;
            }
        }
    }
}

fn input_jobs(
    number_of_jobs: usize,
    job_names: &mut Vec<String>,
    job_values: &mut Vec<f64>,
    job_successfully_allocated: &mut Vec<bool>,
    is_last_input_valid: &mut bool,
) {
    for i in 0..number_of_jobs {
        loop {
            terminal_clear_line(0);
            let job_input: String =
                display_prompt("Enter", &format!("memory size of Job {} (in K)", i + 1));

            match job_input.trim().parse::<f64>() {
                Ok(job_value) => {
                    if job_value <= 0.0 {
                        display_warning("Please enter a valid positive numeric value.\n");
                        display_error(&format!(
                            "Memory size of job cannot less than or equal to zero.\n"
                        ));

                        terminal_cursor_previous_line(3);

                        *is_last_input_valid = false;
                        continue;
                    }
                    job_names.push(format!("Job {}", i + 1));
                    job_values.push(job_value);
                    job_successfully_allocated.push(false);

                    if !*is_last_input_valid {
                        terminal_clear_next_lines(2);
                        terminal_cursor_previous_line(1);
                    }

                    *is_last_input_valid = true;
                    break;
                }
                Err(error) => {
                    display_warning("Please enter a valid positive numeric value.\n");
                    display_error(&format!("Failed to parse number: {}.\n", error));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }
            }
        }
    }
}

fn display_job_table(job_names: &Vec<String>, job_values: &Vec<f64>) {
    let job_values_string: Vec<String> = job_values.iter().map(|x| x.to_string()).collect();
    display_table(
        vec!["Job Name".to_string(), "Job Size".to_string()],
        vec![&job_names, &job_values_string],
    );
}
