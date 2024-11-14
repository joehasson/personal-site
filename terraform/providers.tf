terraform {
    required_providers {
        aws = {
          source  = "hashicorp/aws"
          version = "~> 5.0"
        }
    }

    backend "s3" {
       bucket = "joe-hasson-portfolio-terraform-state"
       key    = "terraform.tfstate"
       region = "eu-west-2"
    }
}

provider "aws" {
  region = "eu-west-2"
}
