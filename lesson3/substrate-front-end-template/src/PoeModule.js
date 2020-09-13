import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Card } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');

  const [digest, setDigest] = useState(0);
  const [owner, setOwner] = useState(0);
  const [blockNumber, setBlockNumber] = useState(0);

  // The currently stored value
  // const [currentValue, setCurrentValue] = useState(0);
  // const [formValue, setFormValue] = useState(0);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, result => {
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  const onFileSelect = (file)=> {
    const fileReader = new FileReader();
    const bufferDigest =()=>{
      const ret = Array.from( new Uint8Array(fileReader.result))
      .map((b=>b.toString(16)
      .padStart(2,'0'))).join('');
      const hash = blake2AsHex(ret, 256);
      setDigest(hash);
    }
    fileReader.onloadend = bufferDigest;
    fileReader.readAsArrayBuffer(file);
  }

  return (
    <Grid.Column width={8}>
      <h1>POE Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            lable='Please select your file'
            onChange={e => onFileSelect(e.target.files[0])}
          >
          </Input>

        </Form.Field>

        <Form.Field>
          <TxButton 
            accountPair={accountPair}
            label="Create Claim"
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams:[ digest ],
              paramFields:[ true ]
            }} />

          <TxButton 
            accountPair={accountPair}
            label="Revoke Claim"
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams:[ digest ],
              paramFields:[ true ]
            }} />
        </Form.Field>

        <Card centered>
          <Card.Content textAlign='center'>
            <div>{status}</div> 
            <div>{`[ Claim Info ] Owner: ${owner}, blockNumber:${blockNumber}`}</div> 
          </Card.Content>
        </Card>


      </Form>
      {/* <Card centered>
        <Card.Content textAlign='center'>
          <Statistic
            label='Current Value'
            value={currentValue}
          />
        </Card.Content>
      </Card> */}
      {/* <Form>
        <Form.Field>
          <Input
            label='New Value'
            state='newValue'
            type='number'
            onChange={(_, { value }) => setFormValue(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Store Something'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'templateModule',
              callable: 'doSomething',
              inputParams: [formValue],
              paramFields: [true]
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form> */}
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.templateModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
