#[derive(Clone, Copy, Debug, Default)]
pub struct FocusPos { pub row: i32, pub col: i32 }


pub struct KeyboardService;


impl KeyboardService {
    pub fn new() -> Self { Self }
    pub fn on_key_down(&mut self, _key: &str) {
        unimplemented!()
    }
    pub fn set_focus(&mut self, _pos: FocusPos) {
        unimplemented!()
    }
    pub fn get_focus(&self) -> FocusPos { FocusPos::default() }
}