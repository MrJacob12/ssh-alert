<h1 align="center">ssh-alert</h1>

<div align="center">

[![GitHub Issues](https://img.shields.io/github/issues/MrJacob12/ssh-alert.svg)](https://github.com/MrJacob12/ssh-alert/issues)
[![License](https://img.shields.io/github/license/MrJacob12/ssh-alert.svg)](/LICENSE)

</div>

<!-- ## 📝 Table of Contents -->
<!-- -  -->
<!-- - [Deployment](#deployment) -->
<!-- - [Usage](#usage) -->
<!-- - [Getting Started](#getting_started)
- [Built Using](#built_using) -->

- [🧐 About ](#-about-)
- [🏁 Getting Started ](#-getting-started-)
  - [Installing](#installing)
- [⛏️ Built Using ](#️-built-using-)
- [📝 License](#-license)
- [✍️ Authors ](#️-authors-)

## 🧐 About <a name = "about"></a>

ssh alert is a simple tool to alert you when someone is trying to login to your server via ssh.
<img src="./s1.png">

## 🏁 Getting Started <a name = "getting_started"></a>

### Installing

```bash
git clone https://github.com/MrJacob12/ssh-alert
```

```rust
// Replace /fern/ssh-alert/main with your path to the binary
let lines = vec!["#!/bin/sh", "if [ ${PAM_TYPE} = \"open_session\" ]; then", "  /fern/ssh-alert/main $PAM_USER $PAM_RHOST $PAM_SERVICE $PAM_TTY `uname -a`", "fi", "exit 0"];

// Replace with your email from and to
let email = Message::builder()
.from("".parse().unwrap())
.to("".parse().unwrap())

// Set your user and password
let creds = Credentials::new("user".to_string(), "password".to_string());
let mailer = SmtpTransport::relay("smtp.gmail.com")
```

```bash
cargo build --release
```

## ⛏️ Built Using <a name = "built_using"></a>

- [Rust](https://www.rust-lang.org/)

## 📝 License

This project is [MIT](https://github.com/MrJacob12/ssh-alert/blob/master/LICENSE) licensed.

## ✍️ Authors <a name = "authors"></a>

- [@MrJacob12](https://github.com/mrjacob12)
