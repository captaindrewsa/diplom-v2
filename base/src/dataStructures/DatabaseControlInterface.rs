use mongodb::bson::Document;
use thiserror::Error;

use super::DataBaseControl::DatabaseClient;

/*
Модуль, отвечающий за трейты связанные с базой данных
*/

pub trait DatabaseGlobalInterface {
    /*
    Трейт для глобального взаимодействия с бд - подключение к ней и проверки
    */
    fn check_and_get_of_connection() -> Result<DatabaseClient, DatabaseErrors>;
    fn test_function();
}

pub trait DatabaseClientInterface {
    /*
    Трейт для взаимодейтсвия с объектом подключения.
    Через него осуществляются все запросы для получения, передачу и изменения данных
    */
    fn find_enzyme_by_entry(&mut self, entry: &str) -> Result<Document, DatabaseErrors>;
    fn find_compound_by_entry(&mut self, entry: &str) -> Result<Document, DatabaseErrors>;
}

#[derive(Error, Debug)]
pub enum DatabaseErrors {
    #[error("Something went wrong")]
    SomethingWithDatabaseGlobal,
}
