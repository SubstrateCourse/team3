import React, { useEffect, useState } from 'react';
import { Form, Input, Grid} from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import {blake2AsHex} from '@polkadot/util-crypto';

function Main (props) {
  //获取api
  const { api } = useSubstrate();
  //获取accountPair
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');

  //声明digest，初始值为空
  const [digest,setDigest]=useState('');
  const [owner,setOwner]=useState('');
  const [blockNumber,setBlockNumber]=useState(0);
  const [accountId,setAccountId]=setStatus('');

  useEffect(() => {
    let unsubscribe;
    //检查poe模块内容是否更新，map，使用key，value
    api.query.poeModule.proofs(digest,(result) => {
      //设值
      setOwner(result[0].toString);
      setBlockNumber(result[1].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);
    //解除监听事件
    return () => unsubscribe && unsubscribe();
  }, [digest,api.query.poeModule]);
  //定义函数
  const handleFileChosen=(file)=>{
    let fileReader=new FileReader();
    //定义函数
    const  bufferToDigest=() =>{
      const content=Array.from(new Uint8Array(fileReader.result))
          .map((b)=>b.toString(16).padStart(2,'0'))
          .join('');
      const hash=blake2AsHex(content,256);
      setDigest(hash);
    }
    fileReader.onloadend=bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  }

  //Input 添加一个输入框，用来输入transfer的账户信息， onChange 表明输入框有变化之后，调用后面handleFileChosen为其赋值
  return (
    <Grid.Column width={8}>
      <h1>Proofs of Existence Module</h1>
      <form>
        <form.Field>
          <Input
              type='file'
              id='file'
              lable='your file'
              onChange={(e)=>handleFileChosen(e.target.files[0])}
           />
          <Input
              type='text'
              state='newValue'
              lable='transfer to AccountId'
              onChange={(_,{value})=>setAccountId(value)}
          />
        </form.Field>
        <form.Field>
          <TxButton
              accountPair={accountPair}
              label='Create Claim'
              setStatus={setStatus}
              type='SIGNED-TX'
              attrs={
                  {
                    palletRpc:'poeModule',
                    callable:'createClaim',
                    inputParams:[digest],
                    paramFields:[true]
                  }
              }
          />
          <TxButton
              accountPair={accountPair}
              label='Revoke Claim'
              setStatus={setStatus}
              type='SIGNED-TX'
              attrs={
                {
                  palletRpc:'poeModule',
                  callable:'revokeClaim',
                  inputParams:[digest],
                  paramFields:[true]
                }
              }
          />
          <TxButton
              accountPair={accountPair}
              label='Transfer Claim'
              setStatus={setStatus}
              type='SIGNED-TX'
              attrs={
                {
                  palletRpc:'poeModule',
                  callable:'transferClaim',
                  inputParams:[digest,accountId],
                  paramFields:[true]
                }
              }
          />
        </form.Field>
        <div>
          {status}
        </div>
        <div>
          {'Claim info,owner:${owner},blockNumber:${blockNumber}'}
        </div>
      </form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  //通过useSubstrate获取api来检查poeModule和poeModule存储proofs是否存在，如果存在进入main，不存在不去渲染返回null
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
