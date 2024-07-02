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

invoke:
	$(eval TMP := $(shell mktemp))
	aws lambda invoke --function-name bashHelloWorld --payload '' $(TMP) --cli-binary-format raw-in-base64-out --profile=CloudboostSandbox --region=ap-southeast-2
	cat $(TMP); echo
	rm $(TMP)
