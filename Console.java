public class Console {
    public static native String hello(String input);
    public static native String config(String input);

    static {
        System.loadLibrary("console");
    }

    public static void main(String[] args) {
        System.out.println(Console.hello("CITA"));
        System.out.println(Console.config("CONFIG"));
    }
}
