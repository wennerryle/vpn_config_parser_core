# VPN Config Parser

автор(ы):

- Поздеев Владислав Евгеньевич (tg: @wennerryle) (email: wennerryle@gmail.com)

Требования:

- [Rust](https://rustup.rs/) (stable >= 1.75.0)
- [Python](https://www.python.org/downloads/) (>= 3.8)
- Git (необязательно если есть bash)

Запуск (bash / zsh / windows: git bash):

windows:

```bash
python -m venv .venv # skip this step if you already do it
source .venv/Scripts/activate
pip install -e . # skip this step if you already do it
maturin develop # add -r flag for release build (https://www.maturin.rs/tutorial)
py ./python
```

linux:

```bash
python3.12 -m venv .venv # skip this step if you already do it
source .venv/bin/activate
pip install -e .
maturin develop
python ./python
```

PS: Данный проект отноится к "mixed Rust/Python project" если следовать документации Maturin.
