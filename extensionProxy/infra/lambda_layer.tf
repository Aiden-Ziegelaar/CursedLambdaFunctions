data "archive_file" "lambda_layer_archive" {
    type        = "zip"
    output_path = "./lambda_layer_payload.zip"

    source_dir = "../build"
}

resource "aws_lambda_layer_version" "lambda_layer" {
    filename   = "lambda_layer_payload.zip"
    layer_name = "rustProxy_layer"

    compatible_runtimes = ["nodejs20.x", "python3.12"]

    compatible_architectures = ["x86_64", "arm64"]

    source_code_hash = data.archive_file.lambda_layer_archive.output_base64sha256
}