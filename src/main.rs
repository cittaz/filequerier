mod errorhandler;
mod sqlhelper;
mod xslxprocessor;

use crate::errorhandler::AppError;
use crate::sqlhelper::SqlHelper;
use crate::xslxprocessor::ExcelFile;
use log::info;
use rusqlite::Connection;
use std::path::Path;

fn main() -> Result<(), AppError> {
    env_logger::init();
    info!("Start");
    //small excel
    // let excel_file_path = Path::new("/home/cittaz/Downloads/sample.xlsx");

    //800k rows excel
    // let excel_file_path = Path::new("/home/cittaz/Downloads/linkedin_job_postings.xlsx");

    //100 rows excel
    let excel_file_path = Path::new("/home/cittaz/Downloads/linkedin_job_postingsTwo.xlsx");

    let excel_file = ExcelFile::new(excel_file_path)?;
    let headers_collection = excel_file.headers;
    let excel_data = excel_file.data;
    let filename = excel_file.filename;

    let conn = open_connection()?;

    let table_name = filename.replace(".", "");
    let sqlexcelhelper = SqlHelper::new(&table_name, &conn, &headers_collection);
    sqlexcelhelper.create_empty_table()?;
    sqlexcelhelper.insert_data_into_sqlite(&excel_data)?;
    let exceldata = sqlexcelhelper.query_all_data()?;
    let tablerowcount = sqlexcelhelper.query_count_data()?;
    info!("row count: {tablerowcount}");

    Ok(())
}

fn open_connection() -> Result<Connection, AppError> {
    let conn = Connection::open_in_memory()?;
    Ok(conn)
}
