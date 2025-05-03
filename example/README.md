# 快速开始模板

复制目录快速开始。

## 使用方法

### 复制示例目录

要在自己的项目中使用这些示例，您需要复制整个目录：

```bash
cp /path/to/embedrs/example -r /path/to/embedrs/project
```

然后重命名目录内的示例：

```bash
mv /path/to/embedrs/project/example -r /path/to/embedrs/project/somename
```

### 修改配置字段

在使用示例之前，您需要修改以下文件中的关键字段：

1. 在 `Cargo.toml` 中：
    - 将 `<NAME>` 修改为该文件所在目录的命名

2. 在 `config.toml` 中：
    - 将 `<CHIP>` 修改为目标芯片
    - 将 `<TARGET_TRIPLE>` 修改目标平台的三元组

3. 在 `openocd.cfg` 中：
    - 将 `<CHIP>` 修改为目标芯片所代表的配置文件

4. 在 `launch.json` 中：
    - 将 `<CHIP>` 修改为目标芯片
    - 将 `<TARGET_TRIPLE>` 修改目标平台的三元组
