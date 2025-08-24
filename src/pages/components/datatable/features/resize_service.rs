pub struct ResizeService;


impl ResizeService {
    pub fn new() -> Self { Self }
    pub fn begin_resize(&mut self, _col_id: &str, _start_x: i32) {
        unimplemented!()
    }
    pub fn update_resize(&mut self, _current_x: i32) {
        unimplemented!()
    }
    pub fn end_resize(&mut self) {
        unimplemented!()
    }
}