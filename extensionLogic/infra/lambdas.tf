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
  name               = "extensionHelloWorld_role"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json

  tags = local.tags
}

data "archive_file" "lambda_function_archive_js" {
  type        = "zip"
  source {
    content = file("../lambda/main.mjs")
    filename = "main.mjs"
  }

  output_path = "lambda_function_payload_js.zip"
}

resource "aws_lambda_function" "extension_lambda_js" {
  filename      = "lambda_function_payload_js.zip"
  function_name = "extensionHelloWorldJs"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "main.handler"

  source_code_hash = data.archive_file.lambda_function_archive_js.output_base64sha256

  runtime = "nodejs20.x"

  architectures = ["arm64"]

  layers = [aws_lambda_layer_version.lambda_layer.arn]

  tags = local.tags
}