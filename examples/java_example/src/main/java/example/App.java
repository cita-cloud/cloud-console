package example;

import com.cita.cloud.ConsoleV1;

public class App {

    public static void main(String[] args) {
        System.out.println(ConsoleV1.updateChainConfig("{}"));
        System.out.println(ConsoleV1.setBlockInterval("localhost:50004", "", "", "4"));
        // String[] validators = {"", "", ""};
        // System.out.println(ConsoleV1.updateValidators("localhost:50004", "", "", validators));
        System.out.println(ConsoleV1.emergencyBrake("localhost:50004", "", "", "off"));
        System.out.println(ConsoleV1.setQuotaLimit("localhost:50004", "", "", "1073741716"));
        // System.out.println(ConsoleV1.updateAdmin("localhost:50004", "", "", ""));
    }
}
