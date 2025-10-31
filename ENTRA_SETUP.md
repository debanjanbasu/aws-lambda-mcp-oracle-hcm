# Microsoft Entra ID Setup for AWS Lambda MCP Gateway

## Prerequisites
- Access to Azure Portal with permissions to register applications
- Tenant ID from your organization

## Step 1: Register Application

1. Navigate to **Azure Portal** → **Microsoft Entra ID** → **App registrations**
2. Click **New registration**
3. Configure:
   - **Name**: `aws-lambda-mcp-gateway` (or your preferred name)
   - **Supported account types**: Accounts in this organizational directory only (Single tenant)
   - **Redirect URI**: Leave blank (not needed for API authentication)
4. Click **Register**

## Step 2: Configure Token Version (CRITICAL)

1. In your app registration, go to **Manifest**
2. Find the line: `"accessTokenAcceptedVersion": null,`
3. Change to: `"accessTokenAcceptedVersion": 2,`
4. Click **Save**

> **Why this matters**: The gateway uses v2.0 tokens. Without this setting, Entra ID issues v1.0 tokens that will be rejected.

## Step 3: Configure Authentication

1. Go to **Authentication** → **Advanced settings**
2. Set:
   - **Allow public client flows**: NO
   - **Access tokens**: YES
   - **ID tokens**: YES (if using ID tokens)

## Step 4: Create Client Secret

1. Go to **Certificates & secrets** → **Client secrets**
2. Click **New client secret**
3. Add description: `Gateway authentication`
4. Set expiration (recommended: 12-24 months)
5. Click **Add**
6. **IMPORTANT**: Copy the secret value immediately (shown only once)

## Step 5: Collect Required Values

From the **Overview** page, note:

```
Tenant ID (Directory ID): xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
Client ID (Application ID): yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy
Client Secret: <value from Step 4>
```

Discovery URL:
```
https://login.microsoftonline.com/{tenant-id}/v2.0/.well-known/openid-configuration
```

> **Important**: 
> - The discovery URL must include `/v2.0/` to match the token version configured in Step 2.
> - The gateway validates that the `aud` (audience) claim in JWT tokens matches the Application ID.
> - When requesting tokens, the `aud` claim must be set to the Application ID (Client ID).

## Verify Token Version

After configuration, decode a test token at [jwt.ms](https://jwt.ms) and verify:
- `"iss"` claim contains `/v2.0` → ✅ Correct
- `"iss"` claim is `https://sts.windows.net/...` → ❌ Wrong version (check manifest)

## Next Steps

See [README.md](README.md) for deployment instructions.

## Optional: Expose API Scopes

For fine-grained access control:

1. Go to **Expose an API** → **Add a scope**
2. Set Application ID URI: `api://{client-id}`
3. Add scope:
   - **Scope name**: `access_gateway`
   - **Who can consent**: Admins and users
   - **Display name**: Access MCP Gateway
   - **Description**: Allows access to Oracle HCM MCP tools

## Optional: Enable On-Behalf-Of Flow

To pass user context (email, name) to the Lambda:

1. Go to **API permissions** → **Add a permission**
2. Select **Microsoft Graph** → **Delegated permissions**
3. Add: `User.Read`, `email`, `profile`, `openid`
4. Click **Grant admin consent**
5. Go to **Token configuration** → **Add optional claim**
6. Select **Access** token type
7. Add claims: `email`, `preferred_username`, `name`
8. Check "Turn on the Microsoft Graph email, profile permission"

> **Note**: With OBO flow, your application exchanges a user token for a new token that includes user claims.

## Reference

- [AWS Bedrock AgentCore - Microsoft Identity Provider](https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/identity-idp-microsoft.html)
