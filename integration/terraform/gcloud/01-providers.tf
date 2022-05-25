provider "google" {
  credentials = file(var.credentials)

  project = var.project

  zone = var.zone
}
