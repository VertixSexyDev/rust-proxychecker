mod utils {
    pub mod logo;
    pub mod createoption;
    pub mod input;
    pub mod filereader;
    pub mod choosefile;
}
mod modules {
    pub mod http;
    pub mod socks4;
    pub mod socks5;
}
fn main() {
    clearscreen::clear().unwrap();
    utils::logo::logo();
    utils::createoption::createoption(1, "Modules");
    utils::createoption::createoption(2, "Exit");
    let menuinput = utils::input::input("> ");
    if menuinput == "1" {
        clearscreen::clear().unwrap();
        utils::logo::logo();
        utils::createoption::createoption(1, "HTTP/HTTPS");
        utils::createoption::createoption(2, "SOCKS4");
        utils::createoption::createoption(3, "SOCKS5");
        let moduleinput = utils::input::input("> ");
        if moduleinput == "1" {
            clearscreen::clear().unwrap();
            utils::logo::logo();
            let path = utils::choosefile::choose();
            utils::filereader::read_file(&path, 1);
        } else if moduleinput == "2" {
            clearscreen::clear().unwrap();
            utils::logo::logo();
            let path = utils::choosefile::choose();
            utils::filereader::read_file(&path, 2);
        } else if moduleinput == "3" {
            clearscreen::clear().unwrap();
            utils::logo::logo();
            let path = utils::choosefile::choose();
            utils::filereader::read_file(&path, 3);
        }
    }
}