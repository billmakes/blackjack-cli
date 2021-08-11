mod game;
use game::Game;

mod graphics;
use graphics::print_game_state_with_hidden_card;
use graphics::prompt_for_user_action;
use graphics::print_game_end;
use graphics::print_game_state;
use graphics::sleep;

mod card;

pub fn main() {
    // create new Game
    let mut game: Game = Game::new();

    // display initial game state
    print_game_state_with_hidden_card(&game);

    // endgame condition breaks loop
    loop {
        // prompt user action
        let choice: String = prompt_for_user_action(&game);
        if choice == "d" {
            game.player_draw_card();
            print_game_state_with_hidden_card(&game);
            if game.player_score > 21 {
                print_game_end(&game, "❌ BUSTED ❌");
                break;
            }

        } else if choice == "s" {
            // check if dealer has won
            if game.dealer_score > game.player_score {
                print_game_end(&game, "❌ YOU LOST ❌");
                break;
            } else if game.dealer_score == game.player_score {
                print_game_end(&game, "✋ It's a DRAW ✋");
                break;
            } else {
                print_game_state(&game, "👀 Dealer revealer his hidden card...\n...\n...");
                while game.dealer_score < game.player_score {
                    // wait 1 sec
                    sleep();
                    // take another card and display
                    game.dealer_draw_card();
                    // check endgame conditions
                    if game.dealer_score > 21 {
                        print_game_end(&game, "🎉 YOU WON 🎉");
                        break;
                    } else if game.dealer_score == game.player_score {
                        print_game_end(&game, "✋ It's a DRAW ✋");
                        break;
                    } else if game.dealer_score > game.player_score {
                        print_game_end(&game, "❌ YOU LOST ❌");
                        break;
                    }
                    print_game_state(&game, "🃏 Dealer has grabbed another card... \n...\n...");
                }
                break;
            }
        } else {
            println!("(INVALID INPUT)");
        }
    }
}
