use base::dataStructures::ParserControl::*;

fn main() {
    // let string_html = get_page_string("https://www.kegg.jp/entry/1.1.1.1")
    //     .await
    //     .unwrap();

    // let test = parse_html(string_html, ParserOptions::Enzyme)
    //     .await
    //     .unwrap();
    // let mut output = String::new();

    // for (tag, value) in test.iter(){
    //     output.push_str(format!("{}:\n\t{}\n", tag, value).as_str());
    // }

    // let mut file = std::fs::File::create("output.txt").unwrap();
    // file.write_all(output.as_bytes());

    test_regex();
}
