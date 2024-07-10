use activity::*;
use std::collections::VecDeque;

// *********************************************************
// Main application
// *********************************************************
fn main() {
    terminal_clear_screen(0);
  
    let mut is_last_input_valid: bool;
    is_last_input_valid = false;

    let mut system: System;
    system = System::new(&mut is_last_input_valid);

    terminal_clear_screen(0);
    display_header();

    system.display_memory_structure();
  
    display_footer();
    display_pause();
    terminal_clear_screen(0);

    system.handle_jobs();

    
    for snapshot in &system.snapshots {
        terminal_clear_screen(0);
        display_header();
        
        snapshot.display_snapshot();

        display_footer();
        display_pause();
        terminal_clear_screen(0);
    }

    terminal_clear_screen(0);
    display_header();

    display_title("Summary\n");
    system.display_snapshots_summary();
  
    display_divider(1);
  
    display_title("Conclusion\n");
    system.display_conclusion();   

    display_footer();
    display_pause();
    terminal_clear_screen(0);

    terminal_clear_screen(0);
}

// *********************************************************
// Subroutines
// *********************************************************
fn get_memory_structure(is_last_input_valid: &mut bool) -> Memory {
    let capacity: f64; 
    let os_size: f64;
    let mut memory: Memory;
    let mut os: Job;

    terminal_clear_screen(0);
    display_header();
  
    capacity = input_memory_capacity(is_last_input_valid);
    os_size = input_os_memory_size(is_last_input_valid, capacity);

    display_footer();
    display_pause();
    terminal_clear_screen(0);

    memory = Memory::new(capacity);
    os = Job::new(0, os_size, usize::MAX);
  
    memory.create_partition(os_size);
    memory.partitions[0].name = "OS Partition".to_string();
    os.name = "OS".to_string();
    memory.partitions[0].load(os, 0);
  
    return memory;
}

fn input_memory_capacity(is_last_input_valid: &mut bool) -> f64 {
    loop {
        let memory_capacity_input: String; 
        memory_capacity_input = display_prompt("Enter", "memory capacity (in M)");

        match memory_capacity_input.trim().parse::<isize>() {
            Ok(memory_capacity) => {
                if memory_capacity <= 0 {
                    display_info("Please enter a valid positive integer value.\n");
                    display_error("Memory capacity cannot be less than or equal to zero.\n");

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }

                if !*is_last_input_valid {
                    terminal_clear_next_lines(2);
                    terminal_cursor_previous_line(1);
                }

                *is_last_input_valid = true;

                return memory_capacity as f64;
            },
            Err(error) => {
                display_info("Please enter a valid positive integer value.\n");
                display_error(&format!("Failed to parse number: {}.\n", error));

                terminal_cursor_previous_line(3);

                *is_last_input_valid = false;
                continue;
            }
        }
    }
}

fn input_os_memory_size(is_last_input_valid: &mut bool, memory_capacity: f64) -> f64 {
    loop {
        let os_memory_size_input: String;
        os_memory_size_input = display_prompt("Enter", "memory size of OS (in M)");
  
        match os_memory_size_input.trim().parse::<f64>() {
            Ok(os_memory_size) => {
                if os_memory_size <= 0.0 {
                    display_info("Please enter a valid positive numeric value.\n");
                    display_error("Memory size of OS cannot be less than or equal to zero.\n");
  
                    terminal_cursor_previous_line(3);
  
                    *is_last_input_valid = false;
                    continue;
                }

                if os_memory_size > memory_capacity {
                    display_info("Please enter a valid positive numeric value.\n");
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
                return os_memory_size;
            },
            Err(error) => {
                display_info("Please enter a valid positive numeric value.\n");
                display_error(&format!("Failed to parse number: {}.\n", error));
  
                terminal_cursor_previous_line(3);
  
                *is_last_input_valid = false;
                continue;
            }
        }
    }
}

fn input_jobs(is_last_input_valid: &mut bool) -> Vec<Job> {
    let jobs_count: usize;
    let mut jobs: Vec<Job>;
    
    jobs = Vec::new();

    terminal_clear_screen(0);
    display_header();

    loop {
        let jobs_count_input: String;
        jobs_count_input = display_prompt("Enter", "number of jobs");

        match jobs_count_input.trim().parse::<isize>() {
            Ok(jobs_count_value) => {
                if jobs_count_value <= 0 {
                    display_info("Please enter a valid positive integer value.\n");
                    display_error(&format!(
                        "Number of jobs cannot be less than or equal to zero.\n"
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
                jobs_count = jobs_count_value as usize;
                break;
            },
            Err(error) => {
                display_info("Please enter a valid positive integer value.\n");
                display_error(&format!("Failed to parse number: {}.\n", error));

                terminal_cursor_previous_line(3);

                *is_last_input_valid = false;
                continue;
            }
        }
    }

    for i in 0..jobs_count {
        let job_size: f64;
        let job_turnaround_time: usize;

        loop {
            let job_size_input: String;
            job_size_input = display_prompt("Enter", &format!("memory size of {} (in M)", text_apply_style(&format!("Job {}",  i + 1), "fg-yellow")));

            match job_size_input.trim().parse::<f64>() {
                Ok(job_size_value) => {
                    if job_size_value <= 0.0 {
                        display_info("Please enter a valid positive numeric value.\n");
                        display_error("Memory size of job cannot be less than or equal to zero.\n");

                        terminal_cursor_previous_line(3);

                        *is_last_input_valid = false;
                        continue;
                    }

                    if !*is_last_input_valid {
                        terminal_clear_next_lines(2);
                        terminal_cursor_previous_line(1);
                    }

                    *is_last_input_valid = true;
                    job_size = job_size_value;
                    break;
                },
                Err(error) => {
                    display_info("Please enter a valid positive numeric value.\n");
                    display_error(&format!("Failed to parse number: {}.\n", error));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }
            }
        }

        loop {
            let job_turnaround_time_input: String;
            job_turnaround_time_input = display_prompt("Enter", &format!("turnaround time of {} (in M)", text_apply_style(&format!("Job {}",  i + 1), "fg-yellow")));

            match job_turnaround_time_input.trim().parse::<usize>() {
                Ok(job_turnaround_time_value) => {
                    if job_turnaround_time_value == 0 {
                        display_info("Please enter a valid positive integer value.\n");
                        display_error(&format!("Turnaround time cannot be zero.\n"));

                        terminal_cursor_previous_line(3);

                        *is_last_input_valid = false;
                        continue;
                    }

                    if !*is_last_input_valid {
                        terminal_clear_next_lines(2);
                        terminal_cursor_previous_line(1);
                    }

                    *is_last_input_valid = true;
                    job_turnaround_time = job_turnaround_time_value;
                    break;
                },
                Err(error) => {
                    display_info("Please enter a valid positive integer value.\n");
                    display_error(&format!("Failed to parse number: {}.\n", error));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }
            }
        }

        jobs.push(Job::new(i + 1, job_size, job_turnaround_time));
    }

    display_footer();
    display_pause();
    terminal_clear_screen(0);
  
    return jobs;
}

// *********************************************************
// Structure definitions and implementations
// *********************************************************
struct System {
    memory: Memory,
    jobs: Vec<Job>,
    main_queue: VecDeque<Job>,
    waiting_queue: VecDeque<Job>,
    snapshots: Vec<Snapshot>,
}

impl System {
    fn new(is_last_input_valid: &mut bool) -> Self {
        return Self {
            memory: get_memory_structure(is_last_input_valid),
            jobs: input_jobs(is_last_input_valid),
            main_queue: VecDeque::new(),
            waiting_queue: VecDeque::new(),
            snapshots: Vec::new(),
        }
    }
    
    fn handle_jobs(self: &mut Self) {
        let mut jobs_count: usize;
        jobs_count = 0;
      
        while !self.jobs.is_empty() {
            self.main_queue.push_front(self.jobs.pop().unwrap());
        }
      
        let mut snapshot: Snapshot = Snapshot::new(self.memory.set_counter);
        snapshot.log.push(Log::Allocating);
      
        while !self.main_queue.is_empty() || jobs_count != 0 {
            if !self.main_queue.is_empty() {
                let current_job: Job;
                let current_job_name: String;
                current_job = self.main_queue.pop_front().unwrap();
                current_job_name = current_job.name.clone();

                match self.memory.allocate(current_job) {
                    Ok(()) => {
                        snapshot.log.push(Log::Loaded(current_job_name));
                        jobs_count += 1;
                    },
                    Err(returned_job) => {
                        match returned_job.status {
                            JobStatus::Waiting => {
                                snapshot.log.push(Log::Waiting(current_job_name));
                                self.waiting_queue.push_back(returned_job);
                            },
                            JobStatus::Initial => {
                                snapshot.log.push(Log::Initial(current_job_name));
                                self.jobs.push(returned_job);
                            },
                            _ => {},
                        }
                    },
                }
            }
            
            if self.main_queue.is_empty() {
                self.memory.finalize_partitions();
                snapshot.log.push(Log::Deallocating);
                let mut set: Set = self.memory.take_set();
              
                match self.memory.deallocate() {
                    Ok(mut unloaded_jobs) => {
                        while !unloaded_jobs.is_empty() {
                            let unloaded_job: Job;
                            unloaded_job = unloaded_jobs.pop().unwrap();
                            jobs_count -= 1;

                            for index in 0..set.jobs_name.len() {
                                if unloaded_job.name == set.jobs_name[index] {
                                    set.jobs_name[index] = format!("[{}]", unloaded_job.name);
                                }
                            }
                            
                            snapshot.log.push(Log::Unloaded(unloaded_job.name.clone()));
                            self.jobs.push(unloaded_job);
                        }
                    },
                    Err(()) => {},
                };
                
                snapshot.log.push(Log::Requeuing);
                while !self.waiting_queue.is_empty() {    
                    self.main_queue.push_front(self.waiting_queue.pop_back().unwrap());
                }
              
                snapshot.set = set;
                self.snapshots.push(snapshot);

                self.memory.set_counter += 1;
              
                snapshot = Snapshot::new(self.memory.set_counter);
                snapshot.log.push(Log::Allocating);
            }
        }
        snapshot.set = self.memory.take_set();
        self.snapshots.push(snapshot);
    }

    fn display_memory_structure(self: &Self) {
        display_title("Memory Structure\n");
        display_labelled("Memory Capacity", &format!("{} M\n", self.memory.capacity));
        display_labelled("OS Memory Size", &format!("{} M\n", self.memory.partitions[0].capacity));

        let mut heading: Vec<String>;
        let mut body: Vec<Vec<String>>;
        let mut jobs_name: Vec<String>;
        let mut jobs_size: Vec<String>;
        let mut jobs_turnaround_time: Vec<String>;

        heading = Vec::new();
        body = Vec::new();
        jobs_name = Vec::new();
        jobs_size = Vec::new();
        jobs_turnaround_time = Vec::new();

        heading.push("Name".to_string());
        heading.push("Size".to_string()); 
        heading.push("Turnaround Time".to_string());
      
        for job in &self.jobs {
            jobs_name.push(job.name.clone());
            jobs_size.push(format!("{} M", job.size));
            jobs_turnaround_time.push(job.turnaround_time.to_string());
        }

        body.push(jobs_name);
        body.push(jobs_size);
        body.push(jobs_turnaround_time);

        display_divider(1);
      
        display_title("Jobs List\n");
        display_table(&heading, &body);
    }

    fn find_longest_set(self: &Self) -> (usize, usize) {
        let mut longest_set_index: usize;
        let mut longest_set_length: usize;
        longest_set_index = 0;
        longest_set_length = 0;

        for (index, snapshot) in self.snapshots.iter().enumerate() {
            if snapshot.set.partitions_name.len() > longest_set_length {
                longest_set_length = index;
                longest_set_length = snapshot.set.partitions_name.len();
            }
        }
        
        return (longest_set_index, longest_set_length);
    }

    fn display_snapshots_summary(self: &Self) {
        let mut heading: Vec<String>;
        let mut body: Vec<Vec<String>>;
        
        let mut partitions_name: Vec<String>;
        let mut partitions_capacity: Vec<String>;
        let mut jobs_label: Vec<Vec<String>>;

        heading = Vec::new();
        body = Vec::new();
      
        partitions_name = Vec::new();
        partitions_capacity = Vec::new();
        jobs_label = Vec::new();
        
        let longest_set_index: usize; 
        let longest_set_length: usize;

        (longest_set_index, longest_set_length) = self.find_longest_set();

        for index in 0..longest_set_length {
            partitions_name.push(self.snapshots[longest_set_index].set.partitions_name[index].clone());
            partitions_capacity.push(self.snapshots[longest_set_index].set.partitions_capacity[index].clone());
        }

        for snapshot in &self.snapshots {
            let mut set;
            set = Vec::new();

            for index in 0..longest_set_length {
                if index > snapshot.set.partitions_name.len() {
                   set.push("FREE".to_string());       
                }
                else {
                    if snapshot.set.jobs_name[index] == "FREE" {
                        set.push(format!("{}", snapshot.set.jobs_name[index]));
                    }
                    else {
                        set.push(format!("{}: {}", snapshot.set.jobs_name[index], snapshot.set.jobs_size[index]));
                    }
                    
                }
            }

            jobs_label.push(set);
        }

        heading.push("Name".to_string());
        heading.push("Capacity".to_string());
        for index in 0..self.snapshots.len() {
            heading.push(format!("Set {}", index + 1));
        }

        body.push(partitions_name);
        body.push(partitions_capacity);
        for index in 0..jobs_label.len() {
            body.push(jobs_label[index].clone());
        }

        display_table(&heading, &body);
    }

    fn display_conclusion(self: &mut Self) {
        let mut loaded_jobs: Vec<&Job>;
        let mut initial_jobs: Vec<&Job>;

        loaded_jobs = Vec::new();
        initial_jobs = Vec::new();

        self.jobs.sort_by(|a, b| a.number.cmp(&b.number));
      
        for job in &self.jobs {
            match job.status {
                JobStatus::Unloaded => {
                    loaded_jobs.push(job);
                },
                JobStatus::Initial => {
                    initial_jobs.push(job);
                },
                _ => {},
            }
        }

        display(&format!("The number of partitions created is {}.\n", self.memory.partitions.len() - 1));
        display(&format!(
            "The number of sets is {}.\n",
            self.memory.set_counter - 1
        ));
        if loaded_jobs.len() != self.jobs.len() {
            display("Not all jobs were successfully loaded.\n");
            display_subtitle("Successful Jobs\n");
            for (index, job) in loaded_jobs.iter().enumerate() {
                display(&format!("\t{}) {} ({} M)\n", index + 1, job.name, job.size));
            }
            display_subtitle("Failed Jobs\n");
            for (index, job) in initial_jobs.iter().enumerate() {
                display(&format!("\t{}) {} ({} M)\n", index + 1, job.name, job.size));
            }
        } else {
            display("All jobs were successfully loaded.\n");
            display_subtitle("Successful Jobs");
            for (index, job) in loaded_jobs.iter().enumerate() {
                display(&format!("\t{}) {} ({} M)\n", index + 1, job.name, job.size));
            }
        }
    }
}

struct Snapshot {
    number: usize,
    log: Vec<Log>,
    set: Set,
}

impl Snapshot {
    fn new(number: usize) -> Self {
        return Self {
            number: number,
            log: Vec::new(),
            set: Set::new(),
        };
    }

    fn display_snapshot(self: &Self) {
        display_title(&format!("Set {}\n", self.number));

        for entry in &self.log {
            match entry {
                Log::Loaded(job_name) => {
                    display_success(&format!("Memory for {} is allocated successfully.\n", job_name));
                },
                Log::Waiting(job_name) => {
                    display_info(&format!("{} is queued for loading.\n", job_name));
                },
                Log::Unloaded(job_name) => {
                    display_success(&format!("Memory for {} is deallocated succesfully.\n", job_name));
                },
                Log::Initial(job_name) => {
                    display_error(&format!("{} is not loaded succesfully.\n", job_name));
                },
                Log::Allocating => {
                    display_log("Allocating memory for jobs...\n");  
                },
                Log::Requeuing => {
                    display_log("Requeuing waiting jobs...\n");  
                },
                Log::Deallocating => {
                    display_log("Deallocating memory for jobs...\n");  
                },
            }
        }

        let mut heading: Vec<String>;
        let mut body: Vec<Vec<String>>;
        heading = Vec::new();
        body = Vec::new();

        heading.push("Name".to_string());
        heading.push("Capacity".to_string());
        heading.push(format!("Set {}", self.number));
        body.push(Vec::new());
        body.push(Vec::new());
        body.push(Vec::new());

        for index in 0..self.set.partitions_name.len() {
            body[0].push(self.set.partitions_name[index].clone());
            body[1].push(self.set.partitions_capacity[index].clone());
            if self.set.jobs_name[index] == "FREE" {
                body[2].push(format!("{}", self.set.jobs_name[index]));
            }
            else {
                body[2].push(format!("{}: {}", self.set.jobs_name[index], self.set.jobs_size[index]));
            }

        }

        display_table(&heading, &body);
    }
}

struct Set {
    partitions_name: Vec<String>,
    partitions_capacity: Vec<String>,
    jobs_name: Vec<String>,
    jobs_size: Vec<String>,
}

impl Set {
    fn new() -> Self {
        return Self {
            partitions_name: Vec::new(),
            partitions_capacity: Vec::new(),
            jobs_name: Vec::new(),
            jobs_size: Vec::new(),
        };
    }
}

struct Memory {
    capacity: f64,
    free_capacity: f64,
    partitions: Vec<Partition>,
    set_counter: usize,
}

impl Memory {
    fn new(capacity: f64) -> Self {
        return Self {
            capacity: capacity,
            free_capacity: capacity,
            partitions: Vec::new(),
            set_counter: 1,
        };
    }

    fn create_partition(self: &mut Self, partition_size: f64) {
        if partition_size <= self.free_capacity {
            self.partitions.push(Partition::new(self.partitions.len(), partition_size));
            self.free_capacity -= partition_size;
        }
    }

    fn allocate(self: &mut Self, mut job: Job) -> Result<(), Job> {
        if !self.has_complete_partitions() {
            self.create_partition(job.size);
        }
      
        for partition in &mut self.partitions {
            match partition.load(job, self.set_counter) {
                Ok(()) => {},
                Err(returned_job) => {
                    job = returned_job;
                    continue;
                },
            };
            
            return Ok(());
        }

        return Err(job);
    }

    fn deallocate(self: &mut Self) -> Result<Vec<Job>, ()> {
        let mut unloaded_jobs: Vec<Job>;
        unloaded_jobs = Vec::new();
      
        for partition in self.partitions.iter_mut().rev() {
            if let Some(unloaded_job) = partition.job.take() {
                if unloaded_job.set_number.unwrap() + unloaded_job.turnaround_time - 1 == self.set_counter {
                    partition.job = Some(unloaded_job);
                    unloaded_jobs.push(partition.unload().unwrap());
                }
                else {
                    partition.job = Some(unloaded_job);
                }
            }
        }

        if !unloaded_jobs.is_empty() {
            return Ok(unloaded_jobs);
        }

        return Err(());
    }

    fn finalize_partitions(self: &mut Self) {
        if !self.has_complete_partitions() {
            self.partitions.push(Partition::new(self.partitions.len(), self.free_capacity));
            self.free_capacity -= self.free_capacity;
        }
        
    }

    fn has_complete_partitions(self: &Self) -> bool {
        return self.free_capacity == 0.0;
    }

    fn take_set(self: &Self) -> Set {
        let mut set: Set;
        set = Set::new();

        for partition in &self.partitions {
            let loaded_job: Option<&Job> = partition.job.as_ref(); 
            let loaded_job_name: String;
            let loaded_job_size: f64;
            match loaded_job {
                Some(job) => {
                    loaded_job_name = job.name.clone();
                    loaded_job_size = job.size;
                },
                None => {
                    loaded_job_name = "FREE".to_string();
                    loaded_job_size = 0.0;
                },
            }

            set.partitions_name.push(partition.name.clone());
            set.partitions_capacity.push(format!("{} M", partition.capacity));
            set.jobs_name.push(loaded_job_name);
            set.jobs_size.push(format!("{} M", loaded_job_size));
        }
        
        return set;
    }
}

struct Partition {
    number: usize,
    name: String,
    capacity: f64,
    job: Option<Job>,
}

impl Partition {
    fn new(number: usize, capacity: f64) -> Self {
        return Self {
            number: number,
            name: format!("Partition {}", number),
            capacity: capacity,
            job: None,
        };
    }

    fn load(self: &mut Self, mut job: Job, set_counter: usize) -> Result<(), Job> {
        if !self.can_contain(&job) {
            return Err(job)
        }
        else {
            job.status = JobStatus::Waiting;
        }
      
        if !self.is_empty() {
            return Err(job);
        }

        job.status = JobStatus::Loaded;
        job.set_number = Some(set_counter);
        self.job = Some(job);

        return Ok(());
    }

    fn unload(self: &mut Self) -> Result<Job, ()> {
        if let Some(mut job) = self.job.take() {
            job.status = JobStatus::Unloaded;
            return Ok(job);
        }

        return Err(());
    }

    fn is_empty(self: &Self) -> bool {
        return self.job.is_none();
    }

    fn can_contain(self: &Self, job: &Job) -> bool {
        return self.capacity >= job.size;
    }
}

struct Job {
    number: usize,
    name: String,
    size: f64,
    turnaround_time: usize,
    status: JobStatus,
    set_number: Option<usize>,
}

impl Job {
    fn new(number: usize, size: f64, turnaround_time: usize) -> Self {
        return Self {
            number: number,
            name: format!("Job {}", number),
            size: size,
            turnaround_time: turnaround_time,
            status: JobStatus::Initial,
            set_number: None,
        };
    }
}

enum JobStatus {
    Loaded,
    Waiting,
    Unloaded,
    Initial,
}

enum Log {
    Loaded(String),
    Waiting(String),
    Unloaded(String),
    Initial(String),
    Allocating,
    Requeuing,
    Deallocating,
}