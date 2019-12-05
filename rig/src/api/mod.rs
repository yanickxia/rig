#[derive(Clone, Debug, Deserialize, Builder)]
pub struct Api {
    pub id: i64,
    pub path: String,
    pub handlers: Vec<Definition>,
}

#[derive(Clone, Debug, Deserialize, Builder)]
pub struct Definition {
    pub filters: Vec<Filter>,
    pub dispatcher: Dispatcher,
}

#[derive(Clone, Debug, Deserialize)]
pub enum FilterType {
    Method,
    Header,
}

#[derive(Clone, Debug, Deserialize, Builder)]
pub struct Filter {
    pub name: FilterType,
    pub method: String,
    pub header: String,
    pub regular_expression: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum DispatcherType {
    Direct,
    ServiceDiscovery,
}

#[derive(Clone, Debug, Deserialize, Builder)]
pub struct Dispatcher {
    pub destination: String,
    pub name: DispatcherType,
}