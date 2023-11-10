import {
  Signer,
  TypedDataDomain,
  TypedDataField,
  TransactionLike,
  BlockTag,
  Provider,
  TransactionRequest,
  TransactionResponse,
} from "ethers";

export class EmailWalletSigner implements Signer {
  _isConnected = false;
  relayerEmail = "";

  provider: null | Provider = null;

  connect(_provider: any): Signer {
    this.provider = _provider;

    // this.relayerEmail = relayerEmail;
    this._isConnected = true;

    return this as Signer;
  }

  ////////////////////
  // State

  /**
   *  Get the address of the Signer.
   */
  async getAddress(): Promise<string> {
    return "0x21E7FDC3A6ac59124b8AF2dc2c13E118EfE8248f";
  }

  /**
   *  Gets the next nonce required for this Signer to send a transaction.
   *
   *  @param blockTag - The blocktag to base the transaction count on, keep in mind
   *         many nodes do not honour this value and silently ignore it [default: ``"latest"``]
   */
  async getNonce(blockTag?: BlockTag): Promise<number> {
    return 123;
  }

  ////////////////////
  // Preparation

  /**
   *  Prepares a {@link TransactionRequest} for calling:
   *  - resolves ``to`` and ``from`` addresses
   *  - if ``from`` is specified , check that it matches this Signer
   *
   *  @param tx - The call to prepare
   */
  async populateCall(tx: TransactionRequest): Promise<TransactionLike<string>> {
    return {};
  }

  /**
   *  Prepares a {@link TransactionRequest} for sending to the network by
   *  populating any missing properties:
   *  - resolves ``to`` and ``from`` addresses
   *  - if ``from`` is specified , check that it matches this Signer
   *  - populates ``nonce`` via ``signer.getNonce("pending")``
   *  - populates ``gasLimit`` via ``signer.estimateGas(tx)``
   *  - populates ``chainId`` via ``signer.provider.getNetwork()``
   *  - populates ``type`` and relevant fee data for that type (``gasPrice``
   *    for legacy transactions, ``maxFeePerGas`` for EIP-1559, etc)
   *
   *  @note Some Signer implementations may skip populating properties that
   *        are populated downstream; for example JsonRpcSigner defers to the
   *        node to populate the nonce and fee data.
   *
   *  @param tx - The call to prepare
   */
  async populateTransaction(tx: TransactionRequest): Promise<TransactionLike<string>> {
    return {};
  }

  ////////////////////
  // Execution

  /**
   *  Estimates the required gas required to execute //tx// on the Blockchain. This
   *  will be the expected amount a transaction will require as its ``gasLimit``
   *  to successfully run all the necessary computations and store the needed state
   *  that the transaction intends.
   *
   *  Keep in mind that this is **best efforts**, since the state of the Blockchain
   *  is in flux, which could affect transaction gas requirements.
   *
   *  @throws UNPREDICTABLE_GAS_LIMIT A transaction that is believed by the node to likely
   *          fail will throw an error during gas estimation. This could indicate that it
   *          will actually fail or that the circumstances are simply too complex for the
   *          node to take into account. In these cases, a manually determined ``gasLimit``
   *          will need to be made.
   */
  async estimateGas(tx: TransactionRequest): Promise<bigint> {
    return BigInt(12);
  }

  /**
   *  Evaluates the //tx// by running it against the current Blockchain state. This
   *  cannot change state and has no cost in ether, as it is effectively simulating
   *  execution.
   *
   *  This can be used to have the Blockchain perform computations based on its state
   *  (e.g. running a Contract's getters) or to simulate the effect of a transaction
   *  before actually performing an operation.
   */
  async call(tx: TransactionRequest): Promise<string> {
    return "done";
  }

  /**
   *  Resolves an ENS Name to an address.
   */
  async resolveName(name: string): Promise<null | string> {
    return null;
  }

  ////////////////////
  // Signing

  /**
   *  Signs %%tx%%, returning the fully signed transaction. This does not
   *  populate any additional properties within the transaction.
   */
  async signTransaction(tx: TransactionRequest): Promise<string> {
    return "signed";
  }

  /**
   *  Sends %%tx%% to the Network. The ``signer.populateTransaction(tx)``
   *  is called first to ensure all necessary properties for the
   *  transaction to be valid have been popualted first.
   */
  async sendTransaction(tx: TransactionRequest): Promise<TransactionResponse> {
    return {} as any;
  }

  /**
   *  Signers an [[link-eip-191]] prefixed personal message.
   *
   *  If the %%message%% is a string, it is signed as UTF-8 encoded bytes. It is **not**
   *  interpretted as a [[BytesLike]]; so the string ``"0x1234"`` is signed as six
   *  characters, **not** two bytes.
   *
   *  To sign that example as two bytes, the Uint8Array should be used
   *  (i.e. ``new Uint8Array([ 0x12, 0x34 ])``).
   */
  async signMessage(message: string | Uint8Array): Promise<string> {
    return "signed";
  }

  /**
   *  Signs the [[link-eip-712]] typed data.
   */
  async signTypedData(
    domain: TypedDataDomain,
    types: Record<string, Array<TypedDataField>>,
    value: Record<string, any>,
  ): Promise<string> {
    return "signed";
  }
}
