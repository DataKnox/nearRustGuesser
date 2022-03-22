use near_sdk::collections::{UnorderedMap};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    log,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault, setup_alloc
};
use near_sdk::{env, near_bindgen};
use near_rng::{Rng};

setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum GameState { Created, InProgress, Completed, NotFound }

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    id: u32,
    gameState: GameState,
    player1: AccountId,
    player1Guess: u32,
    compGuess: u32,
    winner: String,
    loser: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Games {
    records: UnorderedMap<u32, Game>,
}

#[near_bindgen]
impl Games {
    #[init]
    pub fn new()-> Self{
        Self {
            records: UnorderedMap::new(b"m".to_vec()),
          }
    }

    pub fn create_game(&mut self){
        let mut rng = Rng::new(&env::random_seed());
        let mut value = rng.rand_range_u32(0, u32::MAX);
        log!("{}",value.to_string());
        let mut compRNG = Rng::new(&env::random_seed());
        let mut compuGuess = rng.rand_range_u32(1, 10);
        let strHelper = "unk";
        let newgame = self.records.insert(
            &value,
            &Game {
                id: value,
                gameState: GameState::Created,
                player1: env::signer_account_id(),
                compGuess: compuGuess,
                loser: strHelper.to_string(),
                winner: strHelper.to_string(),
                player1Guess: 1          
            }
        );
    }

    pub fn make_guess(&mut self, guess: u32, gameId: u32){
        let mut game = self.records.get(&gameId).expect("ERR_GAME_NOT_FOUND");
        if guess == game.compGuess {
            game.gameState = GameState::Completed;
            game.player1Guess = guess;
            game.winner = env::signer_account_id();
            log!("Puzzle solved. Computer guessed {}", game.compGuess);
            self.records.insert(&gameId, &game);
        } else {
            game.gameState = GameState::Completed;
            game.player1Guess = guess;
            game.loser = env::signer_account_id();
            log!("Game failed. Computer guessed {}", game.compGuess);
            self.records.insert(&gameId, &game);
        }
    }    
}

