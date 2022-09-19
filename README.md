# Workroom

[![pipeline status][pipeline]][commit]

[![Grauwoelfchen's Workroom][photo]][workroom]

The website of [Grauwoelfchen's Wookroom][workroom].


## Setup

```zsh
# use stable
% rustc --version
rustc 1.54.0 (a178d0322 2021-07-26)
```

## Build

```zsh
% make
% ./target/release/workroom
% xdg-open dst/index.html
```

```zsh
# debug
% make build:debug && ./target/debug/workroom && xdg-open ./dst/index.html
# release
% make build:release && ./target/release/workroom && xdg-open ./dst/index.html
```

## License

This project is distributed as multiple licenses by parts.

### Contents (text, image)

`GFDL-1.3`

This work is distributed as the
GNU Free Documentation License. (versior 1.3)

```txt
Permission is granted to copy, distribute and/or modify this document
under the terms of the GNU Free Documentation License, Version 1.3
or any later version published by the Free Software Foundation;
with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
A copy of the license is included in the section entitled "GNU
Free Documentation License".
```

Check the [GNU Free Documentation License](https://www.gnu.org/copyleft/fdl.html).

### Software (program)

`AGPL-3.0`

```txt
This is free software: You can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>
```

See [LICENSE](LICENSE).


```
Workroom
Copyright (C) 2014-2022 Yasuhiro Яша Asaka
```


[pipeline]: https://gitlab.com/grauwoelfchen/workroom/badges/trunk/pipeline.svg
[commit]: https://gitlab.com/grauwoelfchen/workroom/commits/trunk
[photo]: dst/img/workroom-300x245-20171010.jpg
[workroom]: https://grauwoelfchen.net/
