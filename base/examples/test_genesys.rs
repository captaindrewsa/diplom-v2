use base::{
    comp,
    compartment::CompartmentInterface::CompartmentInterface,
    genesys::{Genesys::Genesys, GenesysInterface::GenesysInterface},
};

fn main() {
    let mut sys = Genesys::new();
    // let time_start = std::time::Instant::now();
    let cell = sys.create_compartment().unwrap();
    // let duration = time_start.elapsed().as_micros();
    let core = sys.create_compartment().unwrap();
    // comp!(core).set_name("Ядро");
    // comp!(cell).set_name("Клетка");
    let sub_core = sys.add_compartment(&core).unwrap();
    let sub_sub_core = sys.add_compartment(&sub_core).unwrap();
    sys.insert_daughter_compartment(&cell, &core).unwrap();
    println!("{}", sys.get_information().unwrap());

    // println!("Задержка создания компартмента: {:?}", duration);
    // sys.add_moleculs(vec![&core], vec!["EC 1.3.2.1", "C 00782"]);
    // ...
}
