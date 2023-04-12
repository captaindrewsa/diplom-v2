use std::{cell::RefCell, rc::Weak};

use thiserror::Error;

use crate::compartment::Compartment::Compartment;

use super::MoleculStructuresInterface::Molecul;

pub trait MoleculsControlInterface {
    fn add_moleculs_by_name(
        &mut self,
        compartments: Vec<&Weak<RefCell<Compartment>>>,
        entry_moleculs: Vec<&str>,
    ) -> Result<(), MoleculsControlError>;

    // fn get_moleculOne(&mut self, entry: &str);
}

#[derive(Error, Debug)]
pub enum MoleculsControlError {
    #[error("Something went wrong")]
    SomethingError,
    #[error("Molecule syntax error: {0}")]
    UndefindMolecul(String),
}
