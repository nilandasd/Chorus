pub struct Config<'a> {
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 1 {
            return Err("Too many args");
        }

        Ok(Self{
            filename: args[0].as_str(),
        })
    }
}
