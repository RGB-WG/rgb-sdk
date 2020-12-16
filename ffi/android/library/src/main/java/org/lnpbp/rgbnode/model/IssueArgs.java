package org.lnpbp.rgb.model;

import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import com.fasterxml.jackson.databind.annotation.JsonNaming;

import java.util.HashSet;
import java.util.List;

@JsonNaming(PropertyNamingStrategy.SnakeCaseStrategy.class)
public class IssueArgs {
    private final String network;
    private final String ticker;
    private final String name;
    private final String description;
    private final Integer precision;
    private final List<OutpointCoins> allocations;
    private final HashSet<OutpointCoins> inflation;
    private final OutPoint renomination;
    private final OutPoint epoch;

    public IssueArgs(String network, String ticker, String name, String description, Integer precision, List<OutpointCoins> allocations, HashSet<OutpointCoins> inflation, OutPoint renomination, OutPoint epoch) {
        this.network = network;
        this.ticker = ticker;
        this.name = name;
        this.description = description;
        this.precision = precision;
        this.allocations = allocations;
        this.inflation = inflation;
        this.renomination = renomination;
        this.epoch = epoch;
    }

    public String getNetwork() {
        return network;
    }

    public String getTicker() {
        return ticker;
    }

    public String getName() {
        return name;
    }

    public String getDescription() {
        return description;
    }

    public Integer getPrecision() {
        return precision;
    }

    public List<OutpointCoins> getAllocations() {
        return allocations;
    }

    public HashSet<OutpointCoins> getInflation() {
        return inflation;
    }

    public OutPoint getRenomination() {
        return renomination;
    }

    public OutPoint getEpoch() {
        return epoch;
    }

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
}
