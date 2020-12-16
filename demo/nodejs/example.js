const fs = require('fs')
const rgb = require('../../ffi/nodejs/rgb_node')

const consignmentPath = '/tmp/rgb-node/output/consignment.rgb'

const inputOutpoint = '0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4:0'

const transferData = {
    inputs: [inputOutpoint],
    allocate: [
        { coins: 100, vout: 1, txid: "0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4" }
    ],
    invoice: "rgb20:outpoint1mzu8vz3jly3rzzkdpph583yahv9wktljtfcln6pe2le6n7ehqulstu967t?amount=5&asset=rgb:id1yqqqxya60n725eszngdx8yvwh3pxyk0sp9fszmzxze3nzhgm76ur4dqf2f7gy",
    prototypePsbt: "cHNidP8BAFICAAAAAZ38ZijCbFiZ/hvT3DOGZb/VXXraEPYiCXPfLTht7BJ2AQAAAAD/////AfA9zR0AAAAAFgAUezoAv9wU0neVwrdJAdCdpu8TNXkAAAAATwEENYfPAto/0AiAAAAAlwSLGtBEWx7IJ1UXcnyHtOTrwYogP/oPlMAVZr046QADUbdDiH7h1A3DKmBDck8tZFmztaTXPa7I+64EcvO8Q+IM2QxqT64AAIAAAACATwEENYfPAto/0AiAAAABuQRSQnE5zXjCz/JES+NTzVhgXj5RMoXlKLQH+uP2FzUD0wpel8itvFV9rCrZp+OcFyLrrGnmaLbyZnzB1nHIPKsM2QxqT64AAIABAACAAAEBKwBlzR0AAAAAIgAgLFSGEmxJeAeagU4TcV1l82RZ5NbMre0mbQUIZFuvpjIBBUdSIQKdoSzbWyNWkrkVNq/v5ckcOrlHPY5DtTODarRWKZyIcSEDNys0I07Xz5wf6l0F1EFVeSe+lUKxYusC4ass6AIkwAtSriIGAp2hLNtbI1aSuRU2r+/lyRw6uUc9jkO1M4NqtFYpnIhxENkMak+uAACAAAAAgAAAAAAiBgM3KzQjTtfPnB/qXQXUQVV5J76VQrFi6wLhqyzoAiTACxDZDGpPrgAAgAEAAIAAAAAAACICA57/H1R6HV+S36K6evaslxpL0DukpzSwMVaiVritOh75EO3kXMUAAACAAAAAgAEAAIAA",
    consignmentFile: consignmentPath,
    transactionFile: "/tmp/rgb-node/output/transaction"
}

const assetGenesis = 'genesis1qyfe883hey6jrgj2xvk5g3dfmfqfzm7a4wez4pd2krf7ltsxffd6u6nrvjvvnc8vt9llmp7663pgututl9heuwaudet72ay9j6thc6cetuvhxvsqqya5xjt2w9y4u6sfkuszwwctnrpug5yjxnthmr3mydg05rdrpspcxysnqvvqpfvag2w8jxzzsz9pf8pjfwf0xvln5z7w93yjln3gcnyxsa04jsf2p8vu4sxgppfv0j9qerppqxhvztpqscnjsxvq5gdfy5v6j3wvpjxxqzcerxu07z5u3vrkjkgqusct7cyx8zzezcfpqv3nxjymuhhw96v764n97ek0tv3tkcxh22eusm6a428vtj0hzc7lhmeu0l43dgthshxec2ew9j9q96vzddtkqgqqgzmrxqqqhuxshd'

var rgbNode = null

function main() {
    rgbNode = new rgb.Node("testnet", "../../data")
    console.log("RGB node runtime has started")

    rgbNode.issue(
        "DEMO", "Demo token", null, 8,
        [
            {
                coins: 1000000,
                outpoint: '5aa2d0a8098371ee12b4b59f43ffe6a2de637341258af65936a5baa01da49e9b:0',
            }
        ]
    )
    /*
    rgbNode.transfer(JSON.stringify(transferData.inputs), JSON.stringify(transferData.allocate),
           transferData.invoice, transferData.prototypePsbt, transferData.consignmentFile,
           transferData.transactionFile)
    rgbNode.assetAllocations('rgb1w82xuaxz6lp9symrp3f4r47rylkkxsh506qzkt2n2kjfhrhrt03qrrcm0g')
    */
    let assets = rgbNode.outpointAssets("5aa2d0a8098371ee12b4b59f43ffe6a2de637341258af65936a5baa01da49e9b:0")
    console.log("Asset list for '" + inputOutpoint + "': " + assets)

    try {
        rgbNode.validate(consignmentPath)
        console.log("Validation succeded")
    } catch(e) {
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
