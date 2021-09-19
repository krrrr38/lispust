# lispust

toy lisp-ish language

## http-server

```sh
> cargo run -p http-server
```

Now you can post from other tty
```
> curl -XPOST -d'(+ 3 5)' localhost:8080/lispust                        (git)-[master] -
Number(8)
```
