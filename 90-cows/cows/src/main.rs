


#[derive(Clone, Copy)]
struct Constraints {
  border: f64,
  width: f64,
  height: f64,
  max_velocity: f64
}

#[derive(Clone, Copy)]
struct Cow {
  //nick_name: String,
  id: u64,
  x: f64,
  y: f64,
  constraints: Constraints,
  x_velocity: f64,
  y_velocity: f64
}

// associated function
impl Cow {

  pub fn move_right(&mut self) {
    self.x = self.x + 1.0;
  }

  pub fn something_to_do(&mut self, mut boids: Vec<Cow>) {
    println!("{}", boids.len());
    self.x = self.x + 1.0;
  }





}

fn main() {
  let mut cows_list: Vec<Cow> = vec![];

  let constraints = Constraints {
    border:      5.0,
    width:       800.0,
    height:      800.0,
    max_velocity: 5.0
  };

  let mut bob = Cow {
    id: 99,
    x:10.0,
    y:10.0,
    constraints: constraints,
    x_velocity: 1.0,
    y_velocity: -1.0
};


  let mut i = 0;

  while i < 5 {
      let cow = Cow {
        id:i,
        x:10.0,
        y:10.0,
        constraints: constraints,
        x_velocity: 1.0,
        y_velocity: -1.0,
      };
      i = i + 1;
      cows_list.push(cow)
  }

  let mut index = 0;


  while index < 5 {

    for cow in cows_list.iter_mut() {

      let cows_list_bis: Vec<Cow> = cows_list.clone();

      cow.move_right();
      //cow.something_to_do(cows_list);

      //println!("{}", cow.distance(bob));

      println!("{}: {} {}", cow.id, cow.x, cow.y);


    };


    index+=1;
  }

}
