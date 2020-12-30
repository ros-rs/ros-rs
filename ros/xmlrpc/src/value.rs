
enum Member {
    name(String),
    value(Value)
}

enum Value {
    Nil,
    Boolean(bool),
    Int(i32),
    Double(f64),
    String(String),
    DateTime,
    Base64(Vec<u8>),
    Array(Vec<Value>),
    Struct(Vec<Member>)
}

