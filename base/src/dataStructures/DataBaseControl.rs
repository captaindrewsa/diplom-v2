use mongodb::{
    bson::{doc, Document},
    error::Error,
    sync::Client,
    sync::Database,
};

use super::DatabaseControlInterface::{
    DatabaseClientInterface, DatabaseErrors, DatabaseGlobalInterface,
};
/*
Модуль, отвечающий за взаимодействие с базой данных.
Возвращает обхект, через интерфейс которого можно взаимодействовать с бд - запросы на получение и изменение.
*/

pub struct DatabaseClient {
    //Возвращаемый объект, который дает доступ через трейт DatabaseClientInterface
    db: Database,
}

impl DatabaseClientInterface for DatabaseClient {
    /*
    Имплементация трейта для взаимодействия в бд
    */

    fn find_enzyme_by_entry(&mut self, entry: &str) -> Result<Document, DatabaseErrors> {
        let collection = self.db.collection::<Document>("Enzymes");

        let cursor = collection.find(doc! {"entry":entry}, None).unwrap();
        for elem in cursor {
            let otp = elem.unwrap();
            return Ok(otp);
        }

        Err(DatabaseErrors::SomethingWithDatabaseGlobal)
    }

    fn find_compound_by_entry(&mut self, entry: &str) -> Result<Document, DatabaseErrors> {
        let collection = self.db.collection::<Document>("Compounds");

        let cursor = collection.find(doc! {"entry": entry}, None).unwrap();
        for elem in cursor {
            let otp = elem.unwrap();
            return Ok(otp);
        }

        Err(DatabaseErrors::SomethingWithDatabaseGlobal)
    }
}

pub struct DatabaseGlobal {
    /*
    Организующая структура для глобального взаимодейтсвия с бд -
    1. Подключение к базе данных
    2. Возвращение объекта для взаимодейтсвия с бд
    */
}

impl DatabaseGlobalInterface for DatabaseGlobal {
    fn check_and_get_of_connection() -> Result<DatabaseClient, DatabaseErrors> {
        let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
        let db = client.database("AnnetDB");

        Ok(DatabaseClient { db: db })
        // Ok(db_control_enums::ConnectionIsOk)
    }

    fn test_function() {}
}

impl DatabaseGlobal {}
