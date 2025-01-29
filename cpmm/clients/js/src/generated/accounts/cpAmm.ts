/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  assertAccountExists,
  assertAccountsExist,
  combineCodec,
  decodeAccount,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  fixDecoderSize,
  fixEncoderSize,
  getAddressDecoder,
  getAddressEncoder,
  getBooleanDecoder,
  getBooleanEncoder,
  getBytesDecoder,
  getBytesEncoder,
  getStructDecoder,
  getStructEncoder,
  getU16Decoder,
  getU16Encoder,
  getU64Decoder,
  getU64Encoder,
  transformEncoder,
  type Account,
  type Address,
  type Codec,
  type Decoder,
  type EncodedAccount,
  type Encoder,
  type FetchAccountConfig,
  type FetchAccountsConfig,
  type MaybeAccount,
  type MaybeEncodedAccount,
  type ReadonlyUint8Array,
} from '@solana/web3.js';
import {
  getQ64128Decoder,
  getQ64128Encoder,
  type Q64128,
  type Q64128Args,
} from '../types';

export const CP_AMM_DISCRIMINATOR = new Uint8Array([
  105, 219, 233, 13, 147, 109, 73, 100,
]);

export function getCpAmmDiscriminatorBytes() {
  return fixEncoderSize(getBytesEncoder(), 8).encode(CP_AMM_DISCRIMINATOR);
}

export type CpAmm = {
  discriminator: ReadonlyUint8Array;
  /** Whether the AMM has been initialized. */
  isInitialized: boolean;
  /** Whether the AMM has been launched and is active. */
  isLaunched: boolean;
  /**
   * Initial liquidity that is permanently locked after the pool launch.
   * This stabilizes the pool in case of empty liquidity.
   */
  initialLockedLiquidity: bigint;
  /**
   * Square root of the constant product of the pool, stored as a Q64.128 fixed-point number.
   * This ensures high accuracy during calculations.
   */
  constantProductSqrt: Q64128;
  /** Square root of the ratio between the base and quote tokens, stored as a Q64.128 fixed-point number. */
  baseQuoteRatioSqrt: Q64128;
  /** Amount of base tokens currently in the pool's vault. */
  baseLiquidity: bigint;
  /** Amount of quote tokens currently in the pool's vault. */
  quoteLiquidity: bigint;
  /** Total supply of LP tokens minted to liquidity providers. */
  lpTokensSupply: bigint;
  /** Fee rate for liquidity providers, measured in basis points (1 basis point = 0.01%). */
  providersFeeRateBasisPoints: number;
  /** Protocol fee rate from the associated `AmmsConfig` account, measured in basis points (1 = 0.01%). */
  protocolFeeRateBasisPoints: number;
  /** Accumulated base token fees that can be redeemed by the `AmmsConfig` account's authority. */
  protocolBaseFeesToRedeem: bigint;
  /** Accumulated quote token fees that can be redeemed by the `AmmsConfig` account's authority. */
  protocolQuoteFeesToRedeem: bigint;
  /** Public key of the base token's mint. */
  baseMint: Address;
  /** Public key of the quote token's mint. */
  quoteMint: Address;
  /** Public key of the LP token's mint. */
  lpMint: Address;
  /** Public key of the vault holding the base tokens. */
  baseVault: Address;
  /** Public key of the vault holding the quote tokens. */
  quoteVault: Address;
  /** Public key of the vault holding locked LP tokens. */
  lockedLpVault: Address;
  /** Public key of the associated `AmmsConfig` account. */
  ammsConfig: Address;
  /** Canonical bump seed for the account's PDA. */
  bump: ReadonlyUint8Array;
};

export type CpAmmArgs = {
  /** Whether the AMM has been initialized. */
  isInitialized: boolean;
  /** Whether the AMM has been launched and is active. */
  isLaunched: boolean;
  /**
   * Initial liquidity that is permanently locked after the pool launch.
   * This stabilizes the pool in case of empty liquidity.
   */
  initialLockedLiquidity: number | bigint;
  /**
   * Square root of the constant product of the pool, stored as a Q64.128 fixed-point number.
   * This ensures high accuracy during calculations.
   */
  constantProductSqrt: Q64128Args;
  /** Square root of the ratio between the base and quote tokens, stored as a Q64.128 fixed-point number. */
  baseQuoteRatioSqrt: Q64128Args;
  /** Amount of base tokens currently in the pool's vault. */
  baseLiquidity: number | bigint;
  /** Amount of quote tokens currently in the pool's vault. */
  quoteLiquidity: number | bigint;
  /** Total supply of LP tokens minted to liquidity providers. */
  lpTokensSupply: number | bigint;
  /** Fee rate for liquidity providers, measured in basis points (1 basis point = 0.01%). */
  providersFeeRateBasisPoints: number;
  /** Protocol fee rate from the associated `AmmsConfig` account, measured in basis points (1 = 0.01%). */
  protocolFeeRateBasisPoints: number;
  /** Accumulated base token fees that can be redeemed by the `AmmsConfig` account's authority. */
  protocolBaseFeesToRedeem: number | bigint;
  /** Accumulated quote token fees that can be redeemed by the `AmmsConfig` account's authority. */
  protocolQuoteFeesToRedeem: number | bigint;
  /** Public key of the base token's mint. */
  baseMint: Address;
  /** Public key of the quote token's mint. */
  quoteMint: Address;
  /** Public key of the LP token's mint. */
  lpMint: Address;
  /** Public key of the vault holding the base tokens. */
  baseVault: Address;
  /** Public key of the vault holding the quote tokens. */
  quoteVault: Address;
  /** Public key of the vault holding locked LP tokens. */
  lockedLpVault: Address;
  /** Public key of the associated `AmmsConfig` account. */
  ammsConfig: Address;
  /** Canonical bump seed for the account's PDA. */
  bump: ReadonlyUint8Array;
};

export function getCpAmmEncoder(): Encoder<CpAmmArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', fixEncoderSize(getBytesEncoder(), 8)],
      ['isInitialized', getBooleanEncoder()],
      ['isLaunched', getBooleanEncoder()],
      ['initialLockedLiquidity', getU64Encoder()],
      ['constantProductSqrt', getQ64128Encoder()],
      ['baseQuoteRatioSqrt', getQ64128Encoder()],
      ['baseLiquidity', getU64Encoder()],
      ['quoteLiquidity', getU64Encoder()],
      ['lpTokensSupply', getU64Encoder()],
      ['providersFeeRateBasisPoints', getU16Encoder()],
      ['protocolFeeRateBasisPoints', getU16Encoder()],
      ['protocolBaseFeesToRedeem', getU64Encoder()],
      ['protocolQuoteFeesToRedeem', getU64Encoder()],
      ['baseMint', getAddressEncoder()],
      ['quoteMint', getAddressEncoder()],
      ['lpMint', getAddressEncoder()],
      ['baseVault', getAddressEncoder()],
      ['quoteVault', getAddressEncoder()],
      ['lockedLpVault', getAddressEncoder()],
      ['ammsConfig', getAddressEncoder()],
      ['bump', fixEncoderSize(getBytesEncoder(), 1)],
    ]),
    (value) => ({ ...value, discriminator: CP_AMM_DISCRIMINATOR })
  );
}

export function getCpAmmDecoder(): Decoder<CpAmm> {
  return getStructDecoder([
    ['discriminator', fixDecoderSize(getBytesDecoder(), 8)],
    ['isInitialized', getBooleanDecoder()],
    ['isLaunched', getBooleanDecoder()],
    ['initialLockedLiquidity', getU64Decoder()],
    ['constantProductSqrt', getQ64128Decoder()],
    ['baseQuoteRatioSqrt', getQ64128Decoder()],
    ['baseLiquidity', getU64Decoder()],
    ['quoteLiquidity', getU64Decoder()],
    ['lpTokensSupply', getU64Decoder()],
    ['providersFeeRateBasisPoints', getU16Decoder()],
    ['protocolFeeRateBasisPoints', getU16Decoder()],
    ['protocolBaseFeesToRedeem', getU64Decoder()],
    ['protocolQuoteFeesToRedeem', getU64Decoder()],
    ['baseMint', getAddressDecoder()],
    ['quoteMint', getAddressDecoder()],
    ['lpMint', getAddressDecoder()],
    ['baseVault', getAddressDecoder()],
    ['quoteVault', getAddressDecoder()],
    ['lockedLpVault', getAddressDecoder()],
    ['ammsConfig', getAddressDecoder()],
    ['bump', fixDecoderSize(getBytesDecoder(), 1)],
  ]);
}

export function getCpAmmCodec(): Codec<CpAmmArgs, CpAmm> {
  return combineCodec(getCpAmmEncoder(), getCpAmmDecoder());
}

export function decodeCpAmm<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>
): Account<CpAmm, TAddress>;
export function decodeCpAmm<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>
): MaybeAccount<CpAmm, TAddress>;
export function decodeCpAmm<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>
): Account<CpAmm, TAddress> | MaybeAccount<CpAmm, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getCpAmmDecoder()
  );
}

export async function fetchCpAmm<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<Account<CpAmm, TAddress>> {
  const maybeAccount = await fetchMaybeCpAmm(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeCpAmm<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<MaybeAccount<CpAmm, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeCpAmm(maybeAccount);
}

export async function fetchAllCpAmm(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<Account<CpAmm>[]> {
  const maybeAccounts = await fetchAllMaybeCpAmm(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeCpAmm(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<MaybeAccount<CpAmm>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) => decodeCpAmm(maybeAccount));
}

export function getCpAmmSize(): number {
  return 335;
}
