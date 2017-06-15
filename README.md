# vidalia

## Development

```bash
docker-compose run vidalia
```

Inside the container:

```bash
cd /go/src/vidalia

# Test
go test ./test

# Build and run
go build
./vidalia
```

## RabbitMQ

Once the containers are up, you can access RabbitMQ's management UI
by visiting `http://localhost:15672`.
