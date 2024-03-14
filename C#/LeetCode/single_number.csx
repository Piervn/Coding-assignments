public class Solution
{
    public static int SingleNumber(int[] nums) {
       return nums.Aggregate((a, b) => a ^ b); 
    }
}

Console.WriteLine(Solution.SingleNumber(new int[] { 2, 2, 1 }));
Console.WriteLine(Solution.SingleNumber(new int[] { 4, 1, 2, 1, 2 }));
Console.WriteLine(Solution.SingleNumber(new int[] { 1 }));