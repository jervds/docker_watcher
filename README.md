# docker image seeker

A project that aims to detect new docker image versions to rebuild our own images when outdated.
It will check the last pushed date of an image on a repository (ex: `debian:bullseye` on dockerhub), and compare the last successful pipeline
that ran on our side and that uses the base image (ex: `cicd-tools:base` extends `debian:bullseye`).

The application will then prompt the images that we have to rebuild:
```shell
>>>>>>> Do not refresh image for my_project - debian based images on project id 13658
>>>>>>> Do not refresh image for my_project - golang based images on project id 13658
>>>>>>> Do not refresh image for my_project - golang 1.17.1-alpine3.14 based images on project id 13658
```

## How to use
### Configuration
#### environment variables
| variable | description                                                                                    |
| --- |------------------------------------------------------------------------------------------------|
| GITLAB_TOKEN | A token being used to retrieve information from gitlab                                         |
| GITLAB_API | Defines the API url of your gitlab instance                                                    |
| RUST_LOG | Defines the logging level ( see [env_log](https://docs.rs/env_logger/0.9.0/env_logger/) crate) |

#### config.json
In order to work correctly, the application requires:
- **name**: a human-readable identifier to know which project is concerned by the configuration (for logging purpose only)
- **image**: the image on which our local image is based
- **project_id**: the identifier of the gitlab project in which the image is being used as base image
- **branch**: the branch on which to check the build
```json
[
  {
    "name": "cicd tools - debian based images",
    "image": "debian:bullseye",
    "project_id": "13658",
    "branch": "master"
  },
  {
    "name": "mirror: confluentinc/cp-kafka:latest",
    "image": "confluentinc/cp-kafka:latest",
    "project_id": "91008",
    "branch": "main"
  }
]
```

### How to run
Simply execute `cargo run`.

### How to run tests
Simply execute `cargo test`.