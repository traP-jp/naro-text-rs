# 環境構築 (Windows)

[[toc]]

:::warning
コマンドは手入力ではなく、コピー & ペーストで入力してください。  
手入力だと写し間違いの可能性があります。  
この際、1 行ずつコピーするようにしてください。
:::

## 事前準備

::: tip
質問をするときにはできるだけスクリーンショットを貼るようにしましょう。テキストだけで説明しても解決に必要な情報を全て伝えるのは難しいです。

`Windowsキー+Shift+s`を押すと、矩形選択でスクリーンショットが撮れます。 traQ のメッセージ入力欄に`Ctrl + V`で貼り付けられます。
:::

### WSL の導入

すでに WSL をインストールしている方はこの手順を飛ばして大丈夫です。

WSL は Windows 上で Linux を動かすための仕組みで、`Windows Subsystem for Linux`の略です。

以下のページの Step 1 を行ってください。 Step 2 以降は行わなくて大丈夫です。

https://pg-basic.trap.show/text/chapter-0/enviroment/windows.html#step-1-install-wsl

## VSCode の導入

すでに VSCode をインストールしている方はこの手順を飛ばして大丈夫です。

以下のサイトから`windows`の VSCode のインストーラーをダウンロードして、それを実行してインストールしてください。

https://code.visualstudio.com/download

### 拡張機能の導入

VSCode は拡張機能により様々な言語でのプログラミングをラクにできます。  
次回以降に使うものも最初にまとめて導入しておきましょう。

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - Rust 言語で書いたコードをチェックしてくれたり、プログラムを書くときに補完 (予測変換のような機能) を使えるようになったりします。
- [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)
  - コードの書き方をチェックしてくれます。
- [Prettier - Code formatter](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)
  - コードのフォーマットを整えてくれます。保存時に自動で実行されるような設定をしておくと便利です。
- [Vue Language Features (Volar)](https://marketplace.visualstudio.com/items?itemName=vue.volar)
  - VSCode の Vue3 向けの統合プラグイン。  

インストールが終わったら、反映させるために VSCode を 1 度閉じて開きなおしてください。

## Rust のインストール

Rust をインストールします。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

このコマンドを実行すると、Rust のインストーラーが起動します。  
選択肢を求められたら、 `Enter` を押してください。

インストールが終わったら、一度ターミナルを再起動してください。
その後、以下のコマンドを実行して、Rust のバージョンを確認してください。

```bash
rustc --version
```

`rustc 1.82.0 (f6e511eec 2024-10-15)` のように、バージョン番号が表示されればインストール完了です。

## Go と Task のインストール

ここでは、Go というプログラミング言語の導入をします。

``` bash
sudo apt install tar git
wget https://go.dev/dl/go1.22.3.linux-amd64.tar.gz
sudo rm -rf /usr/local/go && sudo tar -C /usr/local -xzf go1.22.3.linux-amd64.tar.gz
echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.profile
source ~/.profile
```

ここまでで、以下のコマンドを実行して

```bash
go version
```

`go version go.1.22.3`と表示されればインストール完了です。
ここまでできれば、次は以下のコマンドも実行して Task のインストールをしてください。

```sh
go install github.com/go-task/task/v3/cmd/task@latest
```

:::info 詳しく知りたい人向け。

**`Task`って何だ。**

Task は、Go で動いているタスクランナーです。これによって長いコマンドを短くできたり、複数のコマンドを 1 回で実行できたりと、開発においてとても便利なツールです。テンプレートリポジトリに`Taskfile.yaml`というファイルがありますが、このファイルによってコマンドの設定をしています。公式ドキュメントは英語しかありませんが、興味のある人は目を通してみてください。

Task 公式ドキュメント [https://taskfile.dev/](https://taskfile.dev/)

Task GitHub [https://github.com/go-task/task](https://github.com/go-task/task)

:::

## asdf の導入

asdf とは、1 つのプログラムの複数のバージョンを PC 内で管理できるようにするものです。
それ以外にもあとからバージョンを更新するのが容易にもなるので長期的に見るとオススメです。

[公式資料](https://asdf-vm.com/#/core-manage-asdf)

以下のコマンドにより asdf を導入します。

``` bash
sudo apt install git
git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.14.0
echo '. $HOME/.asdf/asdf.sh' >> ~/.bashrc
echo '. $HOME/.asdf/completions/asdf.bash' >> ~/.bashrc
source ~/.bashrc
```

## Node.jsの導入

Vue を使うために、Node.js を入れます。
この講習会では、クライアントサイドを Vue を用いて制作します。

```bash
asdf plugin add nodejs
asdf install nodejs latest
asdf global nodejs latest
```

これで、デフォルトで現在出ている最新のバージョンが適用されるようになりました。

ここで、インストールが正常にできているかを確認します。

```bash
node -v
```

を実行して、バージョン番号が表示されれば OK。

## Docker Desktopのインストール

https://www.docker.com/products/docker-desktop/  
上のリンクからそれぞれの OS にあったものをダウンロードしてインストールしてください。

### WSL2の追加設定 - WSL Backend の有効化

1. 右上の歯車アイコンから `Resources` => `WSL Integration` に移動する。
2. `Enable integration with my default WSL distro`にチェックを入れる。
3. 下に出てくる Distro をすべて有効化する。
4. 最後に、右下の `Apply & Restart` をクリックして設定は完了です。

![WSL Integration](./images/setup-wsl-backend.png)

## Postmanのインストール

[Postman | API Development Environment](https://www.getpostman.com/) は GUI で HTTP リクエストを行えるアプリケーションです。

[ダウンロードページ](https://www.postman.com/downloads/)
