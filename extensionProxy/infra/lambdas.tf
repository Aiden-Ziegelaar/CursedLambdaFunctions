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
  name               = "proxyHelloWorld_role"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

data "archive_file" "lambda_function_archive" {
  type        = "zip"
  source_dir = "../lambda"
  output_path = "lambda_function_payload.zip"
}

resource "aws_lambda_function" "proxy_lambda" {
  filename      = "lambda_function_payload.zip"
  function_name = "proxyHelloWorld"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "main.handler"

  source_code_hash = data.archive_file.lambda_function_archive.output_base64sha256

  runtime = "nodejs20.x"

  layers = [aws_lambda_layer_version.lambda_layer.arn]

  environment {
    variables = {
      AWS_LAMBDA_EXEC_WRAPPER = "/opt/wrapper"
    }
  }
}

resource "aws_lambda_function" "no_proxy_lambda" {
  filename      = "lambda_function_payload.zip"
  function_name = "noProxyHelloWorld"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "main.handler"

  source_code_hash = data.archive_file.lambda_function_archive.output_base64sha256

  runtime = "nodejs20.x"
}