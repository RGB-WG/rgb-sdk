package org.lnpbp.rgb;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;

import org.lnpbp.rgb.model.OutpointCoins;
import org.lnpbp.rgb_autogen.COpaqueStruct;
import org.lnpbp.rgb_autogen.rgb;

import java.util.HashMap;
import java.util.HashSet;
import java.util.List;

public class Runtime {
    private final COpaqueStruct runtime;
    private final String network;
    private final ObjectMapper mapper;

    public Runtime(final String datadir, final String network, final String electrum) throws RuntimeException {
        mapper = new ObjectMapper();
        this.network = network;
        try {
            this.runtime = rgb.rgb_node_run(datadir, network, electrum, 3);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public void issue(final String ticker, final String name, final String description, final byte precision, final List<OutpointCoins> allocations, final HashSet<OutpointCoins> inflation, final OutPoint renomination, final OutPoint epoch) throws RuntimeException {
        try {
            final String allocationsStr = mapper.writeValueAsString(allocations);
            final String inflationStr = mapper.writeValueAsString(inflation);
            final String renominationStr = mapper.writeValueAsString(renomination);
            final String epochStr = mapper.writeValueAsString(epoch);
            rgb.rgb_node_fungible_issue(this.runtime, this.network, ticker, name, description, precision, allocationsStr, inflationStr,
                renominationStr, epochStr);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public void transfer(List<String> inputs, List<OutpointCoins> allocate, String invoice, String prototype_psbt, String consignment_file, String transaction_file) throws RuntimeException {
        try {
            final String inputsStr = mapper.writeValueAsString(inputs);
            final String allocateStr = mapper.writeValueAsString(allocate);
            rgb.rgb_node_fungible_transfer(this.runtime, inputsStr, allocateStr, invoice, prototype_psbt, consignment_file, transaction_file);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public String assetAllocations(String contractId) throws RuntimeException {
        try {
            return rgb.rgb_node_fungible_asset_allocations(this.runtime, contractId);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public String outpointAssets(String outpoint) throws RuntimeException {
        try {
            return rgb.rgb_node_fungible_outpoint_assets(this.runtime, outpoint);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
}
