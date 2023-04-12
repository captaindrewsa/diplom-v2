use regex::Regex;
use reqwest::{self, Response};
use scraper::{self, element_ref::Text, Html, Selector};

/*
Модуль, отвечающий за парсинг web-db (KEGG и т.д.) для дальнейшего предоставления базе данных

*/

pub async fn get_page_string(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    /*
    Функция, возвращающая результат со строковым представлением web-страницы
    */
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    Ok(resp)
}

pub async fn parse_html(
    html: String,
    parametres: ParserOptions,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    /*
    Функция, занимающаяся парсингом веб страницы по параметрам:
    -Enzyme - вычлиняет данные по энзимам и сериализует их
    ...
    */

    let doc = Html::parse_document(&html);

    match parametres {
        ParserOptions::Enzyme => {
            let enzyme_selector_th =
                scraper::Selector::parse("table.w2>tbody>tr>th>span.nowrap").unwrap();
            let enzyme_selector_td = scraper::Selector::parse("table.w2>tbody>tr>td").unwrap();

            let th_vec = doc
                .select(&enzyme_selector_th)
                .map(|x| x.inner_html())
                .collect::<Vec<String>>();

            let td_vec = doc
                .select(&enzyme_selector_td)
                .map(|td| td.text().map(|word| word.to_string()).collect::<String>())
                .collect::<Vec<_>>();

            let otp = th_vec
                .iter()
                .map(|x| x.to_string())
                .zip(td_vec)
                .collect::<Vec<(String, String)>>();

            return Ok(otp);
        }
        ParserOptions::Compound => todo!(),
        ParserOptions::Reaction => todo!(),
        ParserOptions::None => todo!(),
    }
}

pub enum ParserOptions {
    //Перечисление для настроек парсера
    None,
    Enzyme,
    Compound,
    Reaction,
}

pub fn test_regex() -> String {
    let a = "some text and BRITE hierarchy and oth1231234er text".to_string();

    let re = Regex::new(r"\d{}").unwrap();

    let m = re.find(a.as_str()).unwrap();
    // println!("{}", a.replace("and BRITE hierarchy ", "\n"));

    println!("{}", m.as_str());

    m.as_str().to_string()
}

#[cfg(test)]
mod test_parser {

    #[test]
    fn pars_test() {}
}
