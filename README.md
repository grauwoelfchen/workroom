# Workroom

![build](https://gitlab.com/grauwoelfchen/workroom/badges/master/build.svg)

[Grauwoelfchen's Wookroom](https://grauwoelfchen.net/)

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

Copyright (C) 2014-2017 Yasuhiro Asaka

This is free software;
You can redistribute it and/or modify it under the terms of the GNU Affero General Public License (AGPL).

See `LICENSE`.
