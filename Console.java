public class Console {

    public static native String updateChainConfig(String input);

    static {
        System.loadLibrary("console_dylib");
    }

    public static void main(String[] args) {
        System.out.println(Console.updateChainConfig("{}"));
    }
}
