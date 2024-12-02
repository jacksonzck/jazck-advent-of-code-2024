use proc_macro::TokenStream;
use emojis;

#[proc_macro] 
pub fn e(item: TokenStream) -> TokenStream {
    let stringed_input = item.to_string();
    let mut output: Vec<String> = vec![];
    for word in stringed_input.split(" ") { 
        println!("Read a {}", word);
        output.push(word.chars().map(|character| {
            match emojis::get(character.to_string().as_str()) {
                Some(emoji) => emoji.name().replace(" ", "_"),
                None => character.to_string(),
            }
        }).collect());
        println!("Writing a {:#?}", output.last())
    };
    println!("{:#?}", output.join(" "));
    output.join(" ").parse().unwrap()
}

#[proc_macro] 
pub fn f(_item: TokenStream) -> TokenStream {
    "println!(\"Bannana\")".parse().unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        // Of course it works; I wrote it!
        assert!(true);
    }
}
