package example;

import com.cita.cloud.ConsoleV1;

public class App {

    public static void main(String[] args) {
        System.out.println(ConsoleV1.updateChainConfig("{}"));
        System.out.println(ConsoleV1.setBlockInterval("localhost:50004", "", "", "4"));
        String[] validators = { "79803604a6a6e0fc00291e8b9e1ef3f20af1af59", "e963afe7b072b3346fd0ecccdfd907f0984942af",
                "4a7c4fd1270b0e1ef1916fcb77666e793358327c" };
        System.out.println(ConsoleV1.updateValidators("localhost:50004", "", "", validators));
        System.out.println(ConsoleV1.emergencyBrake("localhost:50004", "", "", "off"));
        System.out.println(ConsoleV1.setQuotaLimit("localhost:50004", "", "", "1073741716"));
        System.out.println(ConsoleV1.updateAdmin("localhost:50004", "", "", "e963afe7b072b3346fd0ecccdfd907f0984942af"));
    }
}
