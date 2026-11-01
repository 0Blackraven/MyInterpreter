use std::cell::RefCell;

pub fn push_output(s: &str) {
    OUTPUT_BUFFER.with(|b| {
        let mut buf = b.borrow_mut();
        buf.push_str(s);
        buf.push('\n');
    });
}

pub fn get_output() -> String {
    OUTPUT_BUFFER.with(|b| b.borrow().clone())
}

pub fn clear_output() {
    OUTPUT_BUFFER.with(|b| b.borrow_mut().clear());
}

thread_local! {
    static OUTPUT_BUFFER: RefCell<String> = RefCell::new(String::new());
}