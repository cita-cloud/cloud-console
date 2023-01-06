package com.cita.cloud.v1;

public class CacheDriver {
    String cacheAddr;

    public CacheDriver(String cacheAddr) {
        this.cacheAddr = cacheAddr;
    }

    public String getBlockNumber() {
        return Console.getBlockNumber(this.cacheAddr);
    }

    public String getSystemConfig() {
        return Console.getSystemConfig(this.cacheAddr);
    }

    public String getAbi(String address) {
        return Console.getAbi(this.cacheAddr, address);
    }

    public String getAccountNonce(String address) {
        return Console.getAccountNonce(this.cacheAddr, address);
    }

    public String getBalance(String address) {
        return Console.getBalance(this.cacheAddr, address);
    }

    public String getBlockHash(String block_number) {
        return Console.getBlockHash(this.cacheAddr, block_number);
    }

    public String getBlock(String hash_or_height) {
        return Console.getBlock(this.cacheAddr, hash_or_height);
    }

    public String getCode(String address) {
        return Console.getCode(this.cacheAddr, address);
    }

    public String getReceipt(String hash) {
        return Console.getReceipt(this.cacheAddr, hash);
    }

    public String getTx(String hash) {
        return Console.getTx(this.cacheAddr, hash);
    }

    public String call(String data, String from, String height, String to) {
        return Console.call(this.cacheAddr, data, from, height, to);
    }

    public String create(String block_count, String data, String value) {
        return Console.create(this.cacheAddr, block_count, data, value);
    }

    public String sendTx(String block_count, String data, String to, String value) {
        return Console.sendTx(this.cacheAddr, block_count, data, to, value);
    }


}
