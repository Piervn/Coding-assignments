public class Solution
{
    public static int MaxProfit(int[] prices) {
        int maxProfit = 0;
        int min = prices[0];
        foreach (int price in prices) {
            if (price < min) min = price;
            else maxProfit = Math.Max(maxProfit, price - min);
        }
        return maxProfit;
    }
}

bool test1 = 5 == Solution.MaxProfit([7, 1, 5, 3, 6, 4]);
bool test2 = 0 == Solution.MaxProfit([7, 6, 4, 3, 1]);

Console.WriteLine(test1);
Console.WriteLine(test2);