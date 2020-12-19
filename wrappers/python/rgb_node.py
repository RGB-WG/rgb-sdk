"""
RGB Node Python Wrapper
"""

import sys
sys.path.insert(1, '../../bindings/python')
from rgb import *

class RGBNode:
    def __init__(self, network, datadir, electrum, verbosity):
        self.network = network or "testnet"
        self.datadir = datadir
        self.electrum = electrum

        self.runtime = rgb_node_run(self.datadir, self.network, self.electrum, verbosity)

    def issue(self, ticker, name, description, precision, allocations, inflation, renomination, epoch):
        return rgb_node_fungible_issue(runtime=self.runtime,\
                                        network=self.network,\
                                        ticker=ticker,\
                                        name=name,\
                                        description=description,\
                                        precision=precision,\
                                        allocations=allocations,\
                                        inflation=inflation,\
                                        renomination=renomination,\
                                        epoch=epoch)

    def listAssets(self):
        return rgb_node_fungible_list_assets(self.runtime)

    def assetAllocations(self, contractId):
        return rgb_node_fungible_asset_allocations(self.runtime, contractId)

    def outpointAssets(self, outpoint):
        return rgb_node_fungible_outpoint_assets(self.runtime, outpoint)

    def invoice(self, contractId, amount, outpoint):
        return rgb20_invoice(contractId, amount, outpoint)

    def importAsset(self, asset_genesis):
        return rgb_node_fungible_import_asset(self.runtime, asset_genesis)

    def exportAsset(self, asset_id):
        return rgb_node_fungible_export_asset(self.runtime, asset_id)

    def transfer(self, inputs, allocate, invoice, prototypePsbt, consignmentFile, transactionFile):
        return rgb_node_fungible_transfer(
            self.runtime,
            inputs,
            allocate,
            invoice,
            prototypePsbt,
            consignmentFile,
            transactionFile
        )

    def validate(self, consignment_file):
        return rgb_node_fungible_validate(self.runtime, consignment_file)

    def accept(self, consignment_file, reveal_outpoints):
        return rgb_node_fungible_accept(self.runtime, consignment_file, reveal_outpoints)