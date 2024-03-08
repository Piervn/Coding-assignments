public class Solution
{
    public static bool IsPalindrome(string s) {
        s = new string(s.Where(Char.IsLetterOrDigit).ToArray()).ToLower();
        for (int i = 0; i < s.Length / 2; i++) {
            if (s[i] != s[s.Length - 1 - i]) {
                return false;
            }
        }
        return true;
    }
}

var test1 = "A man, a plan, a canal: Panama";
var test2 = "race a car";

Console.WriteLine(Solution.IsPalindrome(test1));
Console.WriteLine(Solution.IsPalindrome(test2));