package org.lnpbp.rgb.model;

import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import com.fasterxml.jackson.databind.annotation.JsonNaming;

import java.util.HashSet;
import java.util.List;

@JsonNaming(PropertyNamingStrategy.SnakeCaseStrategy.class)
public static class OutpointCoins {
    private final Long coins;
    private final String outpoint;

    public OutpointCoins(Long coins, String outpoint) {
        this.coins = coins;
        this.outpoint = outpoint;
    }

    public Long getCoins() {
        return coins;
    }

    public String getOutpoint() {
        return outpoint;
    }
}

public static class OutPoint {
    private final Integer vout;
    private final String txid;

    public OutPoint(Integer vout, String txid) {
        this.vout = vout;
        this.txid = txid;
    }

    public Integer getVout() {
        return vout;
    }

    public String getTxid() {
        return txid;
    }
}
