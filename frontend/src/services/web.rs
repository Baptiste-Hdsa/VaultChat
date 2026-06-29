pub fn base_url() -> String {
    web_sys::window().unwrap().location().origin().unwrap()
}

pub fn base_hostname() -> String {
    web_sys::window().unwrap().location().host().unwrap()
}
