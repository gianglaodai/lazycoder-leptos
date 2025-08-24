
pub struct GroupService;

impl GroupService {
    pub fn new() -> Self {
        Self
    }
    pub fn set_group_cols(&mut self, _cols: Vec<String>) {
        unimplemented!()
    }
    pub fn group_rows<T>(&self, _rows: Vec<T>, _cols: &[String]) -> GroupResult<T> {
        GroupResult {
            flat: Vec::new(),
            tree: Vec::new(),
        }
    }
}

pub struct GroupResult<T> {
    pub flat: Vec<T>,
    pub tree: Vec<GroupNode<T>>,
}

pub struct GroupNode<T> {
    pub key: String,
    pub children: Vec<GroupNode<T>>,
    pub rows: Vec<T>,
}
