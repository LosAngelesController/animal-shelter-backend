## About the code

This depends on the schema file, which tells it which tables to pull and config.json. The entire runtime is in Rust.

### configuration and setup

make a `config.json` file with your postgres configuration: 

```json
{
    "pghost": "127.0.0.1",
    "pguser": "bob",
    "pgpassword": "correcthorsebatterystaple",
    "pgdbname": "postgres"
}
```