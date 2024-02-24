use crate::errorhandler::AppError;
use calamine::Data::{
    Bool, DateTime, DateTimeIso, DurationIso, Empty, Error as ErrorDataType, Float, Int,
    String as CalString,
};
use calamine::{open_workbook, Reader, Xlsx};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct ExcelFile<'a> {
    pub data: Vec<Vec<String>>,
    pub headers: Vec<String>,
    pub filename: &'a str,
    pub filepath: &'a Path,
}

impl<'a> ExcelFile<'a> {
    pub fn new(filepath: &Path) -> Result<ExcelFile, AppError> {
        let mut workbook: Xlsx<_> = open_workbook(filepath)?;
        let filename = ExcelFile::get_filename(filepath)?;
        let headers = ExcelFile::read_headers(&mut workbook)?;
        let data = ExcelFile::read_data(&mut workbook)?;
        Ok(ExcelFile {
            data,
            headers,
            filename,
            filepath,
        })
    }

    fn get_filename<'b>(file_path: &'_ Path) -> Result<&'_ str, AppError> {
        file_path
            .file_name()
            .ok_or(AppError::GenericError("Missing file name in path".into()))?
            .to_str()
            .ok_or(AppError::GenericError(
                "File name is not valid UTF-8".into(),
            ))
    }
    fn read_headers(file: &mut Xlsx<BufReader<File>>) -> Result<Vec<String>, AppError> {
        let range = file
            .worksheet_range_at(0)
            .ok_or_else(|| AppError::OpenExcelError("No sheet found at index 0".into()))
            .and_then(|res| res.map_err(AppError::from))?;

        let headers = range
            .rows()
            .next()
            .ok_or(AppError::GenericError("Cannot read first row".into()))?;

        let mut headers_coll = Vec::new();
        for (_i, header_cell) in headers.iter().enumerate() {
            headers_coll.push(header_cell.to_string())
        }

        Ok(headers_coll)
    }
    fn read_data(file: &mut Xlsx<BufReader<File>>) -> Result<Vec<Vec<String>>, AppError> {
        let range = file
            .worksheet_range_at(0)
            .ok_or_else(|| AppError::OpenExcelError("No sheet found at index 0".into()))
            .and_then(|res| res.map_err(AppError::from))?;

        let mut data = Vec::new();

        for row in range.rows() {
            let row_data: Vec<String> = row
                .iter()
                .map(|cell| match cell {
                    Empty => "Empty".to_string(),
                    CalString(s) => s.to_string(),
                    Float(f) => f.to_string(),
                    DateTime(dt) => dt.to_string(),
                    Bool(b) => b.to_string(),
                    ErrorDataType(e) => format!("Error: {:?}", e),
                    Int(i) => i.to_string(),
                    DateTimeIso(is) => is.to_string(),
                    DurationIso(di) => di.to_string(),
                })
                .collect();
            data.push(row_data);
        }
        Ok(data)
    }
}
