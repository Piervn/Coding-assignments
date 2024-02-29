public static class Kata
{
    public static int sumTwoSmallestNumbers(int[] numbers) {
        return numbers.OrderBy(i => i).Take(2).Sum();
    }
}

int[] numbers1 = { 5, 8, 12, 19, 22 };
Console.WriteLine(13 == Kata.sumTwoSmallestNumbers(numbers1));

int[] numbers2 = { 19, 5, 42, 2, 77 };
Console.WriteLine(7 == Kata.sumTwoSmallestNumbers(numbers2));

int[] numbers3 = { 10, 343445353, 3453445, 2147483647 };
Console.WriteLine(3453455 == Kata.sumTwoSmallestNumbers(numbers3));