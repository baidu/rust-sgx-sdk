#!/usr/bin/env bash

set -e

DOCKERFILE_1604_NIGHTLY=Dockerfile.1604.nightly
DOCKERFILE_1804_NIGHTLY=Dockerfile.1804.nightly
DOCKERFILE_2004_NIGHTLY=Dockerfile.2004.nightly
DOCKERFILE_centos8_NIGHTLY=Dockerfile.centos8.nightly
DOCKERFILE_fedora31_NIGHTLY=Dockerfile.fedora31.nightly

IMAGE_1604_NIGHTLY=baiduxlab/sgx-rust:1604-1.1.4
IMAGE_1804_NIGHTLY=baiduxlab/sgx-rust:1804-1.1.4
IMAGE_2004_NIGHTLY=baiduxlab/sgx-rust:2004-1.1.4
IMAGE_centos8_NIGHTLY=baiduxlab/sgx-rust:centos8-1.1.4
IMAGE_fedora31_NIGHTLY=baiduxlab/sgx-rust:fedora31-1.1.4

push_one() {
	docker push $1
}

push_one ${IMAGE_1604_NIGHTLY}
push_one ${IMAGE_1804_NIGHTLY}
push_one ${IMAGE_2004_NIGHTLY}
push_one ${IMAGE_centos8_NIGHTLY}
push_one ${IMAGE_fedora31_NIGHTLY}

docker tag ${IMAGE_1804_NIGHTLY} baiduxlab/sgx-rust:latest
push_one baiduxlab/sgx-rust:latest
