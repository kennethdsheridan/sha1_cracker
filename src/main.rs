use std:: {
    env,
    error::Error,
};

const SHA1_HEX_STRING_LENGTH: usize = 40;


fn main() -> Result<(), Box<dyn Error>> {
    // the args is a Vec of Strings
    let args:Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt>, <sha1_hash>");
        return Ok(());
    }
    
    // ensure that the provided hash_to_crack is the compliant
    // with the expected length of the hash which should equal
    // SHA1_HEX_STRING_LENGTH
    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    Ok(())
}
