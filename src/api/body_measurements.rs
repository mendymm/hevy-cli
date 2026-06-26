use super::{Client, Result};
use crate::models::body_measurements::{
    BodyMeasurement, ListBodyMeasurementsResponse, PutBodyMeasurement,
};

impl Client {
    pub fn list_body_measurements(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<ListBodyMeasurementsResponse> {
        let path = format!("/v1/body_measurements?page={page}&pageSize={page_size}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn get_body_measurement(&self, date: &str) -> Result<BodyMeasurement> {
        let path = format!("/v1/body_measurements/{date}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn create_body_measurement(&self, body: &BodyMeasurement) -> Result<()> {
        let json = serde_json::to_string_pretty(body)?;
        dbg!(&json);
        self.make_request("POST", "/v1/body_measurements", Some(&json), || {
            self.post("/v1/body_measurements").send(&json)
        })?;
        Ok(())
    }

    pub fn update_body_measurement(&self, date: &str, body: &PutBodyMeasurement) -> Result<()> {
        let path = format!("/v1/body_measurements/{date}");
        let json = serde_json::to_string_pretty(body)?;
        self.make_request("PUT", &path, Some(&json), || self.put(&path).send(&json))?;
        Ok(())
    }
}
