const rgb = require('../../ffi/nodejs/rgb_node')

const DATA_DIR = "../../data"

const OUTPOINTS = {
    'firstOwner': '5aa2d0a8098371ee12b4b59f43ffe6a2de637341258af65936a5baa01da49e9b:0',
    'secondOwner': '0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4:0'
}

const PSBT = "cHNidP8BAFICAAAAAZ38ZijCbFiZ/hvT3DOGZb/VXXraEPYiCXPfLTht7BJ2AQAAAAD/////" +
    "AfA9zR0AAAAAFgAUezoAv9wU0neVwrdJAdCdpu8TNXkAAAAATwEENYfPAto/0AiAAAAAlwSLGtBEWx7IJ" +
    "1UXcnyHtOTrwYogP/oPlMAVZr046QADUbdDiH7h1A3DKmBDck8tZFmztaTXPa7I+64EcvO8Q+IM2QxqT6" +
    "4AAIAAAACATwEENYfPAto/0AiAAAABuQRSQnE5zXjCz/JES+NTzVhgXj5RMoXlKLQH+uP2FzUD0wpel8i" +
    "tvFV9rCrZp+OcFyLrrGnmaLbyZnzB1nHIPKsM2QxqT64AAIABAACAAAEBKwBlzR0AAAAAIgAgLFSGEmxJ" +
    "eAeagU4TcV1l82RZ5NbMre0mbQUIZFuvpjIBBUdSIQKdoSzbWyNWkrkVNq/v5ckcOrlHPY5DtTODarRWK" +
    "ZyIcSEDNys0I07Xz5wf6l0F1EFVeSe+lUKxYusC4ass6AIkwAtSriIGAp2hLNtbI1aSuRU2r+/lyRw6uU" +
    "c9jkO1M4NqtFYpnIhxENkMak+uAACAAAAAgAAAAAAiBgM3KzQjTtfPnB/qXQXUQVV5J76VQrFi6wLhqyz" +
    "oAiTACxDZDGpPrgAAgAEAAIAAAAAAACICA57/H1R6HV+S36K6evaslxpL0DukpzSwMVaiVritOh75EO3k" +
    "XMUAAACAAAAAgAEAAIAA"

function main() {
    let rgbNode = new rgb.Node("testnet", DATA_DIR)
    console.log("RGB node runtime has started")

    let issuedAmount = 1000000.0;
    rgbNode.issue(
        "DEMO", "Demo token", null, 8,
        [{coins: issuedAmount, outpoint: OUTPOINTS.firstOwner}]
    )
    let assets = rgbNode.listAssets()
    console.log("Known assets:")
    console.log(assets)
    let assetId = assets[0].id
    console.log("Selecting first asset with ID " + assetId)

    let allocations = rgbNode.assetAllocations(assetId);
    console.log("Known allocations for '" + assetId + "': " + allocations)
    let outpointAssets = rgbNode.outpointAssets(OUTPOINTS.firstOwner)
    console.log("Asset list for '" + OUTPOINTS.firstOwner + "': " + outpointAssets)
    let outpointAssets2 = rgbNode.outpointAssets(OUTPOINTS.secondOwner)
    console.log("Asset list for '" + OUTPOINTS.secondOwner + "': " + outpointAssets2)

    let paymentAmount = 100.0
    let {invoice, secret} = rgbNode.invoice(assetId, paymentAmount, OUTPOINTS.secondOwner)
    console.log("Generated invoice " + invoice + " with secret " + secret)

    console.log("Transferring " + paymentAmount + " out of total " + outpointAssets[assetId] + " of " + assetId)
    let consignmentFile = DATA_DIR + '/consignment.rgb'
    let transactionFile = DATA_DIR + '/witness_tx.dat'
    rgbNode.transfer(
        [OUTPOINTS.firstOwner],
        [
            (outpointAssets[assetId][0] - paymentAmount) + '@' + OUTPOINTS.firstOwner
        ],
        invoice,
        PSBT,
        consignmentFile,
        transactionFile
    )

    try {
        rgbNode.validate(consignmentPath)
        console.log("Validation succeded")
    } catch (e) {
        console.log("Validation failed: " + e)
    }
}

console.log("RGB integration demo")
try {
    main()
} catch (e) {
    console.error('Error during demo execution: ' + e)
    process.exit(1)
}
