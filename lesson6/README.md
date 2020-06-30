# Substrate Node Template

A new FRAME-based Substrate node, ready for hacking.

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

## Run

### Single Node Development Chain

Purge any existing developer chain state:

```bash
./target/release/node-template purge-chain --dev
```

Start a development chain with:

```bash
./target/release/node-template --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

Optionally, give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet).

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`) by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

## Advanced: Generate Your Own Substrate Node Template

A substrate node template is always based on a certain version of Substrate. You can inspect it by
opening [Cargo.toml](Cargo.toml) and see the template referred to a specific Substrate commit(
`rev` field), branch, or version.

You can generate your own Substrate node-template based on a particular Substrate
version/commit by running following commands:

```bash
# git clone from the main Substrate repo
git clone https://github.com/paritytech/substrate.git
cd substrate

# Switch to a particular branch or commit of the Substrate repo your node-template based on
git checkout <branch/tag/sha1>

# Run the helper script to generate a node template.
# This script compiles Substrate and takes a while to complete. It takes a relative file path
#   from the current dir. to output the compressed node template.
.maintain/node-template-release.sh ../node-template.tar.gz
```

Noted though you will likely get faster and more thorough support if you stick with the releases
provided in this repository.

### lesson6作业题:

#### 一、补完剩下的代码

                  1. 新增代码修改



​       ![新增小猫修改代码](.\assest\新增小猫修改代码.jpg)

2. 转移小猫代码

 ![转移小猫代码](.\assest\转移小猫代码.jpg)

3. 测试删除小猫代码

​     ![测试删除小猫](.\assest\测试删除小猫.jpg)



#### 二、对比pallet-asset和pallet-balances,简单分析下pallet-asset都有哪些缺失使得其不适合生产环境使用

 注意：以太坊的问之一是状态爆炸

#### pallet-asset：

        1. 非常简单的多资产管理模块
        2. 功能简单；案例只是为了演示功能效果，不适合生产环境使用
        3. 代码中函数方式实现也比较简单

pallet-balance

1. 单一资产管理模块，只支持一个TOKEN
2. 可以通过instance来实现多资产管理，
3. 支持不够灵活

pallet-generic-asset

1. 多资产管理模块
2. 功能较为丰富；比如权限管理、版本控制、数据迁移、token发行/销毁
3. 实现了trait Currency,可以和其他的模块兼容；比如stacking质押模块、双币种支持。。。

#### 三、简单分析下为什么Polkadot配置的Balance类型是u128,而不是类似以太坊的u256

注意：DOT发行量是1000万，精度是12位，年增发率是10%

1. Polkadot基于substrate,而substrate基于Rust语言编写
2. DOT发行量1000W ，数据空间、性能、精确度上u128完全够用
3. 以太坊发行量1个亿，以太坊基于Go语言开发
4. rust以安全和性能都非常优秀，GO语言性能也不错但安全性考虑不是首要 