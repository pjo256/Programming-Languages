struct Weapon {
	pub ap: i32,
	durability: i32,
	pub condition: i32
}

impl Weapon {

	fn new(ap, durability, condition) -> Weapon {
		Weapon { ap: ap, durability: durability, condition: condition }
	}

	fn compute_dmg(&self) -> i32 {
		ap
	}

	fn get_durability(&self) -> i32 {
		durability
	}
}

trait Degradable  {
	fn degrade(&mut self);
	fn repair(&mut self);
}

impl Degradable for Weapon {
	fn degrade(&mut self) {
		if self.condition > 0 { 
			self.condition -= 1; 
		}
	}

	fn repair(&mut self) {
		if self.condition < get_durability(self) { 
			self.condition += 1; 
		}
	}
}
