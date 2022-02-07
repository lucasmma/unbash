
mod domain;
mod infra;

use crate::domain::model::argument;
use whoami;

use crate::infra::bash_manager::BashManager;

fn main() {
    let bash = BashManager{ username: whoami::username() };
    bash.run();
}
