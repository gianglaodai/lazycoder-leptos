pub struct MoveService;


impl MoveService {
    pub fn new() -> Self { Self }
    pub fn begin_drag(&mut self, _col_id: &str, _start_x: i32) {
        unimplemented!()
    }
    pub fn update_drag(&mut self, _current_x: i32) {
        unimplemented!()
    }
    pub fn drop_at(&mut self, _target_index: usize) {
        unimplemented!()
    }
}