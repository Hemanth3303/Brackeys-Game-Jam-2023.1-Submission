/*
Basic Idea
*Score goes down as time goes down.
*'Enemies' run away from player.
*Hit enemy to 'loose', ie, end game (and next harder level if i have time)
*If score==0, you actually loose 
 */

pub mod player;

use macroquad::prelude::*;
use player::Player;

fn window_config() -> Conf {
    return Conf {
        window_title: "Brackeys-Game-Jam-2023.1-Submission".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: 640,
        window_height: 480,
        ..Default::default()
    };
}

#[macroquad::main(window_config)]
async fn main() {
    let mut score: i32=100;
    let mut score_timer: f32=0.0;
    const PLAYER_SIZE: f32=30.0;
    let p1=Player::new(
        Vec2::new(screen_width()/2.0-PLAYER_SIZE/2.0, 
        screen_height()/2.0-PLAYER_SIZE/2.0), 
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE)
    );

    loop {
        //begin update
        score_timer+=get_frame_time()*10.0;
        if score_timer>5.0 {
            score-=1;
            score_timer=0.0;
        }
        //end update
        
        //begin render
        clear_background(BLACK);

        let score=score.to_string();

        draw_text("Score: ", 10.0, 20.0, 32.0, WHITE);
        draw_text(score.as_str(), 100.0, 20.0, 32.0, WHITE);
        p1.draw();

        next_frame().await
        //end render
    }
}