# [<img src="https://github.com/OctoBanon-Main/mefedroniy-client/blob/main/assets/logo.png?raw=true" height="30">]() Mefedroniy Client

[<img src="https://github.com/user-attachments/assets/f2be5caa-6246-4a6a-9bee-2b53086f9afb" height="30">]()
[<img src="https://github.com/user-attachments/assets/4d35191d-1dbc-4391-a761-6ae7f76ba7af" height="30">]()

**Mefedroniy — TUI client for Real Address Chat (RAC).**

[English](README.md) | [Русский](lang/README.ru.md)

Mefedroniy is a TUI client compatible with the Real Address Chat (RAC) protocol, created by Mr. Sugoma and marketed as the "IRC Killer", designed for text-based communication via the terminal.

## Screenshot

![Screenshot of the Mefedroniy Client with visible chat messages](https://github.com/OctoBanon-Main/mefedroniy-client/blob/main/misc/win_client_screenshot.png?raw=true)

## Prerequisites

Before building the project, ensure you have the following installed:

[Rust](https://www.rust-lang.org/tools/install) (latest stable version)

## Steps to Build

1. Clone the repository:

First, clone the project repository to your local machine:

```bash
git clone https://github.com/OctoBanon-Main/mefedroniy-client.git
```

Then, navigate to the project directory:

```bash
cd mefedroniy-client
```

2. Build the project:

To compile the project, run the following command:

```bash
cargo build --release
```

The compiled executable will be available in the `/target/release/` directory.

Alternatively, you can download the precompiled binary from the [Releases](https://github.com/OctoBanon-Main/mefedroniy-client/releases) page.

### RAC Resources Comparison

| Project | Maintainer | Type | Key Features |
|---------|------------|------|-------------|
| [Forbirdden's RAC Hub](https://the-stratosphere-solutions.github.io/RAC-Hub/) | Forbirdden | 🏛️ Original Hub | • Complete client/server list<br>• Protocol documentation<br>• First public catalog |
| [MeexReay's RAC Hub](https://meexreay.github.io/RAC-Hub/) | MeexReay | 🛠️ Community Fork | • Same core data as Forbirdden's<br>• Alternative categorization |
| [Racinfo](https://racinfo.kostyazero.com/) | kostya-zero | 🚀 Independent Project | • Modern documentation approach<br>• Next.js-powered interface |

**Legend:**
- 🏛️ **Original Hub** - The first complete reference
- 🛠️ **Community Fork** - Community-improved version  
- 🚀 **Independent** - Fully separate implementation

## License

This project is licensed under the MIT License.

## See also

- [bRAC](https://github.com/MeexReay/bRAC) - Another rust client for RAC.
- [CRAB](https://gitea.bedohswe.eu.org/pixtaded/crab) - Client and server for RAC on Java.
- [cRACk](https://github.com/pansangg/cRACk) - TUI client on Python.
- [AlmatyD](https://gitea.bedohswe.eu.org/bedohswe/almatyd) - Unofficial server implementation for RAC v1.0.
