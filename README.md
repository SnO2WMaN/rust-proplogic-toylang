# Toy language for Propositional Logic written in Rust

## Raison d'être

このプロジェクトでは，[Development Containers](https://containers.dev)の中に[Nix](https://nixos.org/)を入れて開発環境を構築しています．
これにより，**利用者がNixを自分のマシンに導入していなくても**，限りなく同じ開発環境が（あるいは手元で動かしてみる環境が）ほとんど何の労力無く展開可能であることを実現出来たと信じたいです．

## 開発環境の構築

### Dev Containerを介して開発環境を構築する

Nixを導入していなければこの方法を利用してください．

1. Dev Containerをサポートしている[何らかのツール](https://containers.dev/supporting)を導入する．ほとんどの方にとっては[VS Code Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)で良いでしょう．
1. このプロジェクトをクローンしてDev Containerとしてビルドし，中に入る．（手順は各々のツールの説明に従ってください．）
1. 以上．

### Dev Containerを介せず開発環境を構築する

Nixを導入しているなら，[Nix Flakesを有効化](https://nixos.wiki/wiki/Flakes#Enable_flakes)した上で，`devshell#default`に入ってください．
[direnv](https://github.com/direnv/direnv/wiki/Nix)を利用するのも良いでしょう．

### 開発環境の説明

Nixの設計上，開発環境内のシェルでは常に同じ環境が保たれるはずです．
すなわち，このドキュメントを書いた時点以降で更新していなければ，次のバージョン情報は**あなたの開発環境であったとしても**同じになるはずです．

```shell
$ rustc --version
rustc 1.67.1 (d5a82bbd2 2023-02-07)

$ cargo --version
cargo 1.67.1 (8ecd4f20a 2023-01-10)
```

その他は，標準的なRustの開発と同様に行える筈なので，それに従ってください．

### 成果物の利用

**Note**: 以下の内容は，GitHub Actions上で実際にビルド可能であるかをチェックし，成功していることを前提として記載しています．すなわち，Actionsが失敗している場合はおそらく出来ません．

[![ci.yml](https://github.com/SnO2WMaN/rust-proplogic-toylang/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/SnO2WMaN/rust-proplogic-toylang/actions/workflows/ci.yml?query=branch%3Amain)

（開発しないにしても）手元で簡単にビルドの成果物を試してみたいという方は，以下のコマンドを実行してください．`result/bin/rust-proplogic-toylang`に実行ファイルが生成される筈です．

```shell
$ nix build ".#default"
$ ./result/bin/rust-proplogic-toylang --help
```

あるいは，以下のコマンドで直接実行できます．これは上記と同じ実行結果となります．

```shell
$ nix run ".#default" -- --help
```

## 参考文献

- [puripuri2100/ralcu](https://github.com/puripuri2100/ralcu)
  - Rustについて全く書けなかったので参考にしました．
- [『数理論理学』, 戸次大介](https://www.utp.or.jp/book/b306383.html)
  - 命題論理のシンタクスについてはこれに沿っています．

## 命題論理について

以下はメモ

### シンタクス

```
Var       := p | q | r | s | t

Lfn0      := ⊤ | ⊥
Lfn0Apply := <Lfn0>()

Lfn1      := ¬
Lfn1Apply := <Lfn1>(<Form>)

Lfn2      := ∧ | ∨ | → | ↔
Lfn2Apply := <Lfn2>(<Form>,<Form>)

Form      := <Var> | <Lfn0Apply> | <Lfn1Apply> | <Lfn2Apply>
```
