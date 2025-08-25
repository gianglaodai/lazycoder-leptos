use crate::pages::components::datatable::core::data_source::PivotModel;

#[derive(Clone, Debug, Default)]
pub struct PivotService {
    model: PivotModel,
}

impl PivotService {
    pub fn new() -> Self {
        Self { model: PivotModel::default() }
    }
    /// Set the columns that will be used as pivot dimensions.
    pub fn set_pivot_cols(&mut self, cols: Vec<String>) {
        self.model.pivot_cols = cols;
    }
    /// Set the value columns that will be aggregated for each pivot.
    pub fn set_value_cols(&mut self, cols: Vec<String>) {
        self.model.value_cols = cols;
    }
    /// Get the current pivot model (for callers that need to issue queries).
    pub fn pivot_model(&self) -> PivotModel {
        self.model.clone()
    }
}
