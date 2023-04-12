use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Enzyme {
    //Структура для дальнейшей сериализации энзимов
    pub Entry: String,
    pub Name: Vec<String>,
    pub Class: Vec<String>,
    pub Reaction: Vec<String>,
    pub GeneralReaction: Vec<String>,
    pub Substrate: Vec<String>,
    pub Product: Vec<String>,
    }


    