use std::any::TypeId;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::env;
use std::ffi::{CStr, CString};
use std::hash::{Hash, Hasher};
use std::os::raw::{c_char, c_int, c_void};
use std::slice;
use std::str::FromStr;

use log::LevelFilter;

use serde::Deserialize;

use rgb::lnpbp::bitcoin::OutPoint;
use rgb::lnpbp::bp;
use rgb::lnpbp::rgb::{ContractId, FromBech32};
use rgb::lnpbp::rgb::Consignment;
use rgb::lnpbp::strict_encoding::strict_decode;

use rgb::fungible::{Invoice, IssueStructure, Outcoins};
use rgb::i9n::{Config, Runtime};
use rgb::rgbd::ContractName;
use rgb::util::SealSpec;

#[macro_use]
extern crate amplify;

#[macro_use]
extern crate amplify_derive;

#[macro_use]
extern crate log;

trait CReturnType: Sized + 'static {
    fn from_opaque(other: &COpaqueStruct) -> Result<&mut Self, RequestError> {
        let mut hasher = DefaultHasher::new();
        TypeId::of::<Self>().hash(&mut hasher);
        let ty = hasher.finish();

        if other.ty != ty {
            return Err(RequestError::Runtime(
                rgb::error::BootstrapError::ArgParseError(s!("Type mismatch")),
            ));
        }

        let boxed = unsafe { Box::from_raw(other.ptr.clone() as *mut Self) };
        Ok(Box::leak(boxed))
    }
}
impl CReturnType for Runtime {}
impl CReturnType for String {}
impl CReturnType for () {}

#[repr(C)]
pub struct COpaqueStruct {
    ptr: *const c_void,
    ty: u64,
}

impl COpaqueStruct {
    fn new<T: 'static>(other: T) -> Self {
        let mut hasher = DefaultHasher::new();
        TypeId::of::<T>().hash(&mut hasher);
        let ty = hasher.finish();

        COpaqueStruct {
            ptr: Box::into_raw(Box::new(other)) as *const c_void,
            ty,
        }
    }

    fn raw<T>(ptr: *const T) -> Self {
        COpaqueStruct {
            ptr: ptr as *const c_void,
            ty: 0,
        }
    }
}

#[repr(C)]
pub struct CErrorDetails {
    message: *const c_char,
}

fn string_to_ptr(other: String) -> *const c_char {
    let cstr = match CString::new(other) {
        Ok(cstr) => cstr,
        Err(_) => CString::new(String::from(
            "Error converting string: contains a null-char",
        ))
        .unwrap(),
    };

    cstr.into_raw()
}

fn ptr_to_string(ptr: *const c_char) -> Result<String, RequestError> {
    unsafe { Ok(CStr::from_ptr(ptr).to_string_lossy().into_owned()) }
}

#[repr(C)]
pub enum CResultValue {
    Ok,
    Err,
}

#[repr(C)]
pub struct CResult {
    result: CResultValue,
    inner: COpaqueStruct,
}

impl<T: 'static, E> From<Result<T, E>> for CResult
where
    E: std::fmt::Debug,
{
    fn from(other: Result<T, E>) -> Self {
        match other {
            Ok(d) => CResult {
                result: CResultValue::Ok,
                inner: COpaqueStruct::new(d),
            },
            Err(e) => CResult {
                result: CResultValue::Err,
                inner: COpaqueStruct::raw(string_to_ptr(format!("{:?}", e))),
            },
        }
    }
}

#[repr(C)]
pub struct CResultString {
    result: CResultValue,
    inner: *const c_char,
}

impl From<Result<String, RequestError>> for CResultString
where
    RequestError: std::fmt::Debug,
{
    fn from(other: Result<String, RequestError>) -> Self {
        match other {
            Ok(d) => CResultString {
                result: CResultValue::Ok,
                inner: string_to_ptr(d),
            },
            Err(e) => CResultString {
                result: CResultValue::Err,
                inner: string_to_ptr(format!("{:?}", e)),
            },
        }
    }
}

#[derive(Debug, Display, From, Error)]
#[display(doc_comments)]
#[non_exhaustive]
enum RequestError {
    /// Bech32 error: {_0}
    #[from]
    Bech32(rgb::lnpbp::rgb::bech32::Error),

    /// Input value is not a JSON object or JSON parse error: {_0}
    #[from]
    Json(serde_json::Error),

    /// Invoice error: {_0}
    #[from]
    Invoice(rgb::fungible::InvoiceError),

    /// Input value is not a UTF8 string: {_0}
    #[from]
    Utf8(std::str::Utf8Error),

    /// Invalid network/chain identifier: {_0}
    #[from]
    ChainParse(rgb::lnpbp::bp::chain::ParseError),

    /// Bootstrap error: {_0}
    #[from]
    Runtime(rgb::error::BootstrapError),

    /// Transport error: {_0}
    #[from]
    Transport(rgb::lnpbp::lnp::transport::Error),

    /// Integration error: {_0}
    #[from]
    Integration(rgb::i9n::Error),

    /// Impossible error: {_0}
    #[from]
    Infallible(std::convert::Infallible),

    /// Outpoint parsing error: {_0}
    #[from]
    Outpoint(rgb::lnpbp::bitcoin::blockdata::transaction::ParseOutPointError),

    /// I/O error: {_0}
    #[from]
    Io(std::io::Error),

    /// Input error: {_0}
    #[from]
    Input(String),

    /// Strict encoding error: {_0}
    #[from]
    StrictEncoding(rgb::lnpbp::strict_encoding::Error),
}

fn _get_consignment(
    consignment_bytes: *const u8,
    consignment_length: c_int,
) -> Result<Consignment, RequestError> {
    if consignment_bytes.is_null() {
        return Err(RequestError::Input(s!(
            "consignment bytes cannot be null"
        )));
    };
    let consignment_bytes = unsafe {
        slice::from_raw_parts(consignment_bytes, consignment_length as usize)
    };
    trace!("consignment bytes: {:x?}", consignment_bytes);
    let consignment: Consignment = strict_decode(&consignment_bytes)?;
    Ok(consignment)
}

fn _start_rgb(
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

fn _run_rgb_embedded(
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
        network: network,
        stash_rpc_endpoint: stash_rpc_endpoint,
        stash_pub_endpoint: stash_pub_endpoint,
        fungible_pub_endpoint: fungible_pub_endpoint,
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
fn _start_logger() {
    android_logger::init_once(
        android_logger::Config::default().with_min_level(log::Level::Debug),
    );
}

#[cfg(not(target_os = "android"))]
fn _start_logger() {
    env::set_var("RUST_LOG", "trace");
    ::env_logger::init();
    log::set_max_level(LevelFilter::Trace);
}

#[no_mangle]
pub extern "C" fn start_rgb(
    network: *const c_char,
    stash_rpc_endpoint: *const c_char,
    contract_endpoints: *const c_char,
    threaded: bool,
    datadir: *const c_char,
) -> CResult {
    _start_logger();

    info!("Starting RGB in connected mode...");

    _start_rgb(
        network,
        stash_rpc_endpoint,
        contract_endpoints,
        threaded,
        datadir,
    )
    .into()
}

#[no_mangle]
pub extern "C" fn run_rgb_embedded(
    network: *const c_char,
    datadir: *const c_char,
) -> CResult {
    _start_logger();

    info!("Starting RGB in embedded mode...");

    _run_rgb_embedded(network, datadir).into()
}

#[derive(Debug, Deserialize)]
struct IssueArgs {
    #[serde(with = "serde_with::rust::display_fromstr")]
    network: bp::Chain,
    ticker: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    issue_structure: IssueStructure,
    #[serde(default)]
    allocations: Vec<Outcoins>,
    precision: u8,
    #[serde(default)]
    prune_seals: Vec<SealSpec>,
}

fn _issue(
    runtime: &COpaqueStruct,
    json: *const c_char,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;
    let data: IssueArgs = serde_json::from_str(&ptr_to_string(json)?)?;
    info!("{:?}", data);

    runtime.issue(
        data.network,
        data.ticker,
        data.name,
        data.description,
        data.issue_structure,
        data.allocations,
        data.precision,
        data.prune_seals,
    )?;

    Ok(())
}

#[no_mangle]
pub extern "C" fn issue(
    runtime: &COpaqueStruct,
    json: *const c_char,
) -> CResult {
    _issue(runtime, json).into()
}

fn _transfer(
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

    let allocate: Vec<Outcoins> =
        serde_json::from_str(&ptr_to_string(allocate)?)?;

    let c_invoice = unsafe { CStr::from_ptr(invoice) };
    let invoice = Invoice::from_str(c_invoice.to_str()?)?;

    let c_prototype_psbt = unsafe { CStr::from_ptr(prototype_psbt) };
    let prototype_psbt = c_prototype_psbt.to_str()?.to_string();

    let c_consignment_file = unsafe { CStr::from_ptr(consignment_file) };
    let consignment_file = c_consignment_file.to_str()?.to_string();

    let c_transaction_file = unsafe { CStr::from_ptr(transaction_file) };
    let transaction_file = c_transaction_file.to_str()?.to_string();

    info!(
        "TransferArgs {{ inputs: {:?}, allocate: {:?}, invoice: {:?}, prototype_psbt: {:?}, \
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

#[no_mangle]
pub extern "C" fn transfer(
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

fn _asset_allocations(
    runtime: &COpaqueStruct,
    contract_id: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let c_contract_id = unsafe { CStr::from_ptr(contract_id) };
    let contract_id = ContractId::from_bech32_str(c_contract_id.to_str()?)?;

    debug!("AssetAllocationsArgs {{ contract_id: {:?} }}", contract_id);

    let response = runtime.asset_allocations(contract_id)?;
    let json_response = serde_json::to_string(&response)?;
    Ok(json_response)
}

#[no_mangle]
pub extern "C" fn asset_allocations(
    runtime: &COpaqueStruct,
    contract_id: *const c_char,
) -> CResultString {
    _asset_allocations(runtime, contract_id).into()
}

fn _outpoint_assets(
    runtime: &COpaqueStruct,
    outpoint: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let c_outpoint = unsafe { CStr::from_ptr(outpoint) };
    let outpoint = OutPoint::from_str(c_outpoint.to_str()?)?;

    debug!("OutpointAssets {{ outpoint: {:?} }}", outpoint);

    let response = runtime.outpoint_assets(outpoint)?;
    let json_response = serde_json::to_string(&response)?;
    Ok(json_response)
}

#[no_mangle]
pub extern "C" fn outpoint_assets(
    runtime: &COpaqueStruct,
    outpoint: *const c_char,
) -> CResultString {
    _outpoint_assets(runtime, outpoint).into()
}

fn _accept(
    runtime: &COpaqueStruct,
    consignment_bytes: *const u8,
    consignment_length: c_int,
    reveal_outpoints: *const c_char,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let consignment = _get_consignment(consignment_bytes, consignment_length)?;

    let reveal_outpoints: Vec<bp::blind::OutpointReveal> =
        serde_json::from_str(&ptr_to_string(reveal_outpoints)?)?;

    trace!(
        "AcceptArgs {{ consignment: {:?}, reveal_outpoints: {:?} }}",
        consignment,
        reveal_outpoints
    );

    runtime.accept(consignment, reveal_outpoints)?;

    Ok(())
}

#[no_mangle]
pub extern "C" fn accept(
    runtime: &COpaqueStruct,
    consignment_bytes: *const u8,
    consignment_length: c_int,
    reveal_outpoints: *const c_char,
) -> CResult {
    _accept(
        runtime,
        consignment_bytes,
        consignment_length,
        reveal_outpoints,
    )
    .into()
}

fn _validate(
    runtime: &COpaqueStruct,
    consignment_bytes: *const u8,
    consignment_length: c_int,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let consignment = _get_consignment(consignment_bytes, consignment_length)?;

    trace!("ValidateArgs {{ consignment: {:?} }}", consignment);

    runtime.validate(consignment)?;

    Ok(())
}

#[no_mangle]
pub extern "C" fn validate(
    runtime: &COpaqueStruct,
    consignment_bytes: *const u8,
    consignment_length: c_int,
) -> CResult {
    _validate(runtime, consignment_bytes, consignment_length).into()
}
