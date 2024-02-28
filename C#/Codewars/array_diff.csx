using System.Linq;

public class Kata
{
    public static int[] ArrayDiff(int[] a, int[] b) {
        var set = new HashSet<int>(b);
        return a.Where(x => !set.Contains(x)).ToArray();
    }
}

Console.WriteLine(new int[] { 2 }      .SequenceEqual(Kata.ArrayDiff(new int[] { 1, 2 },    new int[] { 1 })));
Console.WriteLine(new int[] { 2, 2 }   .SequenceEqual(Kata.ArrayDiff(new int[] { 1, 2, 2 }, new int[] { 1 })));
Console.WriteLine(new int[] { 1 }      .SequenceEqual(Kata.ArrayDiff(new int[] { 1, 2, 2 }, new int[] { 2 })));
Console.WriteLine(new int[] { 1, 2, 2 }.SequenceEqual(Kata.ArrayDiff(new int[] { 1, 2, 2 }, new int[] { })));
Console.WriteLine(new int[] { }        .SequenceEqual(Kata.ArrayDiff(new int[] { },         new int[] { 1, 2 })));
Console.WriteLine(new int[] { 3 }      .SequenceEqual(Kata.ArrayDiff(new int[] { 1, 2, 3 }, new int[] { 1, 2 })));