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

data "archive_file" "lambda_function_archive_py" {
  type        = "zip"
  source {
    content = file("../lambda/main.py")
    filename = "main.py"
  }
  output_path = "lambda_function_payload_py.zip"
}

resource "aws_lambda_function" "proxy_lambda_js" {
  filename      = "lambda_function_payload_js.zip"
  function_name = "proxyHelloWorldJs"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "main.handler"

  source_code_hash = data.archive_file.lambda_function_archive_js.output_base64sha256

  runtime = "nodejs20.x"

  layers = [aws_lambda_layer_version.lambda_layer.arn]

  environment {
    variables = {
      AWS_LAMBDA_EXEC_WRAPPER = "/opt/wrapper"
    }
  }

  tags = local.tags
}

resource "aws_lambda_function" "proxy_lambda_py" {
  filename      = "lambda_function_payload_py.zip"
  function_name = "proxyHelloWorldPy"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "main.handler"

  source_code_hash = data.archive_file.lambda_function_archive_py.output_base64sha256

  runtime = "python3.12"

  layers = [aws_lambda_layer_version.lambda_layer.arn]

  environment {
    variables = {
      AWS_LAMBDA_EXEC_WRAPPER = "/opt/wrapper"
    }
  }

  tags = local.tags
}