# Copycat

Copycat 是基于 [Kovi](https://kovi.thricecola.com/) 框架的自动复读插件，可以在群组内自动复读他人发言。  

## 安装

1. 根据[教程](https://kovi.thricecola.com/start/fast.html)创建一个 Kovi 工程
2. 在项目根目录运行
```bash
cargo add kovi-plugin-copycat
```

3. 在 `build_bot!` 宏中传入插件
```rust
let bot = build_bot!(kovi-plugin-copycat /* 和其他你正在使用的插件，用 , 分割 */ );
```

## 配置

Copycat 可以通过 `toml` 文件进行配置。如果配置文件不存在，则使用默认配置。  

```toml
# 默认配置

# [选填]
# 当有 N 个不同群友发出相同消息时，复读一次，不会多次复读同一消息。
# - 当设置为 1 时，每一条消息均会复读一次，因此强烈不建议设置为此数值！
# - 当设置为 0 时，不会复读任何消息。
repeat_after = 2   

# [选填]
# 在指定群聊中启用复读。如果没有此项，则在所有群聊中启用复读。
allow_groups = [123456789] 

```  

配置文件应放置于编译后与可执行文件同级的 `data/kovi-plugin-copycat/config.toml` 中。