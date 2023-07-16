import { cosmos } from '@cosmos-client/core/cjs/proto';

export type AcknowledgementResult =
  | { success: string[] }
  | { error: string[] }
  | { timeout: string };

export type ChannelsList = {
  channels: {
    state: string;
    ordering: string;
    counterparty: {
      port_id: string;
      channel_id: string;
    };
    connection_hops: string[];
    version: string;
    port_id: string;
    channel_id: string;
  }[];
};

// SingleChoiceProposal represents a single governance proposal item (partial object).
export type SingleChoiceProposal = {
  readonly title: string;
  readonly description: string;
  /// The address that created this proposal.
  readonly proposer: string;
  /// The block height at which this proposal was created. Voting
  /// power queries should query for voting power at this block
  /// height.
  readonly start_height: number;
  /// The threshold at which this proposal will pass.
  /// proposal's creation.
  readonly total_power: string;
  readonly proposal: {
    status:
      | 'open'
      | 'rejected'
      | 'passed'
      | 'executed'
      | 'closed'
      | 'execution_failed';
    readonly votes: {
      yes: string;
      no: string;
      abstain: string;
    };
  };
};

export type TotalPowerAtHeightResponse = {
  readonly height: string;
  readonly power: number;
};

export type VotingPowerAtHeightResponse = {
  readonly height: string;
  readonly power: number;
};

// PageRequest is the params of pagination for request
export type PageRequest = {
  'pagination.key'?: string;
  'pagination.offset'?: string;
  'pagination.limit'?: string;
  'pagination.count_total'?: boolean;
};

// AckFailuresResponse is the response model for the contractmanager failures.
export type AckFailuresResponse = {
  failures: Failure[];
  pagination: {
    next_key: string;
    total: string;
  };
};

// Failure represents a single contractmanager failure
type Failure = {
  address: string;
  id: number;
  ack_id: number;
  ack_type: string;
};

export type ScheduleResponse = {
  schedules: Schedule[];
  pagination: {
    next_key: string;
    total: string;
  };
};

// Schedule represents a single cron added schedule
type Schedule = {
  name: string;
  period: number;
  msgs: any[];
};

// BalancesResponse is the response model for the bank balances query.
export type PauseInfoResponse = {
  paused: {
    until_height: number;
  };
  unpaused: Record<string, never>;
};

export const NeutronContract = {
  CROSSFUND: 'crossfund.wasm',
};

export type MultiChoiceOption = {
  description: string;
  msgs: any[];
};

// https://github.com/cosmos/cosmos-sdk/blob/main/proto/cosmos/upgrade/v1beta1/query.proto#L53
export type CurrentPlanResponse = {
  plan: Plan | null;
};

// https://github.com/cosmos/cosmos-sdk/blob/main/proto/cosmos/upgrade/v1beta1/upgrade.proto#L14
export type Plan = {
  name: string;
  height: string;
  info: string;
};

export const nativeToken = (denom: string, amount: string): Asset => ({
  info: nativeTokenInfo(denom),
  amount: amount,
});

export const token = (contractAddr: string, amount: string): Asset => ({
  info: tokenInfo(contractAddr),
  amount: amount,
});

export const nativeTokenInfo = (denom: string): NativeToken => ({
  native_token: {
    denom: denom,
  },
});

export const tokenInfo = (contractAddr: string): Token => ({
  token: {
    contract_addr: contractAddr,
  },
});

export const vestingAccount = (
  addr: string,
  schedules: VestingSchedule[],
): VestingAccount => ({
  address: addr,
  schedules: schedules,
});

export const vestingSchedule = (
  startPoint: VestingSchedulePoint,
  endPoint?: VestingSchedulePoint,
): VestingSchedule => ({
  start_point: startPoint,
  end_point: endPoint,
});

export const vestingSchedulePoint = (
  time: number,
  amount: string,
): VestingSchedulePoint => ({
  time: time,
  amount: amount,
});

export type PoolStatus = {
  assets: Asset[];
  total_share: string;
};

export type Asset = {
  info: Token | NativeToken;
  amount: string;
};

export type Token = {
  token: {
    contract_addr: string;
  };
};

export type NativeToken = {
  native_token: {
    denom: string;
  };
};

export type VestingAccount = {
  address: string;
  schedules: VestingSchedule[];
};

export type VestingSchedule = {
  start_point: VestingSchedulePoint;
  end_point: VestingSchedulePoint | undefined;
};

export type VestingSchedulePoint = {
  time: number;
  amount: string;
};

export type PinnedCodesResponse = {
  code_ids: number[];
};

export type IcaHostParamsResponse = {
  params: {
    host_enabled: boolean;
  };
};

export type GlobalFeeMinGasPrices = {
  minimum_gas_prices: cosmos.base.v1beta1.ICoin[];
};

export type ContractAdminResponse = {
  contract_info: {
    admin: string;
  };
};





export type ProposalInfo = {
  id: number;
  title: string;
  description: string;
  funding: any[];
  applications: [string, Application][];
}

export type AllProposalInfo = {
  proposals: ProposalInfo[];
}


export type ShareHolder = { 
  recipient: Addr, 
  percent_share: number 
}






















export type Addr = string;
export type Expiration = {
  at_height: number;
} | {
  at_time: Timestamp;
} | {
  never: {};
};
export type Timestamp = Uint64;
export type Uint64 = string;
export interface Application {
  accepted: boolean;
  applicants: GoodFee[];
  auditors: GoodFee[];
  deliver_by: Expiration;
  verifications: Addr[];
}
export interface GoodFee {
  percent_share: number;
  recipient: Addr;
}
export type Uint128 = string;
export interface CustodyFunds {
  amount: Uint128;
  locked: boolean;
  proposal_id: number;
  remote?: string | null;
}
export type ExecuteMsg = {
  submit_proposal: {
    description: string;
    title: string;
  };
} | {
  submit_application: {
    application: ApplicationSubmission;
    proposal_id: number;
  };
} | {
  fund_proposal: {
    auto_agree?: boolean | null;
    proposal_id: number;
  };
} | {
  register_i_c_a: {
    connection_id: string;
    proposal_id: number;
  };
} | {
  approve_application: {
    application_sender: Addr;
    proposal_id: number;
  };
} | {
  accept_application: {
    application_sender: Addr;
    proposal_id: number;
  };
} | {
  verify_application: {
    application_sender: Addr;
    proposal_id: number;
    stop_at?: number | null;
  };
};
export interface ApplicationSubmission {
  applicants: GoodFee[];
  auditors: GoodFee[];
  deliver_by: Expiration;
}
export interface InstantiateMsg {}
export interface MigrateMsg {}
export interface QueryInterchainAccountAddressResponse {
  interchain_account_address: string;
  [k: string]: unknown;
}
export type QueryMsg = {
  all_proposals: {};
} | {
  proposal: {
    proposal_id: number;
  };
} | {
  address_funds: {
    address: Addr;
    skip_locked?: boolean | null;
  };
} | {
  interchain_account_address: {
    connection_id: string;
    proposal_id: number;
  };
} | {
  interchain_account_address_from_contract: {
    proposal_id: number;
  };
} | {
  acknowledgement_result: {
    proposal_id: number;
    sequence_id: number;
  };
} | {
  errors_queue: {};
};
export type SudoMsg = {
  response: {
    data: Binary;
    request: RequestPacket;
    [k: string]: unknown;
  };
} | {
  error: {
    details: string;
    request: RequestPacket;
    [k: string]: unknown;
  };
} | {
  timeout: {
    request: RequestPacket;
    [k: string]: unknown;
  };
} | {
  open_ack: {
    channel_id: string;
    counterparty_channel_id: string;
    counterparty_version: string;
    port_id: string;
    [k: string]: unknown;
  };
} | {
  tx_query_result: {
    data: Binary;
    height: Height;
    query_id: number;
    [k: string]: unknown;
  };
} | {
  kv_query_result: {
    query_id: number;
    [k: string]: unknown;
  };
};
export type Binary = string;
export interface RequestPacket {
  data?: Binary | null;
  destination_channel?: string | null;
  destination_port?: string | null;
  sequence?: number | null;
  source_channel?: string | null;
  source_port?: string | null;
  timeout_height?: RequestPacketTimeoutHeight | null;
  timeout_timestamp?: number | null;
  [k: string]: unknown;
}
export interface RequestPacketTimeoutHeight {
  revision_height?: number | null;
  revision_number?: number | null;
  [k: string]: unknown;
}
export interface Height {
  revision_height?: number;
  revision_number?: number;
  [k: string]: unknown;
}