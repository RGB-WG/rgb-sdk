#!/usr/bin/env python3

"""
RGB Python demo application
"""

import sys
sys.path.insert(1, '../../wrappers/python')
from rgb_node import RGBNode
from json import dumps


config = {
    'network': 'testnet',
    'electrum': 'pandora.network:60001',
    'datadir': '/tmp/rgb-node/'
}

issue_data = {
    'network': 'testnet',
    'ticker': 'USDT',
    'name': 'USD Tether',
    'description': 'USD Tether description',
    'precision': 0,
    'allocations': [
        {
            'coins': 6660000,
            'outpoint': '5aa2d0a8098371ee12b4b59f43ffe6a2de637341258af65936a5baa01da49e9b:0',
        }
    ],
    'inflation': [],
    'renomination': None,
    'epoch': None,
}

consignment_path = '/tmp/rgb-node/output/consignment.rgb'

input_outpoint = '0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4:0'

transfer_data = {
    'inputs': [input_outpoint],
    'allocate': [
        { 'coins': 100, 'vout': 1, 'txid': '0313ba7cfcaa66029a1a63918ebc426259f00953016c461663315d1bf6b83ab4' }
    ],
    'invoice': 'rgb20:outpoint1mzu8vz3jly3rzzkdpph583yahv9wktljtfcln6pe2le6n7ehqulstu967t?amount=5&asset=rgb:id1yqqqxya60n725eszngdx8yvwh3pxyk0sp9fszmzxze3nzhgm76ur4dqf2f7gy',
    'prototype_psbt': 'cHNidP8BAFICAAAAAZ38ZijCbFiZ/hvT3DOGZb/VXXraEPYiCXPfLTht7BJ2AQAAAAD/////AfA9zR0AAAAAFgAUezoAv9wU0neVwrdJAdCdpu8TNXkAAAAATwEENYfPAto/0AiAAAAAlwSLGtBEWx7IJ1UXcnyHtOTrwYogP/oPlMAVZr046QADUbdDiH7h1A3DKmBDck8tZFmztaTXPa7I+64EcvO8Q+IM2QxqT64AAIAAAACATwEENYfPAto/0AiAAAABuQRSQnE5zXjCz/JES+NTzVhgXj5RMoXlKLQH+uP2FzUD0wpel8itvFV9rCrZp+OcFyLrrGnmaLbyZnzB1nHIPKsM2QxqT64AAIABAACAAAEBKwBlzR0AAAAAIgAgLFSGEmxJeAeagU4TcV1l82RZ5NbMre0mbQUIZFuvpjIBBUdSIQKdoSzbWyNWkrkVNq/v5ckcOrlHPY5DtTODarRWKZyIcSEDNys0I07Xz5wf6l0F1EFVeSe+lUKxYusC4ass6AIkwAtSriIGAp2hLNtbI1aSuRU2r+/lyRw6uUc9jkO1M4NqtFYpnIhxENkMak+uAACAAAAAgAAAAAAiBgM3KzQjTtfPnB/qXQXUQVV5J76VQrFi6wLhqyzoAiTACxDZDGpPrgAAgAEAAIAAAAAAACICA57/H1R6HV+S36K6evaslxpL0DukpzSwMVaiVritOh75EO3kXMUAAACAAAAAgAEAAIAA',
    'consignment_file': consignment_path,
    'transaction_file': '/tmp/rgb-node/output/transaction'
}

asset_genesis = 'genesis1qyfe883hey6jrgj2xvk5g3dfmfqfzm7a4wez4pd2krf7ltsxffd6u6nrvjvvnc8vt9llmp7663pgututl9heuwaudet72ay9j6thc6cetuvhxvsqqya5xjt2w9y4u6sfkuszwwctnrpug5yjxnthmr3mydg05rdrpspcxysnqvvqpfvag2w8jxzzsz9pf8pjfwf0xvln5z7w93yjln3gcnyxsa04jsf2p8vu4sxgppfv0j9qerppqxhvztpqscnjsxvq5gdfy5v6j3wvpjxxqzcerxuvt4jl3crkjkgqusct7cyx8zzezcfpqv3nxjzmd5xtksanewy4tvl89kg4mvrt49v7fdew2hr79erm3w006au7ha6ch59mcfnvu9ghzezczaxpk54nqyqqyrd3jqq95mx8r'

asset_id = 'rgb1scxapanh6jj9ceapvxgdzr68jumjdu44ezt3ewy4h6ahz8hkd0fs6utwne'


try:
    # Start the node with config
    node = RGBNode(config['network'], config['datadir'], config["electrum"], 0)

    # Issue an asset with asset related data
    node.issue(issue_data['ticker'], issue_data['name'],
            issue_data['description'], issue_data['precision'],
            dumps(issue_data['allocations']), dumps(issue_data['inflation']),
            dumps(issue_data['renomination']), dumps(issue_data['epoch']))
    # Get issued assets
    assets = node.listAssets()
    print("Assets: {}".format(assets))

    # Get invoice
    invoice = node.invoice(asset_id, 66.6, input_outpoint)
    print("\n----------------------------- \n")
    print('invoice: {}'.format(invoice))

    # Get assets associated with outpoint
    assets = node.outpointAssets(input_outpoint)
    print("\n----------------------------- \n")
    print("asset list for '{}': {}".format(input_outpoint, assets))

    # Import an asset and print its genesis
    node.importAsset(asset_genesis)
    genesis = node.exportAsset(asset_id)
    print("\n----------------------------- \n")
    print('genesis: {}'.format(genesis))

except Exception as e:
    print('ERR: ' + str(e))
    sys.exit(1)