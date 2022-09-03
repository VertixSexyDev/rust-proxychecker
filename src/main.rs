mod utils {
    pub mod logo;
    pub mod createoption;
    pub mod input;
    pub mod filereader;
}
mod modules {
    pub mod http;
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
            let path = utils::input::input("Path to proxies: ");
            utils::filereader::read_file(&path, 1);
        }
    }
}