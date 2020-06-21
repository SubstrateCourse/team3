import { ApiPromise, WsProvider } from '@polkadot/api';
import { blake2AsHex } from '@polkadot/util-crypto';
//import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';
//import keyring from '@polkadot/ui-keyring';
const { Keyring } = require('@polkadot/keyring');
const testKeyring = require('@polkadot/keyring/testing');

// config
const WEB_SOCKET = 'ws://localhost:9944';

async function connect() {
  // Construct
  const wsProvider = new WsProvider(WEB_SOCKET);
  const api = await ApiPromise.create({ provider: wsProvider ,types: { Address: 'AccountId' },});
  const keyring = testKeyring.default();

  return { api, keyring };
}

async function submitDocInfo(filePath, comment) {
  console.debug(`submitDocInfo: ${filePath}, ${comment}`);
  try {

    /******
     * 学员们在这里追加逻辑
     *
     * 把 filePath 档档案通过 hash 函数算出它的 hash 值。然后和 comment 一起提交个 extrinsics
     *   到 Substrate。
     ******/
    // const content = Array.from(new Uint8Array(fileReader.result))
    // .map((b) => b.toString(16).padStart(2, '0'))
    // .join('');
    // const hash = blake2AsHex(content, 256);
    const { api, keyring }  = await connect();
    const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    const pair = keyring.getPair(ALICE);


    var crypto = require('crypto');
    var fs = require('fs');
    //读取一个Buffer
    //var buffer = fs.readFileSync('./mindpush.apk');
    var buffer = fs.readFileSync(filePath);
    // var fsHash = crypto.createHash('md5');

    // fsHash.update(buffer);
    // var md5 = fsHash.digest('hex');
    // console.log("文件的MD5是：%s", md5);
    const content = Array.from(buffer)
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');
    const claim = blake2AsHex(content, 256);
    //const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
    const hash = await api.tx.poeModule
      .createClaim(claim, comment)
      .signAndSend(pair);

    console.log('Transfer sent with hash', hash.toHex());

    let count = 0;

    // Subscribe to the new headers on-chain. The callback is fired when new headers
    // are found, the call itself returns a promise with a subscription that can be
    // used to unsubscribe from the newHead subscription
    const unsubscribe = await api.rpc.chain.subscribeNewHeads((header) => {
      console.log(`Chain is at block: #${header.number}`);
  
      if (++count === 256) {
        unsubscribe();
        process.exit(0);
      }
    });
    
    //var hash = await api.tx.poeModule.createClaim(md5).signAndSend(ALICE);
    //console.log('hash: ', hash)

  } catch (err) {
    console.error(`Connect to Substrate error:`, err);
    process.exit(1);
  }

  process.exit(0);
}

async function getUserDocs(acct) {
  console.debug(`getUserDocs: ${acct}`);
  try {
    const api = await connect();

    /******
     * 学员们在这里追加逻辑
     *
     * 通过用户 addr, 取得他所有的创建文件的 hash及相关资料。返回值是：
     * {
     *   "0xabcd1234...": ["my note1", 3],
     *   "0xabcd1235...": ["my note2", 5],
     *   "0xabcd1236...": ["my note3", 7],
     *   ...
     * }
     ******/
    // Connect to Substrate error: TypeError: Cannot read property 'poeModule' of undefined
    // at _callee3$ (/Users/fong/work/polkadot/substrate/team3/lesson4/client/src/script.js:114:37)
    // at tryCatch (/Users/fong/work/polkadot/substrate/team3/lesson4/client/node_modules/regenerator-runtime/runtime.js:45:40)
    // at Generator.invoke [as _invoke] (/Users/fong/work/polkadot/substrate/team3/lesson4/client/node_modules/regenerator-runtime/runtime.js:274:22)
    // at Generator.prototype.<computed> [as next] (/Users/fong/work/polkadot/substrate/team3/lesson4/client/node_modules/regenerator-runtime/runtime.js:97:21)
    // at asyncGeneratorStep (/Users/fong/work/polkadot/substrate/team3/lesson4/client/src/script.js:7:103)
    // at _next (/Users/fong/work/polkadot/substrate/team3/lesson4/client/src/script.js:9:194)
    // at processTicksAndRejections (internal/process/task_queues.js:97:5)

    // Unsolved ::: poeModule' of undefined
    //account2ProofHashList(AccountId): Vec<Bytes>

    const hashVec = await api.query.poeModule.account2ProofHashList(acct);
    const allClaimsPs = hashVec
      .toJSON()
      .map(v => api.query.poeModule.proofs(v));

    const allClaims = [];
    for await (const data of allClaimsPs) {
      allClaims.push(data.toJSON());
    }

    const answer = {};
    hashVec.toJSON().forEach((v, index) => (answer[v] = allClaims[index]));

    console.log(answer);

  } catch (err) {
    console.error(`Connect to Substrate error:`, err);
  }

  process.exit(0);
}

function main() {
  const args = process.argv.slice(2, 5);
  switch (args[0]) {
    case 'submitDocInfo':
      submitDocInfo(args[1], args[2])
      break;
    case 'getUserDocs':
      getUserDocs(args[1]);
      break;
    default:
      console.error('Unknown subcommand. Please use `submitDocInfo` or `getUserDocs` only.')
  }
}

main();
