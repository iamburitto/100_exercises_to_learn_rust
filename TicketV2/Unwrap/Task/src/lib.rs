// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".

pub fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    let new_ticket = match Ticket::new(title.clone(), description, status.clone()) {
        Ok(ticket) => ticket,
        Err(err) => {
            let fallback_ticket = Ticket {
                title,
                description: String::from("Description not provided"),
                status,
            };
            match err.as_str() {
                "Title cannot be empty" |
                "Title cannot be longer than 50 bytes" => {
                    panic!("{err}?")
                },
                _ => fallback_ticket
            }
        }
    };
    new_ticket
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
