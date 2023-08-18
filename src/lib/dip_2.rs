trait Database {
    fn fetch_data(&self) -> String;
}

struct MySQLDatabase;

impl Database for MySQLDatabase {
    fn fetch_data(&self) -> String {
        // Complex logic for fetching data from MySQL
        "Data from MySQL Database".to_string()
    }
}

struct OracleDatabase;

impl Database for OracleDatabase {
    fn fetch_data(&self) -> String {
        // Complex logic for fetching data from Oracle
        "Data from Oracle Database".to_string()
    }
}

struct DataService<T: Database> {
    database: T,
}

impl<T: Database> DataService<T> {
    fn new(database: T) -> Self {
        DataService { database }
    }

    fn fetch_data(&self) -> String {
        self.database.fetch_data()
    }
}
