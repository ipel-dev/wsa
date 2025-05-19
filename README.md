# WSA
### WebSocket API Spec

**This document defines the specification for a WebSocket-based API protocol. It applies to production environments, using compressed formats.**

## From

- Must be either `"server"` or a specific `client_id`
- If `from` is `server`, it's the only global server
- If not, it **must** be a valid `client_id`

## To

- Can be:

  - `"server"`: for messages directed to the server
  - `"client_id"`: when directed from a client to another client (via server proxy)
  - `"client"`: broadcast message to all clients (only allowed **from server**)

### Rules

- `client` can be the target (`to`) if the message originates from the `server`, or is routed **via** the server from another client
- A client **may send** a message with `to: "client"`, but it **must** be relayed through the server, which will handle authentication and authorization.

## Type

Only three values are allowed:

- `"request"`
- `"response"`
- `"event"`

## ID

- Message `id` is chosen by the sender
- It should be the ISO8601 timestamp at the moment of sending, e.g.: 

  ```
  "2025-05-14T11:27:31.536370717+08:00"
  ```

**This feature may be cancelled in the next version. At that time, it will be changed to a string of [a-z0-9]{5}, msg_id.**

- For `response` type messages:

  - `id` **must match** the original `request` or `event` message's ID
  - `response` messages **must not** use a new unique ID

## Request Message Structure

```json
{
  "from": "",
  "to": "",
  "type": "request",
  "id": "",
  "payload": {
    "method": "auth@v1/login",
    "params": {}
  }
}
```

- `method`: target service and versioned endpoint (e.g. `auth@v1/login`)
- `params`: arguments passed to backend service function

## Response Message Structure

```json
{
  "from": "",
  "to": "",
  "type": "response",
  "id": "",
  "payload": {
    "result": "success",
    "receipt": "optional_id_or_null"
  }
}
```

```json
{
  "from": "",
  "to": "",
  "type": "response",
  "id": "",
  "payload": {
    "result": "fail",
    "code": "PERMISSION_DENIED"
  }
}
```

## Event Message Structure

```json
{
  "from": "",
  "to": "",
  "type": "event",
  "id": "",
  "payload": {
    "method": "chat@v1/message",
    "params": {}
  }
}
```

---

# Production Format (Compressed)

## Message Structure

```json
{
  "f": "s",
  "t": "",
  "y": "g",
  "i": "abc12",
  "P": {}
}
```

### Payload for Request/Event

```json
{
  "p": {
    "m": "auth@v1/login",
    "p": ["arg1", "arg2"]
  }
}
```

### Payload for Successful Response

```json
{
  "p": {
    "r": "s",
    "c": "resource_id"
  }
}
```

### Payload for Failure Response

```json
{
  "p": {
    "r": "f",
    "c": "PERMISSION_DENIED"
  }
}
```

---

This format is optimized for bandwidth-sensitive environments by shortening keys and values. It is strongly recommended to use the verbose version during development and the compact version in production deployments.
