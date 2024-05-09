# DES by Rust
## 実行方法
```
$ cargo build
$ cargo run
```
実行ファイルは./target/debug/に生成されるので、それを直接実行しても良い
## オプション
- DEBUG_WHOLE : 各ラウンドの関数出力値および鍵の値を出力
- TEST_MODE : テスト用の関数が利用可能
- DEBUG_TRANS : 初期転置および最終転置のデバッグ
- DEBUG_ROUND : 各ラウンドの出力デバッグ
- DEBUG_ROUND_F : ラウンド内で実行するf関数のデバッグ
### 利用方法
- buildおよびrunの際に、--featuresオプションを追加
  ```
  $ cargo build --features DEBUG_WHOLE
  $ cargo run --features DEBUG_WHOLE
  ```
  複数の場合はダブルクォーテーションで囲む
  ```
  $ cargo build --features "TEST_MODE DEBUG_ROUND"
  $ cargo run --features "TEST_MODE DEBUG_ROUND"
  ```
- Cargo.tomlにてデフォルトのフラグを指定することも可能
  ```
  [features]
  default = ["DEBUG_WHOLE","TEST_MODE"]
  DEBUG_WHOLE = []
  TEST_MODE = []
  DEBUG_TRANS = []
  DEBUG_ROUND = []
  DEBUG_ROUND_F = []
  ```