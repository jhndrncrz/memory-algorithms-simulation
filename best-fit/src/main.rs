use activity::*;

// *********************************************************
// Main application
// *********************************************************
fn main() {
    let system_capacity: f64;
    let os_size: f64;

    let number_of_partitions: usize;
    let mut partitions_name: Vec<String> = vec![];
    let mut partitions_capacity: Vec<f64> = vec![];
    let mut partitions_occupier: Vec<String> = vec![];
    let mut partitions_occupied: Vec<f64> = vec![];
    let mut partitions_unoccupied: Vec<f64> = vec![];
    let mut partitions_is_currently_allocated: Vec<bool> = vec![];
    let mut partitions_to_deallocate: Vec<usize> = vec![];
    let mut number_of_allocated_partitions: usize = 0;

    let number_of_jobs: usize;
    let mut number_of_successful_jobs: usize = 0;
    let mut number_of_failed_jobs: usize = 0;

    let mut jobs_name: Vec<String> = vec![];
    let mut jobs_size: Vec<f64> = vec![];
    let mut jobs_successfully_allocated: Vec<bool> = vec![];
    let mut jobs_primary_queue: Vec<usize> = vec![];
    let mut jobs_waiting_queue: Vec<usize> = vec![];

    let mut allocation_set_history: Vec<Vec<String>> = vec![];
    let mut number_of_allocation_sets: usize = 0;

    let mut is_last_input_valid: bool = true;

    application_start();

    display_header();

    system_capacity = input_system_capacity(&mut is_last_input_valid);
    os_size = input_os_size(system_capacity, &mut is_last_input_valid);
    display_divider(1);

    number_of_partitions = input_number_of_partitions(&mut is_last_input_valid);
    input_partitions(
        system_capacity,
        os_size,
        number_of_partitions,
        &mut partitions_name,
        &mut partitions_capacity,
        &mut partitions_occupier,
        &mut partitions_occupied,
        &mut partitions_unoccupied,
        &mut partitions_is_currently_allocated,
        &mut is_last_input_valid,
    );
    display_divider(1);
    display_pause();
    terminal_clear_screen(0);
    display_header();

    number_of_jobs = input_number_of_jobs(&mut is_last_input_valid);
    input_jobs(
        number_of_jobs,
        &mut jobs_name,
        &mut jobs_size,
        &mut jobs_successfully_allocated,
        &mut is_last_input_valid,
    );
    display_divider(1);
    display_pause();
    terminal_clear_screen(0);
    display_header();

    display_title("Job List\n");
    display_job_table(&jobs_name, &jobs_size);
    display_divider(2);

    display_title("Memory Structure\n");
    display_memory_structure_table(&partitions_name, &partitions_capacity, os_size);
    display_divider(2);
    display_pause();
    terminal_clear_screen(0);
    display_header();

    display_title("Allocation Log\n");
    display_info("Allocating memory for OS...\n");
    display_success("Memory for OS successfully allocated.\n");
    display_allocations_bar(
        os_size,
        number_of_partitions,
        &partitions_name,
        &partitions_occupier,
        &vec![0.0; number_of_partitions],
        &partitions_capacity,
    );
    display_divider(2);
    display_pause();
    terminal_clear_screen(0);
    display_header();

    for job_index in 0..number_of_jobs {
        jobs_primary_queue.insert(0, job_index);
    }

    while !jobs_primary_queue.is_empty()
        || !jobs_waiting_queue.is_empty()
        || number_of_allocated_partitions != 0
    {
        if number_of_allocated_partitions == number_of_partitions
            || (!jobs_waiting_queue.is_empty() && jobs_primary_queue.is_empty())
            || (jobs_waiting_queue.is_empty() && jobs_primary_queue.is_empty())
        {
            let mut partitions_to_deallocate: Vec<usize> = vec![];

            for _ in 0..2 {
                let mut partitions_current_minimum_index: usize = usize::max_value();
                let mut partitions_current_minimum: f64 = f64::INFINITY;
                for partition_index in 0..number_of_partitions {
                    if partitions_is_currently_allocated[partition_index]
                        && partitions_occupied[partition_index] < partitions_current_minimum
                        && !partitions_to_deallocate.contains(&partition_index)
                    {
                        partitions_current_minimum_index = partition_index;
                        partitions_current_minimum = partitions_occupied[partition_index];
                    }
                }

                if partitions_current_minimum_index != usize::max_value() {
                    partitions_to_deallocate.push(partitions_current_minimum_index);
                }
            }
            
            let mut allocation_set: Vec<String> = vec!["".to_string(); number_of_partitions];
            for partition_index in 0..number_of_partitions {
                if partitions_occupied[partition_index] != 0.0 {
                    let allocation_part: String;
                    if partitions_to_deallocate.contains(&partition_index) && partitions_to_deallocate.iter().position(|&el| el == partition_index).unwrap() >= partitions_to_deallocate.len() - 2 {
                        allocation_part = format!(
                            "[{}: {} M]",
                            partitions_occupier[partition_index],
                            partitions_occupied[partition_index],
                        );
                    } else {
                        allocation_part = format!(
                            "{}: {} M",
                            partitions_occupier[partition_index],
                            partitions_occupied[partition_index],
                        );
                    }

                    allocation_set[partition_index] = allocation_part;
                }
            }
            allocation_set_history.push(allocation_set);
            number_of_allocation_sets += 1;

            display_info("Deallocating memory for jobs...\n");
            for _ in 0..2 {
                let partition_index: usize = partitions_to_deallocate.pop().unwrap();
                number_of_allocated_partitions -= 1;
                partitions_occupier[partition_index] = "".to_string();
                partitions_occupied[partition_index] = 0.0;
                partitions_unoccupied[partition_index] = partitions_capacity[partition_index];
                partitions_is_currently_allocated[partition_index] = false;

                display_success(&format!(
                    "Memory for {} successfully deallocated.\n",
                    partitions_name[partition_index]
                ));
            }

            if !jobs_waiting_queue.is_empty() {
                display_info("Requeuing waiting jobs...\n");
                while !jobs_waiting_queue.is_empty() {
                    let job_index: usize = jobs_waiting_queue.pop().unwrap();
                    display_success(&format!(
                        "{} successfully requeued.\n",
                        jobs_name[job_index]
                    ));
                    jobs_primary_queue.push(job_index);
                }
            }
            display_divider(1);

            display_title(&format!("Set #{}\n", number_of_allocation_sets));
            display_subtitle("Allocation Bar\n");
            display_allocations_bar(
                os_size,
                number_of_partitions,
                &partitions_name,
                &partitions_occupier,
                &partitions_occupied,
                &partitions_unoccupied,
            );
            display_subtitle("Allocation Table\n");
            display_allocations_table(
                &partitions_name,
                &partitions_capacity,
                &allocation_set_history,
                os_size,
            );
            display_divider(2);
            display_pause();
            terminal_clear_screen(0);
            display_header();
        }

        if !jobs_primary_queue.is_empty() {
            let job_index = jobs_primary_queue.pop().unwrap();
            let job_name = &jobs_name[job_index];
            let job_size = jobs_size[job_index];
            let mut job_fits_in_some_partition: bool = false;

            let mut partition_index_smallest_unoccupied: usize = 0;
            let mut partition_capacity_smallest_unoccupied: f64 = f64::INFINITY;
            let mut job_can_be_allocated_now: bool = false;

            display_info(&format!("Allocating memory for {}...\n", job_name));
            for partition_index in 0..number_of_partitions {
                if partitions_capacity[partition_index] < job_size {
                    continue;
                }

                job_fits_in_some_partition = true;

                if partitions_is_currently_allocated[partition_index] {
                    continue;
                }

                if partitions_capacity[partition_index] < partition_capacity_smallest_unoccupied{
                    partition_index_smallest_unoccupied = partition_index;
                    partition_capacity_smallest_unoccupied = partitions_capacity[partition_index];
                    job_can_be_allocated_now = true;
                }
            }

            if job_can_be_allocated_now {    
                let partition_name: &String = &partitions_name[partition_index_smallest_unoccupied];
                jobs_successfully_allocated[job_index] = true;
                partitions_is_currently_allocated[partition_index_smallest_unoccupied] = true;
                partitions_occupier[partition_index_smallest_unoccupied] = job_name.clone();
                partitions_occupied[partition_index_smallest_unoccupied] = job_size;
                partitions_unoccupied[partition_index_smallest_unoccupied] =
                    partitions_capacity[partition_index_smallest_unoccupied] - job_size;
                number_of_successful_jobs += 1;
                number_of_allocated_partitions += 1;

                display_success(&format!(
                    "Memory for {} successfully allocated at {}.\n",
                    job_name, partition_name
                ));
                display_divider(1);

                display_subtitle("Allocation Bar\n");
                display_allocations_bar(
                    os_size,
                    number_of_partitions,
                    &partitions_name,
                    &partitions_occupier,
                    &partitions_occupied,
                    &partitions_unoccupied,
                );
            }

            if !jobs_successfully_allocated[job_index] {
                if job_fits_in_some_partition {
                    display_info(&format!("{} is queued for allocation.\n", job_name));
                    jobs_waiting_queue.push(job_index);
                } else {
                    display_warning(&format!(
                        "Memory size of {} does not fit into available memory.\n",
                        job_name
                    ));
                    display_error(&format!(
                        "Memory for {} failed to be allocated.\n",
                        job_name
                    ));
                    number_of_failed_jobs += 1;
                }
            }

            display_divider(2);
            display_pause();
            terminal_clear_screen(0);
            display_header();
        }
    }

    display_title("Summary\n");

    let mut allocation_set: Vec<String> = vec![];
    for _ in 0..number_of_partitions {
        allocation_set.push("".to_string());
    }
    allocation_set_history.push(allocation_set);

    display_allocations_table(
        &partitions_name,
        &partitions_capacity,
        &allocation_set_history,
        os_size,
    );
    display_labelled("Number of Jobs", &format!("{}\n", number_of_jobs));
    display_labelled(
        "Successful Jobs",
        &format!("{}\n", number_of_successful_jobs),
    );
    display_labelled("Failed Jobs", &format!("{}\n", number_of_failed_jobs));
    display_divider(1);

    display_title("Failed Jobs List\n");
    for job_index in 0..number_of_jobs {
        if !jobs_successfully_allocated[job_index] {
            display(&format!(">> {}\n", jobs_name[job_index]));
        }
    }
    display_divider(1);

    display_title("Conclusion\n");
    if number_of_successful_jobs != number_of_jobs {
        display("Not all jobs were successfully executed.\n");
    } else {
        display("All jobs were successfully executed.\n")
    }
    display(&format!(
        "The number of sets of allocations is {}.\n",
        number_of_allocation_sets
    ));

    display("-------------------------------------------------\n");

    application_close();
}

// *********************************************************
// Helping functions for complex display and validated input
// *********************************************************
fn input_system_capacity(is_last_input_valid: &mut bool) -> f64 {
    loop {
        terminal_clear_line(0);
        let memory_capacity_input: String = display_prompt("Enter", "memory capacity (in M)");

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

fn input_os_size(system_capacity: f64, is_last_input_valid: &mut bool) -> f64 {
    loop {
        terminal_clear_line(0);
        let memory_os_input: String = display_prompt("Enter", "memory size of OS (in M)");

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
                if memory_os_value > system_capacity {
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

fn input_number_of_partitions(is_last_input_valid: &mut bool) -> usize {
    loop {
        terminal_clear_line(0);
        let number_of_partitions_input: String = display_prompt("Enter", "number of partitions");

        match number_of_partitions_input.trim().parse::<usize>() {
            Ok(number_of_partitions_value) => {
                if number_of_partitions_value == 0 {
                    display_warning("Please enter a valid positive integer value.\n");
                    display_error(&format!("Number of partitions cannot be zero.\n"));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }

                if !*is_last_input_valid {
                    terminal_clear_next_lines(2);
                    terminal_cursor_previous_line(1);
                }

                *is_last_input_valid = true;
                return number_of_partitions_value;
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

fn input_partitions(
    system_capacity: f64,
    os_size: f64,
    number_of_partitions: usize,
    partitions_name: &mut Vec<String>,
    partitions_capacity: &mut Vec<f64>,
    partitions_occupier: &mut Vec<String>,
    partitions_occupied: &mut Vec<f64>,
    partitions_unoccupied: &mut Vec<f64>,
    partitions_is_currently_allocated: &mut Vec<bool>,
    is_last_input_valid: &mut bool,
) {
    let mut available_capacity: f64 = system_capacity - os_size;
    for i in 0..number_of_partitions - 1 {
        loop {
            terminal_clear_line(0);
            display("\n");
            terminal_clear_line(0);
            display_labelled("Available Memory", &format!("{} M", available_capacity));
            terminal_cursor_previous_line(0);
            let partition_capacity_input: String = display_prompt(
                "Enter",
                &format!("memory capacity of Partition {} (in M)", i + 1),
            );

            match partition_capacity_input.trim().parse::<f64>() {
                Ok(partition_capacity_value) => {
                    if partition_capacity_value <= 0.0 {
                        display("\n");
                        display_warning("Please enter a valid positive numeric value.\n");
                        display_error(&format!(
                            "Memory capacity of partition cannot be less than or equal to zero.\n"
                        ));

                        terminal_cursor_previous_line(4);

                        *is_last_input_valid = false;
                        continue;
                    }

                    if partition_capacity_value >= available_capacity {
                        display("\n");
                        display_warning("Please enter a valid positive numeric value.\n");
                        display_error(&format!(
                            "There must be enough memory left for the remaining partitions.\n"
                        ));

                        terminal_cursor_previous_line(4);

                        *is_last_input_valid = false;
                        continue;
                    }

                    partitions_name.push(format!("Partition {}", i + 1));
                    partitions_capacity.push(partition_capacity_value);
                    partitions_occupier.push("".to_string());
                    partitions_occupied.push(0.0);
                    partitions_unoccupied.push(partition_capacity_value);
                    partitions_is_currently_allocated.push(false);

                    available_capacity -= partition_capacity_value;

                    if !*is_last_input_valid {
                        terminal_clear_next_lines(3);
                        terminal_cursor_previous_line(2);
                    }

                    *is_last_input_valid = true;
                    break;
                }
                Err(error) => {
                    display("\n");
                    display_warning("Please enter a valid positive numeric value.\n");
                    display_error(&format!("Failed to parse number: {}.\n", error));

                    terminal_cursor_previous_line(4);

                    *is_last_input_valid = false;
                    continue;
                }
            }
        }
    }

    display(&format!(
        "Remaining memory ({} M) will be allocated to Partition {}.\n",
        available_capacity, number_of_partitions
    ));
    partitions_name.push(format!("Partition {}", number_of_partitions));
    partitions_capacity.push(available_capacity);
    partitions_occupier.push("".to_string());
    partitions_occupied.push(0.0);
    partitions_unoccupied.push(available_capacity);
    partitions_is_currently_allocated.push(false);
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
    jobs_name: &mut Vec<String>,
    jobs_size: &mut Vec<f64>,
    jobs_successfully_allocated: &mut Vec<bool>,
    is_last_input_valid: &mut bool,
) {
    for i in 0..number_of_jobs {
        loop {
            terminal_clear_line(0);
            let job_size_input: String =
                display_prompt("Enter", &format!("memory size of Job {} (in M)", i + 1));

            match job_size_input.trim().parse::<f64>() {
                Ok(job_size_value) => {
                    if job_size_value <= 0.0 {
                        display_warning("Please enter a valid positive numeric value.\n");
                        display_error(&format!(
                            "Memory size of job cannot be less than or equal to zero.\n"
                        ));

                        terminal_cursor_previous_line(3);

                        *is_last_input_valid = false;
                        continue;
                    }
                    jobs_name.push(format!("Job {}", i + 1));
                    jobs_size.push(job_size_value);
                    jobs_successfully_allocated.push(false);

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

fn display_job_table(jobs_name: &Vec<String>, jobs_size: &Vec<f64>) {
    let jobs_size_string: Vec<String> = jobs_size.iter().map(|x| x.to_string()).collect();
    display_table(
        vec!["Job Name".to_string(), "Job Size".to_string()],
        vec![&jobs_name, &jobs_size_string],
    );
}

fn display_memory_structure_table(
    partitions_name: &Vec<String>,
    partitions_capacity: &Vec<f64>,
    os_size: f64,
) {
    let mut partitions_name_with_os = partitions_name.clone();
    let mut partitions_capacity_with_os = partitions_capacity.clone();

    partitions_name_with_os.insert(0, "OS Partition".to_string());
    partitions_capacity_with_os.insert(0, os_size);
    let partitions_capacity_with_os_string: Vec<String> = partitions_capacity_with_os
        .iter()
        .map(|x| x.to_string())
        .collect();
    display_table(
        vec![
            "Partition Name".to_string(),
            "Partition Capacity".to_string(),
        ],
        vec![
            &partitions_name_with_os,
            &partitions_capacity_with_os_string,
        ],
    );
}

pub fn display_allocations_bar(
    os_size: f64,
    number_of_partitions: usize,
    partitions_name: &Vec<String>,
    partitions_occupier: &Vec<String>,
    partitions_occupied: &Vec<f64>,
    partitions_unoccupied: &Vec<f64>,
) {
    use std::io;
    use std::io::prelude::*;

    let mut titles: Vec<String> = vec!["OS".to_string()];
    let mut values: Vec<f64> = vec![os_size];

    for partition_index in 0..number_of_partitions {
        if partitions_occupier[partition_index] == "".to_string() {
            titles.push(partitions_name[partition_index].clone());
        } else {
            titles.push(format!(
                "{} [{}]",
                partitions_name[partition_index], partitions_occupier[partition_index]
            ));
        }
        values.push(partitions_occupied[partition_index]);
        values.push(partitions_unoccupied[partition_index]);
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

fn display_allocations_table(
    partitions_name: &Vec<String>,
    partitions_capacity: &Vec<f64>,
    allocation_set_history: &Vec<Vec<String>>,
    os_size: f64,
) {
    let mut partitions_name_with_os: Vec<String> = partitions_name.clone();
    let mut partitions_capacity_with_os: Vec<f64> = partitions_capacity.clone();
    let mut allocation_set_history_with_os: Vec<Vec<String>> = allocation_set_history.clone();

    partitions_name_with_os.insert(0, "OS Partition".to_string());
    partitions_capacity_with_os.insert(0, os_size);
    let partitions_capacity_with_os_string: Vec<String> = partitions_capacity_with_os
        .iter()
        .map(|x| format!("{} M", x))
        .collect();

    let mut headers: Vec<String> = vec!["Name".to_string(), "Capacity".to_string()];
    let mut table: Vec<&Vec<String>> = vec![
        &partitions_name_with_os,
        &partitions_capacity_with_os_string,
    ];

    for set_index in 0..allocation_set_history.len() {
        allocation_set_history_with_os[set_index].insert(0, "OS".to_string());
    }

    for set_index in 0..allocation_set_history.len() {
        headers.push(format!("Set {}", set_index + 1));
        table.push(&allocation_set_history_with_os[set_index]);
    }

    display_table(headers, table);
}
