public class Console {
    public static native String hello(String input);

    static {
        System.loadLibrary("console");
    }

    public static void main(String[] args) {

        String output = Console.hello("CITA");
        System.out.println(output);
    }
}
