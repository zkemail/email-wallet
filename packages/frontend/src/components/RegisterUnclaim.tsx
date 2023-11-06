import { useNetwork, useWaitForTransaction, useAccount, useChainId, useFeeData, useBalance } from 'wagmi'
import { buildPoseidon } from "circomlibjs";
import { Dispatch, use, useEffect, useState } from "react";
import { usePrepareUnclaimsHandlerRegisterUnclaimedFund, useUnclaimsHandlerRegisterUnclaimedFund, useTokenRegistryGetTokenAddress, useErc20Decimals, useEmailWalletCoreTokenRegistry, useEmailWalletCoreUnclaimsHandler, useUnclaimsHandlerUnclaimedFundClaimGas, useUnclaimsHandlerMaxFeePerGas, useWeth9Deposit, useErc20Approve, usePrepareWeth9Deposit, usePrepareErc20Approve } from "../abis"
import { ethers } from "ethers";

enum TxState {
    Init,
    Deposited,
    Approved,
    Registered,
}

export function RegisterUnclaim(props: { toEmailAddr: string, tokenName: string, amountStr: string }) {
    const [emailAddrCommit, setEmailAddrCommit] = useState<`0x${string}`>("0x");
    // const [chainId, setChainId] = useState<number>(1);
    const chainId = useChainId();
    const [random, setRandom] = useState<string>("");
    const [txState, setTxState] = useState<TxState>(TxState.Init);
    const { address, connector, isConnected } = useAccount()
    useEffect(() => {
        (async () => {
            if (!props.toEmailAddr) {
                return;
            }
            if (props.toEmailAddr.length > 256) {
                return;
            }
            if (txState != TxState.Approved) {
                return;
            }
            const poseidon = await buildPoseidon();
            const F = poseidon.F;
            const rand: Uint8Array = F.random();
            const randHex = `0x${[...rand]
                .reverse()
                .map(x => x.toString(16).padStart(2, "0"))
                .join("")}`;
            console.log(randHex);
            setRandom(randHex);
            const body = {
                email_address: props.toEmailAddr,
                random: randHex,
            };
            const result = await fetch(`${process.env.NEXT_PUBLIC_RELAYER_HOSTNAME}/api/emailAddrCommit`, {
                method: "POST",
                mode: "cors",
                headers: {
                    "Content-Type": "text/plain",
                },
                body: JSON.stringify(body),
            });
            const emailAddrCommit = await result.text() as `0x${string}`;
            console.log(emailAddrCommit);
            // const addressFields = bytes2fields(padStringToBytes(props.toEmailAddr, 256), F);
            // console.log(addressFields);
            // const emailAddrCommitBytes = poseidon([rand].concat(addressFields));
            // console.log(emailAddrCommitBytes);
            // console.log(emailAddrCommitBytes.toString());
            // const emailAddrCommit: `0x${string}` = `0x${[...emailAddrCommitBytes]
            //     .reverse()
            //     .map(x => x.toString(16).padStart(2, "0"))
            //     .join("")}`;
            // console.log(emailAddrCommit);
            setEmailAddrCommit(emailAddrCommit);
        })()
    }, [props.toEmailAddr, txState]);

    const { data: tokenRegistryAddrRaw } = useEmailWalletCoreTokenRegistry({ address: process.env.NEXT_PUBLIC_CORE_ADDRESS as `0x${string}` });
    const tokenRegistryAddr = tokenRegistryAddrRaw || "0x";
    console.log(tokenRegistryAddr);

    // const { data: wethAddrRaw } = useTokenRegistryGetTokenAddress({ address: tokenRegistryAddr, chainId: chainId, args: [BigInt(chainId), "WETH"] });
    // console.log(wethAddrRaw);
    // const wethAddr = wethAddrRaw || "0x";
    // if (tokenRegistryAddr === undefined) {
    //     console.log("tokenRegistryAddr undefined")
    //     return;
    // }
    console.log(props.tokenName);
    console.log(chainId);
    console.log(BigInt(chainId));
    const { data: tokenAddrRaw } = useTokenRegistryGetTokenAddress({ address: tokenRegistryAddr, chainId: chainId, args: [BigInt(chainId), props.tokenName] });
    console.log(tokenAddrRaw);
    const tokenAddr = tokenAddrRaw || "0x";
    // if (tokenAddr === undefined) {
    //     console.log("tokenAddr undefined")
    //     return;
    // }
    const { data: decimalsSizeRaw } = useErc20Decimals({ address: tokenAddr });
    console.log(decimalsSizeRaw);
    const decimalsSize = decimalsSizeRaw || 18;
    // if (decimalsSize === undefined) {
    //     console.log("decimalsSize undefined")
    //     return;
    // }
    const amount = ethers.parseUnits(props.amountStr, decimalsSize);
    console.log(amount);

    const { data: unclaimsHandlerRaw } = useEmailWalletCoreUnclaimsHandler({ address: process.env.NEXT_PUBLIC_CORE_ADDRESS as `0x${string}` });
    const unclaimsHandler = unclaimsHandlerRaw || "0x";
    console.log(unclaimsHandler);

    if (txState == TxState.Init && props.tokenName != "ETH") {
        setTxState(TxState.Deposited);
    }
    return (
        <div>
            <WethDeposit amount={amount} tokenName={props.tokenName} wethAddr={tokenAddr} txState={txState} setTxState={setTxState}></WethDeposit>
            <Erc20Approve amount={amount} tokenAddr={tokenAddr} spender={unclaimsHandler} txState={txState} setTxState={setTxState}></Erc20Approve>
            <RegisterUnclaimedFund emailAddrCommit={emailAddrCommit} tokenAddr={tokenAddr} amount={amount} unclaimsHandler={unclaimsHandler} toEmailAddr={props.toEmailAddr} random={random} txState={txState} setTxState={setTxState}></RegisterUnclaimedFund>
        </div>
    )
}

function WethDeposit({ amount, tokenName, wethAddr, txState, setTxState }: { amount: bigint, tokenName: string, wethAddr: `0x${string}`, txState: TxState, setTxState: (txState: TxState) => void }) {
    const { address, connector, isConnected } = useAccount();
    console.log(`address: ${address}`);
    const { data: balanceData, } = useBalance({
        address: address,
    });
    console.log(balanceData);
    const { data: feeData } = useFeeData()
    console.log(txState);
    console.log(feeData?.maxFeePerGas);
    const { config } = usePrepareWeth9Deposit({
        address: wethAddr,
        value: amount,
        account: address,
        enabled: txState == TxState.Init && Boolean(amount) && tokenName == "ETH"
    });
    const { data, write } = useWeth9Deposit(config);
    const { isLoading } = useWaitForTransaction({
        hash: data?.hash,
        onSuccess: () => {
            setTxState(TxState.Deposited);
        }
    });
    return (
        <div>
            <button disabled={txState != TxState.Init || !write || isLoading} onClick={() => write?.()}>
                Deposit WETH
            </button>
            {isLoading && <ProcessingMessage hash={data?.hash} />}
        </div >
    )
}

function Erc20Approve({ amount, tokenAddr, spender, txState, setTxState }: { amount: bigint, tokenAddr: `0x${string}`, spender: `0x${string}`, txState: TxState, setTxState: (txState: TxState) => void }) {
    const { address, connector, isConnected } = useAccount();
    const { config } = usePrepareErc20Approve({
        args: [spender, amount],
        address: tokenAddr,
        account: address,
        enabled: txState == TxState.Deposited && Boolean(amount)
    });
    const { data, write } = useErc20Approve(config);
    const { isLoading } = useWaitForTransaction({
        hash: data?.hash,
        onSuccess: () => {
            setTxState(TxState.Approved);
        }
    });
    return (
        <div>
            <button disabled={txState != TxState.Deposited || !write || isLoading} onClick={() => write?.()}>
                Approve Email Wallet for the token
            </button>
            {isLoading && <ProcessingMessage hash={data?.hash} />}
        </div>
    )
}

function RegisterUnclaimedFund({ emailAddrCommit, tokenAddr, amount, unclaimsHandler, toEmailAddr, random, txState, setTxState }: { emailAddrCommit: `0x${string}`, tokenAddr: `0x${string}`, amount: bigint, unclaimsHandler: `0x${string}`, toEmailAddr: string, random: string, txState: TxState, setTxState: (txState: TxState) => void }) {
    const { address, connector, isConnected } = useAccount();
    const { data: gas } = useUnclaimsHandlerUnclaimedFundClaimGas({ address: unclaimsHandler });
    console.log(gas);
    const { data: maxFeePerGas } = useUnclaimsHandlerMaxFeePerGas({ address: unclaimsHandler });
    console.log(maxFeePerGas);
    const value = (gas || BigInt(0)) * (maxFeePerGas || BigInt(0));
    // const now = () => {
    //     const date = new Date();
    //     let timestamp = date.getTime();
    //     timestamp = Math.floor(timestamp / 1000);
    //     timestamp += 60 * 60 * 24 * 7;
    //     return BigInt(timestamp);
    // };
    const [now, setNow] = useState<bigint>(BigInt(0));
    useEffect(() => {
        if (txState == TxState.Init) {
            const date = new Date();
            let timestamp = date.getTime();
            timestamp = Math.floor(timestamp / 1000);
            timestamp += 60 * 60 * 24 * 7;
            setNow(BigInt(timestamp));
        }
    }, [emailAddrCommit, tokenAddr, amount]);

    useEffect(() => {
        (async () => {
            if (txState == TxState.Registered) {
                // const rest = new rm.RestClient("email-wallet-relayer", "http://localhost:3000");
                // await rest.update("/api/unclaim", {
                //     email_address: emailAddrCommit,
                //     random: random,
                //     expire_time: Number(now),
                //     is_fund: true,
                //     tx_hash: data?.hash || "0x"
                // }, {
                //     additionalHeaders: {
                //         "Content-Type": "application/json",
                //     }
                // });
                // const randomHex = `0x${[...random]
                //     .reverse()
                //     .map(x => x.toString(16).padStart(2, "0"))
                //     .join("")}`;
                const body = JSON.stringify({
                    email_address: toEmailAddr,
                    random: random,
                    expiry_time: Number(now),
                    is_fund: true,
                    tx_hash: data?.hash || "0x"
                });
                console.log(body);
                try {
                    const result = await fetch(`${process.env.NEXT_PUBLIC_RELAYER_HOSTNAME}/api/unclaim`, {
                        method: "POST",
                        mode: "cors",
                        headers: {
                            "Content-Type": "text/plain",
                        },
                        body: body,
                    });
                    console.log(await result.text());
                } catch (e) {
                    console.error(e);
                }
                setTxState(TxState.Init);

            }
        })();
    }, [txState])

    const { config } = usePrepareUnclaimsHandlerRegisterUnclaimedFund({
        args: [emailAddrCommit, tokenAddr, amount, now, BigInt(0), ""],
        address: unclaimsHandler,
        value: value,
        account: address,
        enabled: txState == TxState.Approved && Boolean(emailAddrCommit) && Boolean(tokenAddr) && Boolean(amount)
    });
    const { data, write } = useUnclaimsHandlerRegisterUnclaimedFund(config);
    const { isLoading } = useWaitForTransaction({
        hash: data?.hash,
        onSuccess: () => {
            setTxState(TxState.Registered);
        }
    });
    return (
        <div>
            <button disabled={txState != TxState.Approved || !write || isLoading} onClick={() => write?.()}>
                Register Unclaimed Fund
            </button>
            {isLoading && <ProcessingMessage hash={data?.hash} />}
        </div>
    )
}

function ProcessingMessage({ hash }: { hash?: `0x${string}` }) {
    const { chain } = useNetwork()
    const etherscan = chain?.blockExplorers?.etherscan
    return (
        <span>
            Processing transaction...{' '}
            {etherscan && (
                <a href={`${etherscan.url}/tx/${hash}`}>{etherscan.name}</a>
            )}
        </span>
    )
}

// function padStringToBytes(str: string, len: number): Uint8Array {
//     const bytes = new Uint8Array(len);
//     const strBytes = (new TextEncoder).encode(str);
//     bytes.set(strBytes);
//     const empty = new Uint8Array(len - strBytes.length);
//     bytes.set(empty, strBytes.length);
//     return bytes;
// }

// function bytes2fields(bytes: Uint8Array, F: any): bigint[] {
//     const fields: bigint[] = [];
//     for (let i = 0; i < bytes.length; i += 31) {
//         const bytes32 = new Uint8Array(32);
//         bytes32.set(bytes.slice(i, i + 31));
//         const val = F.fromRprLE(bytes32, 0);
//         fields.push(val);
//     }
//     return fields;
// }


