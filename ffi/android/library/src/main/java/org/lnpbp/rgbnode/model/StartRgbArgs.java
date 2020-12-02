package org.lnpbp.rgbnode.model;

import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import com.fasterxml.jackson.databind.annotation.JsonNaming;

import java.util.HashMap;

@JsonNaming(PropertyNamingStrategy.SnakeCaseStrategy.class)
public class StartRgbArgs {
    private final String network;
    private final String datadir;

    public StartRgbArgs(String network, String datadir) {
        this.network = network;
        this.datadir = datadir;
    }

    public String getNetwork() {
        return network;
    }

    public String getDatadir() {
        return datadir;
    }
}
