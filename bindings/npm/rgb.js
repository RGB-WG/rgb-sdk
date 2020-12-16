const rgb = require('./build/Release/rgb')

exports.Node = class Node {
    constructor(dataDir, network) {
        this.network = network || "testnet"
        this.dataDir = dataDir
        this.runtime = rgb.run_rgb_embedded(network, dataDir)
    }

    issue(ticker, name, description, precision, allocations, inflation, renomination, epoch) {
        return rgb.issue(
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
        return JSON.parse(rgb.list_assets(this.runtime))
    }

    assetAllocations(contractId) {
        return JSON.parse(rgb.asset_allocations(this.runtime, contractId))
    }

    outpointAssets(outpoint) {
        return JSON.parse(rgb.outpoint_assets(this.runtime, outpoint))
    }

    invoice(contractId, amount, outpoint) {
        return JSON.parse(rgb.invoice(contractId, amount, outpoint))
    }

    transfer(inputs, allocate, invoice, prototypePsbt, consignmentFile, transactionFile) {
        return rgb.transfer(
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
        return rgb.validate(this.runtime, consignment_file)
    }

    accept(consignment_file, reveal_outpoints) {
      return rgb.accept(this.runtime, consignment_file, reveal_outpoints)
    }
}
