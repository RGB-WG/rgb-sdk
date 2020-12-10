const lib = require('./build/Release/rgb_node')

function array_to_pointer(js_array) {
  cpp_pointer = lib.uint_array(js_array.length)
  for (var i=0; i<js_array.length; i++) {
    lib.uint_array_set(cpp_pointer, i, js_array[i])
  }
  return cpp_pointer
}

exports.startRgb = function (network, stashEndpoint, contractEndpoints, threaded, dataDir) {
  return lib.start_rgb(network, stashEndpoint, JSON.stringify(contractEndpoints), threaded, dataDir)
}

exports.issue = function (
    runtime, network, ticker, name, description, precision, allocations, inflation, renomination, epoch) {
  return lib.issue(
    runtime, network, ticker, name, description, precision, JSON.stringify(allocations),
      JSON.stringify(inflation), JSON.stringify(renomination), JSON.stringify(epoch))
}

exports.transfer = function (runtime, inputs, allocate, invoice, prototypePsbt, consignmentFile, transactionFile) {
  return lib.transfer(
    runtime, JSON.stringify(inputs), JSON.stringify(allocate), invoice, prototypePsbt, consignmentFile, transactionFile)
}

exports.assetAllocations = function (runtime, contractId) {
    return lib.asset_allocations(runtime, contractId)
}

exports.outpointAssets = function (runtime, outpoint) {
    return lib.outpoint_assets(runtime, outpoint)
}

exports.accept = function (runtime, consignment_file, reveal_outpoints) {
  return lib.accept(runtime, consignment_file, reveal_outpoints)
}

exports.validate = function (runtime, consignment_file) {
  return lib.validate(runtime, consignment_file)
}
