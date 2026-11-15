// TODO: Rework the signature of `TicketStore::add_ticket` to use a generic type parameter rather
//  than `impl Trait` syntax.

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    // Using `Into<Ticket>` as the type parameter for `ticket` allows the method to accept any type
    // that can be infallibly converted into a `Ticket`.
    // This can make it nicer to use the method, as it removes the syntax noise of `.into()`
    // from the calling site. It can worsen the quality of the compiler error messages, though.

    pub fn add_ticket<T: Into<Ticket>>(&mut self, ticket: T) {
        (&mut self.tickets).push(ticket.into());
    }

    // When used in argument position, impl Trait is equivalent to a generic parameter with a trait bound:

    // pub fn add_ticket_3<T>(&mut self, ticket: T)
    //     where
    //         T: Into<Ticket>
    // {
    //     (&mut self.tickets).push(ticket.into());
    // }

}
