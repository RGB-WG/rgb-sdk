// RGB C bindings library (librgb)
// Written in 2019 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::os::raw::{c_char, c_double, c_uchar};

use crate::helpers::*;
use crate::internal::*;

#[no_mangle]
pub extern "C" fn rgb_node_connect(
    network: *const c_char,
    stash_rpc_endpoint: *const c_char,
    contract_endpoints: *const c_char,
    threaded: bool,
    datadir: *const c_char,
) -> CResult {
    _start_logger();

    info!("Connecting RGB node...");

    _connect_rgb(
        network,
        stash_rpc_endpoint,
        contract_endpoints,
        threaded,
        datadir,
    )
    .into()
}

#[no_mangle]
pub extern "C" fn rgb_node_run(
    network: *const c_char,
    datadir: *const c_char,
) -> CResult {
    _start_logger();

    info!("Running embedded RGB node...");

    _run_rgb_embedded(network, datadir).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_issue(
    runtime: &COpaqueStruct,
    network: *const c_char,
    ticker: *const c_char,
    name: *const c_char,
    description: *const c_char,
    precision: c_uchar,
    allocations: *const c_char,
    inflation: *const c_char,
    renomination: *const c_char,
    epoch: *const c_char,
) -> CResult {
    _issue(
        runtime,
        network,
        ticker,
        name,
        description,
        precision,
        allocations,
        inflation,
        renomination,
        epoch,
    )
    .into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_list_assets(
    runtime: &COpaqueStruct,
) -> CResultString {
    _list_assets(runtime).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_asset_allocations(
    runtime: &COpaqueStruct,
    contract_id: *const c_char,
) -> CResultString {
    _asset_allocations(runtime, contract_id).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_outpoint_assets(
    runtime: &COpaqueStruct,
    outpoint: *const c_char,
) -> CResultString {
    _outpoint_assets(runtime, outpoint).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_export_asset(
    runtime: &COpaqueStruct,
    asset_id: *const c_char,
) -> CResultString {
    _export_asset(runtime, asset_id).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_import_asset(
    runtime: &COpaqueStruct,
    asset_genesis: *const c_char,
) -> CResult {
    _import_asset(runtime, asset_genesis).into()
}

#[no_mangle]
pub extern "C" fn rgb20_invoice(
    asset_id: *const c_char,
    amount: c_double,
    outpoint: *const c_char,
) -> CResultString {
    _invoice(asset_id, amount, outpoint).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_transfer(
    runtime: &COpaqueStruct,
    inputs: *const c_char,
    allocate: *const c_char,
    invoice: *const c_char,
    prototype_psbt: *const c_char,
    consignment_file: *const c_char,
    transaction_file: *const c_char,
) -> CResult {
    _transfer(
        runtime,
        inputs,
        allocate,
        invoice,
        prototype_psbt,
        consignment_file,
        transaction_file,
    )
    .into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_validate(
    runtime: &COpaqueStruct,
    consignment_file: *const c_char,
) -> CResult {
    _validate(runtime, consignment_file).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_accept(
    runtime: &COpaqueStruct,
    consignment_file: *const c_char,
    reveal_outpoints: *const c_char,
) -> CResult {
    _accept(runtime, consignment_file, reveal_outpoints).into()
}
