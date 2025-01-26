Requirements:

- `rustc 1.84.0` or above

install from [here](https://www.rust-lang.org/tools/install)


Configurations:

Please read the [docs](./docs) folder present in the repo to get instructions on how to configure the env to your email.

How to use:

start the server using

```bash
cargo run
```

we can also use nodemon to rebuild on incremental changes (if you want to change something)

```bash
    npx nodemon --exec "cargo run" ./src/main.rs
```

To send emails using the API, you can use the following `curl` command:

```bash
curl --location 'http://localhost:8080/send-emails' \
--form 'file=@"/path to emails.csv"'
```

Replace `"/path to emails.csv"` with the actual path to your CSV file containing the email addresses.

or you can copy the above cURL and paste in postman, attach the file there and then send the email
