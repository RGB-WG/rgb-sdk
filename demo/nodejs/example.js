const fs = require('fs')
const rgbNode = require('../../ffi/nodejs/rgb_node')

const config = {
    network: "testnet",
    stash_endpoint: "lnpz:/tmp/rgb-node/testnet/stashd.rpc",
    contract_endpoints: {
        Fungible: "lnpz:/tmp/rgb-node/testnet/fungibled.rpc"
    },
    threaded: true,
    datadir: "/tmp/rgb-node"
}

const issueData = {
    network: "testnet",
    ticker: "USDT",
    name: "USD Tether",
    issue_structure: "SingleIssue",
    allocations: [{ coins: 100, vout:0, txid: "0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4" }],
    precision: 0,
    prune_seals: [],
}

const consignmentPath = '/tmp/rgb-node/output/consignment.rgb'

const inputOutpoint = '0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4:0'

const transferData = {
    inputs: [inputOutpoint],
    allocate: [
        { coins: 100, vout:1, txid: "0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4" }
    ],
    invoice: "rgb20:outpoint1mzu8vz3jly3rzzkdpph583yahv9wktljtfcln6pe2le6n7ehqulstu967t?amount=5&asset=rgb:id1yqqqxya60n725eszngdx8yvwh3pxyk0sp9fszmzxze3nzhgm76ur4dqf2f7gy",
    prototype_psbt: "cHNidP8BAFICAAAAAZ38ZijCbFiZ/hvT3DOGZb/VXXraEPYiCXPfLTht7BJ2AQAAAAD/////AfA9zR0AAAAAFgAUezoAv9wU0neVwrdJAdCdpu8TNXkAAAAATwEENYfPAto/0AiAAAAAlwSLGtBEWx7IJ1UXcnyHtOTrwYogP/oPlMAVZr046QADUbdDiH7h1A3DKmBDck8tZFmztaTXPa7I+64EcvO8Q+IM2QxqT64AAIAAAACATwEENYfPAto/0AiAAAABuQRSQnE5zXjCz/JES+NTzVhgXj5RMoXlKLQH+uP2FzUD0wpel8itvFV9rCrZp+OcFyLrrGnmaLbyZnzB1nHIPKsM2QxqT64AAIABAACAAAEBKwBlzR0AAAAAIgAgLFSGEmxJeAeagU4TcV1l82RZ5NbMre0mbQUIZFuvpjIBBUdSIQKdoSzbWyNWkrkVNq/v5ckcOrlHPY5DtTODarRWKZyIcSEDNys0I07Xz5wf6l0F1EFVeSe+lUKxYusC4ass6AIkwAtSriIGAp2hLNtbI1aSuRU2r+/lyRw6uUc9jkO1M4NqtFYpnIhxENkMak+uAACAAAAAgAAAAAAiBgM3KzQjTtfPnB/qXQXUQVV5J76VQrFi6wLhqyzoAiTACxDZDGpPrgAAgAEAAIAAAAAAACICA57/H1R6HV+S36K6evaslxpL0DukpzSwMVaiVritOh75EO3kXMUAAACAAAAAgAEAAIAA",
    consignment_file: consignmentPath,
    transaction_file: "/tmp/rgb-node/output/transaction"
}

var runtime = null

async function main() {
    await rgbNode.startRgb(
        config.network, config.stash_endpoint, config.contract_endpoints, config.threaded, config.datadir)
    .then(r => {
        console.log("RGB node runtime has started")
        runtime = r
        return rgbNode.issue(runtime, issueData)
    })
    /*
    .then(() => {
        return rgbNode.transfer(runtime, transferData.inputs, transferData.allocate,
           transferData.invoice, transferData.prototype_psbt, transferData.consignment_file,
           transferData.transaction_file)
    })
    .then(() => {
        return rgbNode.assetAllocations(runtime, 'rgb1w82xuaxz6lp9symrp3f4r47rylkkxsh506qzkt2n2kjfhrhrt03qrrcm0g')
    })
    .then(allocations => {
    */
    .then(() => {
        //console.log("Allocations: " + allocations)
        console.log("Querying assets")
        return rgbNode.outpointAssets(runtime, inputOutpoint)
    })
    .then(assets => {
        console.log("Asset list for '" + inputOutpoint + "': " + assets)
        console.log(assets)
        return rgbNode.validate(runtime, consignmentPath)
    })
    .then(() => {
        console.log("Validation succeded")
    })
}

console.log("RGB demo")
main().catch(e => {
    console.error('ERR: ' + e)
    process.exit(1)
})
