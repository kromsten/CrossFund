import {
  CosmosWrapper,
  NEUTRON_DENOM,
  WalletWrapper,
} from '../../helpers/cosmos';
import { TestStateLocalCosmosTestNet } from '../common_localcosmosnet';
import { acceptApplication, approveApplication, createProposal, fundProposal, queryCustodyFunds, queryProposalById, queryProposals, submitApplication, verifyApplication } from '../../helpers/goods';

import { NeutronContract } from '../../helpers/types';
import { CodeId } from '../../types';
import { getHeight } from '../../helpers/wait';



describe('Neutron Hackaton Test', () => {
    
    let testState: TestStateLocalCosmosTestNet;
    let neutronChain: CosmosWrapper;
    let neutronAccount: WalletWrapper, 
        secondAccount: WalletWrapper;

    let accountAddress: string, secondAccountAddress: string;
    let contractAddress: string;

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
      
      secondAccount = new WalletWrapper(
        neutronChain,
        testState.wallets.qaNeutronThree.genQaWal1,
      );

      accountAddress = neutronAccount.wallet.address.toString()
      secondAccountAddress = secondAccount.wallet.address.toString()
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


     describe('submitting a proposal', () => {
      test('if can submit a proposal', async () => {
        const res = await createProposal(neutronAccount, contractAddress)
        expect(res.code).toBe(0);
      });

      test('if can query all proposal', async () => {
        const res = await queryProposals(neutronChain, contractAddress)
        
        expect(res).toHaveProperty('proposals');
        expect(res.proposals.length).toBe(1);

        const proposal = res.proposals[0];
        expect(proposal.id).toBe(0);
        expect(proposal.title).toBe('test proposal');
        expect(proposal.description).toBe('test proposal description');
        expect(proposal.funding).toHaveLength(0);
      })

      test('if can query proposal by id', async () => {
        const proposal = await queryProposalById(neutronChain, contractAddress, 0)
        expect(proposal.id).toBe(0);
        expect(proposal.title).toBe('test proposal');
        expect(proposal.description).toBe('test proposal description');
        expect(proposal.funding).toHaveLength(0);
      })
    })

    describe('funding a proposal', () => {
      test('if can sends funds', async () => {
        const res = await queryProposals(neutronChain, contractAddress)
        const { id } = res.proposals[0];
        const fundsRes = await fundProposal(neutronAccount, contractAddress, id, "1000000", NEUTRON_DENOM);
        expect(fundsRes.code).toBe(0);
      })
      
      test('if can proposal now have funds', async () => {
        const proposal = await queryProposalById(neutronChain, contractAddress, 0)
        expect(proposal.funding).toHaveLength(1);
        const [denom, value] = proposal.funding[0];
        expect(denom).toBe(NEUTRON_DENOM);
        expect(value.amount).toBe("1000000");
        expect(value.sender).toBe(accountAddress);
        expect(value.auto_agree).toBe(false);
        expect(value.native).toBe(true);
      })

      test('if funds are in custody', async () => {
        const funds = await queryCustodyFunds(neutronChain, contractAddress, neutronAccount.wallet.address.toString())
        expect(funds).toHaveLength(1);
        const [denom, fund] = funds[0];
        expect(denom).toBe(NEUTRON_DENOM);
        expect(fund.proposal_id).toBe(0);
        expect(fund.locked).toBe(false);
        expect(fund.amount).toBe("1000000");
        expect(fund.remote).toBeNull();
      })
    })

    describe('submitting a application', () => {
      
      let currentHeight;
      
      test('if can submit a application', async () => {

        currentHeight = await getHeight(neutronChain.sdk);

        const res = await submitApplication(
            secondAccount, 
            contractAddress,
            0,
            [{ recipient: secondAccountAddress, percent_share: 99 }],
            [{ recipient: accountAddress, percent_share: 1 }],
            { at_height: currentHeight + 10 },
        )
        expect(res.code).toBe(0);
      });

      test('if the proposal now have applications', async () => {
        const proposal = await queryProposalById(neutronChain, contractAddress, 0)
        expect(proposal.applications).toHaveLength(1);
        const [sender, application] = proposal.applications[0];
        expect(sender).toBe(secondAccountAddress);
        expect(application.accepted).toBe(false);
        expect(application.applicants).toHaveLength(1);
        expect(application.applicants[0].recipient).toBe(secondAccountAddress);
        expect(application.auditors).toHaveLength(1);
        expect(application.auditors[0].recipient).toBe(accountAddress);
        expect(application.verifications).toHaveLength(0);
        expect(application.deliver_by).toHaveProperty("at_height");
        expect((application.deliver_by as any).at_height).toBe(currentHeight + 10);
      })
    })


    describe("approving an application", () => {
      test('if can send approval message', async () => {
        const proposal = await queryProposalById(neutronChain, contractAddress, 0)
        const [sender, _] = proposal.applications[0];
        const res = await approveApplication(neutronAccount, contractAddress, 0, sender)
        expect(res.code).toBe(0);
      })

      test('if the funds are locked now', async () => {
        const funds = await queryCustodyFunds(neutronChain, contractAddress, neutronAccount.wallet.address.toString())
        const first = funds[0];
        expect(first[1].locked).toBe(true);
      })

      test('if locked funds skipping work', async () => {
        const funds = await queryCustodyFunds(neutronChain, contractAddress, neutronAccount.wallet.address.toString(), true)
        expect(funds).toHaveLength(0);
      })
    })

    describe('accepting bounty', () => {
      test('if can accept bounty', async () => {
        const proposal = await queryProposalById(neutronChain, contractAddress, 0)
        const [sender, _] = proposal.applications[0];
        const res = await acceptApplication(secondAccount, contractAddress, 0, sender)
        expect(res.code).toBe(0);
      })

      test('if the application is now accepted', async () => {
        const proposal = await queryProposalById(neutronChain, contractAddress, 0)
        const [_, application] = proposal.applications[0];
        expect(application.accepted).toBe(true);
      })
    })
    
    describe('verifying bounty', () => {
      test('if can verify bounty', async () => {
        const proposal = await queryProposalById(neutronChain, contractAddress, 0)
        const [sender, _] = proposal.applications[0];
        const res = await verifyApplication(neutronAccount, contractAddress, 0, sender)
        expect(res.code).toBe(0);
      })

      test('if the application is now verified', async () => {
        const proposal = await queryProposalById(neutronChain, contractAddress, 0)
        const [_, application] = proposal.applications[0];
        expect(application.verifications).toHaveLength(1);
        expect(application.verifications[0]).toBe(accountAddress);
      })

      test('if the funds are now unlocked', async () => {
        const funder_auditor = await queryCustodyFunds(neutronChain, contractAddress, accountAddress)
        const hacker = await queryCustodyFunds(neutronChain, contractAddress, secondAccountAddress)
        
        expect(funder_auditor[0][1].locked).toBe(false);
        expect(funder_auditor[0][1].amount).toBe('10000');

        expect(hacker[0][1].locked).toBe(false);
        expect(hacker[0][1].amount).toBe('990000');
      })
    })

});