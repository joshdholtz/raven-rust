pub struct Raven {
        dsn: String
}

impl Raven {

	pub fn new(dsn: String) -> Raven {
		Raven { dsn: dsn }
	}

        pub fn capture(&self) {
                println!("Capturing to {}", self.dsn);
        }

}

