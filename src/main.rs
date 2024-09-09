use std::{
    env,
    error::Error,
 }; //import env & error modules from std library

const MD5_HEX_STRING_LENGTH: usize = 32;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect(); //calls args function and returns an iterator "();" since we have multiple arguments which can be collected into a Vector (array that can be resized) of String objects
    
    if args.len() != 3 {
        println!("Usage:"); //println! is a macro, as denoted by the exclamation mark
        println!("md5_cracker: <wordlist.txt> <md5_hash>;");
        return Ok(());
    }

    let hash = args[2].trim(); //get 2nd arg from iterator which is our hash
    if hash.len() != MD5_HEX_STRING_LENGTH {
        return Err("Invalid MD5 Hash".into());
    }
    Ok(())
}

/*resources:
iterator: https://doc.rust-lang.org/book/ch13-02-iterators.html
macros: https://doc.rust-lang.org/book/ch19-06-macros.html
boxing errors: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
trait std error error https://doc.rust-lang.org/std/error/trait.Error.html
*/
