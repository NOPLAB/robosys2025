# b64

標準入力から受け取ったデータをBase64エンコードして標準出力に出力するCLIツールです。

## 概要

- Rustで実装されたBase64エンコーダー
- 外部クレートを使用せず、標準ライブラリのみで実装
- 標準入力からデータを読み込み、Base64エンコードした結果を標準出力に出力

## 使用方法

```bash
echo -n "Hello" | b64
# 出力: SGVsbG8=
```

```bash
cat image.png | b64 > image_base64.txt
```

## ビルド

```bash
cargo build --release
```

## ライセンスおよびコピーライト

- © 2025 nop
- このプロジェクトはMITライセンスの下で公開されています。詳細はLICENSEファイルをご覧ください。

