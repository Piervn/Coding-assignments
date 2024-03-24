public class Solution
{
    public bool IsHappy(int n) {
        HashSet<int> set = new HashSet<int>();
        while (true) {
            int sum = 0;
            while (n > 0) {
                int digit = n % 10;
                sum += digit * digit;
                n /= 10;
            }
            if (sum == 1) return true;
            if (set.Contains(sum)) return false;
            set.Add(sum);
            n = sum;
        }
    }
}