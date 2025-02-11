# Mefedroniy
[<img src="https://github.com/user-attachments/assets/f2be5caa-6246-4a6a-9bee-2b53086f9afb" height="30">]()
[<img src="https://github.com/user-attachments/assets/4d35191d-1dbc-4391-a761-6ae7f76ba7af" height="30">]()

**Mefedroniy — TUI клиент для Real Address Chat (RAC).**

[English](README.md) | [Русский](lang/README.ru.md)

Mefedroniy — это TUI клиент, совместимый с протоколом Real Address Chat (RAC), созданный человеком с ником Mr. Sugoma и позиционируемый как "IRC Killer", предназначенный для текстового общения через терминал.

## Требования

Перед сборкой проекта убедитесь, что у вас установлены следующие компоненты:

- [Rust](https://www.rust-lang.org/tools/install) (последняя стабильная версия)

## Шаги для сборки

1. Клонируйте репозиторий:

Сначала клонируйте репозиторий с помощью следующей команды:

```bash
git clone https://github.com/OctoBanon-Main/mefedroniy-client.git
```

2. Затем перейдите в директорию проекта:

```bash
cd mefedroniy-client
```

3. Соберите проект:

Для компиляции проекта выполните следующую команду:
```bash
cargo build --release
```

Скомпилированный исполнимый файл будет доступен в директории `/target/release/`.

Кроме того, вы можете скачать уже скомпилированный бинарный файл с страницы [Releases](https://github.com/OctoBanon-Main/mefedroniy-client/releases).

## Лицензия

Этот проект лицензирован под Unlicense.

## См. также

- [Цветные юзернеймы](https://github.com/OctoBanon-Main/mefedroniy-client/blob/main/docs/colored_usernames.ru.md) — Информация о поддержке цветных имен пользователей в чате.
- [RAC protocol v1.99.2](https://gitea.bedohswe.eu.org/pixtaded/crab#rac-protocol) — Информация о протоколе.
- [RAC protocol v1](https://bedohswe.eu.org/text/rac/protocol.md.html) — Первая версия протокола.
- [bRAC](https://github.com/MeexReay/bRAC) — Клиент написанный на Rust.
- [CRAB](https://gitea.bedohswe.eu.org/pixtaded/crab) — Неофицальный клиент и сервер на Java.
- [AlmatyD](https://gitea.bedohswe.eu.org/bedohswe/almatyd) — Неофициальная серверная реализация.
