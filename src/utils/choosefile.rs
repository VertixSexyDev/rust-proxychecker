extern crate nfd;

use nfd::Response;

pub fn choose() -> String{
    let result = nfd::open_file_dialog(Some("txt"), None).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    match result {
        Response::Okay(file_path) => {
            return file_path;
        }
        Response::OkayMultiple(_) => todo!(),
        Response::Cancel => todo!(),
    }
}