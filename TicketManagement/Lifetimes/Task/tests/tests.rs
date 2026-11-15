
#[cfg(test)]
mod tests {
    use task_lifetimes::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        // Calls TicketStore::iter(), which returns an iterator over &Ticket.
        // Does not require IntoIterator to be implemented for TicketStore.
        let ticket_refs_iter_fn = store.iter().collect::<Vec<&Ticket>>();

        // Calls IntoIterator for &TicketStore, allowing borrowed iteration
        // without explicitly calling .iter().
        let ticket_refs_into_iter_trait: Vec<&Ticket> = (&store).into_iter().collect();

        assert_eq!(ticket_refs_iter_fn, ticket_refs_into_iter_trait);

        // Calls IntoIterator for TicketStore, consuming the clone
        // and yielding owned Tickets.
        let tickets_cloned_and_owned_into_iter_trait: Vec<Ticket> =
            store.clone().into_iter().collect();

        assert_eq!(tickets_cloned_and_owned_into_iter_trait, *store.tickets());

        // Three kinds of for loops:
        //
        // for loops require IntoIterator, but types implementing Iterator
        // already have IntoIterator automatically implemented for them.

        // Requires IntoIterator for &TicketStore; borrows store.
        for ticket in &store {
            println!("for ticket in &store: {ticket:?}");
        }

        // Only requires TicketStore::iter() because it returns an Iterator.
        // Does not require IntoIterator for TicketStore or &TicketStore.
        for ticket in store.iter() {
            println!("for ticket in store.iter(): {ticket:?}");
        }

        // Requires IntoIterator for TicketStore; consumes only the clone.
        for ticket in store.clone().into_iter() {
            println!("for ticket in store.clone().into_iter(): {ticket:?}");
        }

        // Requires IntoIterator for TicketStore; consumes the original store.
        for ticket in store {
            println!("for ticket in store: {ticket:?}");
        }

        // Won't compile because store has been moved.
        // println!("{}", store.tickets().len());
    }
}
