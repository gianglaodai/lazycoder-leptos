#[derive(Clone, Debug, Default)]
pub struct CsvExportOpts { pub file_name: Option<String>, pub delimiter: char, pub include_headers: bool }


pub struct ExportService;


impl ExportService {
    pub fn new() -> Self { Self }
    pub fn export_csv<T>(&self, _rows: &[T], _opts: &CsvExportOpts) -> String { String::new() }
}