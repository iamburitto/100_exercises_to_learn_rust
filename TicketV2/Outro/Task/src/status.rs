// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, thiserror::Error)]
#[error("`{invalid_status}` is not a valid status. Use one of: ToDo, InProgress, Done")]
pub struct ParseStatusError {
    invalid_status: String,
}

impl TryFrom<String> for Status {
    type Error = ParseStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)
    }
}

impl TryFrom<&str> for Status {
    type Error = ParseStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
        // value.to_owned().try_into()
    }
}

fn validate(status: &str) -> Result<Status, ParseStatusError> {
    match status.to_lowercase().as_str() {
        "todo" => Ok(Status::ToDo),
        "inprogress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        _ => Err(ParseStatusError {
            invalid_status: status.to_owned(),
        }),
    }
}


