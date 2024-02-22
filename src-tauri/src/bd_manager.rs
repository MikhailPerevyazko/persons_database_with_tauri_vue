use std::{collections::HashMap, error::Error};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::storage::{Person, PersonStorage};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerdePerson {
    id: i32,
    name: String,
    surname: String,
    middle_name: String,
    date_of_birth: String,
    gender: bool,
}

impl SerdePerson {
    pub fn new(
        id: i32,
        name: String,
        surname: String,
        middle_name: String,
        date_of_birth: String,
        gender: bool,
    ) -> Self {
        Self {
            id,
            name,
            surname,
            middle_name,
            date_of_birth,
            gender,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerdePersons {
    persons: Vec<SerdePerson>,
}

impl SerdePersons {
    pub fn new(persons: Vec<SerdePerson>) -> Self {
        Self { persons }
    }
}
impl Default for SerdePersons {
    fn default() -> Self {
        Self {
            persons: Vec::new(),
        }
    }
}
impl TryInto<Person> for SerdePerson {
    type Error = Box<dyn Error>;
    fn try_into(self) -> Result<Person, Self::Error> {
        let person = Person::new(
            self.name.to_owned(),
            self.surname.to_owned(),
            self.middle_name.to_owned(),
            NaiveDate::parse_from_str(&self.date_of_birth, "%Y-%m-%d").unwrap(),
            self.gender.to_owned(),
        );
        Ok(person)
    }
}

impl Into<PersonStorage> for SerdePersons {
    fn into(self) -> PersonStorage {
        let result: HashMap<i32, Person> =
            self.persons.iter().fold(HashMap::new(), |mut res, item| {
                if let Ok(person) = item.to_owned().try_into() {
                    println!("add record");
                    let _ = res.insert(item.id, person);
                }
                res
            });
        PersonStorage::new(result)
    }
}
pub trait BDOperation {
    fn load(&self) -> Result<SerdePersons, Box<dyn Error>>;
    fn save(&self, persons: &SerdePersons) -> Result<(), Box<dyn Error>>;
}
