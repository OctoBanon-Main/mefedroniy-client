# Colored usernames

Usernames are displayed in different colors depending on the client the user is connected through. The color coding is as follows:

## Mefedroniy

This client

- Regex: `\u00B0\u0298<(.*?)> (.*)`
- Color: Light Magenta
- Example: `°ʘ<nick> text`

## bRAC

[GTK client on Rust](https://github.com/MeexReay/bRAC)

- Regex: `\uB9AC\u3E70<(.*?)> (.*)`
- Color: Green
- Example: `리㹰<nick> text`

## CRAB

[Client and server bundle on Java](https://gitea.bedohswe.eu.org/pixtaded/crab)

- Regex: `\u2550\u2550\u2550<(.*?)> (.*)`
- Color: Light Red
- Example: `═══<nick> text`

## Tower

GUI client on Rust with Tauri

- Regex: `\u25B2<(.*?)> (.*)`
- Color: White
- Example: `▲<nick> text`

## clRAC

Official client

- Regex: `<(.*?)> (.*)`
- Color: Cyan
- Example: `<nick> text`
