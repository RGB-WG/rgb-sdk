const lib = require('../../bindings/npm/rgblib')

exports.Node = class Node {
    constructor(dataDir, network) {
        this.network = network || "testnet"
        this.dataDir = dataDir
        this.runtime = lib.rgb_node_run(network, dataDir)
    }

    issue(ticker, name, description, precision, allocations, inflation, renomination, epoch) {
        return lib.rgb_node_fungible_issue(
            this.runtime,
            this.network,
            ticker,
            name,
            description || "",
            precision,
            JSON.stringify(allocations || []),
            JSON.stringify(inflation || []),
            JSON.stringify(renomination || null),
            JSON.stringify(epoch || null)
        )
    }

    listAssets() {
        return JSON.parse(lib.rgb_node_fungible_list_assets(this.runtime))
    }

    assetAllocations(contractId) {
        return JSON.parse(lib.rgb_node_fungible_asset_allocations(this.runtime, contractId))
    }

    outpointAssets(outpoint) {
        return JSON.parse(lib.rgb_node_fungible_outpoint_assets(this.runtime, outpoint))
    }

    invoice(contractId, amount, outpoint) {
        return JSON.parse(lib.rgb20_invoice(contractId, amount, outpoint))
    }

    transfer(inputs, allocate, invoice, prototypePsbt, consignmentFile, transactionFile) {
        return lib.rgb_node_fungible_transfer(
            this.runtime,
            JSON.stringify(inputs),
            JSON.stringify(allocate),
            invoice,
            prototypePsbt,
            consignmentFile,
            transactionFile
        )
    }

    validate(consignment_file) {
        return lib.rgb_node_fungible_validate(this.runtime, consignment_file)
    }

    accept(consignment_file, reveal_outpoints) {
      return lib.rgb_node_fungible_accept(this.runtime, consignment_file, reveal_outpoints)
    }
}
