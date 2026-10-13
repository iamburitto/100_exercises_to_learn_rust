// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketDescription` type,
//   enforcing that the description is not empty and is not longer than 500 bytes.
//   Implement the traits required to make the tests pass too.

#[derive(Debug, thiserror::Error)]
pub enum ParseDescriptionError {
    #[error("The description cannot be empty")]
    DescriptionEmpty,
    #[error("The description cannot be longer than 500 bytes")]
    DescriptionLong,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDescription(String);

impl TicketDescription {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for TicketDescription {
    type Error = ParseDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ParseDescriptionError::DescriptionEmpty)
        }
        else if value.len() > 500 {
            Err(ParseDescriptionError::DescriptionLong)
        } else {
            Ok(TicketDescription(value))
        }
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = ParseDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}



