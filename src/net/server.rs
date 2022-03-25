pub struct Server {

}

impl Server {
    pub fn new() -> Server {
        return Server {};
    }
    pub fn get_hello(self) -> String {
        return String::from("Hello!!!")
    }
}