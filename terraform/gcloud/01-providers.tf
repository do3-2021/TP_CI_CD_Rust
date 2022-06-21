provider "google" {
  credentials = file(var.credentials)

  project = var.project

  zone = var.zone
}

provider "helm" {
  kubernetes {
    config_path    = var.kubeconfig_path
    config_context = var.kubeconfig_context
  }
}

provider "kubernetes" {
  config_context = var.kubeconfig_context
  config_path    = var.kubeconfig_path
}
