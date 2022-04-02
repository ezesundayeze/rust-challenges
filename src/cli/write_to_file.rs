use std::fs;
use std::env;
use std::path;

fn main(){
    //Check if the second argument is "yes", if so,then write to the file others do nothing
    if env::args().nth(2) == Some(String::from("yes")){
        let mut speech = String::new();

        speech.push_str("Hello");
        speech.push_str("World");
        
        //COnfirm the file path exists
        if (path::Path::new("speech.txt").exists()){
            println!("Speech already exists")
        }
        fs::write("speech.txt", speech);
    }
}