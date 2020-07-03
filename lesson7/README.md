# lesson7 作业

1. 补完剩下的代码  
https://github.com/SubstrateCourse/substrate-kitties/blob/lesson7/pallets/kitties/src/linked_item.rs

OK

2. 修复单元测试
   

3. 阅读 pallet-membership     
    - a. 分析 add_member 的计算复杂度
    - 存储用Vector，查找用Binanry Search二分查找，时间复杂度是O(lgn)
    - change_members_sorted 时间复杂度 O(n)
    - 
    - b. 分析 pallet-membership 是否适合以下场景下使用，提供原因   
    - 最大成员数量：100
    - pub const MAX_MEMBERS: MemberCount = 100;
      * i. 储存预言机提供者
      * 适合
      * ii. 储存游戏链中每个工会的成员   
      * 不适合，存储的数据量过大
      * iii. 储存 PoA 网络验证人   
      * 适合？