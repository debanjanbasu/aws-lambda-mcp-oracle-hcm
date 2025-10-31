# AWS Lambda MCP Oracle HCM Gateway

AWS Lambda MCP function with Bedrock AgentCore Gateway for Oracle HCM integration.

## Prerequisites

- AWS CLI configured
- SAM CLI installed
- Rust toolchain with cargo-lambda
- UPX (binary compressor)
- Microsoft Entra ID application (see [ENTRA_SETUP.md](ENTRA_SETUP.md))

## Deployment

```bash
./build.sh \
  EntraIdTenantId="<your-tenant-id>" \
  EntraIdClientId="<your-client-id>" \
  LambdaRoleArn="arn:aws:iam::<account-id>:role/<lambda-role-name>" \
  GatewayRoleArn="arn:aws:iam::<account-id>:role/<gateway-role-name>"
```

The build script automatically:
1. Builds the Rust Lambda function (ARM64)
2. Compresses the binary with UPX
3. Uploads tool_schema.json to S3
4. Deploys the CloudFormation stack

> **Note**: The Lambda function uses the `live` alias which automatically creates a new version on each deployment. The gateway always points to this alias, ensuring zero-downtime updates.

## Testing

### Client Credentials Flow (App-to-App)

For service-to-service authentication without user context:

```bash
curl -X POST "https://login.microsoftonline.com/{tenant-id}/oauth2/v2.0/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id={client-id}" \
  -d "client_secret={client-secret}" \
  -d "scope=api://{client-id}/.default" \
  -d "grant_type=client_credentials"
```

### On-Behalf-Of Flow (User Context)

To pass user details (email, name) to the Lambda:

```bash
# Exchange user token for OBO token
curl -X POST "https://login.microsoftonline.com/{tenant-id}/oauth2/v2.0/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id={client-id}" \
  -d "client_secret={client-secret}" \
  -d "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer" \
  -d "assertion={user-access-token}" \
  -d "requested_token_use=on_behalf_of" \
  -d "scope=api://{client-id}/.default"
```

> **Note**: OBO flow requires a user access token from initial user sign-in. The resulting token contains user claims.

Call the gateway:

```bash
curl -X POST "{gateway-url}/mcp/v1/tools/call" \
  -H "Authorization: Bearer <TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{"name": "get_absence_types_for_employee", "arguments": {}}'
```

## Architecture

- **Lambda Function**: Rust-based MCP server with ARM64 architecture
- **Versioning**: Uses `live` alias for atomic deployments and rollback capability
- **Gateway**: Bedrock AgentCore Gateway with Entra ID JWT authentication
- **Tool Schema**: Stored in S3 and referenced by the gateway

## Available Tools

- `get_absence_types_for_employee` - Get absence types for an employee
- `get_all_absence_balances_for_employee` - Get all absence balances
- `get_projected_balance` - Get projected balance for an absence type

## Documentation

- [Entra ID Setup](ENTRA_SETUP.md) - Configure Microsoft Entra ID authentication
