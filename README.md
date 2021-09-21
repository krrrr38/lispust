# lispust

toy lisp-ish language

## cmd

```sh
> cargo run -p lispust-cmd
```

## http-server

```sh
> cargo build -p lispust-http --bin http-server
```

```
> curl -XPOST -d'(+ 3 5)' localhost:8080/lispust
Number(8)
```

## grpc-server

```sh
> export OUR_DIR=proto-rs
> cargo run -p lispust-grpc --bin grpc-server
```

```sh
> cargo run -p lispust-grpc --bin grpc-client "(+ 3 5)"
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/grpc-client '(+ 3 5)'`
Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Mon, 20 Sep 2021 22:01:08 GMT", "grpc-status": "0"} }, message: RunResponse { message: "Number(8)" }, extensions: Extensions }
message = "Number(8)"
```
