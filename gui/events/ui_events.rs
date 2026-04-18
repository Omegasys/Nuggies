#[derive(Debug, Clone)]
pub enum UiEvent {
    Navigate(String),
    SearchChanged(String),
    SelectPackage(String),
    OpenSettings,
    OpenLogs,
    RefreshView,
}
