#![allow(dead_code)]
use std::{
    cell::RefCell,
    char::from_u32,
    fmt::Write,
    rc::{Rc, Weak},
    time::SystemTime,
};

use chrono::format::format;
use rand::{distributions::Alphanumeric, prelude::Distribution, Rng};
use sha2::{Digest, Sha256};

use crate::comp;

use super::CompartmentInterface::{CompartmentErrors, CompartmentInterface};

#[derive(Debug)]
// Структура компартмента. Функциональная единица системы проектирования.
pub struct Compartment {
    // Содержит поля:
    // parent_comp - weak-ссылка на родительский компартмент находящийся в общем хранилизе CompartmentsControl
    // daughter_comp - изменяемый список ссылок на дочерние компартменты, находящиеся в общем хранилище CompartmentsControl
    // name - пользовательский параметр. Используется в случае необходимости наглядного различения компартментов
    parent_comp: Option<Weak<RefCell<Compartment>>>,
    daughter_comp: RefCell<Vec<Weak<RefCell<Compartment>>>>,
    name: String,
    id: String,
    moleculStorage: RefCell<Vec<Rc<RefCell<String>>>>,
}

impl Default for Compartment {
    fn default() -> Self {
        let mut input_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_be_bytes()
            .to_vec();
        let mut input_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let mut hash_data = input_string.as_bytes().to_vec();
        hash_data.append(&mut input_time);

        let mut hasher = Sha256::new();
        hasher.update(hash_data);
        let hash_comp = format!("{:x}", hasher.finalize());

        Self {
            parent_comp: Default::default(),
            daughter_comp: Default::default(),
            name: hash_comp.clone(),
            moleculStorage: Default::default(),
            id: hash_comp.clone(),
        }
    }
}

impl CompartmentInterface for Compartment {
    // Задает имя компартменту
    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
    // Выдает строку с информацией о компартменте
    fn get_inf(&mut self) -> Result<String, std::fmt::Error> {
        let mut report = String::new();
        write!(&mut report, "Имя: {}\n", self.name)?;
        writeln!(
            &mut report,
            "Родительский компартмент: {}",
            {
                if !self.get_parent().is_none() {
                    self.get_parent()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .borrow_mut()
                        .get_name()
                } else {
                    "None".to_string()
                }
            }
        )?;
        writeln!(
            &mut report,
            "Дочерние компартменты: {:?}",
            self.get_daughter()
                .borrow()
                .iter()
                .map(|comp| comp.upgrade().unwrap().borrow().name.clone())
                .collect::<Vec<String>>()
        )?;
        /*
        Добавить вывод списка молекул в компартменте
        */

        return Ok(report);
    }

    fn add_molecul_to_storage(&mut self, molecul: Rc<RefCell<String>>) {
        self.moleculStorage.borrow_mut().push(molecul);
    }

    fn get_name(&mut self) -> String {
        self.name.to_string()
    }

    fn get_id(&mut self) -> String {
        self.id.clone()
    }

    fn get_parent(&mut self) -> Option<Weak<RefCell<Compartment>>> {
        self.parent_comp.clone()
    }

    fn get_daughter(&mut self) -> RefCell<Vec<Weak<RefCell<Compartment>>>> {
        RefCell::clone(&self.daughter_comp)
    }

    fn get_moleculs_storage(&mut self) -> RefCell<Vec<Rc<RefCell<String>>>> {
        self.moleculStorage.clone()
    }

    fn set_parent(&mut self, parent: Weak<RefCell<Compartment>>) {
        self.parent_comp = Some(parent);
    }

    fn add_daughter(&mut self, daughter: Weak<RefCell<Compartment>>) {
        self.daughter_comp.borrow_mut().push(daughter);
    }

    fn get_daughter_id(&mut self) -> Option<Vec<String>> {
        let mut otp_id: Vec<String> = Vec::new();
        for elem in self.get_daughter().borrow().iter() {
            otp_id.push(comp!(elem).get_id());
        }

        if otp_id.is_empty() {
            None
        } else {
            Some(otp_id)
        }
    }

    fn remove_daughert_compartment(
        &mut self,
        compartment: &Weak<RefCell<Compartment>>,
    ) -> Result<(), CompartmentErrors> {
        for (idx, elem) in self.daughter_comp.borrow_mut().iter().enumerate() {
            if comp!(elem).id == comp!(compartment).id {
                self.daughter_comp.borrow_mut().remove(idx);
                return Ok(());
            } else {
                continue;
            }
        }
        return Err(CompartmentErrors::CompartmentDoesNotExist);
    }
}
