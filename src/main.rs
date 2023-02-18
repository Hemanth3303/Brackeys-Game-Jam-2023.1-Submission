
use macroquad::prelude::*;
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
    let mut score: i32=1;
    let mut score_timer: f32=0.0;
    let mut dt;
    let mut lost=false;
    let mut level_count=1;
    const BOX_WIDTH: f32=400.0;
    const BOX_HEIGHT: f32=60.0;
    let box_x: f32=screen_width()/2.0-BOX_WIDTH/2.0;
    let box_y: f32=screen_height()/2.0-BOX_HEIGHT/2.0;
    let mut level_name: String="Level ".to_string();
    let mut max_score_per_level=score+50*level_count/2;
    level_name+=&level_count.to_string();

    loop {
        if !lost {
            //begin update
            dt=get_frame_time();
            score_timer+=dt*10.0;
            if score_timer>0.5 {
                score+=1;
                score_timer=0.0;
            }
            if score <=0 {
                score=50*level_count/2;
                max_score_per_level=score+50*level_count/2;
                level_count+=1;
                level_name="Level ".to_string();
                level_name+=&level_count.to_string();
            }

            if score >=max_score_per_level {
                lost=true;
            }

            if mouse_position().0>box_x && mouse_position().0<box_x+BOX_WIDTH
            && mouse_position().1>box_y && mouse_position().1<box_y+BOX_HEIGHT {
                if is_mouse_button_pressed(MouseButton::Left) {
                    score-=5;
                }
            }

            //end update
            
            //begin render
            clear_background(BLACK);

            let score=score.to_string();

            draw_text("Score: ", 10.0, 20.0, 32.0, WHITE);
            draw_text(score.as_str(), 100.0, 20.0, 32.0, WHITE);
            draw_text(score.as_str(), 100.0, 20.0, 32.0, WHITE);
            draw_text("/", 140.0, 20.0, 32.0, WHITE);
            draw_text(max_score_per_level.to_string().as_str(), 170.0, 20.0, 32.0, WHITE);
            draw_text(&level_name, screen_width()/2.0-55.0, screen_height()/4.0-20.0, 32.0, WHITE);

            draw_rectangle(box_x, box_y, BOX_WIDTH, BOX_HEIGHT, BLUE);
            draw_text("Click Me TO Decrease Score", box_x, box_y+40.0, 32.0, WHITE);

            //end render
        }
        else {
            draw_text("You Lost Idiot", box_x+100.0, box_y+20.0, 32.0, WHITE);
            draw_text("Score: ", box_x+120.0, box_y+60.0, 32.0, WHITE);
            draw_text((level_count*10).to_string().as_str(), box_x+220.0, box_y+60.0, 32.0, WHITE);
        }
        next_frame().await
    }
}