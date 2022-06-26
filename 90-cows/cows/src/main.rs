

#[derive(Clone, Copy)]
struct Cow {
  //nick_name: String,
  x: f64,
  y: f64,
}

// associated function
impl Cow {
  pub fn move_right(&mut self) {
    self.x = self.x + 1.0;
  }
}

fn main() {
  println!("Hello, world!");
  let mut bob = Cow {
    x:10.0,
    y:10.0,
  };

  let mut sam = Cow {
    x:30.0,
    y:30.0,
  };


  //let mut cows_list: Vec<Cow> = vec![];

  let mut cows_list: Vec<Cow> = Vec::new();


  cows_list.push(bob);
  cows_list.push(sam);


  sam.move_right();
  bob.move_right();
  bob.move_right();


  println!("{} {}", sam.x, sam.y);
  println!("{} {}", bob.x, bob.y);


  for cow in cows_list.iter_mut() {
    cow.move_right();
    //&cow.move_right();
    //&cow.move_right();
    //cow.move_right();
    println!("{} {}", cow.x, cow.y)
  }


}
