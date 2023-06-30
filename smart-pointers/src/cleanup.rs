struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Dropping with data {}", self.data);
    }
}

pub fn main() {
    let a = CustomerSmartPointer {
        data: String::from("Hello"),
    };
    let b = CustomerSmartPointer {
        data: String::from("World!"),
    };
    std::mem::drop(b);

    println!("Done")
}
