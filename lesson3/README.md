## 第三课作业  PoE 2

课程里会给出参考资料，大家一定要自己敲一遍**代码**！

注：

1. 提交源代码，运行`cargo test`的测试结果截图，前端UI的截图；
2. 测试应覆盖所有的业务逻辑，如不同的错误场景，以及一切正常的情况下，检查存储的数据是不是预期的那样。
3. 附加题不是必答的，但可以酌情加分。
4. 代码修改在本目录 substrate-node-template 和 substrate-front-end-template 的程序文件里。

第一题：编写存证模块的单元测试代码，包括：

* 创建存证的测试用例；

完成创建存证测试用例代码
![create_claim_test_code](./create_claim_test_code.png)
完成创建存证测试用例演示
![create_claim_test_demo](./create_claim_test_demo.png)
* 撤销存证的测试用例；


完成撤销存证测试用例代码
![revoke_claim_test_code](./revoke_claim_test_code.png)
完成撤销存证测试用例演示
![create_claim_test_demo](./revoke_claim_test_demo.png)
* 转移存证的测试用例；


完成转移存证测试用例代码
![transfer_claim_test_code](./transfer_claim_test_code.png)
完成转移存证测试用例演示
![transfer_claim_test_demo](./transfer_claim_test_demo.png)

第二题：编写存证模块的UI，包括

* 创建存证的UI

![create_claim_UI_demo](./create_claim_UI_demo.png)
* 删除存证的UI

![revoke_claim_UI_demo](./revoke_claim_UI_demo.png)
* 转移存证的UI

![Transfer_claim_UI_demo](./Transfer_claim_UI_demo.png)

第三题（附加题）：实现购买存证的功能代码：

测试结果
![Test_claim_demo](./Test_claim_demo.png)

* 用户A为自己的某个存证记录设置价格；
设置价格代码
![buy_claim_code](./buy_claim_code.png)
演示
![create_claim_price_demo](./create_claim_price_demo.png)

* 用户B可以以一定的价格购买某个存证，当出价高于用户A设置的价格时，则以用户A设定的价格将费用从用户B转移到用户A，再将该存证进行转移。如果出价低于用户A的价格时，则不进行转移，返回错误。

![buy_claim_demo](./buy_claim_demo.png)
![b_buy_a_yes](./b_buy_a_yes.png)
出价低于设置价格演示，返回错误
![buy_claim_err](./buy_claim_err.png)