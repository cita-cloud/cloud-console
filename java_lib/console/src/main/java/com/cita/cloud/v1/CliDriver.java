package com.cita.cloud.v1;

public class CliDriver {

    String controllerAddr;

    String cryptoAddr;

    String adminAddr;

    public CliDriver(String controllerAddr, String cryptoAddr, String adminAddr) {
        this.controllerAddr = controllerAddr;
        this.cryptoAddr = cryptoAddr;
        this.adminAddr = adminAddr;
    }

    public String updateAdmin(String new_admin) {
        return Console.updateAdmin(this.controllerAddr, this.cryptoAddr, this.adminAddr, new_admin);
    }

    public String setBlockInterval(String block_interval) {
        return Console.setBlockInterval(this.controllerAddr, this.cryptoAddr, this.adminAddr, block_interval);
    }

    public String updateValidators(String[] validators) {
        return Console.updateValidators(this.controllerAddr, this.cryptoAddr, this.adminAddr, validators);
    }

    public String emergencyBrake(String on_off) {
        return Console.emergencyBrake(this.controllerAddr, this.cryptoAddr, this.adminAddr, on_off);
    }

    public String setQuotaLimit(String quota_limit) {
        return Console.setQuotaLimit(this.controllerAddr, this.cryptoAddr, this.adminAddr, quota_limit);
    }

}
