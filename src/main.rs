mod entity;
mod game;

use crate::game::Game;

fn main() {
    Game::create(1080, 640).run();
}