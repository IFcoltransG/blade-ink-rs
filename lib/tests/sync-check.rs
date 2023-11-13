use bladeink::story::Story;

trait IsSync: Sync {}
// compile-time assertion that Story implements Sync
impl IsSync for Story {}
