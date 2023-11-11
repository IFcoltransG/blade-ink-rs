use bladeink::story::Story;

trait IsSync: Sync {}
impl IsSync for Story {}
