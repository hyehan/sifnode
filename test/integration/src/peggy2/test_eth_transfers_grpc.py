import time

import siftool_path
from siftool import eth, test_utils, sifchain
from siftool.common import *


fund_amount_eth = 10 * eth.ETH
fund_amount_sif = 2 * test_utils.sifnode_funds_for_transfer_peggy1  # TODO How much rowan do we need? (this is 10**18)


def test_eth_to_ceth_and_back_grpc(ctx):
    _test_eth_to_ceth_and_back_grpc(ctx, 3)


def _test_eth_to_ceth_and_back_grpc(ctx, count):
    # Create/retrieve a test ethereum account
    test_eth_account = ctx.create_and_fund_eth_account(fund_amount=fund_amount_eth * count)

    # create/retrieve a test sifchain account
    test_sif_account = ctx.create_sifchain_addr(fund_amounts=[[fund_amount_sif * count, "rowan"], [fund_amount_eth, ctx.ceth_symbol]])

    # Verify initial balance
    test_sif_account_initial_balance = ctx.get_sifchain_balance(test_sif_account)

    # Send from ethereum to sifchain by locking
    amount_to_send_in_tx = 123456 * eth.GWEI
    total_amount_to_send = amount_to_send_in_tx * count
    assert total_amount_to_send < fund_amount_eth

    ctx.bridge_bank_lock_eth(test_eth_account, test_sif_account, total_amount_to_send)
    ctx.advance_blocks()

    # Verify final balance
    test_sif_account_final_balance = ctx.wait_for_sif_balance_change(test_sif_account, test_sif_account_initial_balance)
    balance_diff = sifchain.balance_delta(test_sif_account_initial_balance, test_sif_account_final_balance)
    assert exactly_one(list(balance_diff.keys())) == ctx.ceth_symbol
    assert balance_diff[ctx.ceth_symbol] == total_amount_to_send

    # Send from sifchain to ethereum by burning on sifchain side,
    # > sifnoded tx ethbridge burn
    # Reduce amount for cross-chain fee. The same formula is used inside this function.
    eth_balance_before = ctx.eth.get_eth_balance(test_eth_account)
    amount_to_send = amount_to_send_in_tx - ctx.eth.cross_chain_fee_base * ctx.eth.cross_chain_burn_fee

    # chain_id, acccount_number and sequence are part of signature bytes and serve for replay protection.
    # chain_id and account_number do not change for the lifetime of chain, whereas sequence needs to be incremented for
    # every transaction.
    # See https://github.com/cosmos/cosmos-sdk/issues/6966
    account = ctx.sifnode_client.query_account(test_sif_account)
    tx_sequence_no = int(account["sequence"])
    account_number = int(account["account_number"])
    assert tx_sequence_no == 0, "Sequenece number should be 0 since we just created this acccount"

    log.debug("Generating {} transactions...".format(count))
    signed_encoded_txs = []
    start_time = time.time()
    for i in range(count):
        # "generate_only" tells sifnode to print a transaction as JSON instead of signing and broadcasting it
        tx = ctx.sifnode_client.send_from_sifchain_to_ethereum(test_sif_account, test_eth_account, amount_to_send,
            ctx.ceth_symbol, generate_only=True)
        signed_tx = ctx.sifnode_client.sign_transaction(tx, test_sif_account, sequence=tx_sequence_no,
            account_number=account_number)
        encoded_tx = ctx.sifnode_client.encode_transaction(signed_tx)
        signed_encoded_txs.append(encoded_tx)
        tx_sequence_no += 1

    log.debug("Transaction generation speed: {:.2f}/s".format((time.time() - start_time) / count))

    randomize = False
    log.debug("Broadcasting {} transactions{}...".format(count, " in random order" if randomize else ""))
    start_time = time.time()
    while signed_encoded_txs:
        next_tx_idx = random.randrange(len(signed_encoded_txs)) if randomize else 0
        tx = signed_encoded_txs.pop(next_tx_idx)
        # result is a BroadcastTxResponse; result.tx_response is a TxResponse containing txhash etc.
        result = ctx.sifnode_client.broadcast_tx(tx)
    log.debug("Transaction broadcast speed: {:.2f}/s".format((time.time() - start_time) / count))

    while True:
        # Verify final balance
        # new_eth_balance = ctx.wait_for_eth_balance_change(test_eth_account, eth_balance_before)
        new_eth_balance = ctx.eth.get_eth_balance(test_eth_account)
        balance_delta = new_eth_balance - eth_balance_before
        log.debug("Balance difference: {} ({:.9f} / {})".format(balance_delta, balance_delta/amount_to_send_in_tx,
                  count * amount_to_send_in_tx - balance_delta))
        time.sleep(3)


def _test_setup(ctx):
    pass


# Enable running directly, i.e. without pytest
if __name__ == "__main__":
    basic_logging_setup()
    from siftool import test_utils
    ctx = test_utils.get_env_ctx()
    _test_eth_to_ceth_and_back_grpc(ctx, 10)
