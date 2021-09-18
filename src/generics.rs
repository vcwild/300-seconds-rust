struct World<T> {
  player: T,
}

impl<T: Damage> World<T> {
  fn damage_player(&mut self, damage: u32) {
    self.player.take_damage(damage);
  }
}

struct Player {
  hp: u32,
}

impl Damage for Player {
  fn take_damage(&mut self, damage: u32) {
    self.hp -= damage;
  }
}

struct DebugPlayer {
  inner: Player,
}

impl Damage for DebugPlayer {
  fn take_damage(&mut self, damage: u32) {
    println!("Damage: {}", damage);
    self.inner.take_damage(damage);
    println!("the player took tamage, hp: {}", self.inner.hp);
  }
}

trait Damage {
  fn take_damage(&mut self, damage: u32);
}

fn main() {
  let mut world = World {
    player: DebugPlayer {
      inner: Player { hp: 100 },
    },
  };

  world.damage_player(100);
}
