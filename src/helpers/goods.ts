import { CosmosWrapper, WalletWrapper } from "./cosmos";
import { Addr, AllProposalInfo, CustodyFunds, Expiration, ProposalInfo, ShareHolder } from "./types";
import { getWithAttempts } from "./wait";



export const createProposal = (
  cm: WalletWrapper,
  contractAddress: string,
  title?: string,
  description?: string,
) => {
  return cm.executeContract(
    contractAddress,
    JSON.stringify({
      submit_proposal: {
        title: title ?? 'test proposal',
        description:  description ?? 'test proposal description',
      }
    }),
  );
}

export const fundProposal = (
  cm: WalletWrapper,
  contractAddress: string,
  proposal_id: number,
  amount: string,
  denom: string,
  auto_agree?: boolean,
) => {
  return cm.executeContract(
    contractAddress,
    JSON.stringify({
      fund_proposal: {
        proposal_id,
        auto_agree,
      }
    }),
    [{ amount, denom }],
  );
}



export const submitApplication = (
  cm: WalletWrapper,
  contractAddress: Addr,
  proposal_id: number,
  applicants: ShareHolder[],
  auditors: ShareHolder[],
  deliver_by: Expiration
) => {
  return cm.executeContract(
    contractAddress,
    JSON.stringify({
      submit_application: {
        proposal_id,
        application: {
          applicants,
          auditors,
          deliver_by,
        }
      }
    }),
  );
}

export const acceptApplication = (
  cm: WalletWrapper,
  contractAddress: string,
  proposal_id: number,
  application_sender: Addr,
) => {
  return cm.executeContract(
    contractAddress,
    JSON.stringify({
      accept_application: {
        proposal_id,
        application_sender
      }
    }),
  );
}


export const approveApplication = (
  cm: WalletWrapper,
  contractAddress: string,
  proposal_id: number,
  application_sender: Addr,
) => {
  return cm.executeContract(
    contractAddress,
    JSON.stringify({
      approve_application: {
        proposal_id,
        application_sender
      }
    }),
  );
}

export const registerICA = (
  cm: WalletWrapper,
  contractAddress: string,
  proposal_id: number,
  connection_id: string,
) => {
  return cm.executeContract(
    contractAddress,
    JSON.stringify({
      "register_i_c_a": {
        proposal_id,
        connection_id
      }
    }),
  );
}



export const verifyApplication = (
  cm: WalletWrapper,
  contractAddress: string,
  proposal_id: number,
  application_sender: Addr,
) => {
  return cm.executeContract(
    contractAddress,
    JSON.stringify({
      verify_application: {
        proposal_id,
        application_sender
      }
    }),
  );
}



export const queryProposals = (
  cm: CosmosWrapper,
  contractAddress: string,
) => {
  return cm.queryContract<AllProposalInfo>(contractAddress, {
    all_proposals: {}
  }
);
}
  

export const queryProposalById = (
  cm: CosmosWrapper,
  contractAddress: string,
  proposal_id: number,
) =>
  cm.queryContract<ProposalInfo>(contractAddress, {
    proposal: { proposal_id }
  }
);


export const queryCustodyFunds = (
  cm: CosmosWrapper,
  contractAddress: string,
  address: string,
  skip_locked?: boolean,
) => {
    return cm.queryContract<[string, CustodyFunds][]>(contractAddress, {
      address_funds: { address, skip_locked }
    });
  }



export const queryICA = (
  cm: CosmosWrapper,
  contractAddress: string,
  proposal_id: number,
  connection_id: string,
  numAttempts = 20,
) =>
  getWithAttempts(
    cm.blockWaiter,
    () =>
      cm.queryContract<{
        interchain_account_address: string;
      }>(contractAddress, {
        interchain_account_address: {
          proposal_id,
          connection_id,
        },
      }),
    async (response) => response != null,
    numAttempts,
  );
