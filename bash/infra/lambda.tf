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
  name               = "bashHelloWorld_role"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
  tags = local.tags 
}

data "archive_file" "lambda" {
  type        = "zip"
  source_dir = "../src"
  output_path = "lambda_function_payload.zip"
}

resource "aws_lambda_function" "bash_lambda" {
  filename      = "lambda_function_payload.zip"
  function_name = "bashHelloWorld"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "handler.handler"

  source_code_hash = data.archive_file.lambda.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["arm64"]

  layers = [aws_lambda_layer_version.lambda_layer.arn]

  tags = local.tags
}