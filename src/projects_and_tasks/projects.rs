use crate::projects_and_tasks::{
    list_with_names::ListWithNames,
    project::{Project, ProjectWithTasks},
    project_error::ProjectError,
    task::Task,
};

#[derive(Debug, PartialEq)]
pub struct Projects {
    projects: Vec<ProjectWithTasks>,
}

impl ListWithNames<ProjectWithTasks> for Projects {
    fn items(&self) -> std::slice::Iter<ProjectWithTasks> {
        self.projects.iter()
    }
}

impl Projects {
    pub fn get_project_with_task(
        &self,
        project_string: &str,
        task_string: &str,
    ) -> Result<(Project, Task), ProjectError> {
        let project_with_tasks = self.find(project_string).map_err(ProjectError::Project)?;
        let project = Project::new(project_with_tasks);
        let task = project_with_tasks.find_task(task_string)?;
        Ok((project, task.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects_and_tasks::{
        project::ProjectWithTasksBuilder, task::TaskBuilder, tasks::TasksBuilder,
    };

    #[test]
    fn it_finds_project_and_task() {
        let task_to_be_found = TaskBuilder::new()
            .with_name("Some special task I want to find".to_string())
            .build();
        let project_to_be_found = ProjectWithTasksBuilder::new()
            .with_name("Some specific Project I want to find".to_string())
            .with_tasks(
                TasksBuilder::new()
                    .with_tasks(vec![
                        TaskBuilder::new()
                            .with_name("Another task".to_string())
                            .build(),
                        task_to_be_found.clone(),
                    ])
                    .build(),
            )
            .build();
        let projects = ProjectsBuilder::new()
            .with_projects(vec![
                ProjectWithTasksBuilder::new()
                    .with_name("Another project".to_string())
                    .build(),
                project_to_be_found.clone(),
            ])
            .build();

        let (project, task) = projects
            .get_project_with_task("specific project", "special task")
            .unwrap();

        assert_eq!(project, Project::new(&project_to_be_found));
        assert_eq!(task, task_to_be_found);
    }
}

pub struct ProjectsBuilder {
    projects: Vec<ProjectWithTasks>,
}

impl ProjectsBuilder {
    pub fn new() -> Self {
        Self::empty()
    }

    pub fn empty() -> Self {
        Self { projects: vec![] }
    }

    pub fn with_projects(mut self, projects: Vec<ProjectWithTasks>) -> Self {
        self.projects = projects;
        self
    }

    pub fn build(self) -> Projects {
        Projects {
            projects: self.projects,
        }
    }
}
