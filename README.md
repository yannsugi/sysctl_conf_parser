# sysctl_conf_parser

## 概要
`sysctl_conf_parser`は、Linuxの`sysctl.conf`と同じ文法の任意のファイルをロードし、辞書型・Map等に格納するライブラリです。

## 使用方法

以下のコマンドを使用して、ファイルをパースできます。

```sh
cargo run -- <path>
```

### サンプル実行例
```sh
cargo run -- tests/source/sample_sysctl.conf
```

このコマンドは、`tests/source/sample_sysctl.conf`ファイルをパースし、結果を表示します。

### 入力例

#### 入力例1
```
endpoint = localhost:3000
debug = true
log.file = /var/log/console.log
```

#### 入力例2
```
endpoint = localhost:3000
# debug = true
log.file = /var/log/console.log
log.name = default.log
```

### 出力例

#### 入力例1の出力例
```
SysctlConf({
  "endpoint": String("localhost:3000"),
  "debug": String("true")
  "log": Map({
    "file": String("/var/log/console.log")
  }),
})
```

#### 入力例2の出力例
```
SysctlConf({
  "endpoint": String("localhost:3000")
  "log": Map({
    "file": String("/var/log/console.log"),
    "name": String("default.log")
  }),
})
```