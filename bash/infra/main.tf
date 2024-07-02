terraform {
  required_providers {
    aws = {
      source = "hashicorp/aws"
      version = "5.56.0"
    }
    archive = {
      source = "hashicorp/archive"
      version = "2.4.2"
    }
  }
}

provider "aws" {
  region = "ap-southeast-2"
  profile = "CloudboostSandbox"
}
