pub enum PinSide { Left, Right }


pub struct PinService;


impl PinService {
    pub fn new() -> Self { Self }
    pub fn pin(&mut self, _col_id: &str, _side: PinSide) {
        unimplemented!()
    }
    pub fn unpin(&mut self, _col_id: &str) {
        unimplemented!()
    }
}