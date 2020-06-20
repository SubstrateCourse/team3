//substrate
const Keyring = require('@polkadot/keyring').default;
const {
    ApiPromise,
    WsProvider
} = require('@polkadot/api');
const {
    stringToHex
} = require('@polkadot/util');

const bs58 = require('bs58');
const { blake2AsHex } = require('@polkadot/util-crypto')

//localhost testnet endpoint
const WS_PROVIDER = 'ws://127.0.0.1:9944';

const provider = new WsProvider(WS_PROVIDER);

const run = async () => {
    const api = await ApiPromise.create({
        provider,
        types: {}
    })

    console.log('api created -----')

    //create keypair
    const keyring = new Keyring({
        type: 'sr25519'
    });

    //const alice = keyring.getPairs()[0];
    const alice = keyring.addFromUri('//Alice');

    //query
    const nonce = await api.query.system.account(alice.address);
    //console.log('nonce:', nonce);

    //create_claim
    try {
        const claim = (blake2AsHex("this is string", 256))
        const utx = api.tx.poeModule.createClaim(claim, 100).sign(alice)

        await utx.send(({ events = [], status }) => {
            if (status.type === 'Future' || status.type === 'Invalid') {
                console.log('future or invalid', parameters)
            } else if (status.isInBlock) {
                console.log('inblock', parameters)
            } else if (status.isFinalized) {
                let isSuccessful = true
                events.forEach(({ phase, event: { data, method, section } }) => {
                    console.log(
                        '\t',
                        phase.toString(),
                        `: ${section}.${method}`,
                        data.toString()
                    )
                    if (method.includes('ExtrinsicFailed')) isSuccessful = false
                    console.log('transaction ', isSuccessful, parameters)
                })
            }
        })
    }
    catch (error) {
        console.log('errors:', error);
    }

    //query

}

run().catch(console.error);