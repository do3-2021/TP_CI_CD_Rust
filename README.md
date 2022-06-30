# TP CI/CD Rust
## Simon LUCIDO, Maxime FOUCHER, Alexandre GOMEZ

This is a sample application written in Rust with database interaction to test GitHub CI/CD pipelines and observability.

## Setup environment

### GCP

To setup the base infrastructure with Google Cloud Platform you'll need a GCP environment with the GKE API enabled.

First setup a service account with the `proprietary` rights, it should **only** be used for terraform scripts.
Then extract the SA key in `json` format and put it wherever you want (just be sure to change the `credentials` variable).
Last run `terraform init` to install all providers and `terraform apply` to deploy all the needed tools for the CI/CD to work.

### Digital Ocean

Terraform files are located at `/terraform/digital`.
It will create a Kubernetes cluster with a Digital Ocean droplet as a worker in fra1 region.

```bash
terraform init
terraform plan --var "do_token=<your-token>"
terraform apply -auto-approve --var "do_token=<your-token>"
```

> The prometheus/grafana stack will not be deployed.

## How the application works

#### Installation

```bash
git clone https://github.com/do3-2021/TP_CI_CD_Rust.git
cd TP_CI_CD_Rust/
cargo install
```

#### Run the application

```bash
export CITY_DB_URL="localhost/database"
export CITY_DB_USER="postgres"
export CITY_DB_PASSWORD="postgres"

cargo run
```

## How the CI/CD works

We have multiple pipelines to run the application.

First, we have the `main` pipeline located at `/.github/workflows/ci.yaml`.

The pipeline are configured to run on the main branch and will run the following steps:

- git checkout
- determine version with [GitVersion](https://github.com/GitTools/GitVersion)
- docker login
- docker build
- docker push

After all the steps are done, the pipeline will trigger the deployment on all the environments.

### Deploy to gcloud

This step connects to the gke cluster and runs `helm upgrade` on the new bumped chart.

## How the Terraform scripts works

### gcloud

This script :

- Deploy a GKE cluster and a compute network to run the app
- Create a node pool to host the kubernetes cluster
- Deploy a stack with prometheus, grafana and basic metrics for kubernetes and a predefined configuration to watch metrics from the application
