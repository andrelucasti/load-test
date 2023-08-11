terraform {
  required_providers {
    docker = {
      source = "kreuzwerker/docker"
      version = "3.0.2"
    }
  }
}
provider "docker" {
  host = "unix:///var/run/docker.sock"
}

# postgres

# Pulls the image
resource "docker_image" "postgres" {
  name = "postgres:15"
}

# Creates the container
resource "docker_container" "postgres" {
  name  = "postgres"
  image = docker_image.postgres.image_id
    ports {
        internal = 5432
        external = 5432
    }
  env = ["POSTGRES_USER=load-test", "POSTGRES_PASSWORD=load-test", "POSTGRES_DB=load-test"]
}
