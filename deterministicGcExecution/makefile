tf-init:
	terraform -chdir=./infra init

tf-plan:
	terraform -chdir=./infra plan

tf-apply:
	terraform -chdir=./infra apply --auto-approve

tf-destroy:
	terraform -chdir=./infra destroy --auto-approve

build-x86_64:
	env GOOS=linux GOARCH=x86_64 go build ./src/main.go
	cp ./main ./build/main_x86_64

build-aarch64:
	env GOOS=linux GOARCH=arm64 go build ./src/main.go
	cp ./main ./build/main_aarch64