mod env_operator;


#[cfg(test)]
mod tests {
    // use dotenv;
    use std::env;

    #[test]
    fn test_env() {
        println!("1111 {:?}",env::current_dir().unwrap());
    }

}