use embassy_executor::Spawner;

pub struct Runtime {
    pub spawner: Spawner,
}

impl Runtime{
    pub fn new(spawner: Spawner) -> Self {
        Self {
            spawner,
        }
    }
}
