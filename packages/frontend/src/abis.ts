import {
  useContractRead,
  UseContractReadConfig,
  useContractWrite,
  UseContractWriteConfig,
  usePrepareContractWrite,
  UsePrepareContractWriteConfig,
  useContractEvent,
  UseContractEventConfig,
} from 'wagmi'
import { ReadContractResult, WriteContractMode, PrepareWriteContractResult } from 'wagmi/actions'

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// AccountHandler
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const accountHandlerABI = [
  {
    stateMutability: 'nonpayable',
    type: 'constructor',
    inputs: [
      { name: '_relayerHandler', internalType: 'address', type: 'address' },
      { name: '_defaultDkimRegistry', internalType: 'address', type: 'address' },
      { name: '_verifier', internalType: 'address', type: 'address' },
      { name: '_walletImplementation', internalType: 'address', type: 'address' },
      { name: '_emailValidityDuration', internalType: 'uint256', type: 'uint256' },
    ],
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousOwner', internalType: 'address', type: 'address', indexed: true },
      { name: 'newOwner', internalType: 'address', type: 'address', indexed: true },
    ],
    name: 'OwnershipTransferred',
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'accountKeyCommitOfPointer',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'emailAddrPointer', internalType: 'bytes32', type: 'bytes32' },
      { name: 'accountKeyCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'walletSalt', internalType: 'bytes32', type: 'bytes32' },
      { name: 'psiPoint', internalType: 'bytes', type: 'bytes' },
      { name: 'proof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'createAccount',
    outputs: [{ name: 'wallet', internalType: 'contract Wallet', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'defaultDkimRegistry',
    outputs: [{ name: '', internalType: 'contract IDKIMRegistry', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'dkimRegistryOfWalletSalt',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'emailNullifiers',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'emailValidityDuration',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'accountKeyCommit', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getInfoOfAccountKeyCommit',
    outputs: [
      {
        name: '',
        internalType: 'struct AccountKeyInfo',
        type: 'tuple',
        components: [
          { name: 'relayer', internalType: 'address', type: 'address' },
          { name: 'initialized', internalType: 'bool', type: 'bool' },
          { name: 'walletSalt', internalType: 'bytes32', type: 'bytes32' },
        ],
      },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'emailAddrPointer', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getWalletOfEmailAddrPointer',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'salt', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getWalletOfSalt',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'infoOfAccountKeyCommit',
    outputs: [
      { name: 'relayer', internalType: 'address', type: 'address' },
      { name: 'initialized', internalType: 'bool', type: 'bool' },
      { name: 'walletSalt', internalType: 'bytes32', type: 'bytes32' },
    ],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'emailAddrPointer', internalType: 'bytes32', type: 'bytes32' },
      { name: 'emailDomain', internalType: 'string', type: 'string' },
      { name: 'emailTimestamp', internalType: 'uint256', type: 'uint256' },
      { name: 'emailNullifier', internalType: 'bytes32', type: 'bytes32' },
      { name: 'dkimPublicKeyHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'proof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'initializeAccount',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: 'walletSalt', internalType: 'bytes32', type: 'bytes32' },
      { name: 'emailDomain', internalType: 'string', type: 'string' },
      { name: 'publicKeyHash', internalType: 'bytes32', type: 'bytes32' },
    ],
    name: 'isDKIMPublicKeyHashValid',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes', type: 'bytes' }],
    name: 'pointerOfPSIPoint',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'relayerHandler',
    outputs: [{ name: '', internalType: 'contract RelayerHandler', type: 'address' }],
  },
  { stateMutability: 'nonpayable', type: 'function', inputs: [], name: 'renounceOwnership', outputs: [] },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'oldAccountKeyCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'newEmailAddrPointer', internalType: 'bytes32', type: 'bytes32' },
      { name: 'newAccountKeyCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'newPSIPoint', internalType: 'bytes', type: 'bytes' },
      {
        name: 'transportEmailProof',
        internalType: 'struct EmailProof',
        type: 'tuple',
        components: [
          { name: 'domain', internalType: 'string', type: 'string' },
          { name: 'timestamp', internalType: 'uint256', type: 'uint256' },
          { name: 'nullifier', internalType: 'bytes32', type: 'bytes32' },
          { name: 'dkimPublicKeyHash', internalType: 'bytes32', type: 'bytes32' },
          { name: 'proof', internalType: 'bytes', type: 'bytes' },
        ],
      },
      { name: 'accountCreationProof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'transportAccount',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'walletSalt', internalType: 'bytes32', type: 'bytes32' },
      { name: 'dkimRegistry', internalType: 'address', type: 'address' },
    ],
    name: 'updateDKIMRegistryOfWalletSalt',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'verifier',
    outputs: [{ name: '', internalType: 'contract IVerifier', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'walletImplementation',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ERC20
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const erc20ABI = [
  {
    stateMutability: 'nonpayable',
    type: 'constructor',
    inputs: [
      { name: 'name_', internalType: 'string', type: 'string' },
      { name: 'symbol_', internalType: 'string', type: 'string' },
    ],
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address', indexed: true },
      { name: 'spender', internalType: 'address', type: 'address', indexed: true },
      { name: 'value', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'from', internalType: 'address', type: 'address', indexed: true },
      { name: 'to', internalType: 'address', type: 'address', indexed: true },
      { name: 'value', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Transfer',
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: 'owner', internalType: 'address', type: 'address' },
      { name: 'spender', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'account', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'subtractedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'decreaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'spender', internalType: 'address', type: 'address' },
      { name: 'addedValue', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'increaseAllowance',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'from', internalType: 'address', type: 'address' },
      { name: 'to', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// EmailWalletCore
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const emailWalletCoreABI = [
  {
    stateMutability: 'nonpayable',
    type: 'constructor',
    inputs: [
      { name: '_relayerHandler', internalType: 'address', type: 'address' },
      { name: '_accountHandler', internalType: 'address', type: 'address' },
      { name: '_unclaimsHandler', internalType: 'address', type: 'address' },
      { name: '_extensionHandler', internalType: 'address', type: 'address' },
      { name: '_verifier', internalType: 'address', type: 'address' },
      { name: '_tokenRegistry', internalType: 'address', type: 'address' },
      { name: '_priceOracle', internalType: 'address', type: 'address' },
      { name: '_wethContract', internalType: 'address', type: 'address' },
      { name: '_maxFeePerGas', internalType: 'uint256', type: 'uint256' },
      { name: '_emailValidityDuration', internalType: 'uint256', type: 'uint256' },
      { name: '_unclaimedFundClaimGas', internalType: 'uint256', type: 'uint256' },
      { name: '_unclaimedStateClaimGas', internalType: 'uint256', type: 'uint256' },
    ],
  },
  { stateMutability: 'payable', type: 'fallback' },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'accountHandler',
    outputs: [{ name: '', internalType: 'contract AccountHandler', type: 'address' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'depositTokenAsExtension',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'emailNullifiers',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'emailValidityDuration',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'target', internalType: 'address', type: 'address' },
      { name: 'data', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'executeAsExtension',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'extensionHandler',
    outputs: [{ name: '', internalType: 'contract ExtensionHandler', type: 'address' }],
  },
  {
    stateMutability: 'payable',
    type: 'function',
    inputs: [
      {
        name: 'emailOp',
        internalType: 'struct EmailOp',
        type: 'tuple',
        components: [
          { name: 'emailAddrPointer', internalType: 'bytes32', type: 'bytes32' },
          { name: 'hasEmailRecipient', internalType: 'bool', type: 'bool' },
          { name: 'recipientEmailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
          { name: 'numRecipientEmailAddrBytes', internalType: 'uint256', type: 'uint256' },
          { name: 'recipientETHAddr', internalType: 'address', type: 'address' },
          { name: 'command', internalType: 'string', type: 'string' },
          { name: 'emailNullifier', internalType: 'bytes32', type: 'bytes32' },
          { name: 'emailDomain', internalType: 'string', type: 'string' },
          { name: 'dkimPublicKeyHash', internalType: 'bytes32', type: 'bytes32' },
          { name: 'timestamp', internalType: 'uint256', type: 'uint256' },
          { name: 'maskedSubject', internalType: 'string', type: 'string' },
          { name: 'feeTokenName', internalType: 'string', type: 'string' },
          { name: 'feePerGas', internalType: 'uint256', type: 'uint256' },
          { name: 'executeCallData', internalType: 'bytes', type: 'bytes' },
          { name: 'extensionName', internalType: 'string', type: 'string' },
          { name: 'newWalletOwner', internalType: 'address', type: 'address' },
          { name: 'newDkimRegistry', internalType: 'address', type: 'address' },
          {
            name: 'walletParams',
            internalType: 'struct WalletParams',
            type: 'tuple',
            components: [
              { name: 'tokenName', internalType: 'string', type: 'string' },
              { name: 'amount', internalType: 'uint256', type: 'uint256' },
            ],
          },
          {
            name: 'extensionParams',
            internalType: 'struct ExtensionParams',
            type: 'tuple',
            components: [
              { name: 'subjectTemplateIndex', internalType: 'uint8', type: 'uint8' },
              { name: 'subjectParams', internalType: 'bytes[]', type: 'bytes[]' },
            ],
          },
          { name: 'emailProof', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'handleEmailOp',
    outputs: [
      { name: 'success', internalType: 'bool', type: 'bool' },
      { name: 'err', internalType: 'bytes', type: 'bytes' },
      { name: 'totalFeeInETH', internalType: 'uint256', type: 'uint256' },
      { name: 'registeredUnclaimId', internalType: 'uint256', type: 'uint256' },
    ],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'defaultExtensions', internalType: 'bytes[]', type: 'bytes[]' }],
    name: 'initialize',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'maxFeePerGas',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'priceOracle',
    outputs: [{ name: '', internalType: 'contract IPriceOracle', type: 'address' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'extensionAddr', internalType: 'address', type: 'address' },
      { name: 'state', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'registerUnclaimedStateAsExtension',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'relayerHandler',
    outputs: [{ name: '', internalType: 'contract RelayerHandler', type: 'address' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'requestTokenAsExtension',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'tokenRegistry',
    outputs: [{ name: '', internalType: 'contract TokenRegistry', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'unclaimedFundClaimGas',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'unclaimedStateClaimGas',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'unclaimsHandler',
    outputs: [{ name: '', internalType: 'contract UnclaimsHandler', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      {
        name: 'emailOp',
        internalType: 'struct EmailOp',
        type: 'tuple',
        components: [
          { name: 'emailAddrPointer', internalType: 'bytes32', type: 'bytes32' },
          { name: 'hasEmailRecipient', internalType: 'bool', type: 'bool' },
          { name: 'recipientEmailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
          { name: 'numRecipientEmailAddrBytes', internalType: 'uint256', type: 'uint256' },
          { name: 'recipientETHAddr', internalType: 'address', type: 'address' },
          { name: 'command', internalType: 'string', type: 'string' },
          { name: 'emailNullifier', internalType: 'bytes32', type: 'bytes32' },
          { name: 'emailDomain', internalType: 'string', type: 'string' },
          { name: 'dkimPublicKeyHash', internalType: 'bytes32', type: 'bytes32' },
          { name: 'timestamp', internalType: 'uint256', type: 'uint256' },
          { name: 'maskedSubject', internalType: 'string', type: 'string' },
          { name: 'feeTokenName', internalType: 'string', type: 'string' },
          { name: 'feePerGas', internalType: 'uint256', type: 'uint256' },
          { name: 'executeCallData', internalType: 'bytes', type: 'bytes' },
          { name: 'extensionName', internalType: 'string', type: 'string' },
          { name: 'newWalletOwner', internalType: 'address', type: 'address' },
          { name: 'newDkimRegistry', internalType: 'address', type: 'address' },
          {
            name: 'walletParams',
            internalType: 'struct WalletParams',
            type: 'tuple',
            components: [
              { name: 'tokenName', internalType: 'string', type: 'string' },
              { name: 'amount', internalType: 'uint256', type: 'uint256' },
            ],
          },
          {
            name: 'extensionParams',
            internalType: 'struct ExtensionParams',
            type: 'tuple',
            components: [
              { name: 'subjectTemplateIndex', internalType: 'uint8', type: 'uint8' },
              { name: 'subjectParams', internalType: 'bytes[]', type: 'bytes[]' },
            ],
          },
          { name: 'emailProof', internalType: 'bytes', type: 'bytes' },
        ],
      },
    ],
    name: 'validateEmailOp',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'verifier',
    outputs: [{ name: '', internalType: 'contract IVerifier', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'wethContract',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  { stateMutability: 'payable', type: 'receive' },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ExtensionHandler
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const extensionHandlerABI = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousOwner', internalType: 'address', type: 'address', indexed: true },
      { name: 'newOwner', internalType: 'address', type: 'address', indexed: true },
    ],
    name: 'OwnershipTransferred',
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'string', type: 'string' }],
    name: 'addressOfExtensionName',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'string', type: 'string' }],
    name: 'defaultExtensionOfCommand',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: 'walletAddr', internalType: 'address', type: 'address' },
      { name: 'command', internalType: 'string', type: 'string' },
    ],
    name: 'getExtensionForCommand',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'extensionAddr', internalType: 'address', type: 'address' }],
    name: 'getSubjectTemplatesOfExtension',
    outputs: [{ name: '', internalType: 'string[][]', type: 'string[][]' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'address', type: 'address' }],
    name: 'maxGasOfExtension',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'name', internalType: 'string', type: 'string' },
      { name: 'addr', internalType: 'address', type: 'address' },
      { name: 'subjectTemplates', internalType: 'string[][]', type: 'string[][]' },
      { name: 'maxExecutionGas', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'publishExtension',
    outputs: [],
  },
  { stateMutability: 'nonpayable', type: 'function', inputs: [], name: 'renounceOwnership', outputs: [] },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'defaultExtensions', internalType: 'bytes[]', type: 'bytes[]' }],
    name: 'setDefaultExtensions',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'walletAddr', internalType: 'address', type: 'address' },
      { name: 'command', internalType: 'string', type: 'string' },
      { name: 'extensionAddr', internalType: 'address', type: 'address' },
    ],
    name: 'setExtensionForCommand',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'subjectTemplatesOfExtension',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'string', type: 'string' },
    ],
    name: 'userExtensionOfCommand',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// RelayerHandler
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const relayerHandlerABI = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousOwner', internalType: 'address', type: 'address', indexed: true },
      { name: 'newOwner', internalType: 'address', type: 'address', indexed: true },
    ],
    name: 'OwnershipTransferred',
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'relayer', internalType: 'address', type: 'address' }],
    name: 'getRandHash',
    outputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'randHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'emailAddr', internalType: 'string', type: 'string' },
      { name: 'hostname', internalType: 'string', type: 'string' },
    ],
    name: 'registerRelayer',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'string', type: 'string' }],
    name: 'relayerOfEmailAddr',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'relayerOfRandHash',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'address', type: 'address' }],
    name: 'relayers',
    outputs: [
      { name: 'randHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'emailAddr', internalType: 'string', type: 'string' },
      { name: 'hostname', internalType: 'string', type: 'string' },
    ],
  },
  { stateMutability: 'nonpayable', type: 'function', inputs: [], name: 'renounceOwnership', outputs: [] },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'hostname', internalType: 'string', type: 'string' }],
    name: 'updateRelayerConfig',
    outputs: [],
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TokenCallbackHandler
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const tokenCallbackHandlerABI = [
  {
    stateMutability: 'pure',
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256[]', type: 'uint256[]' },
      { name: '', internalType: 'uint256[]', type: 'uint256[]' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC1155BatchReceived',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
  },
  {
    stateMutability: 'pure',
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC1155Received',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
  },
  {
    stateMutability: 'pure',
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'onERC721Received',
    outputs: [{ name: '', internalType: 'bytes4', type: 'bytes4' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'interfaceId', internalType: 'bytes4', type: 'bytes4' }],
    name: 'supportsInterface',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'pure',
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'bytes', type: 'bytes' },
      { name: '', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'tokensReceived',
    outputs: [],
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TokenRegistry
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const tokenRegistryABI = [
  { stateMutability: 'nonpayable', type: 'constructor', inputs: [] },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'chainName', internalType: 'string', type: 'string', indexed: true },
      { name: 'chainId', internalType: 'uint256', type: 'uint256', indexed: true },
    ],
    name: 'ChainRegistered',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousOwner', internalType: 'address', type: 'address', indexed: true },
      { name: 'newOwner', internalType: 'address', type: 'address', indexed: true },
    ],
    name: 'OwnershipTransferred',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'chainId', internalType: 'uint256', type: 'uint256', indexed: true },
      { name: 'tokenName', internalType: 'string', type: 'string', indexed: true },
      { name: 'addr', internalType: 'address', type: 'address', indexed: true },
    ],
    name: 'TokenRegistered',
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'string', type: 'string' },
    ],
    name: 'addressOfTokenName',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'string', type: 'string' }],
    name: 'chainIdOfName',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'chainName', internalType: 'string', type: 'string' }],
    name: 'getChainIdOfName',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
      { name: 'tokenName', internalType: 'string', type: 'string' },
    ],
    name: 'getTokenAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: 'chainName', internalType: 'string', type: 'string' },
      { name: 'tokenName', internalType: 'string', type: 'string' },
    ],
    name: 'getTokenAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'tokenName', internalType: 'string', type: 'string' }],
    name: 'getTokenAddress',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
      { name: 'addr', internalType: 'address', type: 'address' },
    ],
    name: 'getTokenNameOfAddress',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'addr', internalType: 'address', type: 'address' }],
    name: 'getTokenNameOfAddress',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  { stateMutability: 'nonpayable', type: 'function', inputs: [], name: 'renounceOwnership', outputs: [] },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'chainName', internalType: 'string', type: 'string' },
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'setChainId',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'tokenName', internalType: 'string', type: 'string' },
      { name: 'addr', internalType: 'address', type: 'address' },
    ],
    name: 'setTokenAddress',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
      { name: 'tokenName', internalType: 'string', type: 'string' },
      { name: 'addr', internalType: 'address', type: 'address' },
    ],
    name: 'setTokenAddress',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'chainId', internalType: 'uint256', type: 'uint256' },
      { name: 'tokenNames', internalType: 'string[]', type: 'string[]' },
      { name: 'addrs', internalType: 'address[]', type: 'address[]' },
    ],
    name: 'setTokenAddresses',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: '', internalType: 'uint256', type: 'uint256' },
      { name: '', internalType: 'address', type: 'address' },
    ],
    name: 'tokenNameOfAddress',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// UnclaimRegisterHandler
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const unclaimRegisterHandlerABI = [
  {
    stateMutability: 'nonpayable',
    type: 'constructor',
    inputs: [{ name: '_unclaimsExpiryDuration', internalType: 'uint256', type: 'uint256' }],
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousOwner', internalType: 'address', type: 'address', indexed: true },
      { name: 'newOwner', internalType: 'address', type: 'address', indexed: true },
    ],
    name: 'OwnershipTransferred',
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'recipientEmailAddrPointer', internalType: 'bytes32', type: 'bytes32' },
      { name: 'relayerRandHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'recipientAddr', internalType: 'address', type: 'address' },
      { name: 'verifier', internalType: 'contract IVerifier', type: 'address' },
      { name: 'proof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'claimUnclaimedFundInternal',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'recipientEmailAddrPointer', internalType: 'bytes32', type: 'bytes32' },
      { name: 'relayerRandHash', internalType: 'bytes32', type: 'bytes32' },
      { name: 'verifier', internalType: 'contract IVerifier', type: 'address' },
      { name: 'proof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'claimUnclaimedStateInternal',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getUnclaimedFundOfEmailAddrCommit',
    outputs: [
      {
        name: '',
        internalType: 'struct UnclaimedFund',
        type: 'tuple',
        components: [
          { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
          { name: 'sender', internalType: 'address', type: 'address' },
          { name: 'tokenAddr', internalType: 'address', type: 'address' },
          { name: 'amount', internalType: 'uint256', type: 'uint256' },
          { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' }],
    name: 'getUnclaimedStateOfEmailAddrCommit',
    outputs: [
      {
        name: '',
        internalType: 'struct UnclaimedState',
        type: 'tuple',
        components: [
          { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
          { name: 'extensionAddr', internalType: 'address', type: 'address' },
          { name: 'sender', internalType: 'address', type: 'address' },
          { name: 'state', internalType: 'bytes', type: 'bytes' },
          { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'msgSender', internalType: 'address', type: 'address' },
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'registerUnclaimedFundInternal',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'extensionAddr', internalType: 'address', type: 'address' },
      { name: 'state', internalType: 'bytes', type: 'bytes' },
      { name: 'recipientEmailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
      { name: 'walletAddr', internalType: 'address', type: 'address' },
    ],
    name: 'registerUnclaimedStateAsExtensionInternal',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'msgSender', internalType: 'address', type: 'address' },
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'extensionAddr', internalType: 'address', type: 'address' },
      { name: 'state', internalType: 'bytes', type: 'bytes' },
      { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'registerUnclaimedStateInternal',
    outputs: [],
  },
  { stateMutability: 'nonpayable', type: 'function', inputs: [], name: 'renounceOwnership', outputs: [] },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'unclaimedFundOfEmailAddrCommit',
    outputs: [
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'bytes32', type: 'bytes32' }],
    name: 'unclaimedStateOfEmailAddrCommit',
    outputs: [
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'extensionAddr', internalType: 'address', type: 'address' },
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'state', internalType: 'bytes', type: 'bytes' },
      { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'unclaimsExpiryDuration',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      {
        name: 'fund',
        internalType: 'struct UnclaimedFund',
        type: 'tuple',
        components: [
          { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
          { name: 'sender', internalType: 'address', type: 'address' },
          { name: 'tokenAddr', internalType: 'address', type: 'address' },
          { name: 'amount', internalType: 'uint256', type: 'uint256' },
          { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
    name: 'updateUnclaimFund',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' }],
    name: 'voidUnclaimedFundInternal',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' }],
    name: 'voidUnclaimedStateInternal',
    outputs: [],
  },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// UnclaimsHandler
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const unclaimsHandlerABI = [
  {
    stateMutability: 'nonpayable',
    type: 'constructor',
    inputs: [
      { name: '_relayerHandler', internalType: 'address', type: 'address' },
      { name: '_accountHandler', internalType: 'address', type: 'address' },
      { name: '_verifier', internalType: 'address', type: 'address' },
      { name: '_unclaimedFundClaimGas', internalType: 'uint256', type: 'uint256' },
      { name: '_unclaimedStateClaimGas', internalType: 'uint256', type: 'uint256' },
      { name: '_unclaimsExpiryDuration', internalType: 'uint256', type: 'uint256' },
      { name: '_maxFeePerGas', internalType: 'uint256', type: 'uint256' },
    ],
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'previousOwner', internalType: 'address', type: 'address', indexed: true },
      { name: 'newOwner', internalType: 'address', type: 'address', indexed: true },
    ],
    name: 'OwnershipTransferred',
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'accountHandler',
    outputs: [{ name: '', internalType: 'contract AccountHandler', type: 'address' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'id', internalType: 'uint256', type: 'uint256' },
      { name: 'recipientEmailAddrPointer', internalType: 'bytes32', type: 'bytes32' },
      { name: 'proof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'claimUnclaimedFund',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'id', internalType: 'uint256', type: 'uint256' },
      { name: 'recipientEmailAddrPointer', internalType: 'bytes32', type: 'bytes32' },
      { name: 'proof', internalType: 'bytes', type: 'bytes' },
    ],
    name: 'claimUnclaimedState',
    outputs: [
      { name: 'success', internalType: 'bool', type: 'bool' },
      { name: 'returnData', internalType: 'bytes', type: 'bytes' },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'id', internalType: 'uint256', type: 'uint256' }],
    name: 'getUnclaimedFund',
    outputs: [
      {
        name: '',
        internalType: 'struct UnclaimedFund',
        type: 'tuple',
        components: [
          { name: 'id', internalType: 'uint256', type: 'uint256' },
          { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
          { name: 'sender', internalType: 'address', type: 'address' },
          { name: 'tokenAddr', internalType: 'address', type: 'address' },
          { name: 'amount', internalType: 'uint256', type: 'uint256' },
          { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: 'id', internalType: 'uint256', type: 'uint256' }],
    name: 'getUnclaimedState',
    outputs: [
      {
        name: '',
        internalType: 'struct UnclaimedState',
        type: 'tuple',
        components: [
          { name: 'id', internalType: 'uint256', type: 'uint256' },
          { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
          { name: 'extensionAddr', internalType: 'address', type: 'address' },
          { name: 'sender', internalType: 'address', type: 'address' },
          { name: 'state', internalType: 'bytes', type: 'bytes' },
          { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
        ],
      },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'maxFeePerGas',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'numUnclaimedFunds',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'numUnclaimedStates',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', internalType: 'address', type: 'address' }],
  },
  {
    stateMutability: 'payable',
    type: 'function',
    inputs: [
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
      { name: 'announceCommitRandomness', internalType: 'uint256', type: 'uint256' },
      { name: 'announceEmailAddr', internalType: 'string', type: 'string' },
    ],
    name: 'registerUnclaimedFund',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'registerUnclaimedFundInternal',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'payable',
    type: 'function',
    inputs: [
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'extensionAddr', internalType: 'address', type: 'address' },
      { name: 'state', internalType: 'bytes', type: 'bytes' },
      { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
      { name: 'announceCommitRandomness', internalType: 'uint256', type: 'uint256' },
      { name: 'announceEmailAddr', internalType: 'string', type: 'string' },
    ],
    name: 'registerUnclaimedState',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'extensionAddr', internalType: 'address', type: 'address' },
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'recipientEmailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'state', internalType: 'bytes', type: 'bytes' },
      { name: 'isInternal', internalType: 'bool', type: 'bool' },
    ],
    name: 'registerUnclaimedStateInternal',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'relayerHandler',
    outputs: [{ name: '', internalType: 'contract RelayerHandler', type: 'address' }],
  },
  { stateMutability: 'nonpayable', type: 'function', inputs: [], name: 'renounceOwnership', outputs: [] },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'newOwner', internalType: 'address', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'unclaimedFundClaimGas',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    name: 'unclaimedFundOfId',
    outputs: [
      { name: 'id', internalType: 'uint256', type: 'uint256' },
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'tokenAddr', internalType: 'address', type: 'address' },
      { name: 'amount', internalType: 'uint256', type: 'uint256' },
      { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'unclaimedStateClaimGas',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
    name: 'unclaimedStateOfId',
    outputs: [
      { name: 'id', internalType: 'uint256', type: 'uint256' },
      { name: 'emailAddrCommit', internalType: 'bytes32', type: 'bytes32' },
      { name: 'extensionAddr', internalType: 'address', type: 'address' },
      { name: 'sender', internalType: 'address', type: 'address' },
      { name: 'state', internalType: 'bytes', type: 'bytes' },
      { name: 'expiryTime', internalType: 'uint256', type: 'uint256' },
    ],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'unclaimsExpiryDuration',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'verifier',
    outputs: [{ name: '', internalType: 'contract IVerifier', type: 'address' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'id', internalType: 'uint256', type: 'uint256' }],
    name: 'voidUnclaimedFund',
    outputs: [],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'id', internalType: 'uint256', type: 'uint256' }],
    name: 'voidUnclaimedState',
    outputs: [
      { name: 'success', internalType: 'bool', type: 'bool' },
      { name: 'returnData', internalType: 'bytes', type: 'bytes' },
    ],
  },
  { stateMutability: 'payable', type: 'receive' },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// WETH9
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const weth9ABI = [
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'src', internalType: 'address', type: 'address', indexed: true },
      { name: 'guy', internalType: 'address', type: 'address', indexed: true },
      { name: 'wad', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Approval',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'dst', internalType: 'address', type: 'address', indexed: true },
      { name: 'wad', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Deposit',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'src', internalType: 'address', type: 'address', indexed: true },
      { name: 'dst', internalType: 'address', type: 'address', indexed: true },
      { name: 'wad', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Transfer',
  },
  {
    type: 'event',
    anonymous: false,
    inputs: [
      { name: 'src', internalType: 'address', type: 'address', indexed: true },
      { name: 'wad', internalType: 'uint256', type: 'uint256', indexed: false },
    ],
    name: 'Withdrawal',
  },
  { stateMutability: 'payable', type: 'fallback' },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [
      { name: '', internalType: 'address', type: 'address' },
      { name: '', internalType: 'address', type: 'address' },
    ],
    name: 'allowance',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'guy', internalType: 'address', type: 'address' },
      { name: 'wad', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'approve',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [{ name: '', internalType: 'address', type: 'address' }],
    name: 'balanceOf',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'decimals',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
  },
  { stateMutability: 'payable', type: 'function', inputs: [], name: 'deposit', outputs: [] },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'name',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'symbol',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
  },
  {
    stateMutability: 'view',
    type: 'function',
    inputs: [],
    name: 'totalSupply',
    outputs: [{ name: '', internalType: 'uint256', type: 'uint256' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'dst', internalType: 'address', type: 'address' },
      { name: 'wad', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transfer',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [
      { name: 'src', internalType: 'address', type: 'address' },
      { name: 'dst', internalType: 'address', type: 'address' },
      { name: 'wad', internalType: 'uint256', type: 'uint256' },
    ],
    name: 'transferFrom',
    outputs: [{ name: '', internalType: 'bool', type: 'bool' }],
  },
  {
    stateMutability: 'nonpayable',
    type: 'function',
    inputs: [{ name: 'wad', internalType: 'uint256', type: 'uint256' }],
    name: 'withdraw',
    outputs: [],
  },
  { stateMutability: 'payable', type: 'receive' },
] as const

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// React
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__.
 */
export function useAccountHandlerRead<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(config: Omit<UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>, 'abi'> = {} as any) {
  return useContractRead({ abi: accountHandlerABI, ...config } as UseContractReadConfig<
    typeof accountHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"accountKeyCommitOfPointer"`.
 */
export function useAccountHandlerAccountKeyCommitOfPointer<
  TFunctionName extends 'accountKeyCommitOfPointer',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'accountKeyCommitOfPointer',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"defaultDkimRegistry"`.
 */
export function useAccountHandlerDefaultDkimRegistry<
  TFunctionName extends 'defaultDkimRegistry',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'defaultDkimRegistry',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"dkimRegistryOfWalletSalt"`.
 */
export function useAccountHandlerDkimRegistryOfWalletSalt<
  TFunctionName extends 'dkimRegistryOfWalletSalt',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'dkimRegistryOfWalletSalt',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"emailNullifiers"`.
 */
export function useAccountHandlerEmailNullifiers<
  TFunctionName extends 'emailNullifiers',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'emailNullifiers',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"emailValidityDuration"`.
 */
export function useAccountHandlerEmailValidityDuration<
  TFunctionName extends 'emailValidityDuration',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'emailValidityDuration',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"getInfoOfAccountKeyCommit"`.
 */
export function useAccountHandlerGetInfoOfAccountKeyCommit<
  TFunctionName extends 'getInfoOfAccountKeyCommit',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'getInfoOfAccountKeyCommit',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"getWalletOfEmailAddrPointer"`.
 */
export function useAccountHandlerGetWalletOfEmailAddrPointer<
  TFunctionName extends 'getWalletOfEmailAddrPointer',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'getWalletOfEmailAddrPointer',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"getWalletOfSalt"`.
 */
export function useAccountHandlerGetWalletOfSalt<
  TFunctionName extends 'getWalletOfSalt',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'getWalletOfSalt',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"infoOfAccountKeyCommit"`.
 */
export function useAccountHandlerInfoOfAccountKeyCommit<
  TFunctionName extends 'infoOfAccountKeyCommit',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'infoOfAccountKeyCommit',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"isDKIMPublicKeyHashValid"`.
 */
export function useAccountHandlerIsDkimPublicKeyHashValid<
  TFunctionName extends 'isDKIMPublicKeyHashValid',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'isDKIMPublicKeyHashValid',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"owner"`.
 */
export function useAccountHandlerOwner<
  TFunctionName extends 'owner',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: accountHandlerABI, functionName: 'owner', ...config } as UseContractReadConfig<
    typeof accountHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"pointerOfPSIPoint"`.
 */
export function useAccountHandlerPointerOfPsiPoint<
  TFunctionName extends 'pointerOfPSIPoint',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'pointerOfPSIPoint',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"relayerHandler"`.
 */
export function useAccountHandlerRelayerHandler<
  TFunctionName extends 'relayerHandler',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: accountHandlerABI, functionName: 'relayerHandler', ...config } as UseContractReadConfig<
    typeof accountHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"verifier"`.
 */
export function useAccountHandlerVerifier<
  TFunctionName extends 'verifier',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: accountHandlerABI, functionName: 'verifier', ...config } as UseContractReadConfig<
    typeof accountHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"walletImplementation"`.
 */
export function useAccountHandlerWalletImplementation<
  TFunctionName extends 'walletImplementation',
  TSelectData = ReadContractResult<typeof accountHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: accountHandlerABI,
    functionName: 'walletImplementation',
    ...config,
  } as UseContractReadConfig<typeof accountHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link accountHandlerABI}__.
 */
export function useAccountHandlerWrite<TFunctionName extends string, TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof accountHandlerABI, string>['request']['abi'],
        TFunctionName,
        TMode
      >
    : UseContractWriteConfig<typeof accountHandlerABI, TFunctionName, TMode> & {
        abi?: never
      } = {} as any,
) {
  return useContractWrite<typeof accountHandlerABI, TFunctionName, TMode>({ abi: accountHandlerABI, ...config } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"createAccount"`.
 */
export function useAccountHandlerCreateAccount<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof accountHandlerABI, 'createAccount'>['request']['abi'],
        'createAccount',
        TMode
      > & { functionName?: 'createAccount' }
    : UseContractWriteConfig<typeof accountHandlerABI, 'createAccount', TMode> & {
        abi?: never
        functionName?: 'createAccount'
      } = {} as any,
) {
  return useContractWrite<typeof accountHandlerABI, 'createAccount', TMode>({
    abi: accountHandlerABI,
    functionName: 'createAccount',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"initializeAccount"`.
 */
export function useAccountHandlerInitializeAccount<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof accountHandlerABI, 'initializeAccount'>['request']['abi'],
        'initializeAccount',
        TMode
      > & { functionName?: 'initializeAccount' }
    : UseContractWriteConfig<typeof accountHandlerABI, 'initializeAccount', TMode> & {
        abi?: never
        functionName?: 'initializeAccount'
      } = {} as any,
) {
  return useContractWrite<typeof accountHandlerABI, 'initializeAccount', TMode>({
    abi: accountHandlerABI,
    functionName: 'initializeAccount',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function useAccountHandlerRenounceOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof accountHandlerABI, 'renounceOwnership'>['request']['abi'],
        'renounceOwnership',
        TMode
      > & { functionName?: 'renounceOwnership' }
    : UseContractWriteConfig<typeof accountHandlerABI, 'renounceOwnership', TMode> & {
        abi?: never
        functionName?: 'renounceOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof accountHandlerABI, 'renounceOwnership', TMode>({
    abi: accountHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function useAccountHandlerTransferOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof accountHandlerABI, 'transferOwnership'>['request']['abi'],
        'transferOwnership',
        TMode
      > & { functionName?: 'transferOwnership' }
    : UseContractWriteConfig<typeof accountHandlerABI, 'transferOwnership', TMode> & {
        abi?: never
        functionName?: 'transferOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof accountHandlerABI, 'transferOwnership', TMode>({
    abi: accountHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"transportAccount"`.
 */
export function useAccountHandlerTransportAccount<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof accountHandlerABI, 'transportAccount'>['request']['abi'],
        'transportAccount',
        TMode
      > & { functionName?: 'transportAccount' }
    : UseContractWriteConfig<typeof accountHandlerABI, 'transportAccount', TMode> & {
        abi?: never
        functionName?: 'transportAccount'
      } = {} as any,
) {
  return useContractWrite<typeof accountHandlerABI, 'transportAccount', TMode>({
    abi: accountHandlerABI,
    functionName: 'transportAccount',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"updateDKIMRegistryOfWalletSalt"`.
 */
export function useAccountHandlerUpdateDkimRegistryOfWalletSalt<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof accountHandlerABI, 'updateDKIMRegistryOfWalletSalt'>['request']['abi'],
        'updateDKIMRegistryOfWalletSalt',
        TMode
      > & { functionName?: 'updateDKIMRegistryOfWalletSalt' }
    : UseContractWriteConfig<typeof accountHandlerABI, 'updateDKIMRegistryOfWalletSalt', TMode> & {
        abi?: never
        functionName?: 'updateDKIMRegistryOfWalletSalt'
      } = {} as any,
) {
  return useContractWrite<typeof accountHandlerABI, 'updateDKIMRegistryOfWalletSalt', TMode>({
    abi: accountHandlerABI,
    functionName: 'updateDKIMRegistryOfWalletSalt',
    ...config,
  } as any)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link accountHandlerABI}__.
 */
export function usePrepareAccountHandlerWrite<TFunctionName extends string>(
  config: Omit<UsePrepareContractWriteConfig<typeof accountHandlerABI, TFunctionName>, 'abi'> = {} as any,
) {
  return usePrepareContractWrite({ abi: accountHandlerABI, ...config } as UsePrepareContractWriteConfig<
    typeof accountHandlerABI,
    TFunctionName
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"createAccount"`.
 */
export function usePrepareAccountHandlerCreateAccount(
  config: Omit<
    UsePrepareContractWriteConfig<typeof accountHandlerABI, 'createAccount'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: accountHandlerABI,
    functionName: 'createAccount',
    ...config,
  } as UsePrepareContractWriteConfig<typeof accountHandlerABI, 'createAccount'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"initializeAccount"`.
 */
export function usePrepareAccountHandlerInitializeAccount(
  config: Omit<
    UsePrepareContractWriteConfig<typeof accountHandlerABI, 'initializeAccount'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: accountHandlerABI,
    functionName: 'initializeAccount',
    ...config,
  } as UsePrepareContractWriteConfig<typeof accountHandlerABI, 'initializeAccount'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function usePrepareAccountHandlerRenounceOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof accountHandlerABI, 'renounceOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: accountHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof accountHandlerABI, 'renounceOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function usePrepareAccountHandlerTransferOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof accountHandlerABI, 'transferOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: accountHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof accountHandlerABI, 'transferOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"transportAccount"`.
 */
export function usePrepareAccountHandlerTransportAccount(
  config: Omit<
    UsePrepareContractWriteConfig<typeof accountHandlerABI, 'transportAccount'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: accountHandlerABI,
    functionName: 'transportAccount',
    ...config,
  } as UsePrepareContractWriteConfig<typeof accountHandlerABI, 'transportAccount'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link accountHandlerABI}__ and `functionName` set to `"updateDKIMRegistryOfWalletSalt"`.
 */
export function usePrepareAccountHandlerUpdateDkimRegistryOfWalletSalt(
  config: Omit<
    UsePrepareContractWriteConfig<typeof accountHandlerABI, 'updateDKIMRegistryOfWalletSalt'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: accountHandlerABI,
    functionName: 'updateDKIMRegistryOfWalletSalt',
    ...config,
  } as UsePrepareContractWriteConfig<typeof accountHandlerABI, 'updateDKIMRegistryOfWalletSalt'>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link accountHandlerABI}__.
 */
export function useAccountHandlerEvent<TEventName extends string>(
  config: Omit<UseContractEventConfig<typeof accountHandlerABI, TEventName>, 'abi'> = {} as any,
) {
  return useContractEvent({ abi: accountHandlerABI, ...config } as UseContractEventConfig<
    typeof accountHandlerABI,
    TEventName
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link accountHandlerABI}__ and `eventName` set to `"OwnershipTransferred"`.
 */
export function useAccountHandlerOwnershipTransferredEvent(
  config: Omit<
    UseContractEventConfig<typeof accountHandlerABI, 'OwnershipTransferred'>,
    'abi' | 'eventName'
  > = {} as any,
) {
  return useContractEvent({
    abi: accountHandlerABI,
    eventName: 'OwnershipTransferred',
    ...config,
  } as UseContractEventConfig<typeof accountHandlerABI, 'OwnershipTransferred'>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link erc20ABI}__.
 */
export function useErc20Read<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof erc20ABI, TFunctionName>,
>(config: Omit<UseContractReadConfig<typeof erc20ABI, TFunctionName, TSelectData>, 'abi'> = {} as any) {
  return useContractRead({ abi: erc20ABI, ...config } as UseContractReadConfig<
    typeof erc20ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"allowance"`.
 */
export function useErc20Allowance<
  TFunctionName extends 'allowance',
  TSelectData = ReadContractResult<typeof erc20ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof erc20ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: erc20ABI, functionName: 'allowance', ...config } as UseContractReadConfig<
    typeof erc20ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"balanceOf"`.
 */
export function useErc20BalanceOf<
  TFunctionName extends 'balanceOf',
  TSelectData = ReadContractResult<typeof erc20ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof erc20ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: erc20ABI, functionName: 'balanceOf', ...config } as UseContractReadConfig<
    typeof erc20ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"decimals"`.
 */
export function useErc20Decimals<
  TFunctionName extends 'decimals',
  TSelectData = ReadContractResult<typeof erc20ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof erc20ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: erc20ABI, functionName: 'decimals', ...config } as UseContractReadConfig<
    typeof erc20ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"name"`.
 */
export function useErc20Name<
  TFunctionName extends 'name',
  TSelectData = ReadContractResult<typeof erc20ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof erc20ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: erc20ABI, functionName: 'name', ...config } as UseContractReadConfig<
    typeof erc20ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"symbol"`.
 */
export function useErc20Symbol<
  TFunctionName extends 'symbol',
  TSelectData = ReadContractResult<typeof erc20ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof erc20ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: erc20ABI, functionName: 'symbol', ...config } as UseContractReadConfig<
    typeof erc20ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"totalSupply"`.
 */
export function useErc20TotalSupply<
  TFunctionName extends 'totalSupply',
  TSelectData = ReadContractResult<typeof erc20ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof erc20ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: erc20ABI, functionName: 'totalSupply', ...config } as UseContractReadConfig<
    typeof erc20ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link erc20ABI}__.
 */
export function useErc20Write<TFunctionName extends string, TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof erc20ABI, string>['request']['abi'],
        TFunctionName,
        TMode
      >
    : UseContractWriteConfig<typeof erc20ABI, TFunctionName, TMode> & {
        abi?: never
      } = {} as any,
) {
  return useContractWrite<typeof erc20ABI, TFunctionName, TMode>({ abi: erc20ABI, ...config } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"approve"`.
 */
export function useErc20Approve<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof erc20ABI, 'approve'>['request']['abi'],
        'approve',
        TMode
      > & { functionName?: 'approve' }
    : UseContractWriteConfig<typeof erc20ABI, 'approve', TMode> & {
        abi?: never
        functionName?: 'approve'
      } = {} as any,
) {
  return useContractWrite<typeof erc20ABI, 'approve', TMode>({
    abi: erc20ABI,
    functionName: 'approve',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"decreaseAllowance"`.
 */
export function useErc20DecreaseAllowance<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof erc20ABI, 'decreaseAllowance'>['request']['abi'],
        'decreaseAllowance',
        TMode
      > & { functionName?: 'decreaseAllowance' }
    : UseContractWriteConfig<typeof erc20ABI, 'decreaseAllowance', TMode> & {
        abi?: never
        functionName?: 'decreaseAllowance'
      } = {} as any,
) {
  return useContractWrite<typeof erc20ABI, 'decreaseAllowance', TMode>({
    abi: erc20ABI,
    functionName: 'decreaseAllowance',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"increaseAllowance"`.
 */
export function useErc20IncreaseAllowance<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof erc20ABI, 'increaseAllowance'>['request']['abi'],
        'increaseAllowance',
        TMode
      > & { functionName?: 'increaseAllowance' }
    : UseContractWriteConfig<typeof erc20ABI, 'increaseAllowance', TMode> & {
        abi?: never
        functionName?: 'increaseAllowance'
      } = {} as any,
) {
  return useContractWrite<typeof erc20ABI, 'increaseAllowance', TMode>({
    abi: erc20ABI,
    functionName: 'increaseAllowance',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"transfer"`.
 */
export function useErc20Transfer<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof erc20ABI, 'transfer'>['request']['abi'],
        'transfer',
        TMode
      > & { functionName?: 'transfer' }
    : UseContractWriteConfig<typeof erc20ABI, 'transfer', TMode> & {
        abi?: never
        functionName?: 'transfer'
      } = {} as any,
) {
  return useContractWrite<typeof erc20ABI, 'transfer', TMode>({
    abi: erc20ABI,
    functionName: 'transfer',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"transferFrom"`.
 */
export function useErc20TransferFrom<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof erc20ABI, 'transferFrom'>['request']['abi'],
        'transferFrom',
        TMode
      > & { functionName?: 'transferFrom' }
    : UseContractWriteConfig<typeof erc20ABI, 'transferFrom', TMode> & {
        abi?: never
        functionName?: 'transferFrom'
      } = {} as any,
) {
  return useContractWrite<typeof erc20ABI, 'transferFrom', TMode>({
    abi: erc20ABI,
    functionName: 'transferFrom',
    ...config,
  } as any)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link erc20ABI}__.
 */
export function usePrepareErc20Write<TFunctionName extends string>(
  config: Omit<UsePrepareContractWriteConfig<typeof erc20ABI, TFunctionName>, 'abi'> = {} as any,
) {
  return usePrepareContractWrite({ abi: erc20ABI, ...config } as UsePrepareContractWriteConfig<
    typeof erc20ABI,
    TFunctionName
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"approve"`.
 */
export function usePrepareErc20Approve(
  config: Omit<UsePrepareContractWriteConfig<typeof erc20ABI, 'approve'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({ abi: erc20ABI, functionName: 'approve', ...config } as UsePrepareContractWriteConfig<
    typeof erc20ABI,
    'approve'
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"decreaseAllowance"`.
 */
export function usePrepareErc20DecreaseAllowance(
  config: Omit<UsePrepareContractWriteConfig<typeof erc20ABI, 'decreaseAllowance'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({
    abi: erc20ABI,
    functionName: 'decreaseAllowance',
    ...config,
  } as UsePrepareContractWriteConfig<typeof erc20ABI, 'decreaseAllowance'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"increaseAllowance"`.
 */
export function usePrepareErc20IncreaseAllowance(
  config: Omit<UsePrepareContractWriteConfig<typeof erc20ABI, 'increaseAllowance'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({
    abi: erc20ABI,
    functionName: 'increaseAllowance',
    ...config,
  } as UsePrepareContractWriteConfig<typeof erc20ABI, 'increaseAllowance'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"transfer"`.
 */
export function usePrepareErc20Transfer(
  config: Omit<UsePrepareContractWriteConfig<typeof erc20ABI, 'transfer'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({
    abi: erc20ABI,
    functionName: 'transfer',
    ...config,
  } as UsePrepareContractWriteConfig<typeof erc20ABI, 'transfer'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link erc20ABI}__ and `functionName` set to `"transferFrom"`.
 */
export function usePrepareErc20TransferFrom(
  config: Omit<UsePrepareContractWriteConfig<typeof erc20ABI, 'transferFrom'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({
    abi: erc20ABI,
    functionName: 'transferFrom',
    ...config,
  } as UsePrepareContractWriteConfig<typeof erc20ABI, 'transferFrom'>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link erc20ABI}__.
 */
export function useErc20Event<TEventName extends string>(
  config: Omit<UseContractEventConfig<typeof erc20ABI, TEventName>, 'abi'> = {} as any,
) {
  return useContractEvent({ abi: erc20ABI, ...config } as UseContractEventConfig<typeof erc20ABI, TEventName>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link erc20ABI}__ and `eventName` set to `"Approval"`.
 */
export function useErc20ApprovalEvent(
  config: Omit<UseContractEventConfig<typeof erc20ABI, 'Approval'>, 'abi' | 'eventName'> = {} as any,
) {
  return useContractEvent({ abi: erc20ABI, eventName: 'Approval', ...config } as UseContractEventConfig<
    typeof erc20ABI,
    'Approval'
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link erc20ABI}__ and `eventName` set to `"Transfer"`.
 */
export function useErc20TransferEvent(
  config: Omit<UseContractEventConfig<typeof erc20ABI, 'Transfer'>, 'abi' | 'eventName'> = {} as any,
) {
  return useContractEvent({ abi: erc20ABI, eventName: 'Transfer', ...config } as UseContractEventConfig<
    typeof erc20ABI,
    'Transfer'
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__.
 */
export function useEmailWalletCoreRead<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(config: Omit<UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>, 'abi'> = {} as any) {
  return useContractRead({ abi: emailWalletCoreABI, ...config } as UseContractReadConfig<
    typeof emailWalletCoreABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"accountHandler"`.
 */
export function useEmailWalletCoreAccountHandler<
  TFunctionName extends 'accountHandler',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: emailWalletCoreABI,
    functionName: 'accountHandler',
    ...config,
  } as UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"emailNullifiers"`.
 */
export function useEmailWalletCoreEmailNullifiers<
  TFunctionName extends 'emailNullifiers',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: emailWalletCoreABI,
    functionName: 'emailNullifiers',
    ...config,
  } as UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"emailValidityDuration"`.
 */
export function useEmailWalletCoreEmailValidityDuration<
  TFunctionName extends 'emailValidityDuration',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: emailWalletCoreABI,
    functionName: 'emailValidityDuration',
    ...config,
  } as UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"extensionHandler"`.
 */
export function useEmailWalletCoreExtensionHandler<
  TFunctionName extends 'extensionHandler',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: emailWalletCoreABI,
    functionName: 'extensionHandler',
    ...config,
  } as UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"maxFeePerGas"`.
 */
export function useEmailWalletCoreMaxFeePerGas<
  TFunctionName extends 'maxFeePerGas',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: emailWalletCoreABI, functionName: 'maxFeePerGas', ...config } as UseContractReadConfig<
    typeof emailWalletCoreABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"priceOracle"`.
 */
export function useEmailWalletCorePriceOracle<
  TFunctionName extends 'priceOracle',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: emailWalletCoreABI, functionName: 'priceOracle', ...config } as UseContractReadConfig<
    typeof emailWalletCoreABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"relayerHandler"`.
 */
export function useEmailWalletCoreRelayerHandler<
  TFunctionName extends 'relayerHandler',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: emailWalletCoreABI,
    functionName: 'relayerHandler',
    ...config,
  } as UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"tokenRegistry"`.
 */
export function useEmailWalletCoreTokenRegistry<
  TFunctionName extends 'tokenRegistry',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: emailWalletCoreABI, functionName: 'tokenRegistry', ...config } as UseContractReadConfig<
    typeof emailWalletCoreABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"unclaimedFundClaimGas"`.
 */
export function useEmailWalletCoreUnclaimedFundClaimGas<
  TFunctionName extends 'unclaimedFundClaimGas',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: emailWalletCoreABI,
    functionName: 'unclaimedFundClaimGas',
    ...config,
  } as UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"unclaimedStateClaimGas"`.
 */
export function useEmailWalletCoreUnclaimedStateClaimGas<
  TFunctionName extends 'unclaimedStateClaimGas',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: emailWalletCoreABI,
    functionName: 'unclaimedStateClaimGas',
    ...config,
  } as UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"unclaimsHandler"`.
 */
export function useEmailWalletCoreUnclaimsHandler<
  TFunctionName extends 'unclaimsHandler',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: emailWalletCoreABI,
    functionName: 'unclaimsHandler',
    ...config,
  } as UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"validateEmailOp"`.
 */
export function useEmailWalletCoreValidateEmailOp<
  TFunctionName extends 'validateEmailOp',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: emailWalletCoreABI,
    functionName: 'validateEmailOp',
    ...config,
  } as UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"verifier"`.
 */
export function useEmailWalletCoreVerifier<
  TFunctionName extends 'verifier',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: emailWalletCoreABI, functionName: 'verifier', ...config } as UseContractReadConfig<
    typeof emailWalletCoreABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"wethContract"`.
 */
export function useEmailWalletCoreWethContract<
  TFunctionName extends 'wethContract',
  TSelectData = ReadContractResult<typeof emailWalletCoreABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof emailWalletCoreABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: emailWalletCoreABI, functionName: 'wethContract', ...config } as UseContractReadConfig<
    typeof emailWalletCoreABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__.
 */
export function useEmailWalletCoreWrite<TFunctionName extends string, TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof emailWalletCoreABI, string>['request']['abi'],
        TFunctionName,
        TMode
      >
    : UseContractWriteConfig<typeof emailWalletCoreABI, TFunctionName, TMode> & {
        abi?: never
      } = {} as any,
) {
  return useContractWrite<typeof emailWalletCoreABI, TFunctionName, TMode>({
    abi: emailWalletCoreABI,
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"depositTokenAsExtension"`.
 */
export function useEmailWalletCoreDepositTokenAsExtension<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof emailWalletCoreABI, 'depositTokenAsExtension'>['request']['abi'],
        'depositTokenAsExtension',
        TMode
      > & { functionName?: 'depositTokenAsExtension' }
    : UseContractWriteConfig<typeof emailWalletCoreABI, 'depositTokenAsExtension', TMode> & {
        abi?: never
        functionName?: 'depositTokenAsExtension'
      } = {} as any,
) {
  return useContractWrite<typeof emailWalletCoreABI, 'depositTokenAsExtension', TMode>({
    abi: emailWalletCoreABI,
    functionName: 'depositTokenAsExtension',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"executeAsExtension"`.
 */
export function useEmailWalletCoreExecuteAsExtension<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof emailWalletCoreABI, 'executeAsExtension'>['request']['abi'],
        'executeAsExtension',
        TMode
      > & { functionName?: 'executeAsExtension' }
    : UseContractWriteConfig<typeof emailWalletCoreABI, 'executeAsExtension', TMode> & {
        abi?: never
        functionName?: 'executeAsExtension'
      } = {} as any,
) {
  return useContractWrite<typeof emailWalletCoreABI, 'executeAsExtension', TMode>({
    abi: emailWalletCoreABI,
    functionName: 'executeAsExtension',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"handleEmailOp"`.
 */
export function useEmailWalletCoreHandleEmailOp<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof emailWalletCoreABI, 'handleEmailOp'>['request']['abi'],
        'handleEmailOp',
        TMode
      > & { functionName?: 'handleEmailOp' }
    : UseContractWriteConfig<typeof emailWalletCoreABI, 'handleEmailOp', TMode> & {
        abi?: never
        functionName?: 'handleEmailOp'
      } = {} as any,
) {
  return useContractWrite<typeof emailWalletCoreABI, 'handleEmailOp', TMode>({
    abi: emailWalletCoreABI,
    functionName: 'handleEmailOp',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"initialize"`.
 */
export function useEmailWalletCoreInitialize<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof emailWalletCoreABI, 'initialize'>['request']['abi'],
        'initialize',
        TMode
      > & { functionName?: 'initialize' }
    : UseContractWriteConfig<typeof emailWalletCoreABI, 'initialize', TMode> & {
        abi?: never
        functionName?: 'initialize'
      } = {} as any,
) {
  return useContractWrite<typeof emailWalletCoreABI, 'initialize', TMode>({
    abi: emailWalletCoreABI,
    functionName: 'initialize',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"registerUnclaimedStateAsExtension"`.
 */
export function useEmailWalletCoreRegisterUnclaimedStateAsExtension<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof emailWalletCoreABI, 'registerUnclaimedStateAsExtension'>['request']['abi'],
        'registerUnclaimedStateAsExtension',
        TMode
      > & { functionName?: 'registerUnclaimedStateAsExtension' }
    : UseContractWriteConfig<typeof emailWalletCoreABI, 'registerUnclaimedStateAsExtension', TMode> & {
        abi?: never
        functionName?: 'registerUnclaimedStateAsExtension'
      } = {} as any,
) {
  return useContractWrite<typeof emailWalletCoreABI, 'registerUnclaimedStateAsExtension', TMode>({
    abi: emailWalletCoreABI,
    functionName: 'registerUnclaimedStateAsExtension',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"requestTokenAsExtension"`.
 */
export function useEmailWalletCoreRequestTokenAsExtension<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof emailWalletCoreABI, 'requestTokenAsExtension'>['request']['abi'],
        'requestTokenAsExtension',
        TMode
      > & { functionName?: 'requestTokenAsExtension' }
    : UseContractWriteConfig<typeof emailWalletCoreABI, 'requestTokenAsExtension', TMode> & {
        abi?: never
        functionName?: 'requestTokenAsExtension'
      } = {} as any,
) {
  return useContractWrite<typeof emailWalletCoreABI, 'requestTokenAsExtension', TMode>({
    abi: emailWalletCoreABI,
    functionName: 'requestTokenAsExtension',
    ...config,
  } as any)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__.
 */
export function usePrepareEmailWalletCoreWrite<TFunctionName extends string>(
  config: Omit<UsePrepareContractWriteConfig<typeof emailWalletCoreABI, TFunctionName>, 'abi'> = {} as any,
) {
  return usePrepareContractWrite({ abi: emailWalletCoreABI, ...config } as UsePrepareContractWriteConfig<
    typeof emailWalletCoreABI,
    TFunctionName
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"depositTokenAsExtension"`.
 */
export function usePrepareEmailWalletCoreDepositTokenAsExtension(
  config: Omit<
    UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'depositTokenAsExtension'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: emailWalletCoreABI,
    functionName: 'depositTokenAsExtension',
    ...config,
  } as UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'depositTokenAsExtension'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"executeAsExtension"`.
 */
export function usePrepareEmailWalletCoreExecuteAsExtension(
  config: Omit<
    UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'executeAsExtension'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: emailWalletCoreABI,
    functionName: 'executeAsExtension',
    ...config,
  } as UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'executeAsExtension'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"handleEmailOp"`.
 */
export function usePrepareEmailWalletCoreHandleEmailOp(
  config: Omit<
    UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'handleEmailOp'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: emailWalletCoreABI,
    functionName: 'handleEmailOp',
    ...config,
  } as UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'handleEmailOp'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"initialize"`.
 */
export function usePrepareEmailWalletCoreInitialize(
  config: Omit<
    UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'initialize'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: emailWalletCoreABI,
    functionName: 'initialize',
    ...config,
  } as UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'initialize'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"registerUnclaimedStateAsExtension"`.
 */
export function usePrepareEmailWalletCoreRegisterUnclaimedStateAsExtension(
  config: Omit<
    UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'registerUnclaimedStateAsExtension'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: emailWalletCoreABI,
    functionName: 'registerUnclaimedStateAsExtension',
    ...config,
  } as UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'registerUnclaimedStateAsExtension'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link emailWalletCoreABI}__ and `functionName` set to `"requestTokenAsExtension"`.
 */
export function usePrepareEmailWalletCoreRequestTokenAsExtension(
  config: Omit<
    UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'requestTokenAsExtension'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: emailWalletCoreABI,
    functionName: 'requestTokenAsExtension',
    ...config,
  } as UsePrepareContractWriteConfig<typeof emailWalletCoreABI, 'requestTokenAsExtension'>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link extensionHandlerABI}__.
 */
export function useExtensionHandlerRead<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof extensionHandlerABI, TFunctionName>,
>(config: Omit<UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>, 'abi'> = {} as any) {
  return useContractRead({ abi: extensionHandlerABI, ...config } as UseContractReadConfig<
    typeof extensionHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"addressOfExtensionName"`.
 */
export function useExtensionHandlerAddressOfExtensionName<
  TFunctionName extends 'addressOfExtensionName',
  TSelectData = ReadContractResult<typeof extensionHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: extensionHandlerABI,
    functionName: 'addressOfExtensionName',
    ...config,
  } as UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"defaultExtensionOfCommand"`.
 */
export function useExtensionHandlerDefaultExtensionOfCommand<
  TFunctionName extends 'defaultExtensionOfCommand',
  TSelectData = ReadContractResult<typeof extensionHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: extensionHandlerABI,
    functionName: 'defaultExtensionOfCommand',
    ...config,
  } as UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"getExtensionForCommand"`.
 */
export function useExtensionHandlerGetExtensionForCommand<
  TFunctionName extends 'getExtensionForCommand',
  TSelectData = ReadContractResult<typeof extensionHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: extensionHandlerABI,
    functionName: 'getExtensionForCommand',
    ...config,
  } as UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"getSubjectTemplatesOfExtension"`.
 */
export function useExtensionHandlerGetSubjectTemplatesOfExtension<
  TFunctionName extends 'getSubjectTemplatesOfExtension',
  TSelectData = ReadContractResult<typeof extensionHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: extensionHandlerABI,
    functionName: 'getSubjectTemplatesOfExtension',
    ...config,
  } as UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"maxGasOfExtension"`.
 */
export function useExtensionHandlerMaxGasOfExtension<
  TFunctionName extends 'maxGasOfExtension',
  TSelectData = ReadContractResult<typeof extensionHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: extensionHandlerABI,
    functionName: 'maxGasOfExtension',
    ...config,
  } as UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"owner"`.
 */
export function useExtensionHandlerOwner<
  TFunctionName extends 'owner',
  TSelectData = ReadContractResult<typeof extensionHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: extensionHandlerABI, functionName: 'owner', ...config } as UseContractReadConfig<
    typeof extensionHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"subjectTemplatesOfExtension"`.
 */
export function useExtensionHandlerSubjectTemplatesOfExtension<
  TFunctionName extends 'subjectTemplatesOfExtension',
  TSelectData = ReadContractResult<typeof extensionHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: extensionHandlerABI,
    functionName: 'subjectTemplatesOfExtension',
    ...config,
  } as UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"userExtensionOfCommand"`.
 */
export function useExtensionHandlerUserExtensionOfCommand<
  TFunctionName extends 'userExtensionOfCommand',
  TSelectData = ReadContractResult<typeof extensionHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: extensionHandlerABI,
    functionName: 'userExtensionOfCommand',
    ...config,
  } as UseContractReadConfig<typeof extensionHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__.
 */
export function useExtensionHandlerWrite<TFunctionName extends string, TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof extensionHandlerABI, string>['request']['abi'],
        TFunctionName,
        TMode
      >
    : UseContractWriteConfig<typeof extensionHandlerABI, TFunctionName, TMode> & {
        abi?: never
      } = {} as any,
) {
  return useContractWrite<typeof extensionHandlerABI, TFunctionName, TMode>({
    abi: extensionHandlerABI,
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"publishExtension"`.
 */
export function useExtensionHandlerPublishExtension<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof extensionHandlerABI, 'publishExtension'>['request']['abi'],
        'publishExtension',
        TMode
      > & { functionName?: 'publishExtension' }
    : UseContractWriteConfig<typeof extensionHandlerABI, 'publishExtension', TMode> & {
        abi?: never
        functionName?: 'publishExtension'
      } = {} as any,
) {
  return useContractWrite<typeof extensionHandlerABI, 'publishExtension', TMode>({
    abi: extensionHandlerABI,
    functionName: 'publishExtension',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function useExtensionHandlerRenounceOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof extensionHandlerABI, 'renounceOwnership'>['request']['abi'],
        'renounceOwnership',
        TMode
      > & { functionName?: 'renounceOwnership' }
    : UseContractWriteConfig<typeof extensionHandlerABI, 'renounceOwnership', TMode> & {
        abi?: never
        functionName?: 'renounceOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof extensionHandlerABI, 'renounceOwnership', TMode>({
    abi: extensionHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"setDefaultExtensions"`.
 */
export function useExtensionHandlerSetDefaultExtensions<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof extensionHandlerABI, 'setDefaultExtensions'>['request']['abi'],
        'setDefaultExtensions',
        TMode
      > & { functionName?: 'setDefaultExtensions' }
    : UseContractWriteConfig<typeof extensionHandlerABI, 'setDefaultExtensions', TMode> & {
        abi?: never
        functionName?: 'setDefaultExtensions'
      } = {} as any,
) {
  return useContractWrite<typeof extensionHandlerABI, 'setDefaultExtensions', TMode>({
    abi: extensionHandlerABI,
    functionName: 'setDefaultExtensions',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"setExtensionForCommand"`.
 */
export function useExtensionHandlerSetExtensionForCommand<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof extensionHandlerABI, 'setExtensionForCommand'>['request']['abi'],
        'setExtensionForCommand',
        TMode
      > & { functionName?: 'setExtensionForCommand' }
    : UseContractWriteConfig<typeof extensionHandlerABI, 'setExtensionForCommand', TMode> & {
        abi?: never
        functionName?: 'setExtensionForCommand'
      } = {} as any,
) {
  return useContractWrite<typeof extensionHandlerABI, 'setExtensionForCommand', TMode>({
    abi: extensionHandlerABI,
    functionName: 'setExtensionForCommand',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function useExtensionHandlerTransferOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof extensionHandlerABI, 'transferOwnership'>['request']['abi'],
        'transferOwnership',
        TMode
      > & { functionName?: 'transferOwnership' }
    : UseContractWriteConfig<typeof extensionHandlerABI, 'transferOwnership', TMode> & {
        abi?: never
        functionName?: 'transferOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof extensionHandlerABI, 'transferOwnership', TMode>({
    abi: extensionHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__.
 */
export function usePrepareExtensionHandlerWrite<TFunctionName extends string>(
  config: Omit<UsePrepareContractWriteConfig<typeof extensionHandlerABI, TFunctionName>, 'abi'> = {} as any,
) {
  return usePrepareContractWrite({ abi: extensionHandlerABI, ...config } as UsePrepareContractWriteConfig<
    typeof extensionHandlerABI,
    TFunctionName
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"publishExtension"`.
 */
export function usePrepareExtensionHandlerPublishExtension(
  config: Omit<
    UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'publishExtension'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: extensionHandlerABI,
    functionName: 'publishExtension',
    ...config,
  } as UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'publishExtension'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function usePrepareExtensionHandlerRenounceOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'renounceOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: extensionHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'renounceOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"setDefaultExtensions"`.
 */
export function usePrepareExtensionHandlerSetDefaultExtensions(
  config: Omit<
    UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'setDefaultExtensions'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: extensionHandlerABI,
    functionName: 'setDefaultExtensions',
    ...config,
  } as UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'setDefaultExtensions'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"setExtensionForCommand"`.
 */
export function usePrepareExtensionHandlerSetExtensionForCommand(
  config: Omit<
    UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'setExtensionForCommand'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: extensionHandlerABI,
    functionName: 'setExtensionForCommand',
    ...config,
  } as UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'setExtensionForCommand'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link extensionHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function usePrepareExtensionHandlerTransferOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'transferOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: extensionHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof extensionHandlerABI, 'transferOwnership'>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link extensionHandlerABI}__.
 */
export function useExtensionHandlerEvent<TEventName extends string>(
  config: Omit<UseContractEventConfig<typeof extensionHandlerABI, TEventName>, 'abi'> = {} as any,
) {
  return useContractEvent({ abi: extensionHandlerABI, ...config } as UseContractEventConfig<
    typeof extensionHandlerABI,
    TEventName
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link extensionHandlerABI}__ and `eventName` set to `"OwnershipTransferred"`.
 */
export function useExtensionHandlerOwnershipTransferredEvent(
  config: Omit<
    UseContractEventConfig<typeof extensionHandlerABI, 'OwnershipTransferred'>,
    'abi' | 'eventName'
  > = {} as any,
) {
  return useContractEvent({
    abi: extensionHandlerABI,
    eventName: 'OwnershipTransferred',
    ...config,
  } as UseContractEventConfig<typeof extensionHandlerABI, 'OwnershipTransferred'>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link relayerHandlerABI}__.
 */
export function useRelayerHandlerRead<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof relayerHandlerABI, TFunctionName>,
>(config: Omit<UseContractReadConfig<typeof relayerHandlerABI, TFunctionName, TSelectData>, 'abi'> = {} as any) {
  return useContractRead({ abi: relayerHandlerABI, ...config } as UseContractReadConfig<
    typeof relayerHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"getRandHash"`.
 */
export function useRelayerHandlerGetRandHash<
  TFunctionName extends 'getRandHash',
  TSelectData = ReadContractResult<typeof relayerHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof relayerHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: relayerHandlerABI, functionName: 'getRandHash', ...config } as UseContractReadConfig<
    typeof relayerHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"owner"`.
 */
export function useRelayerHandlerOwner<
  TFunctionName extends 'owner',
  TSelectData = ReadContractResult<typeof relayerHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof relayerHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: relayerHandlerABI, functionName: 'owner', ...config } as UseContractReadConfig<
    typeof relayerHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"relayerOfEmailAddr"`.
 */
export function useRelayerHandlerRelayerOfEmailAddr<
  TFunctionName extends 'relayerOfEmailAddr',
  TSelectData = ReadContractResult<typeof relayerHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof relayerHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: relayerHandlerABI,
    functionName: 'relayerOfEmailAddr',
    ...config,
  } as UseContractReadConfig<typeof relayerHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"relayerOfRandHash"`.
 */
export function useRelayerHandlerRelayerOfRandHash<
  TFunctionName extends 'relayerOfRandHash',
  TSelectData = ReadContractResult<typeof relayerHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof relayerHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: relayerHandlerABI,
    functionName: 'relayerOfRandHash',
    ...config,
  } as UseContractReadConfig<typeof relayerHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"relayers"`.
 */
export function useRelayerHandlerRelayers<
  TFunctionName extends 'relayers',
  TSelectData = ReadContractResult<typeof relayerHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof relayerHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: relayerHandlerABI, functionName: 'relayers', ...config } as UseContractReadConfig<
    typeof relayerHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__.
 */
export function useRelayerHandlerWrite<TFunctionName extends string, TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof relayerHandlerABI, string>['request']['abi'],
        TFunctionName,
        TMode
      >
    : UseContractWriteConfig<typeof relayerHandlerABI, TFunctionName, TMode> & {
        abi?: never
      } = {} as any,
) {
  return useContractWrite<typeof relayerHandlerABI, TFunctionName, TMode>({ abi: relayerHandlerABI, ...config } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"registerRelayer"`.
 */
export function useRelayerHandlerRegisterRelayer<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof relayerHandlerABI, 'registerRelayer'>['request']['abi'],
        'registerRelayer',
        TMode
      > & { functionName?: 'registerRelayer' }
    : UseContractWriteConfig<typeof relayerHandlerABI, 'registerRelayer', TMode> & {
        abi?: never
        functionName?: 'registerRelayer'
      } = {} as any,
) {
  return useContractWrite<typeof relayerHandlerABI, 'registerRelayer', TMode>({
    abi: relayerHandlerABI,
    functionName: 'registerRelayer',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function useRelayerHandlerRenounceOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof relayerHandlerABI, 'renounceOwnership'>['request']['abi'],
        'renounceOwnership',
        TMode
      > & { functionName?: 'renounceOwnership' }
    : UseContractWriteConfig<typeof relayerHandlerABI, 'renounceOwnership', TMode> & {
        abi?: never
        functionName?: 'renounceOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof relayerHandlerABI, 'renounceOwnership', TMode>({
    abi: relayerHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function useRelayerHandlerTransferOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof relayerHandlerABI, 'transferOwnership'>['request']['abi'],
        'transferOwnership',
        TMode
      > & { functionName?: 'transferOwnership' }
    : UseContractWriteConfig<typeof relayerHandlerABI, 'transferOwnership', TMode> & {
        abi?: never
        functionName?: 'transferOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof relayerHandlerABI, 'transferOwnership', TMode>({
    abi: relayerHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"updateRelayerConfig"`.
 */
export function useRelayerHandlerUpdateRelayerConfig<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof relayerHandlerABI, 'updateRelayerConfig'>['request']['abi'],
        'updateRelayerConfig',
        TMode
      > & { functionName?: 'updateRelayerConfig' }
    : UseContractWriteConfig<typeof relayerHandlerABI, 'updateRelayerConfig', TMode> & {
        abi?: never
        functionName?: 'updateRelayerConfig'
      } = {} as any,
) {
  return useContractWrite<typeof relayerHandlerABI, 'updateRelayerConfig', TMode>({
    abi: relayerHandlerABI,
    functionName: 'updateRelayerConfig',
    ...config,
  } as any)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__.
 */
export function usePrepareRelayerHandlerWrite<TFunctionName extends string>(
  config: Omit<UsePrepareContractWriteConfig<typeof relayerHandlerABI, TFunctionName>, 'abi'> = {} as any,
) {
  return usePrepareContractWrite({ abi: relayerHandlerABI, ...config } as UsePrepareContractWriteConfig<
    typeof relayerHandlerABI,
    TFunctionName
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"registerRelayer"`.
 */
export function usePrepareRelayerHandlerRegisterRelayer(
  config: Omit<
    UsePrepareContractWriteConfig<typeof relayerHandlerABI, 'registerRelayer'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: relayerHandlerABI,
    functionName: 'registerRelayer',
    ...config,
  } as UsePrepareContractWriteConfig<typeof relayerHandlerABI, 'registerRelayer'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function usePrepareRelayerHandlerRenounceOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof relayerHandlerABI, 'renounceOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: relayerHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof relayerHandlerABI, 'renounceOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function usePrepareRelayerHandlerTransferOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof relayerHandlerABI, 'transferOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: relayerHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof relayerHandlerABI, 'transferOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link relayerHandlerABI}__ and `functionName` set to `"updateRelayerConfig"`.
 */
export function usePrepareRelayerHandlerUpdateRelayerConfig(
  config: Omit<
    UsePrepareContractWriteConfig<typeof relayerHandlerABI, 'updateRelayerConfig'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: relayerHandlerABI,
    functionName: 'updateRelayerConfig',
    ...config,
  } as UsePrepareContractWriteConfig<typeof relayerHandlerABI, 'updateRelayerConfig'>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link relayerHandlerABI}__.
 */
export function useRelayerHandlerEvent<TEventName extends string>(
  config: Omit<UseContractEventConfig<typeof relayerHandlerABI, TEventName>, 'abi'> = {} as any,
) {
  return useContractEvent({ abi: relayerHandlerABI, ...config } as UseContractEventConfig<
    typeof relayerHandlerABI,
    TEventName
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link relayerHandlerABI}__ and `eventName` set to `"OwnershipTransferred"`.
 */
export function useRelayerHandlerOwnershipTransferredEvent(
  config: Omit<
    UseContractEventConfig<typeof relayerHandlerABI, 'OwnershipTransferred'>,
    'abi' | 'eventName'
  > = {} as any,
) {
  return useContractEvent({
    abi: relayerHandlerABI,
    eventName: 'OwnershipTransferred',
    ...config,
  } as UseContractEventConfig<typeof relayerHandlerABI, 'OwnershipTransferred'>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenCallbackHandlerABI}__.
 */
export function useTokenCallbackHandlerRead<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof tokenCallbackHandlerABI, TFunctionName>,
>(config: Omit<UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>, 'abi'> = {} as any) {
  return useContractRead({ abi: tokenCallbackHandlerABI, ...config } as UseContractReadConfig<
    typeof tokenCallbackHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenCallbackHandlerABI}__ and `functionName` set to `"onERC1155BatchReceived"`.
 */
export function useTokenCallbackHandlerOnErc1155BatchReceived<
  TFunctionName extends 'onERC1155BatchReceived',
  TSelectData = ReadContractResult<typeof tokenCallbackHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: tokenCallbackHandlerABI,
    functionName: 'onERC1155BatchReceived',
    ...config,
  } as UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenCallbackHandlerABI}__ and `functionName` set to `"onERC1155Received"`.
 */
export function useTokenCallbackHandlerOnErc1155Received<
  TFunctionName extends 'onERC1155Received',
  TSelectData = ReadContractResult<typeof tokenCallbackHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: tokenCallbackHandlerABI,
    functionName: 'onERC1155Received',
    ...config,
  } as UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenCallbackHandlerABI}__ and `functionName` set to `"onERC721Received"`.
 */
export function useTokenCallbackHandlerOnErc721Received<
  TFunctionName extends 'onERC721Received',
  TSelectData = ReadContractResult<typeof tokenCallbackHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: tokenCallbackHandlerABI,
    functionName: 'onERC721Received',
    ...config,
  } as UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenCallbackHandlerABI}__ and `functionName` set to `"supportsInterface"`.
 */
export function useTokenCallbackHandlerSupportsInterface<
  TFunctionName extends 'supportsInterface',
  TSelectData = ReadContractResult<typeof tokenCallbackHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: tokenCallbackHandlerABI,
    functionName: 'supportsInterface',
    ...config,
  } as UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenCallbackHandlerABI}__ and `functionName` set to `"tokensReceived"`.
 */
export function useTokenCallbackHandlerTokensReceived<
  TFunctionName extends 'tokensReceived',
  TSelectData = ReadContractResult<typeof tokenCallbackHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: tokenCallbackHandlerABI,
    functionName: 'tokensReceived',
    ...config,
  } as UseContractReadConfig<typeof tokenCallbackHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenRegistryABI}__.
 */
export function useTokenRegistryRead<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof tokenRegistryABI, TFunctionName>,
>(config: Omit<UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>, 'abi'> = {} as any) {
  return useContractRead({ abi: tokenRegistryABI, ...config } as UseContractReadConfig<
    typeof tokenRegistryABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"addressOfTokenName"`.
 */
export function useTokenRegistryAddressOfTokenName<
  TFunctionName extends 'addressOfTokenName',
  TSelectData = ReadContractResult<typeof tokenRegistryABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: tokenRegistryABI,
    functionName: 'addressOfTokenName',
    ...config,
  } as UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"chainIdOfName"`.
 */
export function useTokenRegistryChainIdOfName<
  TFunctionName extends 'chainIdOfName',
  TSelectData = ReadContractResult<typeof tokenRegistryABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: tokenRegistryABI, functionName: 'chainIdOfName', ...config } as UseContractReadConfig<
    typeof tokenRegistryABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"getChainIdOfName"`.
 */
export function useTokenRegistryGetChainIdOfName<
  TFunctionName extends 'getChainIdOfName',
  TSelectData = ReadContractResult<typeof tokenRegistryABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: tokenRegistryABI,
    functionName: 'getChainIdOfName',
    ...config,
  } as UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"getTokenAddress"`.
 */
export function useTokenRegistryGetTokenAddress<
  TFunctionName extends 'getTokenAddress',
  TSelectData = ReadContractResult<typeof tokenRegistryABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: tokenRegistryABI, functionName: 'getTokenAddress', ...config } as UseContractReadConfig<
    typeof tokenRegistryABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"getTokenNameOfAddress"`.
 */
export function useTokenRegistryGetTokenNameOfAddress<
  TFunctionName extends 'getTokenNameOfAddress',
  TSelectData = ReadContractResult<typeof tokenRegistryABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: tokenRegistryABI,
    functionName: 'getTokenNameOfAddress',
    ...config,
  } as UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"owner"`.
 */
export function useTokenRegistryOwner<
  TFunctionName extends 'owner',
  TSelectData = ReadContractResult<typeof tokenRegistryABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: tokenRegistryABI, functionName: 'owner', ...config } as UseContractReadConfig<
    typeof tokenRegistryABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"tokenNameOfAddress"`.
 */
export function useTokenRegistryTokenNameOfAddress<
  TFunctionName extends 'tokenNameOfAddress',
  TSelectData = ReadContractResult<typeof tokenRegistryABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: tokenRegistryABI,
    functionName: 'tokenNameOfAddress',
    ...config,
  } as UseContractReadConfig<typeof tokenRegistryABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__.
 */
export function useTokenRegistryWrite<TFunctionName extends string, TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof tokenRegistryABI, string>['request']['abi'],
        TFunctionName,
        TMode
      >
    : UseContractWriteConfig<typeof tokenRegistryABI, TFunctionName, TMode> & {
        abi?: never
      } = {} as any,
) {
  return useContractWrite<typeof tokenRegistryABI, TFunctionName, TMode>({ abi: tokenRegistryABI, ...config } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function useTokenRegistryRenounceOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof tokenRegistryABI, 'renounceOwnership'>['request']['abi'],
        'renounceOwnership',
        TMode
      > & { functionName?: 'renounceOwnership' }
    : UseContractWriteConfig<typeof tokenRegistryABI, 'renounceOwnership', TMode> & {
        abi?: never
        functionName?: 'renounceOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof tokenRegistryABI, 'renounceOwnership', TMode>({
    abi: tokenRegistryABI,
    functionName: 'renounceOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"setChainId"`.
 */
export function useTokenRegistrySetChainId<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof tokenRegistryABI, 'setChainId'>['request']['abi'],
        'setChainId',
        TMode
      > & { functionName?: 'setChainId' }
    : UseContractWriteConfig<typeof tokenRegistryABI, 'setChainId', TMode> & {
        abi?: never
        functionName?: 'setChainId'
      } = {} as any,
) {
  return useContractWrite<typeof tokenRegistryABI, 'setChainId', TMode>({
    abi: tokenRegistryABI,
    functionName: 'setChainId',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"setTokenAddress"`.
 */
export function useTokenRegistrySetTokenAddress<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof tokenRegistryABI, 'setTokenAddress'>['request']['abi'],
        'setTokenAddress',
        TMode
      > & { functionName?: 'setTokenAddress' }
    : UseContractWriteConfig<typeof tokenRegistryABI, 'setTokenAddress', TMode> & {
        abi?: never
        functionName?: 'setTokenAddress'
      } = {} as any,
) {
  return useContractWrite<typeof tokenRegistryABI, 'setTokenAddress', TMode>({
    abi: tokenRegistryABI,
    functionName: 'setTokenAddress',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"setTokenAddresses"`.
 */
export function useTokenRegistrySetTokenAddresses<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof tokenRegistryABI, 'setTokenAddresses'>['request']['abi'],
        'setTokenAddresses',
        TMode
      > & { functionName?: 'setTokenAddresses' }
    : UseContractWriteConfig<typeof tokenRegistryABI, 'setTokenAddresses', TMode> & {
        abi?: never
        functionName?: 'setTokenAddresses'
      } = {} as any,
) {
  return useContractWrite<typeof tokenRegistryABI, 'setTokenAddresses', TMode>({
    abi: tokenRegistryABI,
    functionName: 'setTokenAddresses',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function useTokenRegistryTransferOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof tokenRegistryABI, 'transferOwnership'>['request']['abi'],
        'transferOwnership',
        TMode
      > & { functionName?: 'transferOwnership' }
    : UseContractWriteConfig<typeof tokenRegistryABI, 'transferOwnership', TMode> & {
        abi?: never
        functionName?: 'transferOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof tokenRegistryABI, 'transferOwnership', TMode>({
    abi: tokenRegistryABI,
    functionName: 'transferOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__.
 */
export function usePrepareTokenRegistryWrite<TFunctionName extends string>(
  config: Omit<UsePrepareContractWriteConfig<typeof tokenRegistryABI, TFunctionName>, 'abi'> = {} as any,
) {
  return usePrepareContractWrite({ abi: tokenRegistryABI, ...config } as UsePrepareContractWriteConfig<
    typeof tokenRegistryABI,
    TFunctionName
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function usePrepareTokenRegistryRenounceOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'renounceOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: tokenRegistryABI,
    functionName: 'renounceOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'renounceOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"setChainId"`.
 */
export function usePrepareTokenRegistrySetChainId(
  config: Omit<
    UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'setChainId'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: tokenRegistryABI,
    functionName: 'setChainId',
    ...config,
  } as UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'setChainId'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"setTokenAddress"`.
 */
export function usePrepareTokenRegistrySetTokenAddress(
  config: Omit<
    UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'setTokenAddress'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: tokenRegistryABI,
    functionName: 'setTokenAddress',
    ...config,
  } as UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'setTokenAddress'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"setTokenAddresses"`.
 */
export function usePrepareTokenRegistrySetTokenAddresses(
  config: Omit<
    UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'setTokenAddresses'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: tokenRegistryABI,
    functionName: 'setTokenAddresses',
    ...config,
  } as UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'setTokenAddresses'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link tokenRegistryABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function usePrepareTokenRegistryTransferOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'transferOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: tokenRegistryABI,
    functionName: 'transferOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof tokenRegistryABI, 'transferOwnership'>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link tokenRegistryABI}__.
 */
export function useTokenRegistryEvent<TEventName extends string>(
  config: Omit<UseContractEventConfig<typeof tokenRegistryABI, TEventName>, 'abi'> = {} as any,
) {
  return useContractEvent({ abi: tokenRegistryABI, ...config } as UseContractEventConfig<
    typeof tokenRegistryABI,
    TEventName
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link tokenRegistryABI}__ and `eventName` set to `"ChainRegistered"`.
 */
export function useTokenRegistryChainRegisteredEvent(
  config: Omit<UseContractEventConfig<typeof tokenRegistryABI, 'ChainRegistered'>, 'abi' | 'eventName'> = {} as any,
) {
  return useContractEvent({ abi: tokenRegistryABI, eventName: 'ChainRegistered', ...config } as UseContractEventConfig<
    typeof tokenRegistryABI,
    'ChainRegistered'
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link tokenRegistryABI}__ and `eventName` set to `"OwnershipTransferred"`.
 */
export function useTokenRegistryOwnershipTransferredEvent(
  config: Omit<
    UseContractEventConfig<typeof tokenRegistryABI, 'OwnershipTransferred'>,
    'abi' | 'eventName'
  > = {} as any,
) {
  return useContractEvent({
    abi: tokenRegistryABI,
    eventName: 'OwnershipTransferred',
    ...config,
  } as UseContractEventConfig<typeof tokenRegistryABI, 'OwnershipTransferred'>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link tokenRegistryABI}__ and `eventName` set to `"TokenRegistered"`.
 */
export function useTokenRegistryTokenRegisteredEvent(
  config: Omit<UseContractEventConfig<typeof tokenRegistryABI, 'TokenRegistered'>, 'abi' | 'eventName'> = {} as any,
) {
  return useContractEvent({ abi: tokenRegistryABI, eventName: 'TokenRegistered', ...config } as UseContractEventConfig<
    typeof tokenRegistryABI,
    'TokenRegistered'
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__.
 */
export function useUnclaimRegisterHandlerRead<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof unclaimRegisterHandlerABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>, 'abi'> = {} as any,
) {
  return useContractRead({ abi: unclaimRegisterHandlerABI, ...config } as UseContractReadConfig<
    typeof unclaimRegisterHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"getUnclaimedFundOfEmailAddrCommit"`.
 */
export function useUnclaimRegisterHandlerGetUnclaimedFundOfEmailAddrCommit<
  TFunctionName extends 'getUnclaimedFundOfEmailAddrCommit',
  TSelectData = ReadContractResult<typeof unclaimRegisterHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimRegisterHandlerABI,
    functionName: 'getUnclaimedFundOfEmailAddrCommit',
    ...config,
  } as UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"getUnclaimedStateOfEmailAddrCommit"`.
 */
export function useUnclaimRegisterHandlerGetUnclaimedStateOfEmailAddrCommit<
  TFunctionName extends 'getUnclaimedStateOfEmailAddrCommit',
  TSelectData = ReadContractResult<typeof unclaimRegisterHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimRegisterHandlerABI,
    functionName: 'getUnclaimedStateOfEmailAddrCommit',
    ...config,
  } as UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"owner"`.
 */
export function useUnclaimRegisterHandlerOwner<
  TFunctionName extends 'owner',
  TSelectData = ReadContractResult<typeof unclaimRegisterHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: unclaimRegisterHandlerABI, functionName: 'owner', ...config } as UseContractReadConfig<
    typeof unclaimRegisterHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"unclaimedFundOfEmailAddrCommit"`.
 */
export function useUnclaimRegisterHandlerUnclaimedFundOfEmailAddrCommit<
  TFunctionName extends 'unclaimedFundOfEmailAddrCommit',
  TSelectData = ReadContractResult<typeof unclaimRegisterHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimRegisterHandlerABI,
    functionName: 'unclaimedFundOfEmailAddrCommit',
    ...config,
  } as UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"unclaimedStateOfEmailAddrCommit"`.
 */
export function useUnclaimRegisterHandlerUnclaimedStateOfEmailAddrCommit<
  TFunctionName extends 'unclaimedStateOfEmailAddrCommit',
  TSelectData = ReadContractResult<typeof unclaimRegisterHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimRegisterHandlerABI,
    functionName: 'unclaimedStateOfEmailAddrCommit',
    ...config,
  } as UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"unclaimsExpiryDuration"`.
 */
export function useUnclaimRegisterHandlerUnclaimsExpiryDuration<
  TFunctionName extends 'unclaimsExpiryDuration',
  TSelectData = ReadContractResult<typeof unclaimRegisterHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimRegisterHandlerABI,
    functionName: 'unclaimsExpiryDuration',
    ...config,
  } as UseContractReadConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__.
 */
export function useUnclaimRegisterHandlerWrite<
  TFunctionName extends string,
  TMode extends WriteContractMode = undefined,
>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimRegisterHandlerABI, string>['request']['abi'],
        TFunctionName,
        TMode
      >
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, TFunctionName, TMode> & {
        abi?: never
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, TFunctionName, TMode>({
    abi: unclaimRegisterHandlerABI,
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"claimUnclaimedFundInternal"`.
 */
export function useUnclaimRegisterHandlerClaimUnclaimedFundInternal<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimRegisterHandlerABI, 'claimUnclaimedFundInternal'>['request']['abi'],
        'claimUnclaimedFundInternal',
        TMode
      > & { functionName?: 'claimUnclaimedFundInternal' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'claimUnclaimedFundInternal', TMode> & {
        abi?: never
        functionName?: 'claimUnclaimedFundInternal'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'claimUnclaimedFundInternal', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'claimUnclaimedFundInternal',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"claimUnclaimedStateInternal"`.
 */
export function useUnclaimRegisterHandlerClaimUnclaimedStateInternal<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimRegisterHandlerABI, 'claimUnclaimedStateInternal'>['request']['abi'],
        'claimUnclaimedStateInternal',
        TMode
      > & { functionName?: 'claimUnclaimedStateInternal' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'claimUnclaimedStateInternal', TMode> & {
        abi?: never
        functionName?: 'claimUnclaimedStateInternal'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'claimUnclaimedStateInternal', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'claimUnclaimedStateInternal',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"registerUnclaimedFundInternal"`.
 */
export function useUnclaimRegisterHandlerRegisterUnclaimedFundInternal<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimRegisterHandlerABI, 'registerUnclaimedFundInternal'>['request']['abi'],
        'registerUnclaimedFundInternal',
        TMode
      > & { functionName?: 'registerUnclaimedFundInternal' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'registerUnclaimedFundInternal', TMode> & {
        abi?: never
        functionName?: 'registerUnclaimedFundInternal'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'registerUnclaimedFundInternal', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'registerUnclaimedFundInternal',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"registerUnclaimedStateAsExtensionInternal"`.
 */
export function useUnclaimRegisterHandlerRegisterUnclaimedStateAsExtensionInternal<
  TMode extends WriteContractMode = undefined,
>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<
          typeof unclaimRegisterHandlerABI,
          'registerUnclaimedStateAsExtensionInternal'
        >['request']['abi'],
        'registerUnclaimedStateAsExtensionInternal',
        TMode
      > & { functionName?: 'registerUnclaimedStateAsExtensionInternal' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'registerUnclaimedStateAsExtensionInternal', TMode> & {
        abi?: never
        functionName?: 'registerUnclaimedStateAsExtensionInternal'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'registerUnclaimedStateAsExtensionInternal', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'registerUnclaimedStateAsExtensionInternal',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"registerUnclaimedStateInternal"`.
 */
export function useUnclaimRegisterHandlerRegisterUnclaimedStateInternal<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<
          typeof unclaimRegisterHandlerABI,
          'registerUnclaimedStateInternal'
        >['request']['abi'],
        'registerUnclaimedStateInternal',
        TMode
      > & { functionName?: 'registerUnclaimedStateInternal' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'registerUnclaimedStateInternal', TMode> & {
        abi?: never
        functionName?: 'registerUnclaimedStateInternal'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'registerUnclaimedStateInternal', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'registerUnclaimedStateInternal',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function useUnclaimRegisterHandlerRenounceOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimRegisterHandlerABI, 'renounceOwnership'>['request']['abi'],
        'renounceOwnership',
        TMode
      > & { functionName?: 'renounceOwnership' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'renounceOwnership', TMode> & {
        abi?: never
        functionName?: 'renounceOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'renounceOwnership', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function useUnclaimRegisterHandlerTransferOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimRegisterHandlerABI, 'transferOwnership'>['request']['abi'],
        'transferOwnership',
        TMode
      > & { functionName?: 'transferOwnership' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'transferOwnership', TMode> & {
        abi?: never
        functionName?: 'transferOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'transferOwnership', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"updateUnclaimFund"`.
 */
export function useUnclaimRegisterHandlerUpdateUnclaimFund<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimRegisterHandlerABI, 'updateUnclaimFund'>['request']['abi'],
        'updateUnclaimFund',
        TMode
      > & { functionName?: 'updateUnclaimFund' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'updateUnclaimFund', TMode> & {
        abi?: never
        functionName?: 'updateUnclaimFund'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'updateUnclaimFund', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'updateUnclaimFund',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"voidUnclaimedFundInternal"`.
 */
export function useUnclaimRegisterHandlerVoidUnclaimedFundInternal<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimRegisterHandlerABI, 'voidUnclaimedFundInternal'>['request']['abi'],
        'voidUnclaimedFundInternal',
        TMode
      > & { functionName?: 'voidUnclaimedFundInternal' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'voidUnclaimedFundInternal', TMode> & {
        abi?: never
        functionName?: 'voidUnclaimedFundInternal'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'voidUnclaimedFundInternal', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'voidUnclaimedFundInternal',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"voidUnclaimedStateInternal"`.
 */
export function useUnclaimRegisterHandlerVoidUnclaimedStateInternal<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimRegisterHandlerABI, 'voidUnclaimedStateInternal'>['request']['abi'],
        'voidUnclaimedStateInternal',
        TMode
      > & { functionName?: 'voidUnclaimedStateInternal' }
    : UseContractWriteConfig<typeof unclaimRegisterHandlerABI, 'voidUnclaimedStateInternal', TMode> & {
        abi?: never
        functionName?: 'voidUnclaimedStateInternal'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimRegisterHandlerABI, 'voidUnclaimedStateInternal', TMode>({
    abi: unclaimRegisterHandlerABI,
    functionName: 'voidUnclaimedStateInternal',
    ...config,
  } as any)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__.
 */
export function usePrepareUnclaimRegisterHandlerWrite<TFunctionName extends string>(
  config: Omit<UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, TFunctionName>, 'abi'> = {} as any,
) {
  return usePrepareContractWrite({ abi: unclaimRegisterHandlerABI, ...config } as UsePrepareContractWriteConfig<
    typeof unclaimRegisterHandlerABI,
    TFunctionName
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"claimUnclaimedFundInternal"`.
 */
export function usePrepareUnclaimRegisterHandlerClaimUnclaimedFundInternal(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'claimUnclaimedFundInternal'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'claimUnclaimedFundInternal',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'claimUnclaimedFundInternal'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"claimUnclaimedStateInternal"`.
 */
export function usePrepareUnclaimRegisterHandlerClaimUnclaimedStateInternal(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'claimUnclaimedStateInternal'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'claimUnclaimedStateInternal',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'claimUnclaimedStateInternal'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"registerUnclaimedFundInternal"`.
 */
export function usePrepareUnclaimRegisterHandlerRegisterUnclaimedFundInternal(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'registerUnclaimedFundInternal'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'registerUnclaimedFundInternal',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'registerUnclaimedFundInternal'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"registerUnclaimedStateAsExtensionInternal"`.
 */
export function usePrepareUnclaimRegisterHandlerRegisterUnclaimedStateAsExtensionInternal(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'registerUnclaimedStateAsExtensionInternal'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'registerUnclaimedStateAsExtensionInternal',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'registerUnclaimedStateAsExtensionInternal'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"registerUnclaimedStateInternal"`.
 */
export function usePrepareUnclaimRegisterHandlerRegisterUnclaimedStateInternal(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'registerUnclaimedStateInternal'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'registerUnclaimedStateInternal',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'registerUnclaimedStateInternal'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function usePrepareUnclaimRegisterHandlerRenounceOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'renounceOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'renounceOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function usePrepareUnclaimRegisterHandlerTransferOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'transferOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'transferOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"updateUnclaimFund"`.
 */
export function usePrepareUnclaimRegisterHandlerUpdateUnclaimFund(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'updateUnclaimFund'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'updateUnclaimFund',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'updateUnclaimFund'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"voidUnclaimedFundInternal"`.
 */
export function usePrepareUnclaimRegisterHandlerVoidUnclaimedFundInternal(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'voidUnclaimedFundInternal'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'voidUnclaimedFundInternal',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'voidUnclaimedFundInternal'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `functionName` set to `"voidUnclaimedStateInternal"`.
 */
export function usePrepareUnclaimRegisterHandlerVoidUnclaimedStateInternal(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'voidUnclaimedStateInternal'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimRegisterHandlerABI,
    functionName: 'voidUnclaimedStateInternal',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimRegisterHandlerABI, 'voidUnclaimedStateInternal'>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__.
 */
export function useUnclaimRegisterHandlerEvent<TEventName extends string>(
  config: Omit<UseContractEventConfig<typeof unclaimRegisterHandlerABI, TEventName>, 'abi'> = {} as any,
) {
  return useContractEvent({ abi: unclaimRegisterHandlerABI, ...config } as UseContractEventConfig<
    typeof unclaimRegisterHandlerABI,
    TEventName
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link unclaimRegisterHandlerABI}__ and `eventName` set to `"OwnershipTransferred"`.
 */
export function useUnclaimRegisterHandlerOwnershipTransferredEvent(
  config: Omit<
    UseContractEventConfig<typeof unclaimRegisterHandlerABI, 'OwnershipTransferred'>,
    'abi' | 'eventName'
  > = {} as any,
) {
  return useContractEvent({
    abi: unclaimRegisterHandlerABI,
    eventName: 'OwnershipTransferred',
    ...config,
  } as UseContractEventConfig<typeof unclaimRegisterHandlerABI, 'OwnershipTransferred'>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__.
 */
export function useUnclaimsHandlerRead<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(config: Omit<UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>, 'abi'> = {} as any) {
  return useContractRead({ abi: unclaimsHandlerABI, ...config } as UseContractReadConfig<
    typeof unclaimsHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"accountHandler"`.
 */
export function useUnclaimsHandlerAccountHandler<
  TFunctionName extends 'accountHandler',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'accountHandler',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"getUnclaimedFund"`.
 */
export function useUnclaimsHandlerGetUnclaimedFund<
  TFunctionName extends 'getUnclaimedFund',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'getUnclaimedFund',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"getUnclaimedState"`.
 */
export function useUnclaimsHandlerGetUnclaimedState<
  TFunctionName extends 'getUnclaimedState',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'getUnclaimedState',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"maxFeePerGas"`.
 */
export function useUnclaimsHandlerMaxFeePerGas<
  TFunctionName extends 'maxFeePerGas',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: unclaimsHandlerABI, functionName: 'maxFeePerGas', ...config } as UseContractReadConfig<
    typeof unclaimsHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"numUnclaimedFunds"`.
 */
export function useUnclaimsHandlerNumUnclaimedFunds<
  TFunctionName extends 'numUnclaimedFunds',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'numUnclaimedFunds',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"numUnclaimedStates"`.
 */
export function useUnclaimsHandlerNumUnclaimedStates<
  TFunctionName extends 'numUnclaimedStates',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'numUnclaimedStates',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"owner"`.
 */
export function useUnclaimsHandlerOwner<
  TFunctionName extends 'owner',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: unclaimsHandlerABI, functionName: 'owner', ...config } as UseContractReadConfig<
    typeof unclaimsHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"relayerHandler"`.
 */
export function useUnclaimsHandlerRelayerHandler<
  TFunctionName extends 'relayerHandler',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'relayerHandler',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"unclaimedFundClaimGas"`.
 */
export function useUnclaimsHandlerUnclaimedFundClaimGas<
  TFunctionName extends 'unclaimedFundClaimGas',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'unclaimedFundClaimGas',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"unclaimedFundOfId"`.
 */
export function useUnclaimsHandlerUnclaimedFundOfId<
  TFunctionName extends 'unclaimedFundOfId',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'unclaimedFundOfId',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"unclaimedStateClaimGas"`.
 */
export function useUnclaimsHandlerUnclaimedStateClaimGas<
  TFunctionName extends 'unclaimedStateClaimGas',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'unclaimedStateClaimGas',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"unclaimedStateOfId"`.
 */
export function useUnclaimsHandlerUnclaimedStateOfId<
  TFunctionName extends 'unclaimedStateOfId',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'unclaimedStateOfId',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"unclaimsExpiryDuration"`.
 */
export function useUnclaimsHandlerUnclaimsExpiryDuration<
  TFunctionName extends 'unclaimsExpiryDuration',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({
    abi: unclaimsHandlerABI,
    functionName: 'unclaimsExpiryDuration',
    ...config,
  } as UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"verifier"`.
 */
export function useUnclaimsHandlerVerifier<
  TFunctionName extends 'verifier',
  TSelectData = ReadContractResult<typeof unclaimsHandlerABI, TFunctionName>,
>(
  config: Omit<
    UseContractReadConfig<typeof unclaimsHandlerABI, TFunctionName, TSelectData>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return useContractRead({ abi: unclaimsHandlerABI, functionName: 'verifier', ...config } as UseContractReadConfig<
    typeof unclaimsHandlerABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__.
 */
export function useUnclaimsHandlerWrite<TFunctionName extends string, TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, string>['request']['abi'],
        TFunctionName,
        TMode
      >
    : UseContractWriteConfig<typeof unclaimsHandlerABI, TFunctionName, TMode> & {
        abi?: never
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, TFunctionName, TMode>({
    abi: unclaimsHandlerABI,
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"claimUnclaimedFund"`.
 */
export function useUnclaimsHandlerClaimUnclaimedFund<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'claimUnclaimedFund'>['request']['abi'],
        'claimUnclaimedFund',
        TMode
      > & { functionName?: 'claimUnclaimedFund' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'claimUnclaimedFund', TMode> & {
        abi?: never
        functionName?: 'claimUnclaimedFund'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'claimUnclaimedFund', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'claimUnclaimedFund',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"claimUnclaimedState"`.
 */
export function useUnclaimsHandlerClaimUnclaimedState<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'claimUnclaimedState'>['request']['abi'],
        'claimUnclaimedState',
        TMode
      > & { functionName?: 'claimUnclaimedState' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'claimUnclaimedState', TMode> & {
        abi?: never
        functionName?: 'claimUnclaimedState'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'claimUnclaimedState', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'claimUnclaimedState',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"registerUnclaimedFund"`.
 */
export function useUnclaimsHandlerRegisterUnclaimedFund<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'registerUnclaimedFund'>['request']['abi'],
        'registerUnclaimedFund',
        TMode
      > & { functionName?: 'registerUnclaimedFund' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedFund', TMode> & {
        abi?: never
        functionName?: 'registerUnclaimedFund'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'registerUnclaimedFund', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'registerUnclaimedFund',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"registerUnclaimedFundInternal"`.
 */
export function useUnclaimsHandlerRegisterUnclaimedFundInternal<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'registerUnclaimedFundInternal'>['request']['abi'],
        'registerUnclaimedFundInternal',
        TMode
      > & { functionName?: 'registerUnclaimedFundInternal' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedFundInternal', TMode> & {
        abi?: never
        functionName?: 'registerUnclaimedFundInternal'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'registerUnclaimedFundInternal', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'registerUnclaimedFundInternal',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"registerUnclaimedState"`.
 */
export function useUnclaimsHandlerRegisterUnclaimedState<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'registerUnclaimedState'>['request']['abi'],
        'registerUnclaimedState',
        TMode
      > & { functionName?: 'registerUnclaimedState' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedState', TMode> & {
        abi?: never
        functionName?: 'registerUnclaimedState'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'registerUnclaimedState', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'registerUnclaimedState',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"registerUnclaimedStateInternal"`.
 */
export function useUnclaimsHandlerRegisterUnclaimedStateInternal<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'registerUnclaimedStateInternal'>['request']['abi'],
        'registerUnclaimedStateInternal',
        TMode
      > & { functionName?: 'registerUnclaimedStateInternal' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedStateInternal', TMode> & {
        abi?: never
        functionName?: 'registerUnclaimedStateInternal'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'registerUnclaimedStateInternal', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'registerUnclaimedStateInternal',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function useUnclaimsHandlerRenounceOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'renounceOwnership'>['request']['abi'],
        'renounceOwnership',
        TMode
      > & { functionName?: 'renounceOwnership' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'renounceOwnership', TMode> & {
        abi?: never
        functionName?: 'renounceOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'renounceOwnership', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function useUnclaimsHandlerTransferOwnership<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'transferOwnership'>['request']['abi'],
        'transferOwnership',
        TMode
      > & { functionName?: 'transferOwnership' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'transferOwnership', TMode> & {
        abi?: never
        functionName?: 'transferOwnership'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'transferOwnership', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"voidUnclaimedFund"`.
 */
export function useUnclaimsHandlerVoidUnclaimedFund<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'voidUnclaimedFund'>['request']['abi'],
        'voidUnclaimedFund',
        TMode
      > & { functionName?: 'voidUnclaimedFund' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'voidUnclaimedFund', TMode> & {
        abi?: never
        functionName?: 'voidUnclaimedFund'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'voidUnclaimedFund', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'voidUnclaimedFund',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"voidUnclaimedState"`.
 */
export function useUnclaimsHandlerVoidUnclaimedState<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof unclaimsHandlerABI, 'voidUnclaimedState'>['request']['abi'],
        'voidUnclaimedState',
        TMode
      > & { functionName?: 'voidUnclaimedState' }
    : UseContractWriteConfig<typeof unclaimsHandlerABI, 'voidUnclaimedState', TMode> & {
        abi?: never
        functionName?: 'voidUnclaimedState'
      } = {} as any,
) {
  return useContractWrite<typeof unclaimsHandlerABI, 'voidUnclaimedState', TMode>({
    abi: unclaimsHandlerABI,
    functionName: 'voidUnclaimedState',
    ...config,
  } as any)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__.
 */
export function usePrepareUnclaimsHandlerWrite<TFunctionName extends string>(
  config: Omit<UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, TFunctionName>, 'abi'> = {} as any,
) {
  return usePrepareContractWrite({ abi: unclaimsHandlerABI, ...config } as UsePrepareContractWriteConfig<
    typeof unclaimsHandlerABI,
    TFunctionName
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"claimUnclaimedFund"`.
 */
export function usePrepareUnclaimsHandlerClaimUnclaimedFund(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'claimUnclaimedFund'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'claimUnclaimedFund',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'claimUnclaimedFund'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"claimUnclaimedState"`.
 */
export function usePrepareUnclaimsHandlerClaimUnclaimedState(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'claimUnclaimedState'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'claimUnclaimedState',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'claimUnclaimedState'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"registerUnclaimedFund"`.
 */
export function usePrepareUnclaimsHandlerRegisterUnclaimedFund(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedFund'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'registerUnclaimedFund',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedFund'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"registerUnclaimedFundInternal"`.
 */
export function usePrepareUnclaimsHandlerRegisterUnclaimedFundInternal(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedFundInternal'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'registerUnclaimedFundInternal',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedFundInternal'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"registerUnclaimedState"`.
 */
export function usePrepareUnclaimsHandlerRegisterUnclaimedState(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedState'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'registerUnclaimedState',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedState'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"registerUnclaimedStateInternal"`.
 */
export function usePrepareUnclaimsHandlerRegisterUnclaimedStateInternal(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedStateInternal'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'registerUnclaimedStateInternal',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'registerUnclaimedStateInternal'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"renounceOwnership"`.
 */
export function usePrepareUnclaimsHandlerRenounceOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'renounceOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'renounceOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'renounceOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"transferOwnership"`.
 */
export function usePrepareUnclaimsHandlerTransferOwnership(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'transferOwnership'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'transferOwnership',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'transferOwnership'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"voidUnclaimedFund"`.
 */
export function usePrepareUnclaimsHandlerVoidUnclaimedFund(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'voidUnclaimedFund'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'voidUnclaimedFund',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'voidUnclaimedFund'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `functionName` set to `"voidUnclaimedState"`.
 */
export function usePrepareUnclaimsHandlerVoidUnclaimedState(
  config: Omit<
    UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'voidUnclaimedState'>,
    'abi' | 'functionName'
  > = {} as any,
) {
  return usePrepareContractWrite({
    abi: unclaimsHandlerABI,
    functionName: 'voidUnclaimedState',
    ...config,
  } as UsePrepareContractWriteConfig<typeof unclaimsHandlerABI, 'voidUnclaimedState'>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link unclaimsHandlerABI}__.
 */
export function useUnclaimsHandlerEvent<TEventName extends string>(
  config: Omit<UseContractEventConfig<typeof unclaimsHandlerABI, TEventName>, 'abi'> = {} as any,
) {
  return useContractEvent({ abi: unclaimsHandlerABI, ...config } as UseContractEventConfig<
    typeof unclaimsHandlerABI,
    TEventName
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link unclaimsHandlerABI}__ and `eventName` set to `"OwnershipTransferred"`.
 */
export function useUnclaimsHandlerOwnershipTransferredEvent(
  config: Omit<
    UseContractEventConfig<typeof unclaimsHandlerABI, 'OwnershipTransferred'>,
    'abi' | 'eventName'
  > = {} as any,
) {
  return useContractEvent({
    abi: unclaimsHandlerABI,
    eventName: 'OwnershipTransferred',
    ...config,
  } as UseContractEventConfig<typeof unclaimsHandlerABI, 'OwnershipTransferred'>)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link weth9ABI}__.
 */
export function useWeth9Read<
  TFunctionName extends string,
  TSelectData = ReadContractResult<typeof weth9ABI, TFunctionName>,
>(config: Omit<UseContractReadConfig<typeof weth9ABI, TFunctionName, TSelectData>, 'abi'> = {} as any) {
  return useContractRead({ abi: weth9ABI, ...config } as UseContractReadConfig<
    typeof weth9ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"allowance"`.
 */
export function useWeth9Allowance<
  TFunctionName extends 'allowance',
  TSelectData = ReadContractResult<typeof weth9ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof weth9ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: weth9ABI, functionName: 'allowance', ...config } as UseContractReadConfig<
    typeof weth9ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"balanceOf"`.
 */
export function useWeth9BalanceOf<
  TFunctionName extends 'balanceOf',
  TSelectData = ReadContractResult<typeof weth9ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof weth9ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: weth9ABI, functionName: 'balanceOf', ...config } as UseContractReadConfig<
    typeof weth9ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"decimals"`.
 */
export function useWeth9Decimals<
  TFunctionName extends 'decimals',
  TSelectData = ReadContractResult<typeof weth9ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof weth9ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: weth9ABI, functionName: 'decimals', ...config } as UseContractReadConfig<
    typeof weth9ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"name"`.
 */
export function useWeth9Name<
  TFunctionName extends 'name',
  TSelectData = ReadContractResult<typeof weth9ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof weth9ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: weth9ABI, functionName: 'name', ...config } as UseContractReadConfig<
    typeof weth9ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"symbol"`.
 */
export function useWeth9Symbol<
  TFunctionName extends 'symbol',
  TSelectData = ReadContractResult<typeof weth9ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof weth9ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: weth9ABI, functionName: 'symbol', ...config } as UseContractReadConfig<
    typeof weth9ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractRead}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"totalSupply"`.
 */
export function useWeth9TotalSupply<
  TFunctionName extends 'totalSupply',
  TSelectData = ReadContractResult<typeof weth9ABI, TFunctionName>,
>(
  config: Omit<UseContractReadConfig<typeof weth9ABI, TFunctionName, TSelectData>, 'abi' | 'functionName'> = {} as any,
) {
  return useContractRead({ abi: weth9ABI, functionName: 'totalSupply', ...config } as UseContractReadConfig<
    typeof weth9ABI,
    TFunctionName,
    TSelectData
  >)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link weth9ABI}__.
 */
export function useWeth9Write<TFunctionName extends string, TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof weth9ABI, string>['request']['abi'],
        TFunctionName,
        TMode
      >
    : UseContractWriteConfig<typeof weth9ABI, TFunctionName, TMode> & {
        abi?: never
      } = {} as any,
) {
  return useContractWrite<typeof weth9ABI, TFunctionName, TMode>({ abi: weth9ABI, ...config } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"approve"`.
 */
export function useWeth9Approve<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof weth9ABI, 'approve'>['request']['abi'],
        'approve',
        TMode
      > & { functionName?: 'approve' }
    : UseContractWriteConfig<typeof weth9ABI, 'approve', TMode> & {
        abi?: never
        functionName?: 'approve'
      } = {} as any,
) {
  return useContractWrite<typeof weth9ABI, 'approve', TMode>({
    abi: weth9ABI,
    functionName: 'approve',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"deposit"`.
 */
export function useWeth9Deposit<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof weth9ABI, 'deposit'>['request']['abi'],
        'deposit',
        TMode
      > & { functionName?: 'deposit' }
    : UseContractWriteConfig<typeof weth9ABI, 'deposit', TMode> & {
        abi?: never
        functionName?: 'deposit'
      } = {} as any,
) {
  return useContractWrite<typeof weth9ABI, 'deposit', TMode>({
    abi: weth9ABI,
    functionName: 'deposit',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"transfer"`.
 */
export function useWeth9Transfer<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof weth9ABI, 'transfer'>['request']['abi'],
        'transfer',
        TMode
      > & { functionName?: 'transfer' }
    : UseContractWriteConfig<typeof weth9ABI, 'transfer', TMode> & {
        abi?: never
        functionName?: 'transfer'
      } = {} as any,
) {
  return useContractWrite<typeof weth9ABI, 'transfer', TMode>({
    abi: weth9ABI,
    functionName: 'transfer',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"transferFrom"`.
 */
export function useWeth9TransferFrom<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof weth9ABI, 'transferFrom'>['request']['abi'],
        'transferFrom',
        TMode
      > & { functionName?: 'transferFrom' }
    : UseContractWriteConfig<typeof weth9ABI, 'transferFrom', TMode> & {
        abi?: never
        functionName?: 'transferFrom'
      } = {} as any,
) {
  return useContractWrite<typeof weth9ABI, 'transferFrom', TMode>({
    abi: weth9ABI,
    functionName: 'transferFrom',
    ...config,
  } as any)
}

/**
 * Wraps __{@link useContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"withdraw"`.
 */
export function useWeth9Withdraw<TMode extends WriteContractMode = undefined>(
  config: TMode extends 'prepared'
    ? UseContractWriteConfig<
        PrepareWriteContractResult<typeof weth9ABI, 'withdraw'>['request']['abi'],
        'withdraw',
        TMode
      > & { functionName?: 'withdraw' }
    : UseContractWriteConfig<typeof weth9ABI, 'withdraw', TMode> & {
        abi?: never
        functionName?: 'withdraw'
      } = {} as any,
) {
  return useContractWrite<typeof weth9ABI, 'withdraw', TMode>({
    abi: weth9ABI,
    functionName: 'withdraw',
    ...config,
  } as any)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link weth9ABI}__.
 */
export function usePrepareWeth9Write<TFunctionName extends string>(
  config: Omit<UsePrepareContractWriteConfig<typeof weth9ABI, TFunctionName>, 'abi'> = {} as any,
) {
  return usePrepareContractWrite({ abi: weth9ABI, ...config } as UsePrepareContractWriteConfig<
    typeof weth9ABI,
    TFunctionName
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"approve"`.
 */
export function usePrepareWeth9Approve(
  config: Omit<UsePrepareContractWriteConfig<typeof weth9ABI, 'approve'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({ abi: weth9ABI, functionName: 'approve', ...config } as UsePrepareContractWriteConfig<
    typeof weth9ABI,
    'approve'
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"deposit"`.
 */
export function usePrepareWeth9Deposit(
  config: Omit<UsePrepareContractWriteConfig<typeof weth9ABI, 'deposit'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({ abi: weth9ABI, functionName: 'deposit', ...config } as UsePrepareContractWriteConfig<
    typeof weth9ABI,
    'deposit'
  >)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"transfer"`.
 */
export function usePrepareWeth9Transfer(
  config: Omit<UsePrepareContractWriteConfig<typeof weth9ABI, 'transfer'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({
    abi: weth9ABI,
    functionName: 'transfer',
    ...config,
  } as UsePrepareContractWriteConfig<typeof weth9ABI, 'transfer'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"transferFrom"`.
 */
export function usePrepareWeth9TransferFrom(
  config: Omit<UsePrepareContractWriteConfig<typeof weth9ABI, 'transferFrom'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({
    abi: weth9ABI,
    functionName: 'transferFrom',
    ...config,
  } as UsePrepareContractWriteConfig<typeof weth9ABI, 'transferFrom'>)
}

/**
 * Wraps __{@link usePrepareContractWrite}__ with `abi` set to __{@link weth9ABI}__ and `functionName` set to `"withdraw"`.
 */
export function usePrepareWeth9Withdraw(
  config: Omit<UsePrepareContractWriteConfig<typeof weth9ABI, 'withdraw'>, 'abi' | 'functionName'> = {} as any,
) {
  return usePrepareContractWrite({
    abi: weth9ABI,
    functionName: 'withdraw',
    ...config,
  } as UsePrepareContractWriteConfig<typeof weth9ABI, 'withdraw'>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link weth9ABI}__.
 */
export function useWeth9Event<TEventName extends string>(
  config: Omit<UseContractEventConfig<typeof weth9ABI, TEventName>, 'abi'> = {} as any,
) {
  return useContractEvent({ abi: weth9ABI, ...config } as UseContractEventConfig<typeof weth9ABI, TEventName>)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link weth9ABI}__ and `eventName` set to `"Approval"`.
 */
export function useWeth9ApprovalEvent(
  config: Omit<UseContractEventConfig<typeof weth9ABI, 'Approval'>, 'abi' | 'eventName'> = {} as any,
) {
  return useContractEvent({ abi: weth9ABI, eventName: 'Approval', ...config } as UseContractEventConfig<
    typeof weth9ABI,
    'Approval'
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link weth9ABI}__ and `eventName` set to `"Deposit"`.
 */
export function useWeth9DepositEvent(
  config: Omit<UseContractEventConfig<typeof weth9ABI, 'Deposit'>, 'abi' | 'eventName'> = {} as any,
) {
  return useContractEvent({ abi: weth9ABI, eventName: 'Deposit', ...config } as UseContractEventConfig<
    typeof weth9ABI,
    'Deposit'
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link weth9ABI}__ and `eventName` set to `"Transfer"`.
 */
export function useWeth9TransferEvent(
  config: Omit<UseContractEventConfig<typeof weth9ABI, 'Transfer'>, 'abi' | 'eventName'> = {} as any,
) {
  return useContractEvent({ abi: weth9ABI, eventName: 'Transfer', ...config } as UseContractEventConfig<
    typeof weth9ABI,
    'Transfer'
  >)
}

/**
 * Wraps __{@link useContractEvent}__ with `abi` set to __{@link weth9ABI}__ and `eventName` set to `"Withdrawal"`.
 */
export function useWeth9WithdrawalEvent(
  config: Omit<UseContractEventConfig<typeof weth9ABI, 'Withdrawal'>, 'abi' | 'eventName'> = {} as any,
) {
  return useContractEvent({ abi: weth9ABI, eventName: 'Withdrawal', ...config } as UseContractEventConfig<
    typeof weth9ABI,
    'Withdrawal'
  >)
}
