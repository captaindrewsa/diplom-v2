use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: String,
    y: String,
    a: String,
    b: String,
    c: String,
    f: String,
}

fn main() {
    let a = "EC1.1.1.1";
    println!(
        "{:?}",
        a.split("EC").map(|elem| elem.trim()).collect::<Vec<&str>>()[1]
    );

    // Сравнение размеров сериализованных и десериализованных структур

    let test1 = Point {
        x: "Сколько памяти занимает сериализованный и структурный объекты".to_string(),
        y: "Сколько памяти занимает сериализованный и структурный объекты".to_string(),
        a: "Сколько памяти занимает сериализованный и структурный объекты".to_string(),
        b: "Сколько памяти занимает сериализованный и структурный объекты".to_string(),
        c: "Сколько памяти занимает сериализованный и структурный объекты".to_string(),
        f: "Сколько памяти занимает сериализованный и структурный объекты".to_string(),
    };

    let test2 = serde_json::to_string(&test1).unwrap();

    println!(
        "Struct:{}\nString:{}",
        std::mem::size_of_val(&test1),
        std::mem::size_of_val(&test2)
    );
}
