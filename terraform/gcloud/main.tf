terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "3.5.0"
    }

    helm = {
      source  = "hashicorp/helm"
      version = "2.6.0"
    }

    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.11.0"
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

  ip_allocation_policy {
    cluster_ipv4_cidr_block       = "10.76.0.0/14"
    services_secondary_range_name = "gke-cluster-tp-cicd-pods-f80d1a41"
  }
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

resource "google_service_account" "service-account-gh-action" {
  account_id   = "gh-action-sa"
  display_name = "GitHub Action Service Account"
}

resource "kubernetes_namespace" "kube_prometheus_stack_namespace" {
  metadata {
    name = "kube-prometheus"
  }
}

resource "helm_release" "kube_prometheus_stack_release" {
  name      = "kube-prometheus-stack"
  namespace = kubernetes_namespace.kube_prometheus_stack_namespace.metadata[0].name

  chart  = "../../helm/kube-prometheus-stack/chart"
  values = ["${file("../../helm/kube-prometheus-stack/staging.values.yml")}"]
}
