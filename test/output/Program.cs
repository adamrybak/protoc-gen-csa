namespace TestApp;

public class Program
{
    public static void Main()
    {
        Console.WriteLine(DateTime.Now.ToString("O"));
        Console.WriteLine(DateTime.UtcNow.ToString("O"));
        Console.WriteLine(DateTimeOffset.Now.ToString("O"));
        Console.WriteLine(DateTimeOffset.UtcNow.ToString("O"));
        Console.WriteLine(DateOnly.FromDateTime(DateTime.Now).ToString("O"));
        Console.WriteLine(TimeOnly.FromDateTime(DateTime.Now).ToString("O"));
        Console.WriteLine(new TimeSpan(3, 10, 6, 33, 123).ToString());
    }
}