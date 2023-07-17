import {
  COSMOS_DENOM,
  CosmosWrapper,
  NEUTRON_DENOM,
  WalletWrapper,
} from '../../helpers/cosmos';
import { TestStateLocalCosmosTestNet } from '../common_localcosmosnet';

import { createProposal, registerICA, queryICA, queryCustodyFunds, queryProposalById } from '../../helpers/goods';

import { NeutronContract } from '../../helpers/types';
import { CodeId } from '../../types';
import { getWithAttempts } from '../../helpers/wait';


describe('Neutron Hackaton Test', () => {
    
    let testState: TestStateLocalCosmosTestNet;
    let neutronChain: CosmosWrapper;
    let gaiaChain: CosmosWrapper;
    let neutronAccount: WalletWrapper;
    let gaiaAccount: WalletWrapper;

    let accountAddress: string;
    let contractAddress: string;
    let icaAddress: string;

    const connectionId = 'connection-0';

    beforeAll(async () => {
      testState = new TestStateLocalCosmosTestNet();
      await testState.init();
      neutronChain = new CosmosWrapper(
        testState.sdk1,
        testState.blockWaiter1,
        NEUTRON_DENOM,
      );
      neutronAccount = new WalletWrapper(
        neutronChain,
        testState.wallets.qaNeutron.genQaWal1,
      );
      gaiaChain = new CosmosWrapper(
        testState.sdk2,
        testState.blockWaiter2,
        COSMOS_DENOM,
      );
      gaiaAccount = new WalletWrapper(
        gaiaChain,
        testState.wallets.qaCosmos.genQaWal1,
      );

      accountAddress = neutronAccount.wallet.address.toString()
    });


    describe('deploying contract', () => {
      let codeId: CodeId;
      test('store contract', async () => {
        codeId = await neutronAccount.storeWasm(
          NeutronContract.CROSSFUND,
        );
        expect(codeId).toBeGreaterThan(0);
      });

      test('instantiate contract', async () => {
        contractAddress = (
          await neutronAccount.instantiateContract(
            codeId,
            '{}',
            'neutron_interchain_queries',
          )
        )[0]._contract_address;
      });
    });

    describe('registering an interchain account', () => {
      test('can register account', async () => {

        await neutronAccount.msgSend(contractAddress, '1000000');

        await createProposal(neutronAccount, contractAddress)

        const res = await registerICA(
          neutronAccount,
          contractAddress,
          0,
          connectionId
        );

        console.log(res);

        expect(res.code).toBe(0);
      });

      test('if ICA accounts created', async () => {
        const channels = await getWithAttempts(
          neutronChain.blockWaiter,
          () => neutronChain.listIBCChannels(),
          async (channels) => channels.channels.length > 0,
        );

        const proposal_id = 0;
        expect(Array.isArray(channels.channels)).toBe(true);
        expect(channels.channels.length).toBeGreaterThan(1);
        // @ts-ignore
        expect(channels.channels).toIncludeAllPartialMembers([
          {
            port_id: `icacontroller-${contractAddress}.${proposal_id}`,
          },
        ])
      });


      test('if can get the ICA address', async () => {
          const ica = await queryICA(
            neutronChain,
            contractAddress,
            0,
            connectionId,
            10,
          );
          expect(ica.interchain_account_address).toContain('cosmos');
          expect(ica.interchain_account_address.length).toEqual(65);
          icaAddress = ica.interchain_account_address;
        }); 
      })

      describe('funding a project from another chain', () => {

        const amount = "100000000"

        test('if can send funds to the ICA', async () => {
          const res = await gaiaAccount.msgSend(
            icaAddress,
            {
              amount,
              denom: COSMOS_DENOM
            }
          );
          expect(res.code).toBe(0);
        });

        test('that contract havent seen the transfer yet', async () => {
          
          
          const [{ funding }, balance ] = await Promise.all([
            queryProposalById(neutronChain, contractAddress, 0),
            queryCustodyFunds(neutronChain, contractAddress, gaiaAccount.wallet.address.toString())
          ]);
          
          expect(balance.length | funding.length).toBe(0);
        });

        test("that contract sees the transfer after a while", async () => {

          await neutronChain.blockWaiter.waitBlocks(14)
          const proposal = await queryProposalById(neutronChain, contractAddress, 0)
          expect(proposal.funding.length).toBe(1);

          const balance = await queryCustodyFunds(neutronChain, contractAddress, gaiaAccount.wallet.address.toString());
          expect(balance.length).toBe(1);

          const [denom, fund] = balance[0];

          expect(denom).toBe(COSMOS_DENOM);
          expect(fund.amount).toBe(amount);
        })
    });

});

