# sysctl_conf_parser

## 概要
`sysctl_conf_parser`は、Linuxの`sysctl.conf`と同じ文法の任意のファイルをロードし、辞書型・Map等に格納するライブラリです。

## 使用方法

以下のコマンドを使用して、ファイルをパースできます。

```sh
cargo run -- <sysctl_conf_path> <sysctl_conf_schema_path>
```

このコマンドは、指定された`sysctl.conf`ファイルとスキーマファイルを読み込み、パース結果を表示します。

### サンプル実行例
```sh
cargo run -- tests/source/sample_sysctl.conf tests/source/sample_sysctl.schema
```

このコマンドは、`tests/source/sample_sysctl.conf`ファイルをパースし、結果を表示します。
なお、valueが`tests/source/sample_sysctl.schema`の定義された型と一致しない場合、エラーになります。

### Schemaの定義仕様

Schemaファイルは、各設定項目のキーとその値の型を定義します。以下の形式で記述します。
なお、Schemaファイルに定義されていないキーは、`string`として扱います。

```
<key> -> <type>
```

サポートされている型は以下の通りです:
- `string`: 文字列
- `bool`: 真偽値
- `integer`: 整数
- `float`: 浮動小数点数

#### Schemaの例
```
net.ipv4.ip_forward -> integer
net.ipv4.conf.all.rp_filter -> bool
kernel.hostname -> string
vm.swappiness -> float
endpoint -> string
```

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