use std::{cell::RefCell, rc::Rc};

use crate::{
    comp,
    compartment::{self, CompartmentInterface::CompartmentInterface},
    dataStructures::{
        DataBaseControl::DatabaseGlobal,
        DatabaseControlInterface::{DatabaseClientInterface, DatabaseGlobalInterface},
    },
};
use mongodb::bson::Document;

use super::MoleculsControlInterface::{MoleculsControlError, MoleculsControlInterface};

pub struct MoleculsControl {
    /*
    Система контроля создания, записи, хранения и доступа к молекулам
    */
    pub moleculStorageEnzyme: RefCell<Vec<Rc<RefCell<String>>>>,
    pub moleculStorageCompound: RefCell<Vec<Rc<RefCell<String>>>>,
}

impl MoleculsControlInterface for MoleculsControl {
    fn add_moleculs_by_name(
        &mut self,
        compartments: Vec<&std::rc::Weak<RefCell<compartment::Compartment::Compartment>>>,
        entry_moleculs: Vec<&str>,
    ) -> Result<(), MoleculsControlError> {
        /*
        Проверяет наличие молекул в базе данных. В противном случае возвращает ошибку и отменяет проделанные действия (если таковые были)
        */
        let mut db_client = DatabaseGlobal::check_and_get_of_connection().unwrap();
        let mut checked_moleculs: Vec<(Document, &str)> = Vec::new();

        //Цикл обработки молекул и создания списка на добавление
        for molecul in entry_moleculs {
            if molecul.starts_with("EC") {
                let enzyme = db_client
                    .find_enzyme_by_entry(
                        molecul
                            .split("EC")
                            .map(|elem| elem.trim())
                            .collect::<Vec<&str>>()[1],
                    )
                    .unwrap();
                checked_moleculs.push((enzyme, "Enzyme"));
            } else if molecul.starts_with("C") {
                let compound = db_client
                    .find_compound_by_entry(
                        molecul
                            .split("EC")
                            .map(|elem| elem.trim())
                            .collect::<Vec<&str>>()[1],
                    )
                    .unwrap();
                checked_moleculs.push((compound, "Compound"));
            } else {
                //В случае некоректного ввода - ошибка и процесс прекращается
                return Err(MoleculsControlError::UndefindMolecul(molecul.to_string()));
            }
        }

        //Цикл добавления молекул в компартменты
        for (molecul, molecul_type) in checked_moleculs {
            //Сначала молекула конвертируется в ссылку и добавляется в хранилище
            if molecul_type == "Enzyme" {
                let conver_enzyme = Rc::new(RefCell::new(molecul.to_string()));
                self.moleculStorageEnzyme
                    .borrow_mut()
                    .push(Rc::clone(&conver_enzyme));

                //Затем происходит добавление элементов в компартменты
                for comp in compartments.iter() {
                    comp!(comp).add_molecul_to_storage(Rc::clone(&conver_enzyme));
                }
            } else if molecul_type == "Compound" {
                let conver_compound = Rc::new(RefCell::new(molecul.to_string()));
                self.moleculStorageCompound
                    .borrow_mut()
                    .push(Rc::clone(&conver_compound));

                //Затем происходит добавление элементов в компартменты
                for comp in compartments.iter() {
                    comp!(comp).add_molecul_to_storage(Rc::clone(&conver_compound));
                }
            }
        }

        Ok(())
    }
}

// #[derive(Default)]
// pub struct CRS {
//     seq: Box<String>,
// }

// #[derive(Default)]
// pub struct AAseq {
//     seq: Box<String>,
// }

// impl AAseq {
//     pub fn set_seq(&mut self, seq: &str) {
//         self.seq = Box::new(seq.to_string());
//     }
// }

// impl Compile for AAseq {
//     fn compile(&mut self) -> Task {
//         todo!("Обращение в базу данных и формирование таска на основе данных оттуда")
//     }
// }

// #[derive(Default)]
// pub struct Protein {
//     name: Box<String>,
//     aaseq: Weak<RefCell<AAseq>>,
//     cofactors: RefCell<HashMap<String, bool>>,
// }

// impl Protein {
//     pub fn set_name(&mut self, name: &str) {
//         self.name = Box::new(name.to_string());
//     }
//     pub fn set_aaseq(&mut self, seq: &Weak<RefCell<AAseq>>) {
//         self.aaseq = Weak::clone(seq);
//     }
// }

// impl Compile for Protein {
//     fn compile(&mut self) -> Task {
//         self.aaseq.upgrade().unwrap().borrow_mut().compile()
//     }
// }

// pub trait Compile {
//     fn compile(&mut self) -> Task;
// }
