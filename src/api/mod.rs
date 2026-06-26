use ureq::{
    Agent, Body, Error, RequestBuilder,
    http::Response,
    typestate::{WithBody, WithoutBody},
};

pub mod body_measurements;
pub mod exercise_history;
pub mod exercise_templates;
pub mod routine_folders;
pub mod routines;
pub mod user;
pub mod workouts;

const BASE_URL: &str = "https://api.hevyapp.com";
const HOST: &str = "api.hevyapp.com";

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Client {
    api_key: String,
    agent: Agent,
    pub debug: bool,
}

impl Client {
    pub fn new(api_key: impl Into<String>, debug: bool) -> Self {
        Self {
            api_key: api_key.into(),
            debug,
            agent: ureq::Agent::config_builder()
                .http_status_as_error(false)
                .build()
                .new_agent(),
        }
    }

    fn get(&self, path: &str) -> RequestBuilder<WithoutBody> {
        self.agent
            .get(&format!("{BASE_URL}{path}"))
            .header("api-key", &self.api_key)
    }

    fn post(&self, path: &str) -> RequestBuilder<WithBody> {
        self.agent
            .post(&format!("{BASE_URL}{path}"))
            .header("api-key", &self.api_key)
            .header("content-type", "application/json")
    }

    fn put(&self, path: &str) -> RequestBuilder<WithBody> {
        self.agent
            .put(&format!("{BASE_URL}{path}"))
            .header("api-key", &self.api_key)
            .header("content-type", "application/json")
    }

    /// Executes a request via the provided closure, printing the HTTP/1.1 wire format first
    /// when --debug is set. On non-2xx responses, prints status, headers, and body as an error.
    pub(crate) fn make_request<F>(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
        f: F,
    ) -> Result<Response<Body>>
    where
        F: FnOnce() -> std::result::Result<Response<Body>, Error>,
    {
        if self.debug {
            eprintln!("{method} {path} HTTP/1.1");
            eprintln!("Host: {HOST}");
            eprintln!("api-key: {}", self.api_key);
            if body.is_some() {
                eprintln!("Content-Type: application/json");
            }
            eprintln!();
            if let Some(b) = body {
                eprintln!("{b}");
            }
        }

        let mut resp = f()?;

        if resp.status().is_success() {
            return Ok(resp);
        }

        let status = resp.status();
        let headers = resp
            .headers()
            .iter()
            .map(|(k, v)| format!("{k}: {}", v.to_str().unwrap_or("<binary>")))
            .collect::<Vec<_>>()
            .join("\n");
        let body = resp.body_mut().read_to_string()?;
        Err(format!("HTTP {status}\n{headers}\n\n{body}").into())
    }
}
