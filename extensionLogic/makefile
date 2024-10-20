tf-init:
	terraform -chdir=./infra init

tf-plan:
	terraform -chdir=./infra plan

tf-apply:
	terraform -chdir=./infra apply --auto-approve

tf-destroy:
	terraform -chdir=./infra destroy --auto-approve

build-x86_64:
	mkdir -p build
	pushd extension && cargo-lambda lambda build --release && popd
	cp ./extension/target/lambda/extension/bootstrap ./build/bootstrap_x86_64

build-aarch64:
	mkdir -p build
	pushd extension && cargo-lambda lambda build --release --arm64 && popd
	cp ./extension/target/lambda/extension/bootstrap ./build/bootstrap_aarch64

clean: 
	rm -rf build

build: clean build-x86_64 build-aarch64
	mkdir -p build/extensions
	cp ./extension/opt/entrypoint ./build/extensions/rustLogic

build-apply: build tf-apply