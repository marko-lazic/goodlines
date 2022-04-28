use common::data::USERNAMES;
use rand::seq::SliceRandom;

pub fn random_user_credentials() -> (&'static str, &'static str) {
    let username = USERNAMES.choose(&mut rand::thread_rng()).unwrap();
    (username, "12345")
}
