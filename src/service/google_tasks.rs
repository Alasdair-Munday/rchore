use super::google_api::{format_specific_task_url, GoogleApiClient};
use crate::models::tasks::{TaskResponse, Tasks};

pub trait ApiTasks {
    fn fetch_all_tasks(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn fetch_task(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
    fn delete_task(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
    fn update_task(&self, updated_task: Tasks) -> Result<(), Box<dyn std::error::Error>>;
    fn clear_completed_tasks(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl ApiTasks for GoogleApiClient {
    fn fetch_all_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            String::from("tasks"),
        );
        let resp = self.client.get(url).send()?.json::<TaskResponse>()?;
        println!("{:#?}", resp);
        Ok(())
    }

    fn fetch_task(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            format!("{}/{}", "tasks", id),
        );
        let resp = self.client.get(url).send()?.json::<Tasks>()?;
        println!("{:#?}", resp);
        Ok(())
    }

    fn delete_task(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            format!("{}/{}", "tasks", id),
        );
        let resp = self.client.delete(url).send()?;
        println!("{:#?}", resp);
        Ok(())
    }

    fn update_task(&self, updated_task: Tasks) -> Result<(), Box<dyn std::error::Error>> {
        let id = &updated_task.id;
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            format!("{}/{}", "tasks", id.as_ref().unwrap()),
        );
        let resp = self
            .client
            .patch(url)
            .json(&updated_task)
            .send()?
            .json::<Tasks>()?;
        println!("{:#?}", resp);
        Ok(())
    }

    fn clear_completed_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format_specific_task_url(
            &self.base_url,
            String::from("/lists"),
            self.tasklist.as_ref().unwrap().to_string(),
            String::from("clear"),
        );
        let resp = self.client.post(url).send()?.json::<TaskResponse>()?;
        println!("{:#?}", resp);
        Ok(())
    }
}