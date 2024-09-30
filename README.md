This example shows a dioxus app which is first pre-rendered and the hydrated.

See: https://github.com/DioxusLabs/dioxus/issues/3020


To see the error build the project with:

```bash
dx build && cargo build --features prebuild  && simple-http-server ./dist
```

You can use whatever server you want but I like `simple-http-server`.

You will see that even though the output of `i18.translate("title")` is "Ahoj"
the website still shows "Hello".
