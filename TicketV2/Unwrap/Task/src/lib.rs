// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".

// A quick helper to print string addresses
fn print_string_memory(label: &str, s: &String) {
    println!(
        "[Memory] {:<40} Stack Pointer: {:p} | Heap Text Location: {:p}",
        label,
        &s,
        s.as_ptr()
    );
}

// Long version:

// pub fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
//     let new_ticket = match Ticket::new(title.clone(), description, status.clone()) {
//         Ok(ticket) => ticket,
//         Err(err) => {
//             let fallback_ticket = Ticket {
//                 title,
//                 description: String::from("Description not provided"),
//                 status,
//             };
//             match err.as_str() {
//                 "Title cannot be empty" |
//                 "Title cannot be longer than 50 bytes" => {
//                     panic!("{err}?")
//                 },
//                 _ => fallback_ticket
//             }
//         }
//     };
//     new_ticket
// }

// Debug version with printing of addresses:

// pub fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
//
//     println!("\n--- Function Started ---");
//     print_string_memory("easy_ticket() -> initial title", &title);
//
//     // 1. Clone the title. We look at the clone's address right before sending it.
//     let title_clone = title.clone();
//     print_string_memory("easy_ticket() -> title_clone", &title_clone);
//
//     match Ticket::new(title_clone, description, status.clone()) {
//         Ok(ticket) => {
//             println!("\n🎉 Success on first try!");
//             print_string_memory("Returned Ticket -> title", &ticket.title);
//             ticket
//         },
//         Err(error) => {
//             println!("\n❌ First try failed! Error: {}", error);
//             println!("Starting fresh fallback with original parameters...");
//
//             // 2. We use the original title here. Let's see where it goes!
//             print_string_memory("Fallback execution -> original title", &title);
//
//             if error.contains("Description") {
//                 let fallback = Ticket::new(title, "Description not provided".to_string(), status).unwrap();
//                 print_string_memory("Returned Fallback Ticket -> title", &fallback.title);
//                 fallback
//             } else {
//                 panic!("{error}");
//             }
//         }
//     }
// }

pub fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description, status.clone()) {
        Ok(ticket) => ticket,
        Err(error) => {
            if error.contains("Description") {
                Ticket::new(title, "Description not provided".to_string(), status).unwrap()
            } else {
                panic!("{error}");
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".to_string());
        }
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".to_string());
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}

// Custom internal test block to bypass JetBrains output trapping
#[cfg(test)]
mod memory_experiments {
    use super::*;

    #[test]
    fn inspect_my_memory_pointers() {
        let test_title = String::from("Fix the engine dashboard layout");
        let empty_desc = String::from(""); // Triggers validation failure

        // Call the code to execute our tracing println! macros
        let _result = easy_ticket(test_title, empty_desc, Status::ToDo);

        // Force a panic at the absolute end of the test.
        // This forces the IDE test runner to flush and display all print buffers!
        panic!("\n=== TEST FORCE-QUIT: SCROLL UP IN YOUR CONSOLE TO SEE THE POINTER TABLE ===");
    }
}
