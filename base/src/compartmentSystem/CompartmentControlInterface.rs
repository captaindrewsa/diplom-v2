use std::{cell::RefCell, rc::Weak};

use thiserror::Error;

use crate::compartment::Compartment::Compartment;

/*
Модуль, отвечащий за интерфейсы к бизнес-логике
*/

pub trait CompartmentControlInterface {
    // Создает компартмент и возвращается weak-ссылку на него
    fn create_compartment(
        &mut self,
    ) -> Result<Weak<RefCell<Compartment>>, CompartmentControlErrors>;

    // Добавляет дочерний компартмент в переданный по weak-ссылке и возвращает weak на него
    fn add_compartment(
        &mut self,
        par_comp: &Weak<RefCell<Compartment>>,
    ) -> Result<Weak<RefCell<Compartment>>, CompartmentControlErrors>;

    // Связывает два указанных компартмента как родительский и дочерний с проверкой существования в базе данных.
    fn insert_daughter_compartment(
        &mut self,
        par_comp: &Weak<RefCell<Compartment>>,
        daug_comp: &Weak<RefCell<Compartment>>,
    ) -> Result<(), CompartmentControlErrors>;

    fn get_inf(&mut self) -> Result<String, CompartmentControlErrors>;
}

#[derive(Debug, Error)]
pub enum CompartmentControlErrors {
    //Перечисление с элементами для интерфейса для бизнес-логики
    #[error("Something went wrong")]
    SomethingError,
    #[error("Compartment was not found")]
    CompartmentNoFound(String),
    #[error(
        "The child compartment is the parent of one of the parents of the selected compartment"
    )]
    GrandfatherParadox,
    #[error("Compartment is already a child")]
    CompartmentIsAlreadyChild,
    // #[error("Unable to transfer compartments")]
    // CanNotMoveCompartment
}
