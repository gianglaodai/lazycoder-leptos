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

pub fn aggregate_rows<T>(rows: &[T], cols: &[(&str, AggregateFn)]) -> AggMap {
    let mut map = AggMap::new();
    for (id, agg) in cols.iter() {
        let v = match agg {
            AggregateFn::Count => rows.len() as f64,
            // Without knowledge of how to extract numeric values from T generically,
            // we default other aggregations to 0.0. This can be extended when value_getters are wired.
            AggregateFn::Sum
            | AggregateFn::Avg
            | AggregateFn::Min
            | AggregateFn::Max
            | AggregateFn::Custom => 0.0,
        };
        map.insert((*id).to_string(), v);
    }
    map
}
