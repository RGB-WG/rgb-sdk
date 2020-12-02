package org.lnpbp.rgbnode;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;

import org.lnpbp.rgbnode.model.IssueArgs;
import org.lnpbp.rgbnode.model.StartRgbArgs;
import org.lnpbp.rgbnode.model.TransferArgs;
import org.lnpbp.rgbnode_autogen.COpaqueStruct;
import org.lnpbp.rgbnode_autogen.rgb_node;

import java.util.HashMap;
import java.util.List;

public class Runtime {
    private final COpaqueStruct runtime;
    private final ObjectMapper mapper;

    public Runtime(final String network, final String datadir) throws RuntimeException {
        mapper = new ObjectMapper();

        final StartRgbArgs args = new StartRgbArgs(network, datadir);
        try {
            this.runtime = rgb_node.run_rgb_embedded(network, datadir);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public void issue(final String network, final String ticker, final String name, final String description, final String issueStructure, final List<IssueArgs.CoinAllocation> allocations, final Integer precision, final List<IssueArgs.SealSpec> pruneSeals) throws RuntimeException {
        final IssueArgs args = new IssueArgs(network, ticker, name, description, issueStructure, allocations, precision, pruneSeals);
        try {
            final String jsonArgs = mapper.writeValueAsString(args);
            rgb_node.issue(this.runtime, jsonArgs);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public void transfer(List<String> inputs, List<IssueArgs.CoinAllocation> allocate, String invoice, String prototype_psbt, String consignment_file, String transaction_file) throws RuntimeException {
        final TransferArgs args = new TransferArgs(inputs, allocate, invoice, prototype_psbt, consignment_file, transaction_file);
        try {
            final String inputsStr = mapper.writeValueAsString(inputs);
            final String allocateStr = mapper.writeValueAsString(allocate);
            rgb_node.transfer(this.runtime, inputsStr, allocateStr, invoice, prototype_psbt, consignment_file, transaction_file);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
}
