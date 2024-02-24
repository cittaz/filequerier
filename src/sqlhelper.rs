use crate::errorhandler::AppError;
use log::{debug, info};
use rusqlite::Connection;

pub struct SqlHelper<'a> {
    tablename: String,
    connection: &'a Connection,
    columns: &'a Vec<String>,
}

impl<'a> SqlHelper<'a> {
    pub fn new(
        tablename: &str,
        connection: &'a Connection,
        columns: &'a Vec<String>,
    ) -> SqlHelper<'a> {
        SqlHelper {
            tablename: tablename.to_string(),
            connection,
            columns,
        }
    }

    pub fn create_empty_table(&self) -> rusqlite::Result<(), AppError> {
        let mut create_columns = String::new();

        for header in self.columns {
            let mut staging_header = header.replace(" ", "_");
            staging_header.push(']');
            let mut final_header = "[".to_string();
            final_header.push_str(&staging_header);
            create_columns.push_str(&final_header);
            create_columns.push_str(" TEXT, ");
        }

        if create_columns.ends_with(", ") {
            create_columns.truncate(create_columns.len() - 2);
        }

        let create_table_query = format!(
            "CREATE TABLE IF NOT EXISTS {} ({create_columns})",
            &self.tablename
        );
        debug!("{}", create_table_query);

        self.connection.execute(&create_table_query, [])?;
        Ok(())
    }

    pub fn insert_data_into_sqlite(&self, data: &Vec<Vec<String>>) -> Result<(), AppError> {
        // Dynamically create the column list for the SQL statement
        let columns_vec: Vec<String> = self
            .columns
            .iter()
            .map(|column| {
                let mut staging_column = column.replace(" ", "_");
                staging_column.push(']');
                let mut final_column = "[".to_string();
                final_column.push_str(&staging_column);
                final_column
            })
            .collect();

        let columns_list = columns_vec.join(", ");

        // Dynamically create the placeholders list for the SQL statement
        let placeholders: Vec<String> = (1..=self.columns.len())
            .map(|i| format!("?{}", i))
            .collect();
        let placeholders_list = placeholders.join(", ");

        // Construct the full SQL INSERT statement
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            &self.tablename, columns_list, placeholders_list
        );

        // Proceed with data insertion
        for row in data.iter().skip(1) {
            // Ensure that the row has the correct number of columns
            if row.len() != self.columns.len() {
                return Err(rusqlite::Error::ExecuteReturnedResults).map_err(AppError::from)?;
                // Use a more appropriate error
            }
            let params: Vec<&dyn rusqlite::ToSql> =
                row.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
            // Execute the SQL statement for each row
            self.connection.execute(&sql, &params[..])?;
        }

        Ok(())
    }

    pub fn query_all_data(&self) -> Result<Vec<Vec<String>>, AppError> {
        let sql = format!("SELECT * FROM {}", &self.tablename);
        debug!("{}", sql);
        let mut stmt = self.connection.prepare(&sql)?;

        let rows = stmt.query_map([], |row| {
            let columns: Vec<String> = (0..self.columns.iter().count())
                .map(|index| row.get(index).unwrap_or_default())
                .collect();
            Ok(columns)
        })?;

        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.into())
    }

    pub fn query_count_data(&self) -> Result<usize, AppError> {
        let sql = format!("SELECT Count(*) FROM {}", &self.tablename);
        debug!("{}", sql);
        let mut stmt = self.connection.prepare(&sql)?;

        let mut rows = stmt.query_map([], |row| Ok(row.get::<usize, usize>(0)?))?;

        if let Some(row_result) = rows.next() {
            return Ok(row_result?);
        } else {
            // Handle the case where no rows were returned
            return Err(rusqlite::Error::ExecuteReturnedResults).map_err(AppError::from)?;
        }
    }
}
