use std::str::FromStr;

use crate::storage::PersonStorage;

//* Определяем методы для пользовательского интерфейса.
pub trait UI {
    fn add_info(&self, data: &mut PersonStorage) -> String;
    fn show_info(&self, data: &PersonStorage) -> String;
    fn show_all_info(&self, data: &PersonStorage) -> String;
    fn delet_param(&self, data: &mut PersonStorage) -> String;
    fn find_param(&self, data: &PersonStorage) -> String;
}

//* Создаем enum параметров для функции find и delet.
#[derive(Debug, Clone)]
pub enum PersonParam {
    Name,
    Surname,
    MiddleName,
    DateOfBirth,
    Gender,
}

impl FromStr for PersonParam {
    type Err = ();
    fn from_str(input_param: &str) -> Result<Self, Self::Err> {
        match input_param {
            "Name" => Ok(PersonParam::Name),
            "Surname" => Ok(PersonParam::Surname),
            "Middle name" => Ok(PersonParam::MiddleName),
            "Date of birth" => Ok(PersonParam::DateOfBirth),
            "Gender" => Ok(PersonParam::Gender),
            _ => Err(()),
        }
    }
}
