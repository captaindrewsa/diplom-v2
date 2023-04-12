use std::{cell::RefCell, rc::Weak};

use thiserror::Error;

use crate::{
    compartment::Compartment::Compartment,
    compartmentSystem::CompartmentControlInterface::CompartmentControlErrors,
    moleculSystem::MoleculsControlInterface::MoleculsControlError,
};

pub trait GenesysInterface {
    fn create_compartment(
        &mut self,
    ) -> Result<Weak<RefCell<Compartment>>, CompartmentControlErrors>;

    fn add_compartment(
        &mut self,
        par_comp: &Weak<RefCell<Compartment>>,
    ) -> Result<Weak<RefCell<Compartment>>, CompartmentControlErrors>;

    fn insert_daughter_compartment(
        &mut self,
        par_comp: &Weak<RefCell<Compartment>>,
        daug_comp: &Weak<RefCell<Compartment>>,
    ) -> Result<(), CompartmentControlErrors>;

    fn get_information(&mut self) -> Result<String, GenesysErrors>;

    // fn get_moleculOne(&mut self, entry: &str){}
    // fn get_pathway(&mut self, entry: &str);

    fn add_moleculs_by_name(
        &mut self,
        compartments: Vec<&std::rc::Weak<RefCell<Compartment>>>,
        entry_moleculs: Vec<&str>,
    ) -> Result<(), MoleculsControlError>;
}

#[derive(Error, Debug)]
pub enum GenesysErrors {
    #[error("Something went wrong")]
    Error,
}
