data "aws_iam_policy_document" "assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "iam_for_lambda" {
  name               = "jsGcHelloWorld_role"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

data "archive_file" "lambda" {
  type        = "zip"
  source_dir = "../src"
  output_path = "lambda_function_payload.zip"
}

data "archive_file" "lambda_layer_archive" {
  type        = "zip"
  source_dir = "../wrapper"
  output_path = "lambda_layer_payload.zip"
}

resource "aws_lambda_layer_version" "lambda_layer" {
    filename   = "lambda_layer_payload.zip"
    layer_name = "nodeGC_layer"

    compatible_runtimes = ["nodejs20.x"]

    compatible_architectures = ["x86_64", "arm64"]

    source_code_hash = data.archive_file.lambda_layer_archive.output_base64sha256
}

resource "aws_lambda_function" "js_gc_lambda" {
  filename      = "lambda_function_payload.zip"
  function_name = "jsGcHelloWorld"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "main.handler"

  source_code_hash = data.archive_file.lambda.output_base64sha256

  runtime = "nodejs20.x"

  layers = [aws_lambda_layer_version.lambda_layer.arn]

  environment {
    variables = {
      AWS_LAMBDA_EXEC_WRAPPER = "/opt/wrapper"
    }
  }
}