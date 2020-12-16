// RGB C bindings library (librgb)
// Written in 2019 by
//     Alekos Filini,
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>,
//     Zoe Faltiba
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::collections::HashMap;
use std::env;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_uchar};
use std::str::FromStr;

use log::LevelFilter;

use rgb::lnpbp::bitcoin::OutPoint;
use rgb::lnpbp::bp;
use rgb::lnpbp::bp::blind::OutpointReveal;
use rgb::lnpbp::client_side_validation::Conceal;
use rgb::lnpbp::rgb::{Consignment, ContractId, FromBech32, Genesis};

use rgb::api::reply::SyncFormat;
use rgb::fungible::{Asset, Invoice, Outpoint, OutpointCoins, SealCoins};
use rgb::i9n::{Config, Runtime};
use rgb::lnpbp::bitcoin::blockdata::transaction::ParseOutPointError;
use rgb::lnpbp::strict_encoding::strict_decode;
use rgb::rgbd::ContractName;
use rgb::util::file::ReadWrite;
use rgb::DataFormat;

use crate::error::RequestError;
use crate::helpers::*;

pub(crate) fn _start_rgb(
    network: *const c_char,
    stash_rpc_endpoint: *const c_char,
    contract_endpoints: *const c_char,
    threaded: bool,
    datadir: *const c_char,
) -> Result<Runtime, RequestError> {
    let c_network = unsafe { CStr::from_ptr(network) };
    let network = bp::Chain::from_str(c_network.to_str()?)?;

    let c_stash_rpc_endpoint = unsafe { CStr::from_ptr(stash_rpc_endpoint) };
    let stash_rpc_endpoint = c_stash_rpc_endpoint.to_str()?.to_string();

    let contract_endpoints: HashMap<ContractName, String> =
        serde_json::from_str(&ptr_to_string(contract_endpoints)?)?;

    let c_datadir = unsafe { CStr::from_ptr(datadir) };
    let datadir = c_datadir.to_str()?.to_string();

    let config = Config {
        network: network,
        stash_rpc_endpoint: stash_rpc_endpoint,
        contract_endpoints: contract_endpoints
            .into_iter()
            .map(|(k, v)| -> Result<_, RequestError> { Ok((k, v)) })
            .collect::<Result<_, _>>()?,
        threaded: threaded,
        data_dir: datadir,
        ..Config::default()
    };

    info!("{:?}", config);

    let runtime = Runtime::init(config)?;

    Ok(runtime)
}

pub(crate) fn _run_rgb_embedded(
    network: *const c_char,
    datadir: *const c_char,
) -> Result<Runtime, RequestError> {
    let c_network = unsafe { CStr::from_ptr(network) };
    let network = bp::Chain::from_str(c_network.to_str()?)?;

    let c_datadir = unsafe { CStr::from_ptr(datadir) };
    let datadir = c_datadir.to_str()?.to_string();

    let contract_endpoints: HashMap<ContractName, String> =
        [(ContractName::Fungible, s!("inproc://fungible-rpc"))]
            .iter()
            .cloned()
            .collect();
    let stash_rpc_endpoint = s!("inproc://stash-rpc");
    let stash_pub_endpoint = s!("inproc://stash-pub");
    let fungible_pub_endpoint = s!("inproc://fungible-pub");

    let config = Config {
        network,
        stash_rpc_endpoint,
        stash_pub_endpoint,
        fungible_pub_endpoint,
        contract_endpoints: contract_endpoints
            .into_iter()
            .map(|(k, v)| -> Result<_, RequestError> { Ok((k, v.parse()?)) })
            .collect::<Result<_, _>>()?,
        threaded: true,
        data_dir: datadir,
    };

    info!("{:?}", config);

    let runtime = Runtime::init(config)?;

    Ok(runtime)
}

#[cfg(target_os = "android")]
pub(crate) fn _start_logger() {
    android_logger::init_once(
        android_logger::Config::default().with_min_level(log::Level::Debug),
    );
}

#[cfg(not(target_os = "android"))]
pub(crate) fn _start_logger() {
    env::set_var("RUST_LOG", "trace");
    ::env_logger::init();
    log::set_max_level(LevelFilter::Trace);
}

pub(crate) fn _issue(
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
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let network = bp::Chain::from_str(&ptr_to_string(network)?)?;

    let ticker = ptr_to_string(ticker)?;

    let name = ptr_to_string(name)?;

    let description = if description.is_null() {
        None
    } else {
        let description = ptr_to_string(description)?;
        if description.is_empty() {
            None
        } else {
            Some(description)
        }
    };

    let allocations: Vec<OutpointCoins> =
        serde_json::from_str(&ptr_to_string(allocations)?)?;

    let inflation: Vec<OutpointCoins> =
        serde_json::from_str(&ptr_to_string(inflation)?)?;

    let renomination: Option<OutPoint> =
        serde_json::from_str(&ptr_to_string(renomination)?)?;

    let epoch: Option<OutPoint> = serde_json::from_str(&ptr_to_string(epoch)?)?;

    debug!(
        "Issue: {{ network: {}, ticker: {}, name: {}, description: {:?}, \
        precision: {}, allocations: {:?}, inflation: {:?}, renomination: {:?}, \
        epoch: {:?} }}",
        network,
        ticker,
        name,
        description,
        precision,
        allocations,
        inflation,
        renomination,
        epoch
    );

    runtime.issue(
        network,
        ticker,
        name,
        description,
        precision,
        allocations,
        inflation,
        renomination,
        epoch,
    )?;

    Ok(())
}

pub(crate) fn _list_assets(
    runtime: &COpaqueStruct,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let SyncFormat(_, data) = runtime.list_assets(DataFormat::StrictEncode)?;
    let assets: Vec<Asset> = strict_decode(&data)?;

    let json_response = serde_json::to_string(&assets)?;
    Ok(json_response)
}

pub(crate) fn _asset_allocations(
    runtime: &COpaqueStruct,
    contract_id: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let c_contract_id = unsafe { CStr::from_ptr(contract_id) };
    let contract_id = ContractId::from_bech32_str(c_contract_id.to_str()?)?;

    debug!("AssetAllocationsArgs {{ contract_id: {} }}", contract_id);

    let response = runtime.asset_allocations(contract_id)?;
    let json_response = serde_json::to_string(&response)?;
    Ok(json_response)
}

pub(crate) fn _outpoint_assets(
    runtime: &COpaqueStruct,
    outpoint: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let c_outpoint = unsafe { CStr::from_ptr(outpoint) };
    let outpoint = OutPoint::from_str(c_outpoint.to_str()?)?;

    debug!("Listing assets for {}", outpoint);

    let response = runtime.outpoint_assets(outpoint)?;
    let json_response = serde_json::to_string(&response)?;
    Ok(json_response)
}

pub(crate) fn _export_asset(
    runtime: &COpaqueStruct,
    asset_id: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let asset_id = ContractId::from_str(&ptr_to_string(asset_id)?)?;

    debug!("Exporting asset: {}", asset_id);

    let genesis = runtime.export_asset(asset_id)?;
    Ok(genesis.to_string())
}

pub(crate) fn _import_asset(
    runtime: &COpaqueStruct,
    asset_genesis: *const c_char,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let asset_genesis =
        Genesis::from_bech32_str(&ptr_to_string(asset_genesis)?)?;

    debug!("Importing asset: {}", asset_genesis);

    runtime.import_asset(asset_genesis)?;

    Ok(())
}

pub(crate) fn _invoice(
    asset_id: *const c_char,
    amount: c_double,
    outpoint: *const c_char,
) -> Result<String, RequestError> {
    let contract_id = ContractId::from_str(&ptr_to_string(asset_id)?)?;

    let outpoint = OutPoint::from_str(&ptr_to_string(outpoint)?)?;

    let outpoint_reveal = OutpointReveal::from(outpoint);
    let invoice = Invoice {
        contract_id,
        outpoint: Outpoint::BlindedUtxo(outpoint_reveal.conceal()),
        amount,
    };

    debug!(
        "Created invoice {}, blinding factor {}",
        invoice, outpoint_reveal.blinding
    );

    let json_response = json!({
        "invoice": invoice.to_string(),
        "secret": outpoint_reveal.blinding
    });
    Ok(json_response.to_string())
}

pub(crate) fn _transfer(
    runtime: &COpaqueStruct,
    inputs: *const c_char,
    allocate: *const c_char,
    invoice: *const c_char,
    prototype_psbt: *const c_char,
    consignment_file: *const c_char,
    transaction_file: *const c_char,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let inputs: Vec<OutPoint> = serde_json::from_str(&ptr_to_string(inputs)?)?;

    let a: Vec<String> = serde_json::from_str(&ptr_to_string(allocate)?)?;
    let mut allocate: Vec<SealCoins> = Vec::with_capacity(a.len());
    for entry in a {
        allocate.push(
            SealCoins::from_str(&entry).map_err(|_| {
                RequestError::Outpoint(ParseOutPointError::Format)
            })?,
        );
    }

    let c_invoice = unsafe { CStr::from_ptr(invoice) };
    let invoice = Invoice::from_str(c_invoice.to_str()?)?;

    let c_prototype_psbt = unsafe { CStr::from_ptr(prototype_psbt) };
    let prototype_psbt = c_prototype_psbt.to_str()?.to_string();

    let c_consignment_file = unsafe { CStr::from_ptr(consignment_file) };
    let consignment_file = c_consignment_file.to_str()?.to_string();

    let c_transaction_file = unsafe { CStr::from_ptr(transaction_file) };
    let transaction_file = c_transaction_file.to_str()?.to_string();

    debug!(
        "TransferArgs {{ inputs: {:?}, allocate: {:?}, invoice: {}, prototype_psbt: {:?}, \
        consignment_file: {:?}, transaction_file: {:?} }}",
        inputs, allocate, invoice, prototype_psbt, consignment_file, transaction_file
    );

    runtime.transfer(
        inputs,
        allocate,
        invoice,
        prototype_psbt,
        consignment_file,
        transaction_file,
    )?;

    Ok(())
}

pub(crate) fn _validate(
    runtime: &COpaqueStruct,
    consignment_file: *const c_char,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let filename = ptr_to_string(consignment_file)?;
    debug!("Reading consignment from {}", filename);
    let consignment = Consignment::read_file(filename.into())?;

    trace!("ValidateArgs {{ consignment: {:?} }}", consignment);

    runtime.validate(consignment)?;

    Ok(())
}

pub(crate) fn _accept(
    runtime: &COpaqueStruct,
    consignment_file: *const c_char,
    reveal_outpoints: *const c_char,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let filename = ptr_to_string(consignment_file)?;
    debug!("Reading consignment from {}", filename);
    let consignment = Consignment::read_file(filename.into())?;

    let reveal_outpoints: Vec<OutpointReveal> =
        serde_json::from_str(&ptr_to_string(reveal_outpoints)?)?;

    trace!(
        "AcceptArgs {{ consignment: {:?}, reveal_outpoints: {:?} }}",
        consignment,
        reveal_outpoints
    );

    runtime.accept(consignment, reveal_outpoints)?;

    Ok(())
}
