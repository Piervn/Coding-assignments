public class Solution
{
    public static IList<int> GetRow(int rowIndex) {
        int[] res = new int[rowIndex + 1];
        res[0] = 1;
        for (int i = 1; i <= rowIndex; i++) {
            for (int j = i; j >= 1; j--) {
                res[j] += res[j - 1];
            }
        }
        return res;
    }
}

var res1 = Solution.GetRow(3);
var res2 = Solution.GetRow(0);
var res3 = Solution.GetRow(1);

Console.WriteLine(string.Join(", ", res1));
Console.WriteLine(string.Join(", ", res2));
Console.WriteLine(string.Join(", ", res3));