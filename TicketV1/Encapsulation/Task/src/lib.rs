pub mod ticket {
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }

        // TODO: Add three public methods to the `Ticket` struct:
        //  - `title` that returns the `title` field.
        //  - `description` that returns the `description` field.
        //  - `status` that returns the `status` field.

        // Here's how I did it at first:
        //
        // pub fn title(&self) -> String { self.title }
        // pub fn description(&self) -> String { self.description }
        // pub fn status(&self) -> String { self.status }

        // Those functions only borrow the object with &self
        //, but tries to give ownership of the String to the caller
        // Moving that String "out" would leave self.* empty/invalid,
        // even though the caller only lent you self,
        // So Rust doesn't let you.

        // Analogy: returning a read-only pointer into a C struct:
        // the struct keeps ownership of the memory

        pub fn title(self) -> String {
            self.title
        }

        pub fn description(self) -> String {
            self.description
        }

        pub fn status(self) -> String {
            self.status
        }
    }
}