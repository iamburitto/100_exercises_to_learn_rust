// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    temperatures: [Option<i32> ; 7],
}

#[derive(Copy, Clone)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        WeekTemperatures { temperatures: [None ; 7]}
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        let index = weekday2index(&day);
        self.temperatures[index]
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        let index = weekday2index(&day);
        self.temperatures[index] = Some(temperature);
    }
}

/* TODO: Create weekday2index method which converts a Weekday enum variant into its corresponding zero-based index. */
fn weekday2index(weekday: &Weekday) -> usize {
    *weekday as usize
}