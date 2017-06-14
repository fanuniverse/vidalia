```bash
docker build -t vidalia .
docker run --rm -it -v "$(pwd)":/go/src/vidalia vidalia
```

Inside the container:

```bash
cd /go/src/vidalia

# Test
go test ./...

# Build and run
go build
./vidalia
```
