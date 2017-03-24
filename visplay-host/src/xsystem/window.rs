pub struct Window {
    id: u64,
}

impl Window {
    pub fn from_id<N>(id: N) -> Window
    where u64: From<N> {
        Window { id: u64::from(id) }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn show(&self) {
        unimplemented!()
    }

    pub fn hide(&self) {
        unimplemented!()
    }
}