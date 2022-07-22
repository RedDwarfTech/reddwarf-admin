# reddwarf-admin

## Startup

```bash
git clone https://github.com/jiangxiaoqiang/reddwarf-admin.git
# default compile
cargo build
# if the official package download slow
# switch to the mirror address to speed up the build
RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup cargo build
# compile for linux
RUSTUP_DIST_SERVER=https://static.rust-lang.org rustup target add x86_64-unknown-linux-gnu
cargo build --target=x86_64-unknown-linux-gnu
```
