#[derive(Clone, Debug, Deserialize)]
pub struct Api {
    pub id: i64,
    pub path: String,
    pub destination: Dispatcher,
}

impl Api {
    pub fn new(id: i64, path: String, method: String) -> Self {
        Api {
            id,
            path,
            destination: Dispatcher::new(""),
        }
    }

    pub fn builder() -> ApiBuilder {
        ApiBuilder::default()
    }
}

pub struct ApiBuilder {
    pub dispatcher: String,
    pub id: i64,
    pub path: String,
}

impl Default for ApiBuilder {
    fn default() -> Self {
        ApiBuilder {
            dispatcher: "".to_string(),
            id: 0,
            path: "".to_string(),
        }
    }
}

impl ApiBuilder {
    pub fn dispatcher(&mut self, dispatcher: &str) -> &mut Self {
        self.dispatcher = dispatcher.to_string();
        self
    }

    pub fn path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.id = id;
        self
    }

    pub fn finish(&self) -> Api {
        Api {
            id: self.id,
            path: self.path.clone(),
            destination: Dispatcher::new(self.dispatcher.as_str()),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Dispatcher {
    pub destination: String,
}

impl Dispatcher {
    pub fn new(dest: &str) -> Self {
        Dispatcher {
            destination: dest.to_string()
        }
    }
}
