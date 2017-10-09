# Workroom

[![pipeline status][pipeline]][commit]

[![Grauwoelfchen's Workroom][photo]][workroom]


The website of [Grauwoelfchen's Wookroom][workroom].


## Setup

```zsh
% ruby --version
ruby 2.3.1p112 (2016-04-26 revision 54768) [x86_64-linux]

% cp .env.sample .env
% bundle install --path .bundle/gems
```


## Development

See `Procfile`

```zsh
% bundle exec foreman start develp
```


## Test

```zsh
% bundle exec foreman run rake test
```


## License

```
Workroom
Copyright (C) 2014-2017 Yasuhiro Asaka
```

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


[pipeline]: https://gitlab.com/grauwoelfchen/workroom/badges/master/pipeline.svg
[commit]: https://gitlab.com/grauwoelfchen/workroom/commits/master
[photo]: https://dl.grauwoelfchen.net/_shared/photo/20171010/workroom-300x245-20171010.jpg
[workroom]: https://grauwoelfchen.net/
