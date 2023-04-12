use std::{
    self,
    cell::RefCell,
    fmt::Write,
    rc::{Rc, Weak},
};

use crate::{
    comp,
    compartment::{Compartment::Compartment, CompartmentInterface::CompartmentInterface},
};

use super::CompartmentControlInterface::{CompartmentControlErrors, CompartmentControlInterface};

/*
Модуль отвечающий за логику системы компартментов, их хранение, создание, наполнение и взаимодействие между собой
*/

#[derive(Debug)]
// Структура главной системы.
pub struct CompartmentsControl {
    // Содержит поля:
    // compartment_storage - изменяемое хранилище указателей на все компартменты
    compartment_storage: RefCell<Vec<Rc<RefCell<Compartment>>>>,
    NESTING_DEEP: i32,
}

impl Default for CompartmentsControl {
    fn default() -> Self {
        Self {
            compartment_storage: Default::default(),
            NESTING_DEEP: 20,
        }
    }
}

impl CompartmentControlInterface for CompartmentsControl {
    // Создает компартмент и возвращается weak-ссылку на него
    fn create_compartment(
        &mut self,
    ) -> Result<Weak<RefCell<Compartment>>, CompartmentControlErrors> {
        let comp = Rc::new(RefCell::new(Compartment::default()));
        let out_comp = Rc::downgrade(&comp);
        self.compartment_storage.borrow_mut().push(comp);

        return Ok(out_comp);
    }

    // Добавляет дочерний компартмент в переданный по weak-ссылке и возвращает weak на новый
    fn add_compartment(
        &mut self,
        par_comp: &Weak<RefCell<Compartment>>,
    ) -> Result<Weak<RefCell<Compartment>>, CompartmentControlErrors> {
        let daug_comp = self.create_compartment().unwrap();
        let daug_comp2 = Weak::clone(&daug_comp);
        comp!(&daug_comp).set_parent(par_comp.to_owned());
        comp!(par_comp).add_daughter(daug_comp);
        Ok(daug_comp2)
    }

    // Связывает два указанных компартмента как родительский и дочерний.
    fn insert_daughter_compartment(
        &mut self,
        par_comp: &Weak<RefCell<Compartment>>,
        daug_comp: &Weak<RefCell<Compartment>>,
    ) -> Result<(), CompartmentControlErrors> {
        if self.daughter_is_parent(par_comp, daug_comp) {
            return Err(CompartmentControlErrors::GrandfatherParadox);
        } else if comp!(par_comp)
            .get_daughter_id()
            .unwrap_or(vec![])
            .contains(&comp!(daug_comp).get_id())
        {
            return Err(CompartmentControlErrors::CompartmentIsAlreadyChild);
        } else {
            self.move_compartmenet(daug_comp, par_comp)?;
            return Ok(());
        }
    }

    fn get_inf(&mut self) -> Result<String, CompartmentControlErrors> {
        let mut report = String::new();
        for comp in self.compartment_storage.borrow().iter() {
            write!(&mut report, "{}\n", comp.borrow_mut().get_inf().unwrap()).unwrap();
        }

        Ok(report)
    }
}

impl CompartmentsControl {
    //Внутренние функции системы
    fn get_all_parents(&mut self, compartment: &Weak<RefCell<Compartment>>) -> Option<Vec<String>> {
        let mut tmp_vec: Vec<Weak<RefCell<Compartment>>> = Vec::new();

        if let Some(par_comp) = comp!(compartment).get_parent() {
            tmp_vec.push(par_comp);
        } else {
            return None;
        }

        for _ in 0..self.NESTING_DEEP {
            if let Some(par_comp) = comp!(tmp_vec[tmp_vec.len() - 1]).get_parent() {
                tmp_vec.push(par_comp);
                continue;
            } else {
                break;
            }
        }
        let otp_vec = tmp_vec
            .iter()
            .map(|elem| comp!(elem).get_id())
            .collect::<Vec<String>>();
        // println!("{} имеет столько родителей: {}", comp!(compartment).get_name(), &otp_vec.len());
        return Some(otp_vec);
    }

    fn daughter_is_parent(
        &mut self,
        parent_compartment: &Weak<RefCell<Compartment>>,
        daughter_compartment: &Weak<RefCell<Compartment>>,
    ) -> bool {
        if let Some(all_parents) = self.get_all_parents(parent_compartment) {
            // println!(
            //     "{} имеет столько родителей: {}",
            //     comp!(parent_compartment).get_name(),
            //     all_parents.len()
            // );
            if all_parents.contains(&comp!(daughter_compartment).get_id()) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn move_compartmenet(
        &mut self,
        daughter_compartment: &Weak<RefCell<Compartment>>,
        parent_compartment: &Weak<RefCell<Compartment>>,
    ) -> Result<(), CompartmentControlErrors> {
        
        Ok(())
    }
}

#[cfg(test)]
mod test_CompartmentsControl {
    use crate::{
        comp,
        compartment::CompartmentInterface::CompartmentInterface,
        compartmentSystem::{
            CompartmentControl::CompartmentsControl,
            CompartmentControlInterface::CompartmentControlInterface,
        },
    };

    #[test]
    fn test_base_function() {
        let mut sustema = CompartmentsControl::default();
        let cell = sustema.create_compartment().unwrap();
        let core = sustema.create_compartment().unwrap();
        let pyro = sustema.create_compartment().unwrap();
        comp!(&cell).set_name("Клетка");
        comp!(&core).set_name("Ядро");
        comp!(&pyro).set_name("Пироксисома");
        println!("{:?}", sustema.insert_daughter_compartment(&cell, &core));
        println!("{:?}", sustema.insert_daughter_compartment(&core, &pyro));
        println!("-------------");
        println!(
            "Все родители Пироксисомы: {:?}",
            sustema.get_all_parents(&pyro)
        );

        println!("{}", comp!(&cell).get_inf().unwrap());
        println!("{}", comp!(core).get_inf().unwrap());
        println!("{}", comp!(pyro).get_inf().unwrap());
    }
}
