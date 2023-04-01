## About the code

This depends on the schema file, which tells it which tables to pull and config.json. The entire runtime is in Rust.

### configuration and setup

make a `config.json` file with your postgres configuration: 

```json
{
    "pghost": "127.0.0.1",
    "pguser": "bob",
    "pgpassword": "correcthorsebatterystaple",
    "pgdbname": "postgres",
    "pgport": "5432"
}
```

### How to run this in Google Cloud

There's a workflow file that runs on push.

You'll need to add the following secrets into your repo

- `PGCLIENTKEY`: base64 encoded client-key.pem encoded from Google Cloud SQL
- `PGCLIENTCERT`: base64 encoded client-cert.pem encoded from Google Cloud SQL
- `PGSERVERCA`: base64 encoded server-ca.pem encoded from Google Cloud SQL

- `CONFIGJSON`: Raw config.json file from above
- `CLOUD_RUN_PROJECT_NAME`: The project name of your Google Cloud Project
- `SQL_INSTANCE_NAME`: The connection name of your Google Cloud SQL
- `CLOUD_RUN_SERVICE_ACCOUNT_EMAIL`: Service Account of the email
- `secrets.CLOUD_RUN_SERVICE_ACCOUNT`: Raw Json File of the service account key