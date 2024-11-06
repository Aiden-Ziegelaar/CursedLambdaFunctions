make-executable:
	find ./src -type d -exec chmod +x {} \;

tf-init:
	terraform -chdir=./infra init

tf-plan:
	terraform -chdir=./infra plan

tf-apply: make-executable
	terraform -chdir=./infra apply --auto-approve

tf-destroy:
	terraform -chdir=./infra destroy --auto-approve

build:
	mkdir -p layer
	curl -L -o ./layer/jq https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-linux-arm64
	chmod +x ./layer/jq