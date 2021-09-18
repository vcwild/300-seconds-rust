enum State {
  Start,
  Running { hp: u32 },
  GameOver(Animation),
  Undefined,
}

enum Animation {
  Running,
  Stopped,
}

fn main() {
  let mut state = State::Start;

  loop {
    match state {
      State::Start => {
        println!("O jogo começa");
        state = State::Running { hp: 100 };
      }
      State::Running { hp: 0 } => {
        println!("O jogador ficou sem vida :(");
        state = State::GameOver(Animation::Running);
      }
      State::Running { ref mut hp } => {
        println!("o jogador está correndo");
        *hp -= 50;
        println!("o jogador tomou dano, hp: {}", hp);
      }
      State::GameOver(Animation::Running) => {
        println!("~~ Animação do player morrendo ~~");
        state = State::GameOver(Animation::Stopped);
      }
      State::GameOver(Animation::Stopped) => break,

      _ => {
        println!("Movimento inesperado");
      }
    }
  }

  println!("Fim de jogo");
}
