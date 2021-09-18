struct Rect {}
struct Circle {}

trait Drawable {
  fn draw(&self);

  fn describe(&self) {
    println!("Você vê um Drawable");
  }
}

impl Drawable for Rect {
  fn draw(&self) {
    println!("Desenhando um ratângulo");
  }

  fn describe(&self) {
    println!("Você vê um retângulo")
  }
}

impl Drawable for Circle {
  fn draw(&self) {
    println!("Desenhando um círculo");
  }

  fn describe(&self) {
    println!("Você vê um círculo")
  }
}

impl Drawable for String {
  fn draw(&self) {
    println!("{}", self);
  }
}

fn main() {
  let rect = Rect {};
  let circle = Circle {};
  let string = String::from("olha só essa string bonita");

  Drawable::draw(&rect);

  let shapes: Vec<Box<dyn Drawable>> = vec![Box::new(rect), Box::new(circle), Box::new(string)];

  for shape in shapes {
    shape.draw();
    shape.describe();
  }
}
