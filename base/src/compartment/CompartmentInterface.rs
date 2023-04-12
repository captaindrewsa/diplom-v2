use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use thiserror::Error;

use super::Compartment::Compartment;

pub trait CompartmentInterface {
    fn set_name(&mut self, name: &str);
    fn set_parent(&mut self, parent: Weak<RefCell<Compartment>>);
    
    fn add_daughter(&mut self, daughter: Weak<RefCell<Compartment>>);
    fn add_molecul_to_storage(&mut self, molecul: Rc<RefCell<String>>);

    fn get_inf(&mut self) -> Result<String, std::fmt::Error>;
    fn get_name(&mut self) -> String;
    fn get_id(&mut self) -> String;
    fn get_parent(&mut self) -> Option<Weak<RefCell<Compartment>>>;
    fn get_daughter(&mut self) -> RefCell<Vec<Weak<RefCell<Compartment>>>>;
    fn get_daughter_id(&mut self) -> Option<Vec<String>>;
    fn get_moleculs_storage(&mut self) -> RefCell<Vec<Rc<RefCell<String>>>>;

    fn remove_daughert_compartment(
        &mut self,
        compartment: &Weak<RefCell<Compartment>>,
    ) -> Result<(), CompartmentErrors>;

}
#[derive(Debug, Error)]
pub enum CompartmentErrors {
    #[error("No such compartment exists")]
    CompartmentDoesNotExist,
}
