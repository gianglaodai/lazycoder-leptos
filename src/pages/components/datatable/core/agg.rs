use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum AggregateFn {
    Sum,
    Avg,
    Min,
    Max,
    Count,
    Custom,
}

pub type AggMap = HashMap<String, f64>;

pub fn aggregate_rows<T>(_rows: &[T], _cols: &[(&str, AggregateFn)]) -> AggMap {
    AggMap::new()
}
