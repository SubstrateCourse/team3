## 第八课作业


(7 分)

利用 off-chain worker 的概念，算出 1^2 + 2^2 + 3^2 + 4^2 + ....

第一个区块导入结束时算出：1^2
第二个区块导入结束时算出：1^2 + 2^2
第三个区块导入结束时算出：1^2 + 2^2 + 3^2
。。。如此類推。

当第三个区块导入结束时，能對鏈上發出請求：

sum(0) = 1
sum(1) = 5
sum(2) = 14
计算要在链下完成，链上只用作储存。提交到鏈上時用具簽名交易。

/substrate-node-template/pallets/template/src/lib.rs

(3 分)

附加题：写两个单元测试：

第一个是测试链下的计算逻辑
第二个是测试链上的函数

/substrate-node-template/pallets/template/src/mock.ts
/substrate-node-template/pallets/template/src/tests.ts