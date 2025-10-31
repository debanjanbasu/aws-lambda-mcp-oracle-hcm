#!/bin/bash
set -e

echo "Building Lambda function..."
cargo lambda build --release --arm64

echo "Compressing binary with UPX..."
upx --best --lzma target/lambda/aws-lambda-mcp/bootstrap

BUCKET=$(aws cloudformation describe-stacks --stack-name aws-sam-cli-managed-default --query 'Stacks[0].Outputs[?OutputKey==`SourceBucket`].OutputValue' --output text 2>/dev/null)

if [ -z "$BUCKET" ]; then
  echo "Error: SAM bucket not found. Run 'sam deploy --guided' first."
  exit 1
fi

echo "Uploading tool_schema.json to s3://${BUCKET}/"
aws s3 cp tool_schema.json "s3://${BUCKET}/tool_schema.json"

echo "Deploying stack..."
sam deploy --parameter-overrides ToolSchemaS3Uri="s3://${BUCKET}/tool_schema.json" "$@"
