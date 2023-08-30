# schedule-rs

## Description
This is a simple RESTful API for scheduling HTTP requests. It is built using Rust 
language and Actix framework.

***The service is under heavy development, below features may not be available until it is done***

## Usage
### Create a schedule
To create a schedule, send a POST request to `/schedule` with the following body:
```json
{
    "id": "ec3eee49-f876-4ceb-a112-9dc33251e506",
    "request": {
        "url": "https://example.com",
        "method": "GET",
        "body": "Hello world!",
        "headers": {
            "Content-Type": "text/plain"
        }
    },
    "schedule": "0 0 1 1 *"
}
```

The `schedule` field is a cron expression. The above example will send a request to
`https://example.com` with `Content-Type: text/plain` header and `Hello world!` body
on every 1st of January at 00:00.

To schedule one time HTTP request, use `schedule_at` field with ISO 8601 format:
```json
{
    "id": "87a2df69-f049-46e8-8202-b73ebbc54fda",
    "request": {
        "url": "https://example.com",
        "method": "GET",
        "body": "Hello world!",
        "headers": {
            "Content-Type": "text/plain"
        }
    },
    "schedule_at": "2021-01-01T00:00:00Z"
}
```

### Get all schedules
To get all schedules, send a GET request to `/schedule`.

```json
[
    {
        "id": "87a2df69-f049-46e8-8202-b73ebbc54fda",
        "request": {
            "url": "https://example.com",
            "method": "GET",
            "body": "Hello world!",
            "headers": {
              "Content-Type": "text/plain"
            }
        },
        "schedule_at": "2021-01-01T00:00:00Z",
        "status": "scheduled"
    },
    {
        "id": "ec3eee49-f876-4ceb-a112-9dc33251e506",
        "request": {
            "url": "https://example.com",
            "method": "GET",
            "body": "Hello world!",
            "headers": {
                "Content-Type": "text/plain"
            }
        },
        "schedule": "0 0 1 1 *",
        "status": "scheduled"
    }
]
```

### Get a schedule
To get a schedule, send a GET request to `/schedule/{id}`. The `id` is the schedule's
ID. Response body will be the schedule's JSON.
```json
{
    "id": "ec3eee49-f876-4ceb-a112-9dc33251e506",
    "request": {
        "url": "https://example.com",
        "method": "GET",
        "body": "Hello world!",
        "headers": {
          "Content-Type": "text/plain"
        }
    },
    "schedule": "0 0 1 1 *",
    "status": "scheduled"
}
```

status can be `scheduled`, `running`, `failed` or `done`.

`failed` and `done` schedules contains additional info about execution.
```json
{
    "id": "ec3eee49-f876-4ceb-a112-9dc33251e506",
    "url": "https://example.com",
    "method": "GET",
    "body": "Hello world!",
    "headers": {
        "Content-Type": "text/plain"
    },
    "schedule": "0 0 1 1 *",
    "status": "failed",
    "last_error": "Failed to connect to host",
    "executed_at": "2021-01-01T00:00:00Z"
}
```

or done

```json
{
    "id": "ec3eee49-f876-4ceb-a112-9dc33251e506",
    "url": "https://example.com",
    "method": "GET",
    "body": "Hello world!",
    "headers": {
        "Content-Type": "text/plain"
    },
    "schedule": "0 0 1 1 *",
    "status": "done",
    "executed_at": "2021-01-01T00:00:00Z"
}
```


Once done, the schedule will be deleted automatically from the database
when retention policy is met. Default retention policy is 30 days.

### Delete a schedule
To delete a schedule, send a DELETE request to `/schedule/{id}`.

### Update a schedule

To update a schedule, send a PUT request to `/schedule/{id}` with the following body:
```json
{
    "request": {
        "url": "https://example.com",
        "method": "GET",
        "body": "Hello world!",
        "headers": {
            "Content-Type": "text/plain"
        }
    },
    "schedule": "0 0 1 1 *"
}
```

# Callbacks
To get notified when a schedule is executed, you can use callback URL. Callback URL
will be sent a POST request with the following body:
```json
{
    "id": "ec3eee49-f876-4ceb-a112-9dc33251e506",
    "request": {
        "url": "https://example.com",
        "method": "GET",
        "body": "Hello world!",
        "headers": {
            "Content-Type": "text/plain"
        }
    },
    "schedule": "0 0 1 1 *",
    "status": "done",
    "executed_at": "2021-01-01T00:00:00Z"
}
```

To set callback URL, send a PUT request to `/schedule/{id}/callback` with the following body:
```json
{
    "callback": {
      "url": "https://example.com/callback",
      "headers": {
        "Content-Type": "application/json",
        "Authorization": "Bearer token"
      }
    }
}
```
Or set `callback` field when creating or updating a schedule.

# Authentication
To authenticate API calls, you can use API key. To create an API key, and use it
in each request in `X-API-Key` header.

# Database
This service uses Sled as its database. It is a simple embedded database written in Rust.
Copy of the database is stored in `data` directory. You can change the directory by
setting `SCHEDULERS_DB_PATH` environment variable. The database will be replicated
to all schedule-rs nodes in the cluster.

# Configuration
You can configure the service by setting the following environment variables:
- `SCHEDULERS_DB_PATH`: Path to the database directory. Default: `data`
- `SCHEDULERS_RETENTION_POLICY`: Retention policy in days. Default: `30`
- `SCHEDULERS_PORT`: Port to listen to. Default: `8080`
- `SCHEDULERS_HOST`: Host to listen to. Default: machine's hostname
- `SCHEDULERS_API_KEY`: API key to authenticate API calls. Default: `""` this will change
  in the future to support cloud deployment.
- `SCHEDULERS_CALLBACK_TIMEOUT`: Timeout for callback request in seconds. Default: `10`
- `SCHEDULERS_CALLBACK_RETRY_INTERVAL`: Interval between callback retries in seconds. Default: `1, 5, 30`. 
   This also sets number of retries. The above example will retry 3 times with interval of 1, then 5 and 
   finally 30 seconds. Repeating schedule will be executed again after the last retry regardless of previous 
   schedule execution status.

