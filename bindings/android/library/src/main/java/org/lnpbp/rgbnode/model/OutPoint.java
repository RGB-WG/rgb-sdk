package org.lnpbp.rgb.model;

import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import com.fasterxml.jackson.databind.annotation.JsonNaming;

import java.util.HashSet;
import java.util.List;

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
