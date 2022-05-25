terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "3.5.0"
    }
  }
}

resource "google_compute_network" "vpc" {
  name                    = "${var.project}-vpc"
  auto_create_subnetworks = false
}

resource "google_compute_subnetwork" "cluster-subnet" {
  name          = "${var.project}-subnet"
  region        = var.region
  network       = google_compute_network.vpc.name
  ip_cidr_range = "10.10.0.0/24"
}

resource "google_container_cluster" "cluster-tp-cicd" {
  name     = "cluster-tp-cicd"
  location = var.cluster-location

  remove_default_node_pool = true
  initial_node_count       = 1
}

resource "google_container_node_pool" "node-pool-tp-cicd" {
  name       = "node-pool-tp-cicd"
  location   = var.cluster-location
  cluster    = google_container_cluster.cluster-tp-cicd.name
  node_count = 1

  node_config {
    machine_type = "e2-medium"
  }
}
