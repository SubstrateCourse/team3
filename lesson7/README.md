# lesson7 作业

1. 补完剩下的代码  
  https://github.com/SubstrateCourse/substrate-kitties/blob/lesson7/pallets/kitties/src/linked_item.rs

2. 修复单元测试   

3. 阅读 pallet-membership     
    - a. 分析 add_member 的计算复杂度   

    ​    大O统计法计算方式：

            CPU执行代码的逻辑步骤：读取数据 > 运算 > 写数据
            假设每行代码执行时间一样，及做一个单位时间 unit_time
            将所有代码执行时间设为T(n)，T(n)与每行代码执行次数成正比，可得一个关系式。
            #[weight = 50_000_000]
          pub fn add_member(origin, who: T::AccountId) {
             T::AddOrigin::try_origin(origin).map(|_| ()).or_else(ensure_root)?; 1*unit_time
        
             let mut members = <Members<T, I>>::get();   1*unit_time
             let location = members.binary_search(&who).err().ok_or(Error::<T, I>::AlreadyMember)?;
              循环：n*unit_time
              判断结果：n*unit_time
        
             members.insert(location, who.clone());   (n+1)*unit_time
             <Members<T, I>>::put(&members);            1*unit_time
        
             T::MembershipChanged::change_members_sorted(&[who], &[], &members[..]);
                             (2n+1)*unit_time
             Self::deposit_event(RawEvent::MemberAdded); 1*unit_time
          }
        
           
    ```
    一共需要:(5n+6)*unit_time
          将unit_time当作一个标准量，忽略真实值
            所有代码执行时间T(n)与每行代码的执行次数n成正比
            大O统计法公式：T(n) = O(f(n))
            T(n)代表代码执行时间；
            n代表数据规模；
            f(n)代表每行代码执行次数总和；
            O代表T(n)与f(n)成正比
            用大O时间复杂度表示法：
            计算复杂度：T(n)=O(5n+6)
            
    大O时间复杂度并不具体表示代码真正的执行时间，代表的是代码执行时间随数据规模增长的变化趋势，所
    以也叫渐进时间复杂度，简称时间复杂度
    当n很大时（甚至无穷大），公式中的低阶项、常量、系数三部分并不能左右趋势走向，所以这三部分都可以忽略，只需要记录一个最大量级就可以
    T(n)=O(5n+6) ——> n取无穷大取极限———>lim(5n+6)[n——∞] ———> T(n)=O(n);
    ```

    标准答案:
        /// 总计算复杂度: O(MP + logN)
        #[weight = 50_000_000]
        pub fn add_member(origin, who: T::AccountId) {

          // O(1) 
          T::AddOrigin::ensure_origin(origin)?;

          // 一次 DB 读取，编码解码: O(n)
          let mut members = <Members<T, I>>::get();

          // O(log(n)) 
          let location = members.binary_search(&who).err().ok_or(Error::<T, I>::AlreadyMember)?;

          // O(n)
          members.insert(location, who.clone());

          // 一次 DB 读取，编码解码: O(n)
          <Members<T, I>>::put(&members);

          // frame/collective/src/lib.rs:848 `fn change_members_sorted(...){...}`
          // - `O(MP + N)`
          //   - where `M` old-members-count (governance-bounded)
          //   - where `N` new-members-count (governance-bounded)
          //   - where `P` proposals-count
          T::MembershipChanged::change_members_sorted(&[who], &[], &members[..]);
          
          // 参考 substrate/frame/system/src/lib.rs :
          // pub fn deposit_event_indexed(topics: &[T::Hash], event: T::Event) {...}
          // Self::block_number(); 一次 DB 读取，编码解码: O(n)
          // ExecutionPhase::get(); 一次 DB 读取，编码解码: O(n)
          // EventCount::get(); 一次 DB 读取，编码解码: O(n)
          // EventCount::put(new_event_count); 一次 DB 写入，编码解码: O(n)
          // Events::<T>::append(&event); 一次 DB 写入，编码解码: O(n)
          Self::deposit_event(RawEvent::MemberAdded);
        }

    - b. 分析 pallet-membership 是否适合以下场景下使用，提供原因   
      * i. 储存预言机提供者    

      ​            预言机数量级不大可以满足场景需求

      * ii. 储存游戏链中每个工会的成员   

      ​            不适合，游戏会员量级随时间与数量级成正比例，该数据结构性能会越来越低

      * iii. 储存 PoA 网络验证人   

​                  POA网络验证人相对固定，数量级不大也比较适合
 标准答案:
     I:  储存预言机提供者 答：适合。人数不多, 并且增删改不频繁。

     II: 储存游戏链中每个工会的成员 答：不适合。add_member 计算复杂度较高，对于人数太多, 关系复杂，并且增删改频繁的游戏工会成员场景，成本较大，所以不适合。

    III: 储存 PoA 网络验证人 答：适合。人数不多, PoA 验证人相对固定, 并且增删改不频繁。