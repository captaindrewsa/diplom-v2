use base::{
    comp,
    compartment::CompartmentInterface::CompartmentInterface,
    compartmentSystem::{
        CompartmentControl::CompartmentsControl,
        CompartmentControlInterface::CompartmentControlInterface,
    },
};

fn main() {
    let mut sustema = CompartmentsControl::default();
    let cell = sustema.create_compartment().unwrap();
    let mito = sustema.create_compartment().unwrap();
    let lysosome1 = sustema.create_compartment().unwrap();

    comp!(&cell).set_name("Клетка");
    // println!("{}", comp!(&cell).get_id());
    comp!(&mito).set_name("Митохондрия");
    // println!("{}", comp!(&mito).get_id());
    comp!(&lysosome1).set_name("Лизосома 1");

    sustema.insert_daughter_compartment(&cell, &mito).unwrap();

    sustema
        .insert_daughter_compartment(&mito, &lysosome1)
        .unwrap();
    sustema
        .insert_daughter_compartment(&cell, &lysosome1)
        .unwrap();
    // sustema
    //     .insert_daughter_compartment(&lysosome1, &cell)
    //     .unwrap();
    println!("-------------");
    println!("{}", comp!(cell).get_inf().unwrap());
    println!("{}", comp!(mito).get_inf().unwrap());
    println!("{}", comp!(lysosome1).get_inf().unwrap());
}
