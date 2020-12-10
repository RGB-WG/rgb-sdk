#!/usr/bin/env python3

"""
RGB Python demo application
"""

import sys
sys.path.insert(1, '../../ffi/python')
import rgb_node as lib

from json import dumps


config = {
    'network': 'testnet',
    'stash_endpoint': 'lnpz:/tmp/rgb-node/testnet/stashd.rpc',
    'contract_endpoints': {
        'Fungible': 'lnpz:/tmp/rgb-node/testnet/fungibled.rpc'
    },
    'threaded': True,
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

try:
    runtime = lib.start_rgb(
        config['network'],
        config['stash_endpoint'],
        dumps(config['contract_endpoints']),
        config['threaded'],
        config['datadir'])
    lib.issue(runtime, issue_data['network'], issue_data['ticker'], issue_data['name'],
              issue_data['description'], issue_data['precision'],
              dumps(issue_data['allocations']), dumps(issue_data['inflation']),
              dumps(issue_data['renomination']), dumps(issue_data['epoch']))
    assets = lib.outpoint_assets(runtime, '5aa2d0a8098371ee12b4b59f43ffe6a2de637341258af65936a5baa01da49e9b:0')
    print('assets: {}'.format(assets))
except Exception as e:
    print('ERR: ' + str(e))
    sys.exit(1)
