data "archive_file" "lambda_layer_archive" {
    type        = "zip"
    output_path = "./lambda_layer_payload.zip"

    source_dir = "../layer"
}

resource "aws_lambda_layer_version" "lambda_layer" {
    filename   = "lambda_layer_payload.zip"
    layer_name = "bash_jq_arm64"

    compatible_runtimes = ["provided.al2023"]

    compatible_architectures = ["arm64"]

    source_code_hash = data.archive_file.lambda_layer_archive.output_base64sha256
}