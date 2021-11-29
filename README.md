# docker image seeker

A project that aims to detect new docker image versions to rebuild our own images when outdated.
It will check the last pushed date of an image on a repository (ex: `debian:bullseye` on dockerhub), and compare the last successful pipeline
that ran on our side and that uses the base image (ex: `cicd-tools:base` extends `debian:bullseye`).

The application will then prompt the images that we have to rebuild:
```shell
>>>>>>> Do not refresh image for cicd tools - debian based images on project id 13658
>>>>>>> Do not refresh image for cicd tools - golang based images on project id 13658
>>>>>>> Do not refresh image for cicd tools - golang 1.17.1-alpine3.14 based images on project id 13658
```

## How to use
### Configuration
#### environment variables
| variable | description |
| --- | --- |
| GITLAB_TOKEN | A token being used to retrieve information from gitlab |

#### config.json
In order to work correctly, the application requires:
- **name**: a human-readable identifier to know which project is concerned by the configuration
- **registry**: the registry from which the base image is being pulled
- **project_id**: the identifier of the gitlab project in which the image is being used as base image
- **branch**: the branch on which to check the build
```json
[
  {
    "name": "cicd tools - debian based images",
    "registry": "https://hub.docker.com/v2/repositories/library/debian/tags/bullseye",
    "project_id": "13658",
    "branch": "master"
  },
  {
    "name": "cicd tools - golang based images",
    "registry": "https://hub.docker.com/v2/repositories/library/golang/tags/latest",
    "project_id": "13658",
    "branch": "master"
  },
  {
    "name": "cicd tools - golang 1.17.1-alpine3.14 based images",
    "registry": "https://hub.docker.com/v2/repositories/library/golang/tags/1.17.1-alpine3.14",
    "project_id": "13658",
    "branch": "master"
  }
]
```

### How to run
Simply execure `cargo run`.
