using System;

public class Kata
{
    public static int CountBits(int n) {
        int count = 0;
        while (n > 0) {
            count += n & 1;
            n >>= 1;
        }
        return count;
    }
}

Console.WriteLine(0 == Kata.CountBits(0));
Console.WriteLine(1 == Kata.CountBits(4));
Console.WriteLine(3 == Kata.CountBits(7));
Console.WriteLine(2 == Kata.CountBits(9));
Console.WriteLine(2 == Kata.CountBits(10));