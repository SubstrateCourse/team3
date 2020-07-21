# Lesson11指令

## 操作账户

PrivateKey: `99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342`

Address: `0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b`



## node启动

下载同路径下的`frontier.zip`文件到本地，并解压缩。

```bash
cd frontier
cargo build --release
```

启动节点：

```bash
cd frontier
./target/release/frontier-template-node purge-chain --dev
./target/release/frontier-template-node --dev
```

Compile frontier

![](images/1_frontier_compile_ok.png)

frontier running ok

![](images/2_frontier_running.png)

set metamask

![](images/3_metamask_setting.png.png)

get balance ok

![](images/4_balance.png)

transfer ether ok

![](images/5_transfer_eth.png)

https://remix.ethereum.org/
compile a Erc20 token on remix . ok

![](images/6_remix_compile.png)

deply the contract

![](images/7_deploy_comtract.png)

get an error when make a transaction

![](images/8_error_when_make_a_transaction.png)

## 注意事项

gas limit: `4294967295`