


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
  /*
  pub fn move_right(&mut self) {
    self.x = self.x + 1.0;
  }
  */

  fn moving(&mut self) {
    self.x += self.x_velocity;
    self.y += self.y_velocity;

    if self.x <= self.constraints.border || self.x >= self.constraints.width - self.constraints.border {
        self.x -= self.x_velocity;
        self.x = self.x.max(self.constraints.border);
        self.x = self.x.min(self.constraints.width - self.constraints.border);
        self.x_velocity = -self.x_velocity;
        self.x += self.x_velocity;
    }

    if self.y <= self.constraints.border || self.y >= self.constraints.height - self.constraints.border {
        self.y -= self.y_velocity;
        self.y = self.y.max(self.constraints.border);
        self.y = self.y.min(self.constraints.height - self.constraints.border);
        self.y_velocity = -self.y_velocity;
        self.y += self.y_velocity
    }

  }

  fn distance(&mut self, boid: Cow) -> f64 {
    let dist_x = self.x - boid.x;
    let dist_y = self.y - boid.x;
    (dist_x*dist_x + dist_y*dist_y).sqrt()
  }


  fn move_away(&mut self, boids: &Vec<Cow>, min_distance: f64) {
    let mut distance_x = 0.0;
    let mut distance_y = 0.0;
    let mut num_close = 0.0;

    for boid in boids.iter() {

      if boid.x == self.x && boid.y == self.y {
        continue;
      }

      let distance = self.distance(*boid);

      if distance < min_distance {
        num_close +=1.0;
        let mut xdiff = self.x - boid.x;
        let mut ydiff = self.y - boid.y;

        if xdiff >= 0.0 {
          xdiff = min_distance.sqrt() - xdiff;
        } else if xdiff < 0.0 {
          xdiff = -min_distance.sqrt() - xdiff;
        }

        if ydiff >= 0.0 {
          ydiff = min_distance.sqrt() - ydiff;
        } else if ydiff < 0.0 {
          ydiff = -min_distance.sqrt() - ydiff;
        }

        distance_x += xdiff;
        distance_y += ydiff;

      }

      if num_close == 0.0 {

      } else {
        self.x_velocity -= distance_x / 5.0;
        self.y_velocity -= distance_y / 5.0;
      }
    }
  }



}

fn main() {
  println!("Hello, world!");

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



  let mut cows_list: Vec<Cow> = vec![];

  //let mut cows_list: Vec<Cow> = Vec::new();


  let mut i = 0;

  while i < 5 {
      let cow = Cow {
        id:i,
        x:10.0,
        y:10.0,
        constraints: constraints,
        x_velocity: 1.0,
        y_velocity: -1.0
      };
      i = i + 1;
      cows_list.push(cow)
  }

  let mut index = 0;


  while index < 5 {

    for cow in cows_list.iter_mut() {
      let former_x = cow.x;
      let former_y = cow.y;

      cow.move_away(*cows_list, 15.0);
      cow.moving();

      println!("{}", cow.distance(bob));

      println!("{}: {} {}", cow.id, cow.x, cow.y);


    };


    index+=1;
  }

}
