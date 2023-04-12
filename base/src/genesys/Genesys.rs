use crate::{
    compartmentSystem::{
        CompartmentControl::CompartmentsControl,
        CompartmentControlInterface::CompartmentControlInterface,
    },
    dataStructures::{
        DataBaseControl::{DatabaseClient, DatabaseGlobal},
        DatabaseControlInterface::DatabaseGlobalInterface,
    },
    moleculSystem::{
        MoleculsControl::MoleculsControl, MoleculsControlInterface::MoleculsControlInterface,
    },
};

use super::GenesysInterface::{GenesysErrors, GenesysInterface};

pub struct Genesys {
    compartmentSystem: CompartmentsControl,
    moleculsControl: MoleculsControl,
    databaseControl: DatabaseClient,
}

impl Genesys {
    pub fn new() -> Self {
        let db = DatabaseGlobal::check_and_get_of_connection().unwrap();

        Self {
            compartmentSystem: Default::default(),
            moleculsControl: MoleculsControl {
                moleculStorageEnzyme: Default::default(),
                moleculStorageCompound: Default::default(),
            },
            databaseControl: db,
        }
    }
}

impl GenesysInterface for Genesys {
    fn create_compartment(
        &mut self,
    ) -> Result<
        std::rc::Weak<std::cell::RefCell<crate::compartment::Compartment::Compartment>>,
        crate::compartmentSystem::CompartmentControlInterface::CompartmentControlErrors,
    > {
        self.compartmentSystem.create_compartment()
    }

    fn add_compartment(
        &mut self,
        par_comp: &std::rc::Weak<std::cell::RefCell<crate::compartment::Compartment::Compartment>>,
    ) -> Result<
        std::rc::Weak<std::cell::RefCell<crate::compartment::Compartment::Compartment>>,
        crate::compartmentSystem::CompartmentControlInterface::CompartmentControlErrors,
    > {
        self.compartmentSystem.add_compartment(par_comp)
    }

    fn insert_daughter_compartment(
        &mut self,
        par_comp: &std::rc::Weak<std::cell::RefCell<crate::compartment::Compartment::Compartment>>,
        daug_comp: &std::rc::Weak<std::cell::RefCell<crate::compartment::Compartment::Compartment>>,
    ) -> Result<(), crate::compartmentSystem::CompartmentControlInterface::CompartmentControlErrors>
    {
        self.compartmentSystem
            .insert_daughter_compartment(par_comp, daug_comp)
    }

    fn add_moleculs_by_name(
        &mut self,
        compartments: Vec<
            &std::rc::Weak<std::cell::RefCell<crate::compartment::Compartment::Compartment>>,
        >,
        entry_moleculs: Vec<&str>,
    ) -> Result<(), crate::moleculSystem::MoleculsControlInterface::MoleculsControlError> {
        self.moleculsControl
            .add_moleculs_by_name(compartments, entry_moleculs)
    }

    fn get_information(&mut self) -> Result<String, super::GenesysInterface::GenesysErrors> {
        if self.compartmentSystem.get_inf().is_ok() {
            Ok(self.compartmentSystem.get_inf().unwrap())
        } else {
            Err(GenesysErrors::Error)
        }
    }
}

pub mod Macroses {
    #[macro_export]
    macro_rules! comp {
        ($comp:expr) => {
            $comp.upgrade().unwrap().borrow_mut()
        };
    }
}
