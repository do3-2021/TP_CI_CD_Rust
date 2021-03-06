variable "credentials" {
  default = "service-account-key.json"
}

variable "project" {
  default = "boxwood-charmer-347912"
}

variable "zone" {
  default = "europe-west1-b"
}

variable "region" {
  default = "europe-west1"
}

variable "cluster-location" {
  default = "europe-west1-b"
}

variable "kubeconfig_path" {
  default = "~/.kube/config"
}

variable "kubeconfig_context" {
  default = "gke_boxwood-charmer-347912_europe-west1-b_cluster-tp-cicd"
}
