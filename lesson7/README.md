# lesson7 作业

1. 补完剩下的代码  
https://github.com/SubstrateCourse/substrate-kitties/blob/lesson7/pallets/kitties/src/linked_item.rs
2. 修复单元测试   
3. 阅读 pallet-membership     
    - a. 分析 add_member 的计算复杂度   
    - b. 分析 pallet-membership 是否适合以下场景下使用，提供原因   
      * i. 储存预言机提供者
         //不太清楚 ，提供者与链上的关系
      * ii. 储存游戏链中每个工会的成员   
        不太适合 ，但可以根据实际的需求，存储部分信息。
      * iii. 储存 PoA 网络验证人   
        适合数据模式比较单一
      
